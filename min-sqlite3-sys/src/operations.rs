//! This module contains operational SQL functions that provides
//! you to execute statements like read/write/update/delete,
//! begin/commit transactions, etc.

#![forbid(missing_docs)]
#![allow(clippy::not_unsafe_ptr_arg_deref)] // not stable, has false-positive results. so just keep it off for this module.

use std::{
    ffi::{CStr, CString},
    os, ptr,
};

use crate::connection::Database;
use crate::{bindings::sqlite3_stmt, ehandle::MinSqliteWrapperError};
use crate::{bindings::*, statement::SqlStatement};

/// Defines the helper functions that work on the columns of the data rows received.
pub trait ColumnCapabilities<'a> {
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
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized;

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
    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized;
}

/// Null type alias that equals to ()
pub type SqliteNull = ();

/// An alias to imitate null value for some of the
/// sqlite3 operations
pub const SQLITE_NULL: SqliteNull = ();

impl<'a> ColumnCapabilities<'a> for Option<i8> {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe {
            if sqlite3_column_type(stmt, i as os::raw::c_int) as u32 == COLUMN_NULL {
                return Ok(None);
            }

            Ok(Some(sqlite3_column_int64(stmt, i as os::raw::c_int) as i8))
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        if let Some(t) = self {
            unsafe {
                return SqlitePrimaryResult::from_i8(sqlite3_bind_int64(
                    stmt,
                    i as os::raw::c_int,
                    t as os::raw::c_longlong,
                ) as i8);
            }
        }

        SqlitePrimaryResult::MisMatch
    }
}

impl<'a> ColumnCapabilities<'a> for i8 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe { Ok(sqlite3_column_int64(stmt, i as os::raw::c_int) as Self) }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe {
            SqlitePrimaryResult::from_i8(sqlite3_bind_int64(
                stmt,
                i as os::raw::c_int,
                self as os::raw::c_longlong,
            ) as i8)
        }
    }
}

impl<'a> ColumnCapabilities<'a> for Option<u8> {
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe {
            if sqlite3_column_type(stmt, i as os::raw::c_int) as u32 == COLUMN_NULL {
                return Ok(None);
            }

            Ok(Some(sqlite3_column_int64(stmt, i as os::raw::c_int) as u8))
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        if let Some(t) = self {
            unsafe {
                return SqlitePrimaryResult::from_i8(sqlite3_bind_int64(
                    stmt,
                    i as os::raw::c_int,
                    t as os::raw::c_longlong,
                ) as i8);
            }
        }

        SqlitePrimaryResult::MisMatch
    }
}

impl<'a> ColumnCapabilities<'a> for u8 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe { Ok(sqlite3_column_int64(stmt, i as os::raw::c_int) as Self) }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe {
            SqlitePrimaryResult::from_i8(sqlite3_bind_int64(
                stmt,
                i as os::raw::c_int,
                self as os::raw::c_longlong,
            ) as i8)
        }
    }
}

impl<'a> ColumnCapabilities<'a> for Option<i16> {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe {
            if sqlite3_column_type(stmt, i as os::raw::c_int) as u32 == COLUMN_NULL {
                return Ok(None);
            }

            Ok(Some(sqlite3_column_int64(stmt, i as os::raw::c_int) as i16))
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        if let Some(t) = self {
            unsafe {
                return SqlitePrimaryResult::from_i8(sqlite3_bind_int64(
                    stmt,
                    i as os::raw::c_int,
                    t as os::raw::c_longlong,
                ) as i8);
            }
        }

        SqlitePrimaryResult::MisMatch
    }
}

impl<'a> ColumnCapabilities<'a> for i16 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe { Ok(sqlite3_column_int64(stmt, i as os::raw::c_int) as Self) }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe {
            SqlitePrimaryResult::from_i8(sqlite3_bind_int64(
                stmt,
                i as os::raw::c_int,
                self as os::raw::c_longlong,
            ) as i8)
        }
    }
}

