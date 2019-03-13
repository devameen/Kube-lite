
# Hugely accelerates release builds if registry has already been cached.
CARGO_CACHE = ~/.cargo
ifneq "$(wildcard $(CARGO_CACHE) )" ""
	CACHE_ARG = -v $(CARGO_CACHE)/git:/home/rust/.cargo/git -v $(CARGO_CACHE)/registry:/home/rust/.cargo/registry
else
	CACHE_ARG =
endif

clean:
	if [ -d "target" ]; then \
		rm -rf target ; \
	fi

build:
	cargo build

test:
	cargo test

build-release:	# musl build
	docker run --rm -t $(CACHE_ARG) -v $$(pwd):/home/rust/src ekidd/rust-musl-builder:nightly cargo build --release
