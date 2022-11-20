.PHONY: test test_check clippy 

test:
	cargo test

test_check:
	cargo test -- --no-run

clippy:
	cargo clippy  -- -D warnings