use std;

import core::ctypes::*;
import core::either::{left, right};
import core::option::{some, none};
import core::result::{ok, err};
import either = core::either::t;
import result = core::result::t;

const PCRE_CASELESS: int          = 0x00000001; // Compile
const PCRE_MULTILINE: int         = 0x00000002; // Compile
const PCRE_DOTALL: int            = 0x00000004; // Compile
const PCRE_EXTENDED: int          = 0x00000008; // Compile
const PCRE_ANCHORED: int          = 0x00000010; // Compile, exec, DFA exec
const PCRE_DOLLAR_ENDONLY: int    = 0x00000020; // Compile
const PCRE_EXTRA: int             = 0x00000040; // Compile
const PCRE_NOTBOL: int            = 0x00000080; // Exec, DFA exec
const PCRE_NOTEOL: int            = 0x00000100; // Exec, DFA exec
const PCRE_UNGREEDY: int          = 0x00000200; // Compile
const PCRE_NOTEMPTY: int          = 0x00000400; // Exec, DFA exec
const PCRE_UTF8: int              = 0x00000800; // Compile
const PCRE_NO_AUTO_CAPTURE: int   = 0x00001000; // Compile
const PCRE_NO_UTF8_CHECK: int     = 0x00002000; // Compile, exec, DFA exec
const PCRE_AUTO_CALLOUT: int      = 0x00004000; // Compile
const PCRE_PARTIAL_SOFT: int      = 0x00008000; // Exec, DFA exec
const PCRE_PARTIAL: int           = 0x00008000; // Backwards compatible synonym
const PCRE_DFA_SHORTEST: int      = 0x00010000; // DFA exec
const PCRE_DFA_RESTART: int       = 0x00020000; // DFA exec
const PCRE_FIRSTLINE: int         = 0x00040000; // Compile
const PCRE_DUPNAMES: int          = 0x00080000; // Compile
const PCRE_NEWLINE_CR: int        = 0x00100000; // Compile, exec, DFA exec
const PCRE_NEWLINE_LF: int        = 0x00200000; // Compile, exec, DFA exec
const PCRE_NEWLINE_CRLF: int      = 0x00300000; // Compile, exec, DFA exec
const PCRE_NEWLINE_ANY: int       = 0x00400000; // Compile, exec, DFA exec
const PCRE_NEWLINE_ANYCRLF: int   = 0x00500000; // Compile, exec, DFA exec
const PCRE_BSR_ANYCRLF: int       = 0x00800000; // Compile, exec, DFA exec
const PCRE_BSR_UNICODE: int       = 0x01000000; // Compile, exec, DFA exec
const PCRE_JAVASCRIPT_COMPAT: int = 0x02000000; // Compile
const PCRE_NO_START_OPTIMIZE: int = 0x04000000; // Compile, exec, DFA exec
const PCRE_NO_START_OPTIMISE: int = 0x04000000; // Synonym
const PCRE_PARTIAL_HARD: int      = 0x08000000; // Exec, DFA exec
const PCRE_NOTEMPTY_ATSTART: int  = 0x10000000; // Exec, DFA exec
const PCRE_UCP: int               = 0x20000000; // Compile

enum pcre {}
enum pcre_extra {}
resource pcre_res(p: *pcre) {
    c::free(p as *void);
}

/*
Type: exec_result

The result type of <exec>.
*/
type exec_result = result<match, match_err>;

/*
Type: match_result

The result type of <match>.
*/
type match_result = result<match, either<compile_err, match_err>>;

/*
Type: compile_result

The result type of <compile>.
*/
type compile_result = result<@pcre_res, compile_err>;

/*
Type: match_err

The type that represents match error.
*/
type match_err = int;

/*
Type: compile_err

The type that represents compile error.
*/
type compile_err = {
    code: int,
    reason: str,
    offset: uint,
};

type either_err = either<compile_err, match_err>;

#[nolink]
#[abi = "cdecl"]
native mod c {
    fn free(p: *void);
}

