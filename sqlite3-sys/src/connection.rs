use std::{ffi::CString, os::unix::prelude::OsStrExt, path::Path};

use crate::{
    bindings::{sqlite3_close, sqlite3_open},
    core::Database,
};

pub trait Connection {
    fn db_open<T>(path: T) -> Self
    where
        T: AsRef<Path>;
    fn db_close(self);
}

impl Connection for Database {
    fn db_open<T>(db_path: T) -> Self
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

    fn db_close(self) {
        unsafe {
            sqlite3_close(self.rp);
        }
    }
}
