check:
	cargo clippy
	cargo clippy --features=debug_env
	cargo test -- --nocapture
	cargo test --features=debug_env -- --nocapture

expand:
	cargo expand --lib --tests vecx > target/vecx.rs
	cargo expand --lib --tests mapx > target/mapx.rs
	cargo expand --lib --tests > target/all.rs

