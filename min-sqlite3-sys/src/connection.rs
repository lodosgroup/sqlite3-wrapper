//! This module contains trait and functions for SQLite database
//! connection.

#![forbid(missing_docs)]

use std::{ffi::CString, os::unix::prelude::OsStrExt, path::Path};

use crate::{
    bindings::{sqlite3_close, sqlite3_open},
    prelude::*,
};

/// Main database struct that provides core
/// operations in order to work with SQLite.
pub struct Database {
    /// Binded pointer of the sqlite3 instance.
    pub(crate) rp: *mut crate::bindings::sqlite3,
}

/// Specifies the core operations of the SQLite connection.
pub trait Connection {
    /// Opens a database and creates a new database connection. If the filename does not exist,
    /// it will be created. The file will be opened read/write if possible. If not, the file
    /// will be opened read-only.
    ///
    /// # Panics
    /// - If the read/write permissions are missing on the database file.
    /// - If the database file isn't a valid SQLite file or it's corrupted.
    ///
    /// # Usage
    /// ```ignore
    /// let db_path = Path::new("./example.db");
    /// Database::open(db_path);
    /// ```
    fn open<T>(path: T) -> Self
    where
        T: AsRef<Path>;

    /// The sqlite3_close() is destructor for the sqlite3 object. Returns
    /// SqlitePrimaryResult::Ok if the sqlite3 object is successfully destroyed
    /// and all associated resources are deallocated.
    ///
    /// # Usage
    /// ```ignore
    /// let db_path = Path::new("./example.db");
    /// let db = Database::open(db_path);
    /// let status = db.close();
    ///
    /// if SqlitePrimaryResult::Ok != status {
    ///     ...
    /// }
    /// ```
    fn close(self) -> SqlitePrimaryResult;
}

impl Connection for Database {
    fn open<T>(db_path: T) -> Self
    where
        T: AsRef<Path>,
    {
        let mut rp = 0 as *mut _;
        let path = CString::new(db_path.as_ref().as_os_str().as_bytes()).unwrap();
        unsafe {
            sqlite3_open(path.as_ptr(), &mut rp);
        }

        Database { rp }
    }

    fn close(self) -> SqlitePrimaryResult {
        unsafe { SqlitePrimaryResult::from_i8(sqlite3_close(self.rp) as i8) }
    }
}
