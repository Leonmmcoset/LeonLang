use super::{Env, Value};

// Import various built-in library modules
mod basic;
mod request;
mod time;
mod color;

// Re-export registration functions from various modules
pub use basic::register_basic_functions;
pub use request::register_request_functions;
pub use time::register_time_functions;
pub use color::register_color_functions;

// Export type aliases for internal use
type Function = Box<dyn Fn(Vec<Value>) -> Result<Value, String>>;