use crate::{
    bindings::{sqlite3_step, sqlite3_stmt, QueryStatus},
    operations::Get,
};

#[derive(Debug, Copy, Clone)]
pub struct SqlStatement(pub *mut sqlite3_stmt);

impl SqlStatement {
    #[inline]
    pub fn new(statement: *mut sqlite3_stmt) -> Self {
        Self(statement)
    }

    #[inline]
    pub fn get(&mut self) -> QueryStatus {
        match unsafe { sqlite3_step(self.0) } {
            100 => QueryStatus::FoundRow,
            101 => QueryStatus::Done,
            _ => QueryStatus::UnrecognizedStatus,
        }
    }

    #[inline]
    pub fn read<T: Get>(&self, i: usize) -> T {
        Get::read(self, i)
    }
}
