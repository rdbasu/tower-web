extern crate bytes;
#[macro_use]
extern crate futures;
extern crate hyper;
extern crate http;
#[macro_use]
extern crate log;
extern crate tokio;
extern crate tower;
extern crate serde;
extern crate serde_json;

pub mod codegen;
pub mod resource;
pub mod routing;
pub mod service;

mod builder;
mod error;
mod response;
mod run;

pub use builder::ServiceBuilder;
pub use error::Error;
pub use resource::Resource;
pub use response::IntoResponse;
pub use service::Service;

// ===== proc_macro_hack junk =====

#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate tower_web_macros;

#[doc(hidden)]
pub use tower_web_macros::*;

proc_macro_item_decl! {
    /// Implement a Web Service.
    impl_web! => impl_web_impl
}

// ===== end proc_macro_hack junk =====
