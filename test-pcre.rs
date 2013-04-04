extern mod std;
extern mod pcre;

use pcre::*;

#[cfg(test)]
mod test_util {
    pub trait OptionUtil<T> {
        fn is_some_and(blk: &fn(T) -> bool) -> bool;
        fn is_none_and(blk: &fn() -> bool) -> bool;
    }

    impl<T: Copy> Option<T>: OptionUtil<T> {
        fn is_some_and(blk: &fn(T) -> bool) -> bool {
            match self {
                Some(t) => blk(t),
                None => false,
            }
        }

        // Who wants?
        fn is_none_and(blk: &fn() -> bool) -> bool {
            match self {
                Some(_) => false,
                None => blk(),
            }
        }
    }

    pub trait ResultUtil<T, U> {
        fn is_ok_and(blk: &fn(T) -> bool) -> bool;
        fn is_err_and(blk: &fn(U) -> bool) -> bool;
    }

    impl<T: Copy, U: Copy> Result<T, U>: ResultUtil<T, U> {
        fn is_ok_and(blk: &fn(T) -> bool) -> bool {
            match self {
                Ok(t) => blk(t),
                Err(_) => false,
            }
        }

        fn is_err_and(blk: &fn(U) -> bool) -> bool {
            match self {
                Ok(_) => false,
                Err(u) => blk(u),
            }
        }
    }

    #[test]
    fn test_option_util() {
        let s = Some(42);

        assert  s.is_some();
        assert  s.is_some_and(|i| i == 42);
        assert !s.is_some_and(|i| i != 42);

        assert !s.is_none();
        assert !s.is_none_and(|| true);
        assert !s.is_none_and(|| false);

        let n = None::<()>;

        assert  n.is_none();
        assert  n.is_none_and(|| true);
        assert !n.is_none_and(|| false);

        assert !n.is_some();
        assert !n.is_some_and(|_nil| true);
        assert !n.is_some_and(|_nil| false);

    }

    #[test]
    fn test_result_util() {
        let o: Result<int, ()> = Ok(42);

        assert  o.is_ok();
        assert  o.is_ok_and(|i| i == 42);
        assert !o.is_ok_and(|i| i != 42);

        assert !o.is_err();
        assert !o.is_err_and(|_nil| true);
        assert !o.is_err_and(|_nil| false);

        let e: Result<(), int> = Err(42);

        assert  e.is_err();
        assert  e.is_err_and(|i| i == 42);
        assert !e.is_err_and(|i| i != 42);

        assert !e.is_ok();
        assert !e.is_ok_and(|_nil| true);
        assert !e.is_ok_and(|_nil| false);
    }
}

#[cfg(test)]
mod tests {
    use test_util::*;
    use pcre::*;

    #[test]
    fn test_compile() {
        let r = compile("foo", 0);
        assert r.is_ok();

        let r = compile("foo(", 0);
        assert do r.is_err_and |e| {
            assert e.code == 14;
            assert e.reason == @~"missing )";
            assert e.offset == 4u;
            true
        }
    }

    #[test]
    fn test_search() {
        let r = search("(foo)bar", "foobar", 0);
        assert r.is_ok();

        let r = search(~"(foo)bar", "foobar", 0);
        assert r.is_ok();

        let r = search(@"(foo)bar", "foobar", 0);
        assert r.is_ok();

        let c = compile("(foo)bar", 0);
        let p = c.get();

        let r = search(p, "foobar", 0);
        assert r.is_ok();

        let r = search(c, "foobar", 0);
        assert r.is_ok();

        let r = search("foo(", "foobar", 0);
        match r {
            Err(CompileErr(e)) => {
                assert e.code == 14;
                assert e.reason == @~"missing )";
                assert e.offset == 4u;
            }
            _ => { die!(); }
        }

        let r = search("(foo)bar", "baz", 0);
        assert is_nomatch(r);

        let r = search("はにほ", "いろはにほへと", 0);
        assert r.is_ok_and(|m| m.begin() == 6u && m.end() == 15u);

        let r = search("ちりぬ", "いろはにほへと", 0);
        assert is_nomatch(r);
    }

    // compile() accepts compile options (obviously)
    #[test]
    fn test_search_options_0() {
        let r = search(compile("foobar", PCRE_CASELESS), "FOOBAR", 0);
        assert r.is_ok();
    }

    // search() accepts compile options
    #[test]
    fn test_search_options_1() {
        let r = search("foobar", "FOOBAR", PCRE_CASELESS);
        assert r.is_ok();
    }

    // Inline options supersedes exec-time compile options
    #[test]
    fn test_search_options_2() {
        let r = search("(?-i)foobar", "FOOBAR", PCRE_CASELESS);
        assert r.is_err();
    }

    // Compile-time compile options supersedes exec-time compile options
    #[test]
    fn test_search_options_3() {
        let r = search(compile("foobar", 0), "FOOBAR", PCRE_CASELESS);
        assert r.is_err();
    }