#[abi = "cdecl"]
native mod pcre {
    fn pcre_compile2(pattern: *c_char, options: c_int,
                     errorcodeptr: *c_int,
                     errptr: **c_char, erroffset: *c_int,
                     tableptr: *c_char) -> *pcre;
    fn pcre_exec(code: *pcre, extra: *pcre_extra,
                 subject: *c_char, length: c_int, startoffset: c_int,
                 options: c_int, ovector: * c_int, ovecsize: c_int) -> c_int;
    fn pcre_fullinfo(code: *pcre, extra: *pcre_extra,
                     what: c_int, where: *c_int) -> c_int;
}

fn compile(pattern: str, options: int) -> compile_result unsafe {
    let options = options | PCRE_NO_UTF8_CHECK; // str is always valid
    let errcode = 0 as c_int;
    let errreason: *c_char = ptr::null();
    let erroffset = 0 as c_int;
    let p = str::as_buf(pattern) {|pat|
        pcre::pcre_compile2(pat as *c_char,
                            options as c_int,
                            ptr::addr_of(errcode),
                            ptr::addr_of(errreason),
                            ptr::addr_of(erroffset),
                            ptr::null())
    };
    if p == ptr::null() {
        let offset = char_offset_from_byte_offset(pattern, erroffset as uint);
        ret err({code: errcode as int,
                 reason: str::from_cstr(errreason as *u8),
                 offset: offset});
    }
    ret ok(@pcre_res(p));
}

fn exec(pcre_res: @pcre_res,
        subject: str, offset: uint,
        options: int, count_FIXME: int) -> exec_result unsafe {
    // FIXME: Weirdly pcre_fullinfo() doesn't work inside exec().
    //        It yields 0, which means success, but the 4th arg isn't updated.
    //let count = 0 as c_int;
    //pcre::pcre_fullinfo(**pcre_res, ptr::null(),
    //                    2 as c_int /* PCRE_INFO_CAPTURECOUNT */,
    //                    ptr::addr_of(count));
    let count = (count_FIXME + 1) as c_int;

    let offset = byte_offset_from_char_offset(subject, offset);
    let options = options | PCRE_NO_UTF8_CHECK; // str is always valid

    let ovec = vec::init_elt((count as uint) * 3u, 0u as c_int);
    let ret_code = str::as_buf(subject) {|s|
        pcre::pcre_exec(**pcre_res, ptr::null(),
                        s as *c_char, str::byte_len(subject) as c_int,
                        offset as c_int, options as c_int,
                        vec::to_ptr(ovec) as *c_int,
                        count * (3 as c_int)) as int
    };

    if ret_code < 0 { ret err(ret_code as match_err); }

    // cut off the working space
    vec::unsafe::set_len(ovec, count as uint * 2u);

    let captures: [uint] = [];
    vec::reserve(captures, vec::len(ovec));
    for o in ovec {
        if o as int < 0 { cont; }
        vec::push(captures, char_offset_from_byte_offset(subject, o as uint));
    }
    assert vec::len(captures) % 2u == 0u;

    ret ok({subject: subject, _pcre_res: pcre_res, _captures: captures});
}

type match = {
    subject: str,
    _pcre_res: @pcre_res,
    _captures: [uint],
    // FIXME: we may cache these values for reuse
    // mutable _substrs: option<[str]>,
    // mutable _names: option<std::map<str, uint>>,
};

iface match_like {
    fn matched() -> str;
    fn prematch() -> str;
    fn postmatch() -> str;
    fn begin() -> uint;
    fn end() -> uint;

    fn group(uint) -> option<str>;
    fn named_group(str) -> option<str>;
    fn groups() -> [str];
    fn groups_iter(fn(str));
    fn group_count() -> uint;
    fn group_names() -> [str];
}

impl of match_like for match {
    fn matched() -> str {
        ret str::slice(self.subject, self.begin(), self.end());
    }

    fn prematch() -> str {
        ret str::slice(self.subject, 0u, self.begin());
    }

    fn postmatch() -> str {
        ret str::slice(self.subject ,self.end(),
                       str::char_len(self.subject));
    }

    fn begin() -> uint {
        ret self._captures[0];
    }

