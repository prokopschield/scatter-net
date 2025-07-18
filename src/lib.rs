#![allow(clippy::module_name_repetitions)]

#[allow(clippy::missing_errors_doc)]
#[allow(dead_code)]
mod legacy;

pub use legacy::*;

mod scatter_net;

pub use scatter_net::*;
