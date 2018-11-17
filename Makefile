.PHONY: all watch-and-test

all: watch-and-test

watch-and-test:
	cargo watch -x 'test -- --nocapture'
