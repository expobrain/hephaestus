rust:
	cp Cargo.base.toml Cargo.toml
	cat Cargo.test.toml >> Cargo.toml

py:
	cp Cargo.base.toml Cargo.toml
	cat Cargo.setuptools.toml >> Cargo.toml

develop: py
	pip install -e .

test_rust: rust
	cargo test

test_py: develop
	pytest tests/

fmt:
	cargo fmt
	isort -rc .
	black .

fix:
	cargo fix -Z unstable-options --clippy
