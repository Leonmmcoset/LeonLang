use super::{Env, Value};

// 导入各个内置库模块
mod basic;
mod request;

// 重新导出各个模块的注册函数
pub use basic::register_basic_functions;
pub use request::register_request_functions;

// 导出类型别名供内部使用
type Function = Box<dyn Fn(Vec<Value>) -> Result<Value, String>>;