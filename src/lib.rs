#[macro_use]
extern crate vst;
extern crate hound;
extern crate nfd;

use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use hound;
use nfd::Response;

#[derive(Default)]
struct Granulizor {
    grain_size: f32,
    source_samples: Vec<f32>,
}

impl Default for Granulizor {
    fn default() -> Granulizor {
        let mut sample_select = String::new();
        while sample_select.is_empty() {
            let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
                panic!(e);
            });
            match result {
                Response::Okay(file_path) => sample_select = file_path,
                Response::OkayMultiple(files) => println!("You Must Select a Single File"),
                Response::Cancel => println!("Please Select a Sample"),
            }
        }
        let mut reader = hound::WavReader::open(sample_select).unwrap();
        Granulizor {
            grain_size: 20_f32,
            source_samples: reader.samples::<i32>();
            }),
        }
    }
}

impl Plugin for Granulizor {
    fn get_info(&self) -> Info {
        Info {
            name: "Granulizor".to_string(),
            unique_id: 3332,
            inputs: 0,
            outputs: 2,
            parameters: 1,
            category: Category::Synth,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        
    }
}

plugin_main!(Whisper); 