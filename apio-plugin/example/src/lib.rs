use apio_plugin::{Plugin, declare_plugin, Frame};

#[derive(Debug,Default)]
struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn name(&self) -> &'static str {
        "Example"
    }

    fn get_frame(&self, frame: apio_plugin::Frame, frame_ctx: apio_plugin::FrameContext) -> Vec<apio_plugin::Frame> {
        vec![Frame::new([&[],&[],&[]], None)]
    }
}

declare_plugin!{ExamplePlugin, ExamplePlugin::default}