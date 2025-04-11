mod request_transformer;

use crate::proxy::gateway::Ctx;
use async_trait::async_trait;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use pingora::prelude::Session;
use serde_json::Value;

pub type PluginConf = Value;

type NewPlugin = dyn Fn(PluginConf) -> Result<Box<dyn Plugin>, ()> + Send + Sync;

#[macro_export]
macro_rules! register_plugin {
    ($name:ident, $ty:ty) => {
        #[ctor::ctor]
        fn register_plugin_ctor() {
            PLUGIN_FACTORY.register(stringify!($name), |params| Ok(Box::new(<$ty>::new(params))));
        }
    };
}

#[async_trait]
pub trait Plugin: Sync + Send {
    fn new(plugin_conf: Value) -> Self
    where
        Self: Sized;

    async fn request_filter(&self, _session: &mut Session, _ctx: &mut Ctx)
                            -> pingora::Result<bool>;

    async fn request_body_filter(
        &self,
        _session: &mut Session,
        _body: &mut Option<bytes::Bytes>,
        _end_of_stream: bool,
        _ctx: &mut Ctx,
    ) -> pingora::Result<()> {
        Ok(())
    }
}

pub static PLUGIN_FACTORY: Lazy<PluginFactory> = Lazy::new(PluginFactory::new);

pub struct PluginFactory {
    plugins: DashMap<String, Box<NewPlugin>>,
}

impl PluginFactory {
    pub fn new() -> Self {
        Self {
            plugins: DashMap::new(),
        }
    }

    /// Register a new plugin creator function
    pub fn register<F>(&self, category: &str, creator: F)
    where
        F: Fn(Value) -> Result<Box<dyn Plugin>, ()> + Send + Sync + 'static,
    // F: NewPlugin,
    {
        self.plugins.insert(category.to_string(), Box::new(creator));
    }

    /// Create a new plugin instance by name
    pub fn create(&self, conf: Value) -> Result<Box<dyn Plugin>, ()> {
        let plugin_type = conf.get("type").and_then(|it| it.as_str()).unwrap();

        self.plugins.get(plugin_type).ok_or(()).and_then(|creator| {
            return creator(conf);
        })
    }
}
