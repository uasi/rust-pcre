rust-pcre
=========

PCRE bindings for Rust.

SYNOPSIS
--------

    use std;
    use pcre;
    import pcre::*;
    import pcre::consts::*;

    fn main() {
      let m_result = match("b(..)", "FooBar", PCRE_CASELESS);
      let m = m_result.get();
      assert m.matched() == "Bar";
      assert m.group(1).get() == "ar";
    }

DESCRIPTION
-----------

Characteristics:

- Type-safe: compile(), match(), etc. return a core::result::result.
  Failed compilation or matching return full error infomation.
- Generic: Pattern can be a string, a compiled pattern object,
  and even the result that compile() yields.
