/// Main database struct that provides core
/// operations in order to work with SQLite.
pub struct Database {
    /// Binded pointer of the sqlite3 instance.
    pub(crate) rp: *mut crate::bindings::sqlite3,
}

pub use crate::bindings::SqlitePrimaryResult;
pub use crate::connection::Connection;
pub use crate::operations::Operations;
pub use crate::statement::PreparedStatementStatus;
