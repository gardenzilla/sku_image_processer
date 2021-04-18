include ../ENV.list
export $(shell sed 's/=.*//' ../ENV.list)

.PHONY: release, test, dev, run

release:
	cargo update
	cargo build --release
	strip target/release/sku_imgprocesser_microservice

build:
	cargo update
	cargo build

run:
	cargo update
	cargo run

dev:
	# . ./ENV.sh; backper
	cargo run;

test:
	cargo test