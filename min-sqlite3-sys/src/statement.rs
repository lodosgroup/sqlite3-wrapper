//! This module contains data-types and functions to provide
//! prepared statement functionality.

#![forbid(missing_docs)]

use crate::{
    bindings::{sqlite3_finalize, sqlite3_step, sqlite3_stmt},
    ehandle::MinSqliteWrapperError,
    operations::ColumnCapabilities,
    prelude::*,
};

/// This enumeration is the list of the possible status outcomes for the
/// `execute_prepared(&mut self)` function.
#[non_exhaustive]
#[repr(i8)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PreparedStatementStatus {
    /// Indicates the actual error type id from SQLITE as an inner value.
    Other(i32) = -1,
    /// Indicates that another row of output is available.
    FoundRow,
    /// Indicates that an operation has completed.
    Done,
}

#[derive(Clone)]
/// Binded instance of the sqlite3_stmt.
pub struct SqlStatement(*mut sqlite3_stmt);

unsafe impl Send for SqlStatement {}
unsafe impl Sync for SqlStatement {}

impl Drop for SqlStatement {
    fn drop(&mut self) {
        self.kill();
    }
}

/// Provides prepared statement functionality.
impl<'a> SqlStatement {
    /// Creates SqlStatement instance.
    ///
    /// # Usage
    /// let stmt_p = ptr::null_mut();
    /// SqlStatement::new(stmt_p);
    /// ```
    #[inline]
    pub(crate) fn new(statement: *mut sqlite3_stmt) -> Self {
        Self(statement)
    }

    /// Executes the prepared statement and returns PreparedStatementStatus for data and error
    /// handling.
    ///
    /// # Usage
    /// let db_path = Path::new("./example.db");
    /// let db = Database::open(db_path).unwrap();
    ///
    /// let statement = String::from(
    ///     "SELECT * FROM example_table WHERE ID = '15';"
    /// );
    ///
    /// let mut sql = db.prepare(statement, None::<Box<dyn FnOnce(SqlitePrimaryResult, String)>>).unwrap();
    ///
    /// while let PreparedStatementStatus::FoundRow = sql.execute_prepared() {
    ///     ...
    /// }
    ///
    /// sql.kill();
    /// db.close();
    /// ```
    #[inline]
    pub fn execute_prepared(&mut self) -> PreparedStatementStatus {
        match unsafe { sqlite3_step(self.0) } {
            100 => PreparedStatementStatus::FoundRow,
            101 => PreparedStatementStatus::Done,
            other_id => PreparedStatementStatus::Other(other_id),
        }
    }

    /// Reads the column data of the rows that returns from the SQL query.
    ///
    /// # Panics
    /// - If the data type is incorrectly specified.
    /// - If the column index doesn't match.
    ///
    /// # Usage
    /// ```
    /// #[derive(Debug)]
    /// struct Item {
    ///     id: i64,
    ///     name: String,
    ///     tag: String,
    /// }
    ///
    /// let db_path = Path::new("./example.db");
    /// let db = Database::open(db_path).unwrap();
    ///
    /// let statement = String::from(
    ///     "SELECT * FROM example_table WHERE ID = '15';"
    /// );
    ///
    /// let mut sql = db.prepare(statement, None::<Box<dyn FnOnce(SqlitePrimaryResult, String)>>).unwrap();
    ///
    /// while let PreparedStatementStatus::FoundRow = sql.execute_prepared() {
    ///     println!(
    ///         "id = {}, name = {}, tag = {}",
    ///         sql.get_data::<i64>(0).unwrap(),
    ///         sql.get_data::<String>(1).unwrap(),
    ///         sql.get_data::<String>(2).unwrap(),
    ///     );
    ///
    ///     // OR
    ///
    ///     println!(
    ///         "{:?}",
    ///         Item {
    ///             id: sql.get_data(0).unwrap(),
    ///             name: sql.get_data(1).unwrap(),
    ///             tag: sql.get_data(2).unwrap(),
    ///         }
    ///     );
    /// }
    ///
    /// sql.kill();
    /// db.close();
    /// ```
    #[inline]
    pub fn get_data<T: ColumnCapabilities<'a>>(
        &'a self,
        i: usize,
    ) -> Result<T, MinSqliteWrapperError> {
        ColumnCapabilities::get_data(self.0, i)
    }

    /// Binds the value of a parameter to a prepared statement indicator.
    ///
    /// Supported indicator patterns:
    /// - ?
    /// - ?NNN
    /// - :VVV
    /// - @VVV
    /// - $VVV
    ///
    /// Returns `SqlitePrimaryResult:Ok` on success or an error code if anything goes wrong.
    /// `SqlitePrimaryResult::Range` is returned if the parameter index is out of range.
    ///
    /// # IMPORTANT
    /// The first argument isn't index of the column. It's simply index of the
    /// indicator and always starts at 1. If the first argument is given zero,
    /// the function will return `SqlitePrimaryResult::Range`.
    ///
    /// # Usage
    /// ```
    /// let db_path = Path::new("./example.db");
    /// let db = Database::open(db_path).unwrap();
    ///
    /// let statement = String::from(
    ///     "SELECT * FROM example_table WHERE ID = ;"
    /// );
    ///
    /// let mut sql = db.prepare(statement, None::<Box<dyn FnOnce(SqlitePrimaryResult, String)>>).unwrap();
    ///
    /// let status = sql.bind_val(1, 5);
    /// // You can do some checks by
    /// assert_eq!(status, SqlitePrimaryResult::Ok);
    /// // or
    /// if status == SqlitePrimaryResult::Range {
    ///     panic!("Out of index on sql.bind_val!");
    /// }
    ///
    /// sql.kill();
    /// db.close();
    /// ```
    #[inline]
    pub fn bind_val<T: ColumnCapabilities<'a>>(&'a self, i: usize, val: T) -> SqlitePrimaryResult {
        if i == 0 {
            return SqlitePrimaryResult::Range;
        }

        ColumnCapabilities::bind_val(val, self.0, i)
    }

    /// Called to destroy prepared statement. This function must be called for
    /// each prepared statement. Otherwise some resource leaks might happen.
    ///
    /// # Usage
    /// let db_path = Path::new("./example.db");
    /// let db = Database::open(db_path).unwrap();
    ///
    /// let statement = String::from(
    ///     "SELECT * FROM example_table WHERE ID = '15';"
    /// );
    ///
    /// let mut sql = db.prepare(statement, None::<Box<dyn FnOnce(SqlitePrimaryResult, String)>>).unwrap();
    ///
    /// sql.kill();
    /// db.close();
    /// ```
    #[inline]
    pub fn kill(&self) -> SqlitePrimaryResult {
        unsafe { SqlitePrimaryResult::from(sqlite3_finalize(self.0)) }
    }
}
