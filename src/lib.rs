#[macro_use]
extern crate vst;

use vst::prelude::*;

struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn new(_host: HostCallback) -> Self {
        BasicPlugin
    }

    fn get_info(&self) -> Info {
        Info {
            name: "Basic Plugin".to_string(),
            unique_id: 1357,
            inputs: 0,
            outputs: 2,
            category: Category::Synth,
            ..Default::default()
        }
    }
}

plugin_main!(BasicPlugin);