# `iron_middlefiddle` - Route specific middleware made easy

[![Crates.io](https://meritbadge.herokuapp.com/iron-middlefiddle)](https://crates.io/crates/iron-middlefiddle)
[![Docs](https://docs.rs/iron-middlefiddle/badge.svg)](https://docs.rs/iron-middlefiddle)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/elliotekj/iron-middlefiddle/blob/master/LICENSE)

There are cases where you may only want specific routes within
a [`Router`](https://docs.rs/router/0.5.1/router/struct.Router.html) to use some
piece of middleware (e.g. routes that require the user to be logged in). Adding
route-specific middleware in Iron is a repetitive and messy business which is
where `iron_middlefiddle` comes in. It provides a convenience macro for adding
one or more
[`BeforeMiddleware`](https://docs.rs/iron/0.5.1/iron/middleware/trait.BeforeMiddleware.html)
or
[`AfterMiddleware`](https://docs.rs/iron/0.5.1/iron/middleware/trait.AfterMiddleware.html)s
to multiple routes at once.

## Installation

If you're using Cargo, just add `iron_middlefiddle` to your Cargo.toml:

```toml
[dependencies]
iron-middlefiddle = "0.1.0"
```

## Example usage

```rust
#[macro_use]
extern crate iron_middlefiddle;
extern crate iron;
extern crate mount;
extern crate router;

use iron_middlefiddle::Middleware;
use mount::Mount;
use router::Router;

mod controllers;
mod middleware;

fn main() {
    let mut frontend_router = Router::new();

    // Add some `frontend_router` routes and the middleware they should use:

    middlefiddle! {
        router => frontend_router,
        routes => {
            lorem: get "/lorem" => controllers::lorem::index,
            ipsum: get "/ipsum" => controllers::ipsum::index,
            dolor: get "/dolor" => controllers::dolor::index,
        },
        middleware => {
            Middleware::BeforeMiddleware => middleware::auth::TokenValidity,
        },
    };

    // Add some more `frontend_router` routes that aren't going to need the middleware:

    frontend_router.get("/amet", controllers::amet::index, "amet");

    // The usual…

    let mut mount = Mount::new();
    mount.mount("/", frontend_router)

    Iron::new(mount).http("127.0.0.1:8888").unwrap();
}
```

## Documentation

The documentation for `iron_middlefiddle` is [available on
docs.rs](https://docs.rs/iron-middlefiddle).

## License

`iron_middlefiddle` is released under the MIT
[`LICENSE`](https://github.com/elliotekj/iron-middlefiddle/blob/master/LICENSE).

If you require a different license to be able to use this crate, please get in
touch.

## About

This crate was written by [Elliot Jackson](https://elliotekj.com).

- Blog: [https://elliotekj.com](https://elliotekj.com)
- Email: elliot@elliotekj.com
