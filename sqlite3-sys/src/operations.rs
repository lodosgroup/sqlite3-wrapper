use std::ffi::CString;
use std::{ffi::CStr, os, ptr};

use crate::{
    bindings::{
        sqlite3_column_blob, sqlite3_column_bytes, sqlite3_column_double, sqlite3_column_int64,
        sqlite3_column_text, sqlite3_exec, sqlite3_prepare_v2,
    },
    core::{Database, SqlitePrimaryResult},
    statement::SqlStatement,
};

pub trait Get {
    fn read(statement: &SqlStatement, i: usize) -> Self;
}

impl Get for i8 {
    #[inline]
    fn read(statement: &SqlStatement, i: usize) -> Self {
        unsafe { sqlite3_column_int64(statement.0, i as os::raw::c_int) as Self }
    }
}

impl Get for u8 {
    #[inline]
    fn read(statement: &SqlStatement, i: usize) -> Self {
        unsafe { sqlite3_column_int64(statement.0, i as os::raw::c_int) as Self }
    }
}

impl Get for i16 {
    #[inline]
    fn read(statement: &SqlStatement, i: usize) -> Self {
        unsafe { sqlite3_column_int64(statement.0, i as os::raw::c_int) as Self }
    }
}

impl Get for u16 {
    #[inline]
    fn read(statement: &SqlStatement, i: usize) -> Self {
        unsafe { sqlite3_column_int64(statement.0, i as os::raw::c_int) as Self }
    }
}

impl Get for i32 {
    #[inline]
    fn read(statement: &SqlStatement, i: usize) -> Self {
        unsafe { sqlite3_column_int64(statement.0, i as os::raw::c_int) as Self }
    }
}

impl Get for u32 {
    #[inline]
    fn read(statement: &SqlStatement, i: usize) -> Self {
        unsafe { sqlite3_column_int64(statement.0, i as os::raw::c_int) as Self }
    }
}

impl Get for i64 {
    #[inline]
    fn read(statement: &SqlStatement, i: usize) -> Self {
        unsafe { sqlite3_column_int64(statement.0, i as os::raw::c_int) as Self }
    }
}

impl Get for f32 {
    #[inline]
    fn read(statement: &SqlStatement, i: usize) -> Self {
        unsafe { sqlite3_column_double(statement.0, i as os::raw::c_int) as Self }
    }
}

impl Get for f64 {
    #[inline]
    fn read(statement: &SqlStatement, i: usize) -> Self {
        unsafe { sqlite3_column_double(statement.0, i as os::raw::c_int) as Self }
    }
}

impl Get for &str {
    #[inline]
    fn read(statement: &SqlStatement, i: usize) -> Self {
        let result = unsafe { sqlite3_column_text(statement.0, i as os::raw::c_int) };

        unsafe { CStr::from_ptr(result as *const _).to_str().unwrap() }
    }
}

impl Get for String {
    #[inline]
    fn read(statement: &SqlStatement, i: usize) -> Self {
        let result = unsafe { sqlite3_column_text(statement.0, i as os::raw::c_int) };

        unsafe {
            CStr::from_ptr(result as *const _)
                .to_str()
                .unwrap()
                .to_owned()
        }
    }
}

impl Get for Vec<u8> {
    #[inline]
    fn read(statement: &SqlStatement, i: usize) -> Self {
        use std::ptr::copy_nonoverlapping as copy;
        unsafe {
            let pointer = sqlite3_column_blob(statement.0, i as os::raw::c_int);
            if pointer.is_null() {
                return vec![];
            }

            let count = sqlite3_column_bytes(statement.0, i as os::raw::c_int) as usize;
            let mut buffer = Vec::with_capacity(count);
            buffer.set_len(count);
            copy(pointer as *const u8, buffer.as_mut_ptr(), count);
            buffer
        }
    }
}

pub trait Operations {
    fn execute<F>(&self, statement: String, callback_fn: Option<F>) -> i32
    where
        F: FnOnce(i32, String);
    fn execute_and_read<F>(&self, statement: String, callback_fn: Option<F>) -> SqlStatement
    where
        F: FnOnce(i32, String);
}

impl Operations for Database {
    fn execute<F>(&self, statement: String, callback_fn: Option<F>) -> i32
    where
        F: FnOnce(i32, String),
    {
        let st = CString::new(&*statement).unwrap();
        unsafe {
            let status = sqlite3_exec(self.rp, st.as_ptr(), None, 0 as *mut _, 0 as *mut _);

            if callback_fn.is_some() && status != SqlitePrimaryResult::Ok as i32 {
                callback_fn.unwrap()(status, statement);
            }

            status
        }
    }

    fn execute_and_read<F>(&self, statement: String, callback_fn: Option<F>) -> SqlStatement
    where
        F: FnOnce(i32, String),
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
                callback_fn.unwrap()(status, statement);
            }
        }

        SqlStatement::new(stmt)
    }
}
