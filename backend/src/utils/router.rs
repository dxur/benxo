use axum::{routing::MethodRouter, Router};

pub trait RoutePacked<S> {
    fn route_packed(self, route: (&str, MethodRouter<S>)) -> Self;
    fn nest_packed(self, router: (&str, Router<S>)) -> Self;
}

impl<S: Clone + Send + Sync + 'static> RoutePacked<S> for Router<S> {
    fn route_packed(self, route: (&str, MethodRouter<S>)) -> Self {
        self.route(route.0, route.1)
    }

    fn nest_packed(self, router: (&str, Router<S>)) -> Self {
        self.nest(router.0, router.1)
    }
}
