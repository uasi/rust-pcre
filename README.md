rust-pcre
=========

PCRE bindings for Rust.

SYNOPSIS
--------

```rust
extern mod std;
extern mod pcre;
use pcre::*;

/// 1. Basic Usage
fn basic_usage() {
    let pat = "b(.*)";
    let subj = "FooBar";
    let opts = PCRE_CASELESS;

    let search_result = search(pat, subj, opts);

    match search_result {
        Ok(match_obj) => {
            io::println(fmt!("match=%s, group1=%s",
                             match_obj.matched(),
                             match_obj.group(1)));
        }
        Err(regex_err) => match regex_err {
            CompileErr(comp_err) => {
                io::println(fmt!("compile failed: %s",
                                 fmt_compile_err(comp_err)));
            }
            ExecErr(_) => {
                io::println("pattern not matched");
            }
        }
    }
}

/// 2. Patterns and Pattern-Like Values
fn patterns() {
    // CompoleResult, Pattern, &str, ~str, and @str types are all
    // implement PatternLike trait.
    // So each of these values can be passed to search() etc. as a pattern.
    let comp_result: CompileResult = compile("pat.*", PCRE_CASELESS);
    let pat_obj    : Pattern       = comp_result.get();
    let static_str                 = "(?i)pat.*";
    let owned_str                  = ~"(?i)pat.*";
    let managed_str                = @"(?i)pat.*";

    assert search(comp_result, "Patty", 0).is_ok();
    assert search(pat_obj    , "Patty", 0).is_ok();
    assert search(static_str , "Patty", 0).is_ok();
    assert search(owned_str  , "Patty", 0).is_ok();
    assert search(managed_str, "Patty", 0).is_ok();
}

/// 3. Search Functions
fn search_fns() {
    let pat = "pattern";
    let subj = "subject";
    let offset = 3u;
    let opts = PCRE_CASELESS;

    let _ = search(pat, subj, opts);
    let _ = search_from(pat, subj, offset, opts);
}

/// 4. Replace Functions
fn replace_fns() {
    let pat = "pattern";
    let subj = "subject";
    let repl = "replacement";
    let offset = 3u;
    let opts = PCRE_CASELESS;

    let _ = replace(pat, subj, repl, opts);
    let _ = replace_from(pat, subj, repl, offset, opts);
    let _ = replace_fn(pat, subj, |match_obj| { repl }, opts);
    let _ = replace_fn_from(pat, subj, |match_obj| { repl }, offset, opts);
    // And replace_all_*() variants.
}
```

CONTRIBUTING
------------

1. Fork it
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create new Pull Request
