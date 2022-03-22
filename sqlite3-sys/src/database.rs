use std::{ffi::CString, os::unix::prelude::OsStrExt, path::Path};

use crate::{
    bindings::{sqlite3_close, sqlite3_open},
    Connection, Database,
};

impl Connection for Database {
    fn db_open<T: AsRef<Path>>(db_path: T) -> Self {
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
