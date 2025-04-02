pub mod route_conf_locator;
pub mod route_refresh;
pub mod static_file_conf_locator;

use crate::plugin::{Plugin, PluginConf, PLUGIN_FACTORY};
use crate::proxy::gateway::Ctx;
use arc_swap::ArcSwap;
use matchit::{Match, MatchError, Router};
use pingora::http::RequestHeader;
use serde::Deserialize;
use std::sync::Arc;
use pingora::cache::cache_control::Cacheable::No;

pub struct Route {
    path: String,

    // predicate: Box<dyn Predicate>,
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl TryFrom<RouteConf> for Route {
    type Error = ();

    fn try_from(conf: RouteConf) -> Result<Self, Self::Error> {
        let plugins = conf
            .plugins
            .iter()
            .map(|plugin_conf| PLUGIN_FACTORY.create(plugin_conf.clone()).unwrap())
            .collect();
        Ok(Route {
            path: conf.path.clone(),
            plugins,
        })
    }
}

#[derive(Deserialize)]
pub struct RouteConf {
    path: String,
    plugins: Vec<PluginConf>,
}

pub trait Predicate: Sync + Send {
    fn check(&self, ctx: Ctx) -> bool;
}

pub struct RouteStore {
    router: ArcSwap<Router<Arc<Route>>>,
}

impl RouteStore {
    pub fn new() -> Self {
        Self {
            router: Default::default(),
        }
    }

    pub fn match_route(&self, req: &mut RequestHeader, ctx: &mut Ctx) -> Option<Arc<Route>> {
        let guard = self.router.load();
        let result = guard.at(req.uri.path());

        match result {
            Ok(value) => {
                ctx.route = Some(value.value.clone());
                // ctx.path_param = Some(value.params);
                Some(value.value.clone())
            }
            Err(_) => {
                None
            }
        }
    }
}
