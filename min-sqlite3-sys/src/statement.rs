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
    /// Indicates that the type of error is currently not supported/handled by the library.
    UnrecognizedStatus = -1,
    /// Indicates that another row of output is available.
    FoundRow,
    /// Indicates that an operation has completed.
    Done,
}

#[derive(Copy, Clone)]
/// Binded instance of the sqlite3_stmt.
pub struct SqlStatement(*mut sqlite3_stmt);

/// Provides prepared statement functionality.
impl<'a> SqlStatement {
    /// Creates SqlStatement instance.
    ///
    /// # Usage
    /// ```ignore
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
    /// ```ignore
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
            _ => PreparedStatementStatus::UnrecognizedStatus,
        }
    }

    /// Returns the column data of the rows that returns from the SQL query.
    ///
    /// # Panics
    /// - If the data type is incorrectly specified.
    /// - If the column index doesn't match.
    ///
    /// # Usage
    /// ```ignore
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
        Ok(ColumnCapabilities::get_data(self.0, i)?)
    }

    /// Called to destroy prepared statement. This function must be called for
    /// each prepared statement. Otherwise some resource leaks might happen.
    ///
    /// # Usage
    /// ```ignore
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
        unsafe { SqlitePrimaryResult::from_i8(sqlite3_finalize(self.0) as i8) }
    }
}
