//! This is the main (and only for now) application Error type.
//! It's using 'thiserror' as it reduces boilerplate error code while providing rich error typing.
//!
//! Notes:
//!     - The strategy is to start with one Error type for the whole application and then seggregate as needed.
//!     - Since everything is typed from the start, renaming and refactoring become relatively trivial.
//!     - By best practices, `anyhow` is not used in application code, but can be used in unit or integration test (will be in dev_dependencies when used)
//!

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	// 上下文失败
	CtxFail,

	// 一个值不符合预期的类型
	XValueNotOfType(&'static str),

	// 属性未找到
	XPropertyNotFound(String),

	// 储存创建失败
	StoreFailToCreate(String),

	// modql库相关错误
	Modql(modql::Error),

	// JSON序列化/反序列化错误，使用serde_json库
	JsonSerde(serde_json::Error),

	// modql操作符不支持
	ModqlOperatorNotSupported(String),

	// surrealdb库相关错误
	Surreal(surrealdb::err::Error),

	// I/O错误
	IO(std::io::Error),
}

// region:    --- Froms
// 为了简化错误处理，我们实现了一系列的From trait，这样我们就可以在代码中直接使用?操作符来处理错误
impl From<modql::Error> for Error {
	fn from(val: modql::Error) -> Self {
		Error::Modql(val)
	}
}
impl From<serde_json::Error> for Error {
	fn from(val: serde_json::Error) -> Self {
		Error::JsonSerde(val)
	}
}
impl From<surrealdb::err::Error> for Error {
	fn from(val: surrealdb::err::Error) -> Self {
		Error::Surreal(val)
	}
}
impl From<std::io::Error> for Error {
	fn from(val: std::io::Error) -> Self {
		Error::IO(val)
	}
}
// endregion: --- Froms

// region:    --- Error Boiler
// 实现Display和Error trait，这样我们就可以直接使用{:?}来打印错误信息
impl std::fmt::Display for Error {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boiler