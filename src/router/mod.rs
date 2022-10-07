use yew_router::prelude::Link;
mod routes;
mod switch;

pub use routes::*;
pub use switch::*;
pub type SneuLink = Link<SneuRoutes>;
