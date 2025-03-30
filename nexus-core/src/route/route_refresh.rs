use crate::route::route_conf_locator::RouteConfLocator;
use crate::route::{Route, RouteStore};
use async_trait::async_trait;
use matchit::Router;
use pingora::server::ShutdownWatch;
use pingora::services::background::BackgroundService;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;

pub struct RouteRefresh<T>
where
    T: RouteConfLocator,
{
    locators: T,
    route_store: Arc<RouteStore>,
}

impl<T> RouteRefresh<T>
where
    T: RouteConfLocator,
{
    pub fn new(locators: T, route_store: Arc<RouteStore>) -> RouteRefresh<T> {
        RouteRefresh {
            locators,
            route_store,
        }
    }

    async fn load_route(&self) {
        let mut router = Router::new();
        let route_conf = self.locators.get_routes().await;

        let routes: Vec<Route> = route_conf
            .into_iter()
            .map(|conf| Route::try_from(conf).unwrap())
            .collect();

        for route in routes {
            router.insert(route.path.clone(), route).unwrap();
        }
        self.route_store.router.store(Arc::new(router));
    }
}

#[async_trait]
impl<T> BackgroundService for RouteRefresh<T>
where
    T: RouteConfLocator,
{
    async fn start(&self, mut shutdown: ShutdownWatch) {
        let mut period = interval(Duration::from_secs(15));
        loop {
            tokio::select! {
                biased;
                _ = period.tick() => {
                    self.load_route().await;
                }
                _ = shutdown.changed() => {
                    break;
                }
            }
        }
    }
}
