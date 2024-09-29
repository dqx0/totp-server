start-go-server:
	go run server/go/main.go

start-go-client:
	go run example_client/go/main.go

start-rust-server:
	cargo run --bin totp-server

start-rust-client:
	cargo run --bin totp-client

# Go サーバー + Go クライアント
test-go-go:
	start cmd /c "make start-go-server"
	timeout 5
	make start-go-client

# Go サーバー + Rust クライアント
test-go-rust:
	start cmd /c "make start-go-server"
	timeout 5
	make start-rust-client

# Rust サーバー + Go クライアント
test-rust-go:
	start cmd /c "make start-rust-server"
	timeout 5
	make start-go-client

# Rust サーバー + Rust クライアント
test-rust-rust:
	start cmd /c "make start-rust-server"
	timeout 5
	make start-rust-client

# すべての組み合わせをテスト
test-all: test-go-go test-go-rust test-rust-go test-rust-rust
