use std::path::Path;

mod bindings;

pub mod connection;
pub mod operations;

pub struct Database {
    pub rp: *mut bindings::sqlite3,
}

pub trait Connection {
    fn db_open<T>(path: T) -> Self
    where
        T: AsRef<Path>;
    fn db_close(self);
}

pub trait Operations {
    fn exec_statement<F>(&self, statement: String, callback_fn: Option<F>) -> i32
    where
        F: FnOnce(i32, String);
}
