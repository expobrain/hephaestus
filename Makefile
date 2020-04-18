develop:
	pip install -e .

test:
	cargo test
	pytest tests/

fmt:
	cargo fmt
	isort -rc .
	black .

fix:
	cargo fix -Z unstable-options --clippy
