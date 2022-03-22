use std::os;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sqlite3 {
    __: [u8; 0],
}

extern "C" {
    pub fn sqlite3_open(file_path: *const os::raw::c_char, db: *mut *mut sqlite3)
        -> os::raw::c_int;
}

extern "C" {
    pub fn sqlite3_close(db: *mut sqlite3) -> os::raw::c_int;
}

extern "C" {
    pub fn sqlite3_exec(
        db: *mut sqlite3,
        sql_statement: *const os::raw::c_char,
        callback: Option<
            unsafe extern "C" fn(
                a: *mut os::raw::c_void,
                b: os::raw::c_int,
                c: *mut *mut os::raw::c_char,
                d: *mut *mut os::raw::c_char,
            ) -> os::raw::c_int,
        >,
        callback_a: *mut os::raw::c_void,
        errmsg: *mut *mut os::raw::c_char,
    ) -> os::raw::c_int;
}
