use std::{ffi::OsStr, error::Error, collections::HashMap};
use libloading::{Library, Symbol};
use log::*;
use apio_plugin::*;

pub struct PluginManager {
    plugins: HashMap<String,Box<dyn Plugin>>,
    loaded_libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: HashMap::new(),
            loaded_libraries: Vec::new(),
        }
    }

    pub fn get_plugin(&self, name: &str) -> Option<&Box<dyn Plugin>> {
        self.plugins.get(name)
    }

    /// # Safety
    /// Execution of extenal code `_plugin_create` symbol
    /// 
    /// Must trust code and ensure it is memory safe
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
        self.plugins.insert(plugin.name().to_owned(), plugin);
        Ok(())
    }

    /// Unload all plugins and loaded plugin libraries, making sure to fire 
    /// their `on_plugin_unload()` methods so they can do any necessary cleanup.
    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for plugin in self.plugins.drain() {
            trace!("Firing on_plugin_unload for {:?}", plugin.1.name());
            plugin.1.on_plugin_unload();
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

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum PluginLoadError {
    CreateFail,
    LoadFail
}


#[derive(Debug, Clone)]
/// Directional FilterGraph
pub struct FilterGraph<'a> {
    chains: Vec<Vec<(&'a str,Option<&'a str>)>>
}

impl<'a> FilterGraph<'a> {
   pub(crate) fn new(chains: Vec<Vec<(&'a str,Option<&'a str>)>>) -> Self {
        Self { chains }
   }
}

pub struct FilterNode<'a> {
    call_fn: fn(frame: Frame, frame_ctx: FrameContext, args: Option<&str>) -> Vec<Frame<'a>>,
    args: Option<&'a str>
}

/// Parse the filter graph
/// 
/// # Example
/// `"[padin,padin2]filter{key=value1:key=value}[padout],filter2;[padout]filter3"`
pub fn parse_filtergraph(s: &str) -> Result<FilterGraph, Box<dyn Error + Send + Sync + 'static>> {
    let mut temp = Vec::<Vec<(&str,Option<&str>)>>::new();
    for chain in s.trim().split(';') {
        let mut bigchain = Vec::<(&str,Option<&str>)>::new();
        // Vec<(startpad, endpad, chain)>
        let mut smallchains = Vec::<(&str, &str, Vec<(&str,Option<&str>)>)>::new();
        let (inpad, outpad, chain) = parse_pads(chain);
        eprintln!("inpads: {:#?}", inpad);
        eprintln!("outpads: {:#?}", outpad);
        eprintln!("chain left: {}", chain);
        for filters in chain.trim().split(',') {
            let (filterinpad, filteroutpad, mut filters) = parse_pads(filters);
            let mut args: Option<&str> = None;
            if filters.contains("{") && filters.contains("}") {
                args = Some(&filters[filters.find('{').unwrap()+1..filters.find("}").unwrap()]);
                filters = &filters[..filters.find('{').unwrap()];
            }
            bigchain.push((filters, args))
        }
        eprintln!("{:#?}", bigchain);
        temp.push(bigchain);
    }
    Ok(FilterGraph::new(temp))
}

pub fn parse_pads(mut s: &str) -> (Vec<&str>, Vec<&str>, &str) {
    let mut inpad: Vec<&str> = Vec::new();
    let mut outpad:Vec<&str> = Vec::new();
    if s.starts_with('[') {
        inpad = s[1..s.find(']').unwrap()].split(':').map(|x| x.trim()).collect();
        s = &s[s.find(']').unwrap()+1..];
    }
    if s.ends_with(']') {
        outpad = s[s.rfind('[').unwrap()+1..s.len()-1].split(':').map(|x| x.trim()).collect();
        s = &s[..s.rfind(']').unwrap()];
    }
    return (inpad, outpad, s)
}

impl<'a> IntoIterator for FilterGraph<'a> {
    type Item = Vec<(&'a str,Option<&'a str>)>;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.chains.into_iter()
    }
}