impl<'a> ColumnCapabilities<'a> for Option<u16> {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe {
            if sqlite3_column_type(stmt, i as os::raw::c_int) as u32 == COLUMN_NULL {
                return Ok(None);
            }

            Ok(Some(sqlite3_column_int64(stmt, i as os::raw::c_int) as u16))
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        if let Some(t) = self {
            unsafe {
                return SqlitePrimaryResult::from_i8(sqlite3_bind_int64(
                    stmt,
                    i as os::raw::c_int,
                    t as os::raw::c_longlong,
                ) as i8);
            }
        }

        SqlitePrimaryResult::MisMatch
    }
}

impl<'a> ColumnCapabilities<'a> for u16 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe { Ok(sqlite3_column_int64(stmt, i as os::raw::c_int) as Self) }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe {
            SqlitePrimaryResult::from_i8(sqlite3_bind_int64(
                stmt,
                i as os::raw::c_int,
                self as os::raw::c_longlong,
            ) as i8)
        }
    }
}

impl<'a> ColumnCapabilities<'a> for Option<i32> {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe {
            if sqlite3_column_type(stmt, i as os::raw::c_int) as u32 == COLUMN_NULL {
                return Ok(None);
            }

            Ok(Some(sqlite3_column_int64(stmt, i as os::raw::c_int) as i32))
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        if let Some(t) = self {
            unsafe {
                return SqlitePrimaryResult::from_i8(sqlite3_bind_int64(
                    stmt,
                    i as os::raw::c_int,
                    t as os::raw::c_longlong,
                ) as i8);
            }
        }

        SqlitePrimaryResult::MisMatch
    }
}

impl<'a> ColumnCapabilities<'a> for i32 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe { Ok(sqlite3_column_int64(stmt, i as os::raw::c_int) as Self) }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe {
            SqlitePrimaryResult::from_i8(sqlite3_bind_int64(
                stmt,
                i as os::raw::c_int,
                self as os::raw::c_longlong,
            ) as i8)
        }
    }
}

impl<'a> ColumnCapabilities<'a> for Option<u32> {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe {
            if sqlite3_column_type(stmt, i as os::raw::c_int) as u32 == COLUMN_NULL {
                return Ok(None);
            }

            Ok(Some(sqlite3_column_int64(stmt, i as os::raw::c_int) as u32))
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        if let Some(t) = self {
            unsafe {
                return SqlitePrimaryResult::from_i8(sqlite3_bind_int64(
                    stmt,
                    i as os::raw::c_int,
                    t as os::raw::c_longlong,
                ) as i8);
            }
        }

        SqlitePrimaryResult::MisMatch
    }
}

impl<'a> ColumnCapabilities<'a> for u32 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe { Ok(sqlite3_column_int64(stmt, i as os::raw::c_int) as Self) }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe {
            SqlitePrimaryResult::from_i8(sqlite3_bind_int64(
                stmt,
                i as os::raw::c_int,
                self as os::raw::c_longlong,
            ) as i8)
        }
    }
}

impl<'a> ColumnCapabilities<'a> for Option<i64> {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe {
            if sqlite3_column_type(stmt, i as os::raw::c_int) as u32 == COLUMN_NULL {
                return Ok(None);
            }

            Ok(Some(sqlite3_column_int64(stmt, i as os::raw::c_int) as i64))
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        if let Some(t) = self {
            unsafe {
                return SqlitePrimaryResult::from_i8(sqlite3_bind_int64(
                    stmt,
                    i as os::raw::c_int,
                    t as os::raw::c_longlong,
                ) as i8);
            }
        }

        SqlitePrimaryResult::MisMatch
    }
}

impl<'a> ColumnCapabilities<'a> for i64 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe { Ok(sqlite3_column_int64(stmt, i as os::raw::c_int) as Self) }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe {
            SqlitePrimaryResult::from_i8(sqlite3_bind_int64(
                stmt,
                i as os::raw::c_int,
                self as os::raw::c_longlong,
            ) as i8)
        }
    }
}

