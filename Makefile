

gen-bindings:
	forge bind --bindings-path ./crates/bindings --crate-name bindings --optimize true --alloy --alloy-version v0.9.2 --optimizer-runs 3 --overwrite
