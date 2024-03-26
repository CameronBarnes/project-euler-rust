set dotenv-load := true

work problem:
		nvim problem-{{problem}}

create problem:
		cargo generate --path ./template --name problem-{{problem}}

bench-all:
		cargo bench -q > benchmarks.txt

bench problem:
		cargo bench --bench problem-{{problem}}

test-all:
		cargo nextest r

test problem:
		cargo nextest r -p problem-{{problem}}

run problem:
		cargo run --release -p problem-{{problem}} --bin solution
