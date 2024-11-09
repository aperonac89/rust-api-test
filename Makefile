prepare_project:
	cargo add actix_web
	cargo add actix_cors
	cargo add serde --features derive
	cargo add serde_json
	cargo add env_logger
	cargo add chrono --features serde
	cargo add dotenv

run:
	RUST_LOG=main=debug cargo watch -x run