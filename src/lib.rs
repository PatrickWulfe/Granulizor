#[macro_use]
extern crate vst;
extern crate hound;

use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use vst::api::FileSelect;
use hound;

#[derive(Default)]
struct Granulizer;

impl Plugin for Granulizer {
    fn get_info(&self) -> Info {
        Info {
            name: "Granulizor".to_string(),
            unique_id: 3332,
            inputs: 0,
            outputs: 2,
            category: Category::Synth,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        
    }
}

plugin_main!(Whisper); 