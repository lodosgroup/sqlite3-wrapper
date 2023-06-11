//! This module contains trait and functions for SQLite database
//! connection.

#![forbid(missing_docs)]

use std::{ffi::CString, os::unix::prelude::OsStrExt, path::Path, ptr};

use crate::{
    bindings::{sqlite3_close, sqlite3_open},
    ehandle::MinSqliteWrapperError,
    prelude::*,
};

/// Main database struct that provides core
/// operations in order to work with SQLite.
pub struct Database {
    /// Binded pointer of the sqlite3 instance.
    pub(crate) rp: *mut crate::bindings::sqlite3,
}

/// Specifies the core operations of the SQLite connection.
pub trait Connection<'a> {
    /// Opens a database and creates a new database connection. If the filename does not exist,
    /// it will be created. The file will be opened read/write if possible. If not, the file
    /// will be opened read-only.
    ///
    /// # Panics
    /// - If the read/write permissions are missing on the database file.
    /// - If the database file isn't a valid SQLite file or it's corrupted.
    ///
    /// # Usage
    /// let db_path = Path::new("./example.db");
    /// Database::open(db_path).unwrap();
    /// ```
    fn open<T>(path: T) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
        T: AsRef<Path>;

    /// The sqlite3_close() is destructor for the sqlite3 object. Returns
    /// SqlitePrimaryResult::Ok if the sqlite3 object is successfully destroyed
    /// and all associated resources are deallocated.
    ///
    /// # Usage
    /// let db_path = Path::new("./example.db");
    /// let db = Database::open(db_path).unwrap();
    /// let status = db.close();
    ///
    /// if SqlitePrimaryResult::Ok != status {
    ///     ...
    /// }
    /// ```
    fn close(self) -> SqlitePrimaryResult;
}

impl<'a> Connection<'a> for Database {
    fn open<T>(db_path: T) -> Result<Self, MinSqliteWrapperError<'a>>
    where
        Self: Sized,
        T: AsRef<Path>,
    {
        let mut rp = ptr::null_mut();
        let path = CString::new(db_path.as_ref().as_os_str().as_bytes())?;
        unsafe {
            sqlite3_open(path.as_ptr(), &mut rp);
        }

        Ok(Database { rp })
    }

    fn close(self) -> SqlitePrimaryResult {
        sqlite_close(self.rp)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        sqlite_close(self.rp);
    }
}

#[inline]
fn sqlite_close(rp: *mut crate::bindings::sqlite3) -> SqlitePrimaryResult {
    unsafe { SqlitePrimaryResult::from(sqlite3_close(rp)) }
}
