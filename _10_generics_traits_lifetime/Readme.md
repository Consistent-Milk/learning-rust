Both of these commands does the same thing in this case, but tests in Rust are supposed to be formatted and used through #[test] and cargo test. So cargo run shouldn't be used.

cargo run -> Compiles and runs as binary executable
cargo test -> Runs the unit test on main.rs
