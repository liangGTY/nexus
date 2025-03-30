use crate::route::RouteConf;
use async_trait::async_trait;

// 作为一个 background service 放进去
#[async_trait]
pub trait RouteConfLocator: Sync + Send {
    /// 这里应该是加载 RouteConf 而不是直接加载 Route 吧 ？？？？？
    async fn get_routes(&self) -> Vec<RouteConf>;
}