    #[test]
    fn test_replace() {
        let r = replace("bcd", "AbcdbcdbcdE", "BCD", 0);
        assert r.is_ok_and(|s| s == @~"ABCDbcdbcdE");
    }

    #[test]
    fn test_replace_from() {
        let r = replace_from("bcd", "AbcdbcdbcdE", "BCD", 2u, 0);
        assert r.is_ok_and(|s| s == @~"AbcdBCDbcdE");
    }

    #[test]
    fn test_replace_fn() {
        let r = replace_fn("bcd", "AbcdbcdbcdE",
                           |m| { str::to_upper(m.matched()) }, 0);
        assert r.is_ok_and(|s| s == @~"ABCDbcdbcdE");
    }

    #[test]
    fn test_replace_fn_from() {
        let r = replace_fn_from("bcd", "AbcdbcdbcdE",
                                |m| { str::to_upper(m.matched()) }, 2u, 0);
        assert r.is_ok_and(|s| s == @~"AbcdBCDbcdE");
    }

    #[test]
    fn test_replace_all() {
        let r = replace_all("bcd", "AbcdbcdbcdE", "BCD", 0);
        assert r.is_ok_and(|s| s == @~"ABCDBCDBCDE");
    }

    #[test]
    fn test_replace_all_from() {
        let r = replace_all_from("bcd", "AbcdbcdbcdE", "BCD", 2u, 0);
        assert r.is_ok_and(|s| s == @~"AbcdBCDBCDE");
    }

    #[test]
    fn test_replace_all_fn() {
        let r = replace_all_fn("bcd", "AbcdbcdbcdE",
                                |m| { str::to_upper(m.matched()) }, 0);
        assert r.is_ok_and(|s| s == @~"ABCDBCDBCDE");
    }

    #[test]
    fn test_replace_all_fn_from() {
        let r = replace_all_fn_from("bcd", "AbcdbcdbcdE",
                                    |m| { str::to_upper(m.matched()) }, 2u, 0);
        assert r.is_ok_and(|s| s == @~"AbcdBCDBCDE");
    }
}

#[cfg(test)]
mod test_match_extensions {
    use test_util::*;
    use pcre::*;

    #[test]
    fn test_group() {
        let r = search("(foo)bar(baz)", "foobarbaz", 0);
        assert do r.is_ok_and |m| {
            assert m.group(0u).is_some_and(|s| s == @~"foobarbaz");
            assert m.group(1u).is_some_and(|s| s == @~"foo");
            assert m.group(2u).is_some_and(|s| s == @~"baz");
            assert m.group(3u).is_none();
            true
        }
    }

    #[test]
    fn test_subgroups() {
        let r = search("(foo)bar(baz)", "foobarbaz", 0);
        assert do r.is_ok_and |m| {
            do vec::all2(m.subgroups(), ~[~"foo", ~"baz"]) |s, t| { s == t }
        }
    }

    #[test]
    fn test_group_count() {
        let r = search("foobarbaz", "foobarbaz", 0);
        assert r.is_ok_and(|m| m.group_count() == 0u);

        let r = search("(foo)bar(baz)", "foobarbaz", 0);
        assert r.is_ok_and(|m| m.group_count() == 2u);

        let r = search("(?:foo)bar", "foobar", 0);
        assert r.is_ok_and(|m| m.group_count() == 0u);

        let r = search("(?:(foo)|baz)bar", "foobar", 0);
        assert r.is_ok_and(|m| m.group_count() == 1u);

        let r = search("(?:foo|(baz))bar", "foobar", 0);
        assert r.is_ok_and(|m| m.group_count() == 1u);
    }

    #[test]
    fn test_unmatched_group() {
        let r = search("((foo)|bar)_(baz)", "bar_baz", 0);
        assert do r.is_ok_and |m| {
            assert m.group(1u).is_some_and(|s| s == @~"bar");
            assert m.group(2u).is_none();
            assert m.group(3u).is_some_and(|s| s == @~"baz");
            true
        };

        let r = search("(foo)?(bar)(baz)", "barbaz", 0);
        assert do r.is_ok_and |m| {
            assert m.group(1u).is_none();
            assert m.group(2u).is_some_and(|s| s == @~"bar");
            assert m.group(3u).is_some_and(|s| s == @~"baz");
            true
        };

        let r = search("(foo)?(?<bar_name>bar)", "bar", 0);
        assert do r.is_ok_and |m| {
            assert m.named_group("bar_name").is_some_and(|s| s == @~"bar");
            true
        };
    }

    #[test]
    fn test_group_names() {
        let r = search("(?<foo_name>foo)bar", "foobar", 0);
        assert do r.is_ok_and |m| {
            do vec::all2(m.group_names(), ~[~"foo_name"]) |s, t| { s == t }
        }
    }

    #[test]
    fn test_named_group() {
        let r = search("(?<foo_name>f..)bar", "foobar", 0);
        assert do r.is_ok_and |m| {
            assert m.named_group("foo_name").is_some_and(|s| s == @~"foo");
            assert m.named_group("nonexistent").is_none();
            true
        }
    }
}
