pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type BoxResult<T> = Result<T, BoxError>;

pub use cxx_memory::let_cxx;
pub use cxx_swift::{swift, util};