impl<'a> ColumnCapabilities<'a> for Option<f32> {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe {
            if sqlite3_column_type(stmt, i as os::raw::c_int) as u32 == COLUMN_NULL {
                return Ok(None);
            }

            Ok(Some(sqlite3_column_double(stmt, i as os::raw::c_int) as f32))
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        if let Some(t) = self {
            unsafe {
                return SqlitePrimaryResult::from_i8(sqlite3_bind_double(
                    stmt,
                    i as os::raw::c_int,
                    t.into(),
                ) as i8);
            }
        }

        SqlitePrimaryResult::MisMatch
    }
}

impl<'a> ColumnCapabilities<'a> for f32 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe { Ok(sqlite3_column_double(stmt, i as os::raw::c_int) as Self) }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe {
            SqlitePrimaryResult::from_i8(
                sqlite3_bind_double(stmt, i as os::raw::c_int, self.into()) as i8,
            )
        }
    }
}

impl<'a> ColumnCapabilities<'a> for Option<f64> {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe {
            if sqlite3_column_type(stmt, i as os::raw::c_int) as u32 == COLUMN_NULL {
                return Ok(None);
            }

            Ok(Some(sqlite3_column_double(stmt, i as os::raw::c_int) as f64))
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        if let Some(t) = self {
            unsafe {
                return SqlitePrimaryResult::from_i8(sqlite3_bind_double(
                    stmt,
                    i as os::raw::c_int,
                    t,
                ) as i8);
            }
        }

        SqlitePrimaryResult::MisMatch
    }
}

impl<'a> ColumnCapabilities<'a> for f64 {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe { Ok(sqlite3_column_double(stmt, i as os::raw::c_int) as Self) }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe {
            SqlitePrimaryResult::from_i8(sqlite3_bind_double(stmt, i as os::raw::c_int, self) as i8)
        }
    }
}

impl<'a> ColumnCapabilities<'a> for Option<&str> {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe {
            if sqlite3_column_type(stmt, i as os::raw::c_int) as u32 == COLUMN_NULL {
                return Ok(None);
            }

            let result = sqlite3_column_text(stmt, i as os::raw::c_int);
            Ok(Some(CStr::from_ptr(result as *const _).to_str()?))
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        if let Some(t) = self {
            unsafe {
                return SqlitePrimaryResult::from_i8(sqlite3_bind_text(
                    stmt,
                    i as os::raw::c_int,
                    t.as_ptr() as *const _,
                    t.len() as os::raw::c_int,
                    sqlite_transient(),
                ) as i8);
            }
        }

        SqlitePrimaryResult::MisMatch
    }
}

impl<'a> ColumnCapabilities<'a> for &str {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe {
            let result = sqlite3_column_text(stmt, i as os::raw::c_int);
            Ok(CStr::from_ptr(result as *const _).to_str()?)
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe {
            SqlitePrimaryResult::from_i8(sqlite3_bind_text(
                stmt,
                i as os::raw::c_int,
                self.as_ptr() as *const _,
                self.len() as os::raw::c_int,
                sqlite_transient(),
            ) as i8)
        }
    }
}

impl<'a> ColumnCapabilities<'a> for Option<String> {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe {
            if sqlite3_column_type(stmt, i as os::raw::c_int) as u32 == COLUMN_NULL {
                return Ok(None);
            }

            let result = sqlite3_column_text(stmt, i as os::raw::c_int);
            Ok(Some(
                CStr::from_ptr(result as *const _).to_str()?.to_owned(),
            ))
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        if let Some(t) = self {
            unsafe {
                return SqlitePrimaryResult::from_i8(sqlite3_bind_text(
                    stmt,
                    i as os::raw::c_int,
                    t.as_ptr() as *const _,
                    t.len() as os::raw::c_int,
                    sqlite_transient(),
                ) as i8);
            }
        }

