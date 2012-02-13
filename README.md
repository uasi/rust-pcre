rust-pcre
=========

A PCRE binding For Rust.

Characteristics:

- Type-safe: compile(), match(), etc. return a result::t.
  Failed compilation or matching return full error infomation.
- Generic: match() accepts a pattern string, a compiled pattern object,
  and even the result::t that compile() yields.
