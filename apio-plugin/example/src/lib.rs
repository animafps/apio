use apio_plugin::{Plugin, declare_plugin, Frame};

#[derive(Debug,Default)]
struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn name(&self) -> &'static str {
        "example"
    }

    fn get_frame(&self, frame: &Frame, args: &Option<&str>) -> Vec<&Frame> {
        todo!()
    }
}

declare_plugin!{ExamplePlugin, ExamplePlugin::default}