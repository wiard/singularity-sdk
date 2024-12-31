pub trait Plugin {
    fn on_event(&self, event: &str);
    fn register(&self);
}

pub struct PluginRegistry {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        PluginRegistry { plugins: vec![] }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        plugin.register();
        self.plugins.push(plugin);
    }

    pub fn notify_plugins(&self, event: &str) {
        for plugin in &self.plugins {
            plugin.on_event(event);
        }
    }
}

