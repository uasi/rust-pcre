rust-pcre
=========

PCRE bindings for Rust.

SYNOPSIS
--------

    extern mod std;
    extern mod pcre;
    use pcre::*;

    fn main() {
        let s_result = search("b(..)", "FooBar", PCRE_CASELESS);
        let m = s_result.get();
        assert m.matched() == "Bar";
        assert m.group(1).get() == "ar";

        // Pattern can be a str, a Pattern, or a CompileResult.
        let _ = search("pat", "string", 0);
        let _ = search(compile("pat").get(), "string", 0);
        let _ = search(compile("pat"), "string", 0);
    }

DESCRIPTION
-----------

Characteristics:

- Type-safe: compile(), search(), etc. return a core::result::Result.
  Failed compilation or search return full error infomation.
- Generic: Pattern can be a string, a compiled pattern object,
  and even the result that compile() yields.
