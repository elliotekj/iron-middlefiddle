extern crate iron;

use iron::{Handler, IronResult, Request, Response, Chain};

pub enum Middleware {
    BeforeMiddleware(Box<iron::BeforeMiddleware>),
    AfterMiddleware(Box<iron::AfterMiddleware>),
}

pub struct Middlefiddle {
    chain: Chain,
}

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

#[macro_export]
macro_rules! middlefiddle {
    (
        router => $router:expr,
        routes => {
            $(
                $route_id:ident: $route_method:ident $route:expr => $route_handler:expr
            ),+
            $(,)* // Handle trailing commas
        },
        middleware => {
            $(
                $middleware_type:expr => $middleware_handler:expr
            ),*
            $(,)* // Handle trailing commas
        }
        $(,)* // Handle trailing commas
    ) => ({
        use iron_middlefiddle::Route;

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
