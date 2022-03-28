use std::ffi::CString;
use std::{ffi::CStr, os, ptr};

use crate::bindings::sqlite3_stmt;
use crate::{
    bindings::{
        sqlite3_column_blob, sqlite3_column_bytes, sqlite3_column_double, sqlite3_column_int64,
        sqlite3_column_text, sqlite3_exec, sqlite3_prepare_v2,
    },
    core::{Database, SqlitePrimaryResult},
    statement::SqlStatement,
};

/// Defines the helper functions that work on the columns of the data rows received.
pub trait ColumnCapabilities {
    /// Reads the column data of the rows that returns from the SQL query.
    ///
    /// # Panics
    /// - If the data type is incorrectly specified.
    /// - If the column index doesn't match.
    ///
    /// # Usage
    /// ```ignore
    ///
    /// #[derive(Debug)]
    /// struct Item {
    ///     id: i64,
    ///     name: String,
    ///     tag: String,
    /// }
    ///
    /// let db_path = Path::new("./example.db");
    /// let db = Database::open(db_path);
    ///
    /// let statement = String::from(
    ///     "SELECT * FROM example_table WHERE ID = '15';"
    /// );
    ///
    /// let mut sql = db.prepare(statement, None::<Box<dyn FnOnce(SqlitePrimaryResult, String)>>);
    ///
    /// while let PreparedStatementStatus::FoundRow = sql.execute_prepared() {
    ///     println!(
    ///         "id = {}, name = {}, tag = {}",
    ///         sql.get_data::<i64>(0),
    ///         sql.get_data::<String>(1),
    ///         sql.get_data::<String>(2),
    ///     );
    ///
    ///     // OR
    ///
    ///     println!(
    ///         "{:?}",
    ///         Item {
    ///             id: sql.get_data(0),
    ///             name: sql.get_data(1),
    ///             tag: sql.get_data(2),
    ///         }
    ///     );
    /// }
    ///
    /// sql.kill();
    /// db.close();
    /// ```
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Self;
}

impl ColumnCapabilities for i8 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Self {
        unsafe { sqlite3_column_int64(stmt, i as os::raw::c_int) as Self }
    }
}

impl ColumnCapabilities for u8 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Self {
        unsafe { sqlite3_column_int64(stmt, i as os::raw::c_int) as Self }
    }
}

impl ColumnCapabilities for i16 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Self {
        unsafe { sqlite3_column_int64(stmt, i as os::raw::c_int) as Self }
    }
}

impl ColumnCapabilities for u16 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Self {
        unsafe { sqlite3_column_int64(stmt, i as os::raw::c_int) as Self }
    }
}

impl ColumnCapabilities for i32 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Self {
        unsafe { sqlite3_column_int64(stmt, i as os::raw::c_int) as Self }
    }
}

impl ColumnCapabilities for u32 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Self {
        unsafe { sqlite3_column_int64(stmt, i as os::raw::c_int) as Self }
    }
}

impl ColumnCapabilities for i64 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Self {
        unsafe { sqlite3_column_int64(stmt, i as os::raw::c_int) as Self }
    }
}

impl ColumnCapabilities for f32 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Self {
        unsafe { sqlite3_column_double(stmt, i as os::raw::c_int) as Self }
    }
}

impl ColumnCapabilities for f64 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Self {
        unsafe { sqlite3_column_double(stmt, i as os::raw::c_int) as Self }
    }
}

impl ColumnCapabilities for &str {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Self {
        let result = unsafe { sqlite3_column_text(stmt, i as os::raw::c_int) };

        unsafe { CStr::from_ptr(result as *const _).to_str().unwrap() }
    }
}

impl ColumnCapabilities for String {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Self {
        let result = unsafe { sqlite3_column_text(stmt, i as os::raw::c_int) };

        unsafe {
            CStr::from_ptr(result as *const _)
                .to_str()
                .unwrap()
                .to_owned()
        }
    }
}

impl ColumnCapabilities for Vec<u8> {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Self {
        use ptr::copy_nonoverlapping as copy;
        unsafe {
            let pointer = sqlite3_column_blob(stmt, i as os::raw::c_int);
            if pointer.is_null() {
                return vec![];
            }

            let count = sqlite3_column_bytes(stmt, i as os::raw::c_int) as usize;
            let mut buffer = Vec::with_capacity(count);
            buffer.set_len(count);
            copy(pointer as *const u8, buffer.as_mut_ptr(), count);
            buffer
        }
    }
}

/// Defines SQL functions.
pub trait Operations {
    /// A wrapper around prepare(), execute_prepared(), and kill(), that allows an
    /// application to run multiple statements of SQL without having to use a lot of Rust code.
    ///
    /// # Warning
    /// This function does not provide to read data from SQLite.
    ///
    /// # Usage
    /// ```ignore
    /// let db_path = Path::new("./example.db");
    /// let db = Database::open(db_path);
    ///
    /// let status = db.execute(statement, None::<Box<dyn FnOnce(SqlitePrimaryResult, String)>>);
    ///
    /// if status != SqlitePrimaryResult::Ok {
    ///    ...
    /// }
    ///
    /// db.close();
    /// ```
    fn execute<F>(&self, statement: String, callback_fn: Option<F>) -> SqlitePrimaryResult
    where
        F: FnOnce(SqlitePrimaryResult, String);

    /// Prepares SQL operation to be executed and then destroy.
    ///
    /// # Warning
    /// kill() must be called for each result of the prepare() function in order to avoid resource leak.
    ///
    ///
    /// # Usage
    /// ```ignore
    /// let db_path = Path::new("./example.db");
    /// let db = Database::open(db_path);
    ///
    /// let statement = String::from(
    ///     "SELECT * FROM example_table WHERE ID = '15';"
    /// );
    ///
    /// let mut sql = db.prepare(statement, None::<Box<dyn FnOnce(SqlitePrimaryResult, String)>>);
    ///
    /// while let PreparedStatementStatus::FoundRow = sql.execute_prepared() {
    ///     ...
    /// }
    ///
    /// sql.kill();
    /// db.close();
    /// ```
    fn prepare<F>(&self, statement: String, callback_fn: Option<F>) -> SqlStatement
    where
        F: FnOnce(SqlitePrimaryResult, String);
}

impl Operations for Database {
    fn execute<F>(&self, statement: String, callback_fn: Option<F>) -> SqlitePrimaryResult
    where
        F: FnOnce(SqlitePrimaryResult, String),
    {
        let st = CString::new(&*statement).unwrap();
        unsafe {
            let status = sqlite3_exec(self.rp, st.as_ptr(), None, 0 as *mut _, 0 as *mut _);

            if callback_fn.is_some() && status != SqlitePrimaryResult::Ok as i32 {
                callback_fn.unwrap()(SqlitePrimaryResult::from_i8(status as i8), statement);
            }

            SqlitePrimaryResult::from_i8(status as i8)
        }
    }

    fn prepare<F>(&self, statement: String, callback_fn: Option<F>) -> SqlStatement
    where
        F: FnOnce(SqlitePrimaryResult, String),
    {
        let st = CString::new(&*statement).unwrap();
        let mut stmt = ptr::null_mut();
        let mut tail = ptr::null();

        unsafe {
            let status = sqlite3_prepare_v2(
                self.rp,
                st.as_ptr(),
                statement.len() as os::raw::c_int,
                &mut stmt,
                &mut tail,
            );

            if callback_fn.is_some() && status != SqlitePrimaryResult::Ok as i32 {
                callback_fn.unwrap()(SqlitePrimaryResult::from_i8(status as i8), statement);
            }
        }

        SqlStatement::new(stmt)
    }
}
