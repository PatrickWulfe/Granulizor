#[macro_use]
extern crate vst;
extern crate sample;
extern crate instrument;
extern crate find_folder;
pub extern crate hound;

pub mod wav_parser;
pub mod pitcher;

use vst::plugin::{Info, Plugin, Category, CanDo};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::api::{Supported, Events};
use wav_parser::StereoFrame;
use std::path::PathBuf;

pub struct Granulizor{
    grain_size_control: f64,
    sample_start_control: f64,
    pitch_toggle: f64,
    sample_select_control: f32,
    assets_path: PathBuf,
    frames: Vec<StereoFrame>,
    sample_rate: f64,
    time: usize,
    note: Option<u8>,
    repitched_sample: Vec<StereoFrame>,
}


impl Granulizor {
    fn init_assets_path(&mut self) {
        // The find_folder crate does not work with VSTs, I think this has to do with how the DAW or VST host loads the dll file
        // For now, the "assets" folder location must be hard coded in.
        //let mut dll_folder = std::env::current_dir().unwrap();
        //self.assets_path = find_folder::Search::ParentsThenKids(5, 5).of(dll_folder).for_folder("assets").unwrap();
        self.assets_path = PathBuf::from("E:\\Devel\\Repositories\\School\\granulizor\\assets\\pads.wav");
    }

    fn set_sample(&mut self) {
        if self.sample_select_control < 0.5 {
            self.assets_path.set_file_name("pads.wav");
        } else {
            self.assets_path.set_file_name("plucks.wav"); // Will add more, added 2nd for proof of concept
        }
        self.frames = wav_parser::parse_wav(self.assets_path.clone()).unwrap();
    }

    fn get_hz(&self) -> f64 {
        // Returns the frequency of the sample based on number of samples and sample rate
        (self.frames.len() as f64 / 44.1) / 1000.0
    }

    fn midi_pitch_to_hz(pitch: u8) -> f64 { // From vst crate's sine synth example
        const A4_PITCH: i8 = 69;
        const A4_FREQ: f64 = 440.0;

        // Midi notes can be 0-127
        ((f64::from(pitch as i8 - A4_PITCH)) / 12.).exp2() * A4_FREQ
    }

    fn init_pitched_samples(&mut self) {
        let grain_len = self.get_grain_size();
        let grain_start = self.get_grain_start();
        let mut to_pitch = Vec::new();
        for i in grain_start..(grain_start + grain_len) {
            to_pitch.push(self.frames[i].copy());
        }
        self.repitched_sample = pitcher::repitch(self.get_hz(), Granulizor::midi_pitch_to_hz(self.note.unwrap()), to_pitch);
    }

    // Process incoming midi event
    fn process_midi_event(&mut self, data: [u8; 3]) {
        match data[0] {
            128 => self.note_off(data[1]),
            144 => self.note_on(data[1]),
            _ => (),
        }
    }

    fn note_on(&mut self, note: u8) {
        self.note = Some(note);
        self.init_pitched_samples();
    }

    fn note_off(&mut self, note: u8) {
        if self.note == Some(note) {
            self.note = None;
        }
    }

    fn get_grain_size(&self) -> usize {
        // Set the minimum grain size to 20 ms and the max to either the sample length or 0.5 seconds
        std::cmp::max(20, ((std::cmp::min(self.frames.len(), ((self.sample_rate/1000.0) * 500.0) as usize) as f64) * self.grain_size_control) as usize)
    }

    fn get_grain_start(&self) -> usize {
        std::cmp::min(self.frames.len() - self.get_grain_size(), (self.frames.len() as f64 * self.sample_start_control) as usize)
    }
}

impl Default for Granulizor {
    fn default() -> Granulizor {
        let mut g = Granulizor {
            grain_size_control: 1_f64,
            sample_start_control: 0.0,
            pitch_toggle: 0.0,
            sample_select_control: 0.0,
            assets_path: PathBuf::new(),
            frames: Vec::new(),
            sample_rate: 44100.0,
            time: 0,
            note: None,
            repitched_sample: Vec::new(),
        };
        g.init_assets_path();
        g.set_sample();
        g
    }
}

impl Plugin for Granulizor {
    fn get_info(&self) -> Info {
        Info {
            name: "Granulizor".to_string(),
            unique_id: 102090,
            inputs: 2,
            outputs: 2,
            parameters: 4,
            category: Category::Synth,
            initial_delay: 0,
            ..Info::default()
        }
    }

    // Functions for parameters
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.grain_size_control as f32,
            1 => self.sample_start_control as f32,
            2 => self.pitch_toggle as f32,
            3 => self.sample_select_control,
            _ => 0.0,
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        match index {
            0 => self.grain_size_control = value as f64,
            1 => self.sample_start_control = value.min(0.99) as f64,
            2 => self.pitch_toggle = value as f64,
            3 => {
                self.sample_select_control = value;
                self.set_sample();
            },
            _ => (),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String{
        match index {
            0 => "Grain Size".to_string(),
            1 => "Sample Start".to_string(),
            2 => "Toggle Pitched Mode".to_string(),
            3 => "Sample Select".to_string(),
            _ => "".to_string(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{}", self.grain_size_control * 130.0 + 20.0),
            1 => format!("{}", self.sample_start_control * 100.0),
            2 => if self.pitch_toggle > 0.5 {
                "On".to_string()
            } else {
                "Off".to_string()
            },
            3 => {
                match self.assets_path.file_name() {
                    Some(sample) => format!("{:?}", sample),
                    None => "None".to_string(),
                }
            },
            _ => "".to_string(),
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "ms".to_string(),
            1 => "%".to_string(),
            _ => "".to_string(),
        }
    }

    // Function for handling events
    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            match event {
                Event::Midi(ev) => self.process_midi_event(ev.data),
                _ => (),
            }
        }
    }

    // Function for signal processing
    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {

        let (inputs, mut outputs) = buffer.split();
        if outputs.len() != 2 {
            return;
        }
        let (mut l, mut r) = outputs.split_at_mut(1);
        let stereo_out = l[0].iter_mut().zip(r[0].iter_mut());
            let pitched_len = self.repitched_sample.len();

        for (left_out, right_out) in stereo_out {
            if let Some(current_note) = self.note {
                if self.pitch_toggle > 0.5 {
                    *left_out = self.repitched_sample[(self.time % pitched_len)].get_left();
                    *right_out = self.repitched_sample[(self.time % pitched_len)].get_right();
                }
                else {
                    *left_out = self.frames[(self.time % self.get_grain_size()) + self.get_grain_start()].get_left();
                    *right_out = self.frames[(self.time % self.get_grain_size()) + self.get_grain_start()].get_right();
                }
                self.time += 1_usize;
            } else {
                *left_out = 0.0;
                *right_out = 0.0;
                self.time = 0;
            }
        }
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }
}

plugin_main!(Granulizor);