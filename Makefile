CARGO := cargo --offline

.PHONY: all dev debug rel release test

all: release

dev: debug

debug:
	$(CARGO) build --lib

rel: release

release:
	$(CARGO) build --release --lib

test:
	$(CARGO) test --release -- --nocapture