        SqlitePrimaryResult::MisMatch
    }
}

impl<'a> ColumnCapabilities<'a> for String {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unsafe {
            let result = sqlite3_column_text(stmt, i as os::raw::c_int);
            Ok(CStr::from_ptr(result as *const _).to_str()?.to_owned())
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe {
            SqlitePrimaryResult::from_i8(sqlite3_bind_text(
                stmt,
                i as os::raw::c_int,
                self.as_ptr() as *const _,
                self.len() as os::raw::c_int,
                sqlite_transient(),
            ) as i8)
        }
    }
}

impl<'a> ColumnCapabilities<'a> for Option<Vec<u8>> {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        use ptr::copy_nonoverlapping as copy;
        unsafe {
            if sqlite3_column_type(stmt, i as os::raw::c_int) as u32 == COLUMN_NULL {
                return Ok(None);
            }

            let pointer = sqlite3_column_blob(stmt, i as os::raw::c_int);
            if pointer.is_null() {
                return Ok(Some(vec![]));
            }

            let count = sqlite3_column_bytes(stmt, i as os::raw::c_int) as usize;
            let mut buffer = Vec::with_capacity(count);
            #[allow(clippy::uninit_vec)]
            buffer.set_len(count); // need to allocate every single location in vec before copying the buffer
            copy(pointer as *const u8, buffer.as_mut_ptr(), count);
            Ok(Some(buffer))
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe {
            if let Some(t) = self {
                if t.is_empty() {
                    return SqlitePrimaryResult::from_i8(sqlite3_bind_zeroblob64(
                        stmt,
                        i as os::raw::c_int,
                        0,
                    ) as i8);
                }

                return SqlitePrimaryResult::from_i8(sqlite3_bind_blob(
                    stmt,
                    i as os::raw::c_int,
                    t.as_ptr() as *const _,
                    t.len() as os::raw::c_int,
                    sqlite_transient(),
                ) as i8);
            }
        }

        SqlitePrimaryResult::MisMatch
    }
}

impl<'a> ColumnCapabilities<'a> for Vec<u8> {
    #[inline]
    fn get_data(stmt: *mut sqlite3_stmt, i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        use ptr::copy_nonoverlapping as copy;
        unsafe {
            let pointer = sqlite3_column_blob(stmt, i as os::raw::c_int);
            if pointer.is_null() {
                return Ok(vec![]);
            }

            let count = sqlite3_column_bytes(stmt, i as os::raw::c_int) as usize;
            let mut buffer = Vec::with_capacity(count);
            #[allow(clippy::uninit_vec)]
            buffer.set_len(count); // need to allocate every single location in vec before copying the buffer
            copy(pointer as *const u8, buffer.as_mut_ptr(), count);
            Ok(buffer)
        }
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe {
            if self.is_empty() {
                return SqlitePrimaryResult::from_i8(sqlite3_bind_zeroblob64(
                    stmt,
                    i as os::raw::c_int,
                    0,
                ) as i8);
            }

            SqlitePrimaryResult::from_i8(sqlite3_bind_blob(
                stmt,
                i as os::raw::c_int,
                self.as_ptr() as *const _,
                self.len() as os::raw::c_int,
                sqlite_transient(),
            ) as i8)
        }
    }
}

impl<'a> ColumnCapabilities<'a> for Option<&[u8]> {
    fn get_data(_stmt: *mut sqlite3_stmt, _i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        if let Some(t) = self {
            unsafe {
                if t.is_empty() {
                    return SqlitePrimaryResult::from_i8(sqlite3_bind_zeroblob64(
                        stmt,
                        i as os::raw::c_int,
                        0,
                    ) as i8);
                }

                return SqlitePrimaryResult::from_i8(sqlite3_bind_blob(
                    stmt,
                    i as os::raw::c_int,
                    t.as_ptr() as *const _,
                    t.len() as os::raw::c_int,
                    sqlite_transient(),
                ) as i8);
            }
        }

        SqlitePrimaryResult::MisMatch
    }
}

