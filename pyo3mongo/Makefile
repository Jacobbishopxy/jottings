ifneq (,$(wildcard .env))
    include .env
    export
endif

pyenv:
	conda activate py310

pydep:
	pip install maturin

dev:
	maturin develop

# run:
# 	python pyo3.py

create_vertex:
	cargo run --bin create_vertex -- -u ${MONGO_URI} --name "node-1"

# as an example, we do not know source & target ids in advance
# create_edge:
# 	cargo run --bin create_edge -- -u ${MONGO_URI} --source=xxx --target=xxx

truncate_all:
	cargo run --bin truncate_all -- -u ${MONGO_URI}
