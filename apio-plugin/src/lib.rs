use std::any::Any;
use y4m::{Colorspace, Ratio};
pub use y4m::Frame;

pub trait Plugin: Any + Send + Sync {
    /// Get a name describing the `Plugin`. Used in finding the plugin
    fn name(&self) -> &'static str;
    /// A callback fired immediately after the plugin is loaded. Usually used 
    /// for initialization.
    fn on_plugin_load(&self) {}
    /// A callback fired immediately before the plugin is unloaded. Use this if
    /// you need to do any cleanup.
    fn on_plugin_unload(&self) {}
    /// A callback fired when then next frame is availiable from the previous filter
    fn get_frame(&self, frame: &Frame, args: &Option<&str>) -> Vec<&Frame> {vec![]}
    /// If MultiInput feature enabled will 
    fn get_frames(&self, frames: Vec<&Frame>, args: Option<&str>) -> Vec<&Frame> {vec![]}
    /// Get the supported features for the plugin
    fn features(&self) -> Vec<PluginFeatures> {vec![]}
}

pub enum PluginFeatures {
    /// Accept multiple frames per input, the value is the number of frames
    MultiInput(u64),
    OpenCl,
}

pub trait PluginArgs: Any + Send + Sync {
    /// Parse the args from the input
    fn from_input(&self, input: Option<&str>) -> Self;
}

#[derive(Debug, Clone)]
pub enum ArgType {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String)
}

pub struct FrameContext {
    colourspace: Colorspace,
    framerate: Ratio,
    width: usize,
    height: usize,
    pixel_aspect: Ratio,
    bit_depth: usize,
    bytes_per_sample: usize,
}

impl FrameContext {
    pub fn get_colorspace(&self) -> Colorspace {
        self.colourspace
    }
    pub fn get_framerate(&self) -> Ratio {
        self.framerate
    }
    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }
    pub fn get_pixel_aspect(&self) -> Ratio {
        self.pixel_aspect
    }
    pub fn get_bit_depth(&self) -> usize {
        self.bit_depth
    }
    pub fn get_bytes_per_sample(&self) -> usize {
        self.bytes_per_sample
    }
}


/// Declare a plugin type and its constructor.
///
/// # Notes
///
/// This works by automatically generating an `extern "C"` function with a
/// pre-defined signature and symbol name. Therefore you will only be able to
/// declare one plugin per library.
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn $crate::Plugin {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<dyn $crate::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}