RUSTC := rustc
RUSTC_OPTS :=
SOURCE := pcre.rc pcre.rs consts.rs

all: build

build: $(SOURCE)
	$(RUSTC) $(RUSTC_OPTS) --lib pcre.rc

test: clean
	$(RUSTC) $(RUSTC_OPTS) -L . --test pcre.rc -o pcretest~
	./pcretest~

clean:
	rm -rf pcre pcretest~ *.dSYM *.dylib *.so