impl<'a> ColumnCapabilities<'a> for &[u8] {
    fn get_data(_stmt: *mut sqlite3_stmt, _i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe {
            if self.is_empty() {
                return SqlitePrimaryResult::from_i8(sqlite3_bind_zeroblob64(
                    stmt,
                    i as os::raw::c_int,
                    0,
                ) as i8);
            }

            SqlitePrimaryResult::from_i8(sqlite3_bind_blob(
                stmt,
                i as os::raw::c_int,
                self.as_ptr() as *const _,
                self.len() as os::raw::c_int,
                sqlite_transient(),
            ) as i8)
        }
    }
}

impl<'a> ColumnCapabilities<'a> for Option<SqliteNull> {
    fn get_data(_stmt: *mut sqlite3_stmt, _i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe { SqlitePrimaryResult::from_i8(sqlite3_bind_null(stmt, i as os::raw::c_int) as i8) }
    }
}

impl<'a> ColumnCapabilities<'a> for SqliteNull {
    fn get_data(_stmt: *mut sqlite3_stmt, _i: usize) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn bind_val(self, stmt: *mut sqlite3_stmt, i: usize) -> SqlitePrimaryResult
    where
        Self: Sized,
    {
        unsafe { SqlitePrimaryResult::from_i8(sqlite3_bind_null(stmt, i as os::raw::c_int) as i8) }
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
    /// let db_path = Path::new("./example.db");
    /// let db = Database::open(db_path).unwrap();
    ///
    /// let status = db.execute(statement, None::<Box<dyn FnOnce(SqlitePrimaryResult, String)>>).unwrap();
    ///
    /// if status != SqlitePrimaryResult::Ok {
    ///    ...
    /// }
    ///
    /// db.close();
    /// ```
    fn execute<'a, F>(
        &self,
        statement: String,
        callback_fn: Option<F>,
    ) -> Result<SqlitePrimaryResult, MinSqliteWrapperError<'a>>
    where
        F: FnOnce(SqlitePrimaryResult, String);

    /// Prepares SQL operation to be executed and then destroy.
    ///
    /// # Warning
    /// kill() must be called for each result of the prepare() function in order to avoid resource leak.
    ///
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
    fn prepare<'a, F>(
        &self,
        statement: String,
        callback_fn: Option<F>,
    ) -> Result<SqlStatement, MinSqliteWrapperError<'a>>
    where
        F: FnOnce(SqlitePrimaryResult, String);
}

impl Operations for Database {
    fn execute<'a, F>(
        &self,
        statement: String,
        callback_fn: Option<F>,
    ) -> Result<SqlitePrimaryResult, MinSqliteWrapperError<'a>>
    where
        F: FnOnce(SqlitePrimaryResult, String),
    {
        let st = CString::new(&*statement)?;
        unsafe {
            let status = sqlite3_exec(self.rp, st.as_ptr(), None, ptr::null_mut(), ptr::null_mut());

            if status != SqlitePrimaryResult::Ok as i32 {
                if let Some(func) = callback_fn {
                    func(SqlitePrimaryResult::from_i8(status as i8), statement);
                }
            }

            Ok(SqlitePrimaryResult::from_i8(status as i8))
        }
    }

    fn prepare<'a, F>(
        &self,
        statement: String,
        callback_fn: Option<F>,
    ) -> Result<SqlStatement, MinSqliteWrapperError<'a>>
    where
        F: FnOnce(SqlitePrimaryResult, String),
    {
        let st = CString::new(&*statement)?;
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

            if status != SqlitePrimaryResult::Ok as i32 {
                if let Some(func) = callback_fn {
                    func(SqlitePrimaryResult::from_i8(status as i8), statement);
                }
            }
        }

        Ok(SqlStatement::new(stmt))
    }
}
