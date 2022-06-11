.PHONY: run
run:
	cargo run --bin flexvg -- ./examples/simple.yaml
	cargo run --bin flexvg -- ./examples/gridish.yaml
	cargo run --bin flexvg -- ./examples/platform.yaml