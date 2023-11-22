use std::ffi::OsStr;
use std::any::Any;
use libloading::{Library, Symbol};
use log::*;
pub use y4m::Frame;

pub trait Plugin: Any + Send + Sync {
    /// Get a name describing the `Plugin`.
    fn name(&self) -> &'static str;
    /// A callback fired immediately after the plugin is loaded. Usually used 
    /// for initialization.
    fn on_plugin_load(&self) {}
    /// A callback fired immediately before the plugin is unloaded. Use this if
    /// you need to do any cleanup.
    fn on_plugin_unload(&self) {}
    /// A callback fired when then next frame is availiable from the previous filter
    fn get_frame(&self, frame: Frame, frame_ctx: FrameContext) -> Vec<Frame>;
}

pub struct FrameContext {
    pub num: usize,
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

pub struct PluginManager {
    pub plugins: Vec<Box<dyn Plugin>>,
    loaded_libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
            loaded_libraries: Vec::new(),
        }
    }

    pub unsafe fn load_plugin<P: AsRef<OsStr>>(&mut self, filename: P) -> Result<(),PluginLoadError> {
        type PluginCreate = unsafe fn() -> *mut dyn Plugin;

        let lib = Library::new(filename.as_ref()).unwrap();

        // We need to keep the library around otherwise our plugin's vtable will
        // point to garbage. We do this little dance to make sure the library
        // doesn't end up getting moved.
        self.loaded_libraries.push(lib);

        let lib = self.loaded_libraries.last().unwrap();

        let constructor: Symbol<PluginCreate> = lib.get(b"_plugin_create")
            .unwrap();
        let boxed_raw = constructor();
        let plugin = Box::from_raw(boxed_raw);
        plugin.on_plugin_load();
        debug!("Loaded {:?} plugin", plugin.name());
        self.plugins.push(plugin);
        Ok(())
    }

    /// Unload all plugins and loaded plugin libraries, making sure to fire 
    /// their `on_plugin_unload()` methods so they can do any necessary cleanup.
    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for plugin in self.plugins.drain(..) {
            trace!("Firing on_plugin_unload for {:?}", plugin.name());
            plugin.on_plugin_unload();
        }

        for lib in self.loaded_libraries.drain(..) {
            drop(lib);
        }
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() || !self.loaded_libraries.is_empty() {
            self.unload();
        }
    }
}

#[derive(Debug)]
pub enum PluginLoadError {
    CreateFail,
    LoadFail
}