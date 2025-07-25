use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/products/{id}"]
    ProductDetail(i32),
    #[to = "/"]
    HomePage,
}
