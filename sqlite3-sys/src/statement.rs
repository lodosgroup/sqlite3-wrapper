use crate::{
    bindings::{sqlite3_finalize, sqlite3_step, sqlite3_stmt},
    operations::ColumnCapabilities,
};

#[non_exhaustive]
#[repr(i32)]
pub enum PreparedStatementStatus {
    UnrecognizedStatus = -1,
    FoundRow,
    Done,
}

#[derive(Debug, Copy, Clone)]
pub struct SqlStatement(pub *mut sqlite3_stmt);

impl SqlStatement {
    #[inline]
    pub fn new(statement: *mut sqlite3_stmt) -> Self {
        Self(statement)
    }

    #[inline]
    pub fn execute_prepared(&mut self) -> PreparedStatementStatus {
        match unsafe { sqlite3_step(self.0) } {
            100 => PreparedStatementStatus::FoundRow,
            101 => PreparedStatementStatus::Done,
            _ => PreparedStatementStatus::UnrecognizedStatus,
        }
    }

    #[inline]
    pub fn get_data<T: ColumnCapabilities>(&self, i: usize) -> T {
        ColumnCapabilities::get_data(self.0, i)
    }

    #[inline]
    pub fn kill(&self) {
        unsafe { sqlite3_finalize(self.0) };
    }
}
