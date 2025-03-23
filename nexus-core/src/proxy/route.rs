use crate::proxy::gateway::Ctx;
use arc_swap::ArcSwap;
use async_trait::async_trait;
use pingora::server::ShutdownWatch;
use pingora::services::background::BackgroundService;
use std::any::Any;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;

pub struct Route {
    id: String,

    predicate: Box<dyn Predicate>,

    plugins: Vec<Box<dyn Plugin<Config = dyn Any>>>,
}

pub trait Plugin: Sync + Send {
    type Config;
}

pub trait Predicate: Sync + Send {
    fn check(&self, ctx: Ctx) -> bool;
}

// 作为一个 background service 放进去
#[async_trait]
pub trait RouteLocator: Sync + Send {
    /// 全量查询路由信息
    async fn get_routes(&self) -> Vec<Route>;
}

pub struct RouteManager<T>
where
    T: RouteLocator,
{
    inner: Box<T>,
    routes: ArcSwap<Vec<Route>>,
}

impl<T: RouteLocator> RouteManager<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Box::new(inner),
            routes: Default::default(),
        }
    }

    pub fn match_route(&self, path: &str) -> Option<Route> {
        todo!()
    }
}

#[async_trait]
impl RouteLocator for Box<dyn RouteLocator> {
    async fn get_routes(&self) -> Vec<Route> {
        // 委托给内部的实现
        (**self).get_routes().await
    }
}

#[async_trait]
impl<T> BackgroundService for RouteManager<T>
where
    T: RouteLocator,
{
    async fn start(&self, mut shutdown: ShutdownWatch) {
        let mut period = interval(Duration::from_secs(5));
        loop {
            tokio::select! {
                biased;
                _ = period.tick() => {
                    let routes = self.inner.get_routes().await;
                    self.routes.store(Arc::new(routes));
                }
                _ = shutdown.changed() => {
                    break;
                }
            }
        }
    }
}
