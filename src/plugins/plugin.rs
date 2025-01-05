pub trait Plugin {
    fn on_event(&self, event: &str);
    fn register(&self);
}

pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
#[allow(dead_code)]
    pub fn new() -> Self {
        PluginRegistry { plugins: vec![] }
    }

#[allow(dead_code)]
    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        plugin.register();
        self.plugins.push(plugin);
    }

#[allow(dead_code)]
    pub fn notify_plugins(&self, event: &str) {
        for plugin in &self.plugins {
            plugin.on_event(event);
        }
    }
}

