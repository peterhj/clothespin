CARGO := cargo --offline

.PHONY: all dev debug rel release test

all: release

dev: debug

debug:
	$(CARGO) build --lib --bins --examples

rel: release

release:
	$(CARGO) build --release --lib --bins --examples

test:
	$(CARGO) test --release -- --nocapture
