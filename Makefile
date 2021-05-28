release: workflow/giphy-alfredworkflow

workflow/giphy-alfredworkflow: target/release/giphy-alfredworkflow
	cp -f target/release/giphy-alfredworkflow workflow/giphy-alfredworkflow

target/release/giphy-alfredworkflow: Cargo.* src/*
	cargo build --release
