use std::path::Path;

mod bindings;

pub mod database;
pub mod operations;

pub struct Database {
    pub rp: *mut bindings::sqlite3,
}

pub trait Connection {
    fn db_open<T: AsRef<Path>>(path: T) -> Self;
    fn db_close(self);
}

pub trait Operations {
    fn exec_statement<F>(&self, statement: String, callback_fn: Option<F>)
    where
        F: FnOnce(i32, String);
}
