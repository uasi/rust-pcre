RUSTC := rustc
SOURCE := pcre.rc pcre.rs

all: build

build: $(SOURCE)
	$(RUSTC) --lib pcre.rc

test: clean build
	$(RUSTC) -L . --test test-pcre.rs
	./test-pcre

clean:
	rm -rf test-pcre *.dSYM *.dylib *.so
