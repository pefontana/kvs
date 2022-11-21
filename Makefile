.PHONY: test test_check clippy t

test:
	cargo test

test_check:
	cargo test no-run

t:
	cargo test --tests

clippy:
	cargo clippy  -- -D warnings