    fn end() -> uint {
        ret self._captures[1];
    }

    fn group(idx: uint) -> option<str> {
        if idx >= self.group_count() {
            ret none;
        }
        ret some(str::slice(self.subject,
                            self._captures[(idx + 1u) * 2u],
                            self._captures[(idx + 1u) * 2u + 1u]));
    }

    fn named_group(name: str) -> option<str> {
        (name);
        fail;
    }

    fn groups() -> [str] {
        let v = [];
        vec::reserve(v, self.group_count());
        self.groups_iter {|elt| vec::push(v, elt); }
        ret v;
    }

    fn groups_iter(blk: fn(str)) {
        uint::range(0u, self.group_count()) {|i|
            alt self.group(i) {
              some(s) { blk(s); }
              none { fail; }
            }
        }
    }

    fn group_count() -> uint {
        ret vec::len(self._captures) / 2u - 1u;
    }

    fn group_names() -> [str] {
        fail;
    }
}

iface pattern_like {
    fn compile() -> compile_result;
}

impl of pattern_like for str {
    fn compile() -> compile_result { compile(self, 0) }
}

impl of pattern_like for @pcre_res {
    fn compile() -> compile_result { ok(self) }
}

impl of pattern_like for compile_result {
    fn compile() -> compile_result { self }
}

fn match_from<T: pattern_like>(pattern: T, subject: str,
                               offset: uint, options: int) -> match_result {
    let c = pattern.compile();
    alt c {
      ok(p) {
        // FIXME: see exec()
        let count_FIXME = 0 as c_int;
        pcre::pcre_fullinfo(**p, ptr::null(),
                            2 as c_int /* PCRE_INFO_CAPTURECOUNT */,
                            ptr::addr_of(count_FIXME));
        let e = exec(p, subject, offset, options, count_FIXME as int);
        alt e {
          ok(match) {
            ret ok(match);
          }
          err(match_err) {
            ret err(right(match_err));
          }
        }
      }
      err(compile_err) {
          ret err(left(compile_err));
      }
    }
}

fn match<T: pattern_like>(pattern: T, subject: str,
                          options: int) -> match_result {
    ret match_from(pattern, subject, 0u, options);
}

fn fmt_compile_err(e: compile_err) -> str {
    ret #fmt("error %d: %s at offset %u", e.code, e.reason, e.offset);
}

fn char_offset_from_byte_offset(s: str, byte_offset: uint) -> uint {
    if byte_offset == 0u { ret 0u; }
    let v = str::bytes(s);
    let subv = vec::slice(v, 0u, byte_offset);
    let subs = str::from_bytes(subv);
    ret str::char_len(subs);
}

fn byte_offset_from_char_offset(s: str, char_offset: uint) -> uint {
    if char_offset == 0u { ret 0u; }
    let subs = str::slice(s, 0u, char_offset);
    let v = str::bytes(subs);
    ret vec::len(v);
}

fn replace<T: pattern_like>(pattern: T, subject: str, repl: str,
                            options: int) -> result<str, either_err> {
    ret replace_fn_from(pattern, subject, {|_m| repl }, 0u, options);
}

fn replace_from<T: pattern_like>(pattern: T, subject: str, repl: str,
                                 offset: uint, options: int)
                                 -> result<str, either_err> {
    ret replace_fn_from(pattern, subject, {|_m| repl }, offset, options);
}

fn replace_fn<T: pattern_like>(pattern: T, subject: str,
                               repl_fn: fn(match) -> str, options: int)
                               -> result<str, either_err> {
    ret replace_fn_from(pattern, subject, repl_fn, 0u, options);
}

fn replace_fn_from<T: pattern_like>(pattern: T, subject: str,
                                    repl_fn: fn(match) -> str, offset: uint,
                                    options: int)
                                    -> result<str, either_err> {
    let r = match_from(pattern, subject, offset, options);
    alt r {
      ok(m) {
        ret ok(m.prematch() + repl_fn(m) + m.postmatch());
      }
      err(e) { ret err(e); }
    }
}

