use std::ffi::CString;

use crate::{bindings::sqlite3_exec, Database, Operations};

impl Operations for Database {
    fn exec_statement<F>(&self, statement: String, callback_fn: Option<F>) -> i32
    where
        F: FnOnce(i32, String),
    {
        let st = CString::new(&*statement).unwrap();
        unsafe {
            let status = sqlite3_exec(self.rp, st.as_ptr(), None, 0 as *mut _, 0 as *mut _);

            if callback_fn.is_some() && status != 0 {
                callback_fn.unwrap()(status, statement);
            }

            status
        }
    }
}
