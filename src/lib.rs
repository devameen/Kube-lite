#![feature(async_await, await_macro, futures_api)]

#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate tokio;

pub mod api;
pub mod client;
pub mod error;
pub mod future;
