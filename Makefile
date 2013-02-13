RUSTC := rustc
RUSTC_OPTS :=
SOURCE := pcre.rc pcre.rs consts.rs

all: build

build: $(SOURCE)
	$(RUSTC) $(RUSTC_OPTS) --lib pcre.rc

test: clean build
	$(RUSTC) $(RUSTC_OPTS) -L . --test test-pcre.rs
	./test-pcre

clean:
	rm -rf test-pcre *.dSYM *.dylib *.so
