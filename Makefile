RUSTC := rustc
SOURCE := pcre.rc pcre.rs

all: build

build: $(SOURCE)
	$(RUSTC) --lib pcre.rc

test: build
	$(RUSTC) --test pcre.rs
	./pcre

clean:
	rm -rf pcre pcre.dSYM libpcre-*
