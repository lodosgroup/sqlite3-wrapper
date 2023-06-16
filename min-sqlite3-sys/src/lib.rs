//! Very minimal sqlite wrapper package built specifically for lod package manager and Unix systems. If you need complete box of sqlite database, consider using [rusqlite](https://github.com/rusqlite/rusqlite).
//!
//! ## Adding lib to the project
//! In your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! min-sqlite3-sys = "1.4"
//! ```
//!
//! In build.rs of your binary crate:
//! ```rust
//! use std::{env, path::Path};
//!
//! fn main() {
//!     let home_path = env::var("HOME").expect("HOME environment variable is not set.");
//!     let target_dir = Path::new(&home_path).join(".local/share/min_sqlite3_sys");
//!
//!     println!("cargo:rustc-link-arg=-Wl,-rpath={}", target_dir.display());
//! }
//! ```
//!
//! ## Usage
//! Simple usage:
//!
//! ```rust
//! use std::path::Path;
//!
//! use min_sqlite3_sys::prelude::*;
//!
//! fn main() {
//!     let db = Database::open(Path::new("example.db")).unwrap();
//!     let statement = String::from(
//!         "CREATE TABLE IF NOT EXISTS items(
//!                  id      PRIMARY KEY,
//!                  name    TEXT,
//!                  tag     TEXT
//!              );
//!          ",
//!     );
//!
//!     let status = db.execute(
//!         statement,
//!         None::<Box<dyn FnOnce(SqlitePrimaryResult, String)>>,
//!     ).unwrap();
//!
//!     if status != SqlitePrimaryResult::Ok {
//!         // handle the problem
//!     }
//!
//!     db.close();
//! }
//! ```
//!
//! Simple usage with callback function:
//! ```rust
//! use std::path::Path;
//!
//! use min_sqlite3_sys::prelude::*;
//!
//! fn callback_function(status: SqlitePrimaryResult, sql_statement: String) {
//!     println!(
//!         "{} did not successfully executed. The error status is: {:?}.",
//!         sql_statement, status
//!     );
//! }
//!
//! fn main() {
//!     let db = Database::open(Path::new("example.db")).unwrap();
//!     let statement = String::from(
//!         "CREATE TABLE IF NOT EXISTS items(
//!                  id      PRIMARY KEY,
//!                  name    TEXT,
//!                  tag     TEXT
//!              );
//!          ",
//!     );
//!
//!     db.execute(statement, Some(callback_function)).unwrap();
//!
//!     db.close();
//! }
//! ```
//!
//! Simple usage with retrieving some data:
//! ```rust
//! #![allow(dead_code)]
//! use std::path::Path;
//!
//! use min_sqlite3_sys::prelude::*;
//!
//! fn callback_function(status: SqlitePrimaryResult, sql_statement: String) {
//!     println!(
//!         "{} did not successfully executed. The error status is: {:?}.",
//!         sql_statement, status
//!     );
//! }
//!
//! #[derive(Debug)]
//! struct Item {
//!     id: i64,
//!     name: String,
//!     tag: String,
//! }
//!
//! fn main() {
//!     let db = Database::open(Path::new("example.db")).unwrap();
//!     let statement = String::from("SELECT * FROM items WHERE name = 'Onur';");
//!
//!     let mut sql = db.prepare(statement, Some(callback_function)).unwrap();
//!
//!     // Iterate the results
//!     while let PreparedStatementStatus::FoundRow = sql.execute_prepared() {
//!         println!(
//!             "id = {}, name = {}, tag = {}",
//!             sql.get_data::<i64>(0).unwrap(),
//!             sql.get_data::<String>(1).unwrap(),
//!             sql.get_data::<String>(2).unwrap(),
//!         );
//!
//!         // Or simply
//!         println!(
//!             "{:?}",
//!             Item {
//!                 id: sql.get_data(0).unwrap(),
//!                 name: sql.get_data(1).unwrap(),
//!                 tag: sql.get_data(2).unwrap(),
//!             }
//!         );
//!     }
//!     // Must be called for each `prepare()` result.
//!     sql.kill();
//!
//!     db.close();
//! }
//! ```
//!
//! Simple usage with retrieving some data + binding values:
//! ```rust
//! #![allow(dead_code)]
//! use std::path::Path;
//!
//! use min_sqlite3_sys::prelude::*;
//!
//! fn callback_function(status: SqlitePrimaryResult, sql_statement: String) {
//!     println!(
//!         "{} did not successfully executed. The error status is: {:?}.",
//!         sql_statement, status
//!     );
//! }
//!
//! #[derive(Debug)]
//! struct Item {
//!     id: i64,
//!     name: String,
//!     tag: String,
//! }
//!
//! fn main() {
//!     let db = Database::open(Path::new("example.db")).unwrap();
//!     let statement = String::from("SELECT * FROM items WHERE name = ?;");
//!
//!     let mut sql = db.prepare(statement, Some(callback_function)).unwrap();
//!     let status = sql.bind_val(1, "Onur");
//!     // You can do some checks by
//!     assert_eq!(status, SqlitePrimaryResult::Ok);
//!     // or
//!     if status == SqlitePrimaryResult::Range {
//!         panic!("Out of index on sql.bind_val!");
//!     }
//!
//!     // Iterate the results
//!     while let PreparedStatementStatus::FoundRow = sql.execute_prepared() {
//!         println!(
//!             "id = {}, name = {}, tag = {}",
//!             sql.get_data::<i64>(0).unwrap(),
//!             sql.get_data::<String>(1).unwrap(),
//!             sql.get_data::<String>(2).unwrap(),
//!         );
//!
//!         // Or simply
//!         println!(
//!             "{:?}",
//!             Item {
//!                 id: sql.get_data(0).unwrap(),
//!                 name: sql.get_data(1).unwrap(),
//!                 tag: sql.get_data(2).unwrap(),
//!             }
//!         );
//!     }
//!     // Must be called for each `prepare()` result.
//!     sql.kill();
//!
//!     db.close();
//! }
//! ```
//!
//! ## Notes
//! In order to not inflate the build outputs of your projects, the library executes sqlite functions from dynamic library using C ABI via FFI. Meaning, your build output will not include sqlite sources.
//!
//! This library does not use any SQLite library on your system to ensure that the package doesn't get affected by SQLite versions. Instead, the sqlite3-builder crate compiles the sqlite3 sources under the c_source directory as dynamic library and puts that under the '~/.local/share/min_sqlite3_sys'.

#![allow(clippy::needless_doctest_main)]

pub mod bindings;
pub mod connection;
pub mod ehandle;
pub mod operations;
pub mod statement;

pub mod prelude;
