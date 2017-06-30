//! There are cases where you may only want specific routes within
//! a [`Router`](https://docs.rs/router/0.5.1/router/struct.Router.html) to use some piece of
//! middleware (e.g. routes that require the user to be logged in). Adding route-specific
//! middleware in Iron is a repetitive and messy business which is where `iron_middlefiddle` comes
//! in. It provides a convenience macro for adding one or more
//! [`BeforeMiddleware`](https://docs.rs/iron/0.5.1/iron/middleware/trait.BeforeMiddleware.html) or
//! [`AfterMiddleware`](https://docs.rs/iron/0.5.1/iron/middleware/trait.AfterMiddleware.html)s to
//! multiple routes at once.
//!
//! ## Example usage
//!
//! ```rust,no_run
//! #[macro_use]
//! extern crate iron_middlefiddle;
//! extern crate iron;
//! extern crate mount;
//! extern crate router;
//!
//! use iron_middlefiddle::Middleware;
//! use mount::Mount;
//! use router::Router;
//!
//! mod controllers;
//! mod middleware;
//!
//! fn main() {
//!     let mut frontend_router = Router::new();
//!
//!     // Add some `frontend_router` routes and the middleware they should use:
//!
//!     middlefiddle! {
//!         router => frontend_router,
//!         routes => {
//!             lorem: get "/lorem" => controllers::lorem::index,
//!             ipsum: get "/ipsum" => controllers::ipsum::index,
//!             dolor: get "/dolor" => controllers::dolor::index,
//!         },
//!         middleware => {
//!             Middleware::BeforeMiddleware => middleware::auth::TokenValidity,
//!         },
//!     };
//!
//!     // Add some more `frontend_router` routes that aren't going to need the middleware:
//!
//!     frontend_router.get("/amet", controllers::amet::index, "amet");
//!
//!     // The usual…
//!
//!     let mut mount = Mount::new();
//!     mount.mount("/", frontend_router)
//!
//!     Iron::new(mount).http("127.0.0.1:8888").unwrap();
//! }
//! ```

extern crate iron;

use iron::{Handler, IronResult, Request, Response, Chain};

/// Specifies the type of middleware you are passing to the routes.
///
/// ```rust,no_run
/// middlefiddle! {
///     router => some_router,
///     routes => {
///         // Some routes…
///     },
///     middleware => {
///         Middleware::BeforeMiddleware => middleware::SomeMiddleware,
///         Middleware::BeforeMiddleware => middleware::SomeOtherMiddleware,
///         Middleware::AfterMiddleware => middleware::SomeMoreMiddleware,
///     },
/// };
/// ```

pub enum Middleware {
    /// ```rust,no_run
    /// middlefiddle! {
    ///     router => some_router,
    ///     routes => {
    ///         // Some routes…
    ///     },
    ///     middleware => {
    ///         Middleware::BeforeMiddleware => middleware::SomeMiddleware,
    ///     },
    /// };
    /// ```

    BeforeMiddleware(Box<iron::BeforeMiddleware>),

    /// ```rust,no_run
    /// middlefiddle! {
    ///     router => some_router,
    ///     routes => {
    ///         // Some routes…
    ///     },
    ///     middleware => {
    ///         Middleware::AfterMiddleware => middleware::SomeMiddleware,
    ///     },
    /// };
    /// ```

    AfterMiddleware(Box<iron::AfterMiddleware>),
}

#[doc(hidden)]
pub struct Middlefiddle {
    chain: Chain,
}

#[doc(hidden)]
pub struct Route {
    pub id: Option<String>,
    pub method: String,
    pub route: Option<String>,
    pub handler: Option<Box<Handler>>,
}

impl Middlefiddle {
    pub fn new<H: Handler>(handler: H, middleware: Vec<Box<Middleware>>) -> Self {
        let mut chain = Chain::new(handler);

        for m in middleware.into_iter() {
            match *m {
                Middleware::BeforeMiddleware(i) => {
                    chain.link_before(i);
                },
                Middleware::AfterMiddleware(i) => {
                    chain.link_after(i);
                }
            }
        }

        Middlefiddle {
            chain: chain
        }
    }
}