//fn replace_all<T: pattern_like>(pattern: T, subject: str, repl: str,
//                                options: int) -> result<str, either_err> {
//    fail;
//}
//
//fn replace_all_<T: pattern_like>(pattern: T, subject: str,
//                                 repl_fn: fn(match) -> str, options: int)
//                                 -> result<str, either_err> {
//    fail;
//}

pure fn is_nomatch(mr: match_result) -> bool {
    ret alt mr {
      err(right(-1)) { true }
      _ { false }
    };
}

#[test]
mod test {
    import result::*;

    #[test]
    fn test_compile() {
        import result::*;

        let p = compile("foo", 0);
        assert success(p);

        let p = compile("foo(", 0);
        alt p {
          err(e) {
            assert e.code == 14;
            assert e.reason == "missing )";
            assert e.offset == 4u;
          }
          _ { fail; }
        }
    }

    #[test]
    fn test_match() {
        let p = match("(foo)bar", "foobar", 0);
        assert success(p);

        let c = compile("(foo)bar", 0);
        let p = match(c, "foobar", 0);
        assert success(p);

        let p = match("foo(", "foobar", 0);
        alt p {
          err(left(e)) {
            assert e.code == 14;
            assert e.reason == "missing )";
            assert e.offset == 4u;
          }
          _ { fail; }
        }

        let p = match("(foo)bar", "baz", 0);
        assert is_nomatch(p);
    }

    #[test]
    fn test_replace() {
        let r = replace("bcd", "AbcdE", "BCD", 0);
        alt r {
          ok(s) { assert s == "ABCDE"; }
          _ { fail; }
        }
    }

    #[test]
    fn test_replace_from() {
        let r = replace_from("bcd", "AbcdbcdE", "BCD", 2u, 0);
        alt r {
          ok(s) { assert s == "AbcdBCDE"; }
          _ { fail; }
        }
    }

    #[test]
    fn test_replace_fn() {
        let r = replace_fn("bcd", "AbcdE",
                           {|m| str::to_upper(m.matched()) }, 0);
        alt r {
          ok(s) { assert s == "ABCDE"; }
          _ { fail; }
        }
    }

    #[test]
    fn test_replace_fn_from() {
        let r = replace_fn_from("bcd", "AbcdbcdE",
                                {|m| str::to_upper(m.matched()) }, 2u, 0);
        alt r {
          ok(s) { assert s == "AbcdBCDE"; }
          _ { fail; }
        }
    }
}

#[test]
mod test_match_like {
    #[test]
    fn test_group() {
        let r = match("(foo)bar(baz)", "foobarbaz", 0);
        alt r {
          ok(m) {
            alt m.group(0u) { some(s) { assert s == "foo"; } _ { fail; } }
            alt m.group(1u) { some(s) { assert s == "baz"; } _ { fail; } }
            alt m.group(2u) { none { assert true; } _ { fail; } }
          }
          _ { fail; }
        }
    }

    #[test]
    fn test_groups() {
        let r = match("(foo)bar(baz)", "foobarbaz", 0);
        alt r {
          ok(m) {
            assert vec::all2(m.groups(), ["foo", "baz"]) {|s, t| s == t };
          }
          _ { fail; }
        }
    }

    #[test]
    fn test_group_count() {
        let r = match("foobarbaz", "foobarbaz", 0);
        alt r {
          ok(m) {
            assert m.group_count() == 0u;
          }
          _ { fail; }
        }

        let r = match("(foo)bar(baz)", "foobarbaz", 0);
        alt r {
          ok(m) {
            assert m.group_count() == 2u;
          }
          _ { fail; }
        }

        let r = match("(?:foo)bar", "foobar", 0);
        alt r {
          ok(m) {
            assert m.group_count() == 0u;
          }
          _ { fail; }
        }

        let r = match("(?:(foo)|baz)bar", "foobar", 0);
        alt r {
          ok(m) {
            assert m.group_count() == 1u;
          }
          _ { fail; }
        }

        let r = match("(?:foo|(baz))bar", "foobar", 0);
        alt r {
          ok(m) {
            assert m.group_count() == 0u;
          }
          _ { fail; }
        }
    }
}
