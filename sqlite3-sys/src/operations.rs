use std::ffi::CString;

use crate::{bindings::sqlite3_exec, Database, Operations};

impl Operations for Database {
    fn exec_statement(&self, statement: String) {
        let st = CString::new(statement).unwrap();
        unsafe {
            sqlite3_exec(self.rp, st.as_ptr(), None, 0 as *mut _, 0 as *mut _);
        }
    }
}