impl Handler for Middlefiddle {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.chain.handle(req)
    }
}

/// The main `iron_middlefiddle` macro.
///
///
/// ```rust,no_run
/// middlefiddle! {
///     // Pass the router you want the routes to be added to to `router`:
///     router => some_router,
///
///     // `routes` takes one or more routes. Each one of them will have every
///     // piece of middleware in `middleware` applied to them:
///     routes => {
///         // An example route that follows the formatting
///         // `route_id: method "/an/example/path" => controller`:
///         lorem: get "/lorem" => controllers::lorem::index,
///
///         // There can be as many of these as you like…
///     },
///
///     // `middleware` takes all of the middlewares you want to apply to each
///     // of the routes specified above.
///     middleware => {
///         // An example `BeforeMiddleware`:
///         Middleware::BeforeMiddleware => middleware::SomeBeforeMiddleware,
///
///         // An example `AfterMiddleware`:
///         Middleware::AfterMiddleware => middleware::SomeAfterMiddleware,
///
///         // There can be as many of these as you like…
///     },
/// };
/// ```
///
/// ## Notes
///
/// - The formatting of the contents of `routes => { ... }` intentionally matches that of the router
/// crate's own [router macro](https://docs.rs/router/0.5.1/router/macro.router.html) in an effort
/// to make any potential refactoring easier.
///
/// - `$route_method` supports the following methods (must be lowercase):
///     - `get`
///     - `post`
///     - `put`
///     - `delete`
///     - `head`
///     - `patch`
///     - `options`
///     - `any`

#[macro_export]
macro_rules! middlefiddle {
    (
        router => $router:expr,
        routes => {
            $(
                $route_id:ident: $route_method:ident $route:expr => $route_handler:expr
            ),+
            $(,)*
        },
        middleware => {
            $(
                $middleware_type:expr => $middleware_handler:expr
            ),*
            $(,)*
        }
        $(,)*
    ) => ({
        use iron_middlefiddle::{Middlefiddle, Route};

        let mut routes = Vec::new();

        $(
            routes.push(Route {
                id: Some(stringify!($route_id).to_string()),
                method: stringify!($route_method).to_string(),
                route: Some($route.to_string()),
                handler: Some(Box::new($route_handler)),
            });
        )*

        for mut route in routes {
            let mut middleware = Vec::new();

            $(
                middleware.push(Box::new($middleware_type(Box::new($middleware_handler))));
            )*

            let route_id = route.id.take();
            let route_route = route.route.take();
            let route_handler = route.handler.take();
            let middleware_chain = Middlefiddle::new(route_handler.unwrap(), middleware);

            match route.method.as_ref() {
                "get" => { $router.get(route_route.unwrap().to_string(), middleware_chain, route_id.unwrap().to_string()); },
                "post" => { $router.post(route_route.unwrap().to_string(), middleware_chain, route_id.unwrap().to_string()); },
                "put" => { $router.put(route_route.unwrap().to_string(), middleware_chain, route_id.unwrap().to_string()); },
                "delete" => { $router.delete(route_route.unwrap().to_string(), middleware_chain, route_id.unwrap().to_string()); },
                "head" => { $router.head(route_route.unwrap().to_string(), middleware_chain, route_id.unwrap().to_string()); },
                "patch" => { $router.patch(route_route.unwrap().to_string(), middleware_chain, route_id.unwrap().to_string()); },
                "options" => { $router.options(route_route.unwrap().to_string(), middleware_chain, route_id.unwrap().to_string()); },
                "any" => { $router.any(route_route.unwrap().to_string(), middleware_chain, route_id.unwrap().to_string()); },
                _ => {}
            }
        }
    });
}
