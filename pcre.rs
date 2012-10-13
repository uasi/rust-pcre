extern mod std;

use core::libc::{c_char, c_int, c_void};
use core::option::{Some, None};
use  core::result::{Ok, Err};
use core::either::Either;
use core::result::Result;

use consts::*;
mod consts {
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

    const COMPILE_OPTIONS: int        = 0x27fc7a7f;
    const EXEC_OPTIONS: int           = 0x1df0a590;

    //const COMPILE_OPTIONS: int =
    //    PCRE_CASELESS
    //  | PCRE_MULTILINE
    //  | PCRE_DOTALL
    //  | PCRE_EXTENDED
    //  | PCRE_ANCHORED
    //  | PCRE_DOLLAR_ENDONLY
    //  | PCRE_EXTRA
    //  | PCRE_UNGREEDY
    //  | PCRE_UTF8
    //  | PCRE_NO_AUTO_CAPTURE
    //  | PCRE_NO_UTF8_CHECK
    //  | PCRE_AUTO_CALLOUT
    //  | PCRE_FIRSTLINE
    //  | PCRE_DUPNAMES
    //  | PCRE_NEWLINE_CR
    //  | PCRE_NEWLINE_LF
    //  | PCRE_NEWLINE_CRLF
    //  | PCRE_NEWLINE_ANY
    //  | PCRE_NEWLINE_ANYCRLF
    //  | PCRE_BSR_ANYCRLF
    //  | PCRE_BSR_UNICODE
    //  | PCRE_JAVASCRIPT_COMPAT
    //  | PCRE_NO_START_OPTIMIZE
    //  | PCRE_NO_START_OPTIMISE
    //  | PCRE_UCP;

    //const EXEC_OPTIONS: int =
    //    PCRE_ANCHORED
    //  | PCRE_NOTBOL
    //  | PCRE_NOTEOL
    //  | PCRE_NOTEMPTY
    //  | PCRE_NO_UTF8_CHECK
    //  | PCRE_PARTIAL_SOFT
    //  | PCRE_PARTIAL
    //  | PCRE_NEWLINE_CR
    //  | PCRE_NEWLINE_LF
    //  | PCRE_NEWLINE_CRLF
    //  | PCRE_NEWLINE_ANY
    //  | PCRE_NEWLINE_ANYCRLF
    //  | PCRE_BSR_ANYCRLF
    //  | PCRE_BSR_UNICODE
    //  | PCRE_NO_START_OPTIMIZE
    //  | PCRE_NO_START_OPTIMISE
    //  | PCRE_PARTIAL_HARD
    //  | PCRE_NOTEMPTY_ATSTART;

    const PCRE_ERROR_NOMATCH: int        =  -1;
    const PCRE_ERROR_NULL: int           =  -2;
    const PCRE_ERROR_BADOPTION: int      =  -3;
    const PCRE_ERROR_BADMAGIC: int       =  -4;
    const PCRE_ERROR_UNKNOWN_OPCODE: int =  -5;
    const PCRE_ERROR_UNKNOWN_NODE: int   =  -5;  // For backward compatibility
    const PCRE_ERROR_NOMEMORY: int       =  -6;
    const PCRE_ERROR_NOSUBSTRING: int    =  -7;
    const PCRE_ERROR_MATCHLIMIT: int     =  -8;
    const PCRE_ERROR_CALLOUT: int        =  -9;  // Never used by PCRE itself
    const PCRE_ERROR_BADUTF8: int        = -10;
    const PCRE_ERROR_BADUTF8_OFFSET: int = -11;
    const PCRE_ERROR_PARTIAL: int        = -12;
    const PCRE_ERROR_BADPARTIAL: int     = -13;
    const PCRE_ERROR_INTERNAL: int       = -14;
    const PCRE_ERROR_BADCOUNT: int       = -15;
    const PCRE_ERROR_DFA_UITEM: int      = -16;
    const PCRE_ERROR_DFA_UCOND: int      = -17;
    const PCRE_ERROR_DFA_UMLIMIT: int    = -18;
    const PCRE_ERROR_DFA_WSSIZE: int     = -19;
    const PCRE_ERROR_DFA_RECURSE: int    = -20;
    const PCRE_ERROR_RECURSIONLIMIT: int = -21;
    const PCRE_ERROR_NULLWSLIMIT: int    = -22;  // No longer actually used
    const PCRE_ERROR_BADNEWLINE: int     = -23;
    const PCRE_ERROR_BADOFFSET: int      = -24;
    const PCRE_ERROR_SHORTUTF8: int      = -25;

    const PCRE_INFO_OPTIONS: int         =   0;
    const PCRE_INFO_SIZE: int            =   1;
    const PCRE_INFO_CAPTURECOUNT: int    =   2;
    const PCRE_INFO_BACKREFMAX: int      =   3;
    const PCRE_INFO_FIRSTBYTE: int       =   4;
    const PCRE_INFO_FIRSTCHAR: int       =   4; // For backwards compatibility
    const PCRE_INFO_FIRSTTABLE: int      =   5;
    const PCRE_INFO_LASTLITERAL: int     =   6;
    const PCRE_INFO_NAMEENTRYSIZE: int   =   7;
    const PCRE_INFO_NAMECOUNT: int       =   8;
    const PCRE_INFO_NAMETABLE: int       =   9;
    const PCRE_INFO_STUDYSIZE: int       =  10;
    const PCRE_INFO_DEFAULT_TABLES: int  =  11;
    const PCRE_INFO_OKPARTIAL: int       =  12;
    const PCRE_INFO_JCHANGED: int        =  13;
    const PCRE_INFO_HASCRORLF: int       =  14;
    const PCRE_INFO_MINLENGTH: int       =  15;
}

enum Pcre {}
enum PcreExtra {}
struct PcreRes {
    p: *Pcre,
    drop { c::free(self.p as *c_void); }
}

#[doc = "
The result type of <compile>.
"]
type CompileResult = Result<Pattern, CompileErr>;

#[doc = "
The result type of <exec>.
"]
type ExecResult = Result<Match, MatchErr>;

#[doc = "
The result type of <search>.
"]
type MatchResult = Result<Match, RegexErr>;

#[doc = "
The result type of <replace>.
"]
type ReplaceResult = Result<@~str, RegexErr>;

#[doc = "
The type that represents compile error.
"]
type CompileErr = {
    code: int,
    reason: @~str,
    offset: uint,
};

#[doc = "
The type that represents match error.
"]
type MatchErr = int;

#[doc = "
Either compile or match error.
"]
enum RegexErr {
    CompileErr(CompileErr),
    MatchErr(MatchErr),
}

type Pattern = {
    str: @~str,
    _pcre_res: @PcreRes,
};

type Match = {
    subject: @~str,
    pattern: Pattern,
    _captures: @~[uint],
};

#[nolink]
#[abi = "cdecl"]
extern mod c {
    fn free(p: *c_void);
}

#[abi = "cdecl"]
extern mod pcre {
    fn pcre_compile2(pattern: *c_char, options: c_int,
                     errorcodeptr: *c_int,
                     errptr: **c_char, erroffset: *c_int,
                     tableptr: *c_char) -> *Pcre;
    fn pcre_exec(code: *Pcre, extra: *PcreExtra,
                 subject: *c_char, length: c_int, startoffset: c_int,
                 options: c_int, ovector: * c_int, ovecsize: c_int) -> c_int;
    fn pcre_fullinfo(code: *Pcre, extra: *PcreExtra,
                     what: c_int, where: *c_void) -> c_int;
    fn pcre_get_stringnumber(code: *Pcre, name: *c_char) -> c_int;
}

trait PatternUtil {
    fn info_capture_count() -> uint;
    fn info_name_count() -> uint;
    fn info_name_entry_size() -> uint;
    fn with_name_table(blk: fn(*u8));
    fn group_count() -> uint;
    fn group_names() -> ~[~str];
}

impl Pattern: PatternUtil {
    fn info_capture_count() -> uint {
        let count = -1 as c_int;
        pcre::pcre_fullinfo(self._pcre_res.p, ptr::null(),
                            PCRE_INFO_CAPTURECOUNT as c_int,
                            ptr::addr_of(&count) as *c_void);
        assert count >= 0 as c_int;
        return count as uint;
    }

    fn info_name_count() -> uint {
        let count = -1 as c_int;
        pcre::pcre_fullinfo(self._pcre_res.p, ptr::null(),
                            PCRE_INFO_NAMECOUNT as c_int,
                            ptr::addr_of(&count) as *c_void);
        assert count >= 0 as c_int;
        return count as uint;
    }

    fn info_name_entry_size() -> uint {
        let size = -1 as c_int;
        pcre::pcre_fullinfo(self._pcre_res.p, ptr::null(),
                            PCRE_INFO_NAMEENTRYSIZE as c_int,
                            ptr::addr_of(&size) as *c_void);
        assert size >= 0 as c_int;
        return size as uint;
    }

    fn with_name_table(blk: fn(*u8)) {
        let table = ptr::null::<u8>();
        unsafe {
            pcre::pcre_fullinfo(self._pcre_res.p, ptr::null(),
                                PCRE_INFO_NAMETABLE as c_int,
                                ptr::addr_of(&table) as *c_void);
        }
        assert table != ptr::null::<u8>();
        blk(table);
    }

    fn group_count() -> uint {
        return self.info_capture_count();
    }

    fn group_names() -> ~[~str] {
        let count = self.info_name_count();
        if count == 0u { return ~[]; }
        let size = self.info_name_entry_size();
        let mut names: ~[~str] = ~[];
        do self.with_name_table |table| {
            for uint::range(0u, count) |i| {
                unsafe {
                    let p = ptr::offset(table, size * i + 2u);
                    let s = str::raw::from_c_str(p as *c_char);
                    vec::push(&mut names, s);
                }
            }
        }
        return names;
    }
}

trait PatternLike {
    fn compile(options: int) -> CompileResult;
}

impl &str: PatternLike {
    fn compile(options: int) -> CompileResult { compile(self, options) }
}

impl ~str: PatternLike {
    fn compile(options: int) -> CompileResult { compile(self, options) }
}

impl @str: PatternLike {
    fn compile(options: int) -> CompileResult { compile(self, options) }
}

impl Pattern: PatternLike {
    fn compile(_options: int) -> CompileResult { Ok(self) }
}

impl CompileResult: PatternLike {
    fn compile(_options: int) -> CompileResult { self }
}

trait MatchExtensions {
    fn matched() -> ~str;
    fn prematch() -> ~str;
    fn postmatch() -> ~str;
    fn begin() -> uint;
    fn end() -> uint;
    fn group(i: uint) -> Option<@~str>;
    fn named_group(name: &str) -> Option<@~str>;
    fn subgroups() -> ~[~str];
    fn subgroups_iter(blk: fn(&str));
    fn group_count() -> uint;
    fn group_names() -> ~[~str];
}

impl Match: MatchExtensions {
    fn matched() -> ~str {
        return str::slice(*self.subject, self.begin(), self.end());
    }

    fn prematch() -> ~str {
        return str::slice(*self.subject, 0u, self.begin());
    }

    fn postmatch() -> ~str {
        return str::slice(*self.subject ,self.end(),
                          str::char_len(*self.subject));
    }

    fn begin() -> uint {
        return self._captures[0];
    }

    fn end() -> uint {
        return self._captures[1];
    }

    fn group(i: uint) -> Option<@~str> {
        if i > self.group_count() {
            return None;
        }
        return Some(@str::slice(*self.subject,
                                self._captures[i * 2u],
                                self._captures[i * 2u + 1u]));
    }

    fn named_group(name: &str) -> Option<@~str> {
        let i = str::as_buf(name, |s, _n| {
            pcre::pcre_get_stringnumber(self.pattern._pcre_res.p, s as *c_char)
        });
        if i <= 0 as c_int { return None; }
        return self.group(i as uint);
    }

    fn subgroups() -> ~[~str] {
        let mut v = ~[];
        vec::reserve(&mut v, self.group_count());
        do self.subgroups_iter |subgroup| { vec::push(&mut v, str::from_slice(subgroup)); }
        return v;
    }

    fn subgroups_iter(blk: fn(&str)) {
        for uint::range(1u, self.group_count() + 1u) |i| {
            match self.group(i) {
              Some(s) => blk(*s),
              None => fail,
            }
        }
    }

    fn group_count() -> uint {
        return vec::len(*self._captures) / 2u - 1u;
    }

    fn group_names() -> ~[~str] {
        return self.pattern.group_names();
    }
}

fn compile(pattern: &str, options: int) -> CompileResult unsafe {
    if options | COMPILE_OPTIONS != COMPILE_OPTIONS {
        #warn("unrecognized option bit(s) are set");
    }

    let options = options | PCRE_NO_UTF8_CHECK; // str is always valid
    let errcode = 0 as c_int;
    let errreason: *c_char = ptr::null();
    let erroffset = 0 as c_int;
    let p = str::as_buf(pattern, |pat, _n| {
        pcre::pcre_compile2(pat as *c_char,
                            options as c_int,
                            ptr::addr_of(&errcode),
                            ptr::addr_of(&errreason),
                            ptr::addr_of(&erroffset),
                            ptr::null())
    });
    if p == ptr::null() {
        return Err({code: errcode as int,
                    reason: @str::raw::from_c_str(errreason),
                    offset: erroffset as uint});
    }
    return Ok({str: @str::from_slice(pattern), _pcre_res: @PcreRes {p: p}});
}

fn exec(pattern: Pattern,
        subject: &str, offset: uint,
        options: int) -> ExecResult unsafe {

    if (options | EXEC_OPTIONS) != EXEC_OPTIONS {
        #warn("unrecognized option bit(s) are set");
    }

    let options = options | PCRE_NO_UTF8_CHECK; // str is always valid

    let count = (pattern.info_capture_count() + 1u) as c_int;
    let mut ovec = vec::from_elem((count as uint) * 3u, 0u as c_int);

    let ret_code = str::as_buf(subject, |s, _n| {
        pcre::pcre_exec(pattern._pcre_res.p, ptr::null(),
                        s as *c_char, str::len(subject) as c_int,
                        offset as c_int, options as c_int,
                        vec::raw::to_ptr(ovec) as *c_int,
                        count * (3 as c_int)) as int
    });

    if ret_code < 0 { return Err(ret_code as MatchErr); }

    // Cut off the working space
    vec::raw::set_len(&mut ovec, count as uint * 2u);

    let mut captures: ~[uint] = ~[];
    vec::reserve(&mut captures, vec::len(ovec));
    for ovec.each |o| {
        if *o as int < 0 { loop; }
        vec::push(&mut captures, *o as uint);
    }
    assert vec::len(captures) % 2u == 0u;

    return Ok({subject: @str::from_slice(subject), pattern: pattern, _captures: @captures});
}

fn search<T: PatternLike>(pattern: T, subject: &str,
                          options: int) -> MatchResult {
    return search_from(pattern, subject, 0u, options);
}

fn search_from<T: PatternLike>(pattern: T, subject: &str,
                               offset: uint, options: int) -> MatchResult {
    assert offset <= str::len(subject);

    let c_opts = options & COMPILE_OPTIONS;
    let e_opts = options & EXEC_OPTIONS;

    let c = pattern.compile(c_opts);
    match c {
      Ok(pattern) => {
        let e = exec(pattern, subject, offset, e_opts);
        match e {
          Ok(m) => {
            return Ok(m);
          }
          Err(m_err) => {
            return Err(MatchErr(m_err));
          }
        }
      }
      Err(c_err) => {
          return Err(CompileErr(c_err));
      }
    }
}

fn replace<T: PatternLike Copy>(pattern: T, subject: &str, repl: &str,
                                options: int) -> ReplaceResult {
    return replace_fn_from(pattern, subject, |_m| { str::from_slice(repl) }, 0u, options);
}

fn replace_from<T: PatternLike Copy>(pattern: T, subject: &str, repl: &str,
                                     offset: uint, options: int)
                                     -> ReplaceResult {
    return replace_fn_from(pattern, subject, |_m| { str::from_slice(repl) }, offset, options);
}

fn replace_fn<T: PatternLike Copy>(pattern: T, subject: &str,
                                   repl_fn: fn(Match) -> ~str, options: int)
                                   -> ReplaceResult {
    return replace_fn_from(pattern, subject, repl_fn, 0u, options);
}

fn replace_fn_from<T: PatternLike Copy>(pattern: T, subject: &str,
                                        repl_fn: fn(Match) -> ~str, offset: uint,
                                        options: int)
                                        -> ReplaceResult {
    let r = search_from(pattern, subject, offset, options);
    match r {
      Ok(m) => {
        return Ok(@(m.prematch() + repl_fn(m) + m.postmatch()));
      }
      Err(e) => { return Err(e); }
    }
}

fn replace_all<T: PatternLike Copy>(pattern: T, subject: &str,
                                    repl: &str,
                                    options: int)
                                    -> ReplaceResult {
    return replace_all_fn_from(pattern, subject, |_m| { str::from_slice(repl) }, 0u, options);
}

fn replace_all_fn<T: PatternLike Copy>(pattern: T, subject: &str,
                                       repl_fn: fn(Match) -> ~str,
                                       options: int)
                                       -> ReplaceResult {
    return replace_all_fn_from(pattern, subject, repl_fn, 0u, options);
}

fn replace_all_from<T: PatternLike Copy>(pattern: T, subject: &str,
                                         repl: &str,
                                         offset: uint,
                                         options: int)
                                         -> ReplaceResult {
    return replace_all_fn_from(pattern, subject, |_m| { str::from_slice(repl) }, offset, options);
}

fn replace_all_fn_from<T: PatternLike Copy>(pattern: T, subject: &str,
                                            repl_fn: fn(Match) -> ~str,
                                            offset: uint,
                                            options: int)
                                            -> ReplaceResult {
    let mut offset = offset;
    let subject_len = str::len(subject);
    assert offset <= subject_len;

    let mut s = str::slice(subject, 0, offset);
    loop {
        let r = search_from(pattern, subject, offset, options);
        match r {
          Ok(m) => {
            s += str::slice(subject, offset, m.begin());
            s += repl_fn(m);
            offset = m.end();
          }
          Err(MatchErr(e)) if e == PCRE_ERROR_NOMATCH => {
            if offset != subject_len {
                s += str::slice(subject, offset, subject_len);
            }
            break;
          }
          Err(e) => {
            return Err(copy e);
          }
        }
    }
    return Ok(@s);
}

fn fmt_compile_err(e: CompileErr) -> ~str {
    return fmt!("error %d: %s at offset %u", e.code, *e.reason, e.offset);
}


#[doc = "
Returns true iff mr indicates that the subject did not match the pattern.
"]
pure fn is_nomatch(mr: MatchResult) -> bool {
    match mr {
      Err(MatchErr(e)) if e == PCRE_ERROR_NOMATCH => true,
      _ => false,
    }
}

#[cfg(test)]
mod test_util {
    pub trait OptionUtil<T> {
        fn is_some_and(blk: fn(T) -> bool) -> bool;
        fn is_none_and(blk: fn() -> bool) -> bool;
    }

    impl<T: Copy> Option<T>: OptionUtil<T> {
        fn is_some_and(blk: fn(T) -> bool) -> bool {
            match self {
              Some(t) => blk(t),
              None => false,
            }
        }

        // Who wants?
        fn is_none_and(blk: fn() -> bool) -> bool {
            match self {
              Some(_) => false,
              None => blk(),
            }
        }
    }

    pub trait ResultUtil<T, U> {
        fn is_ok_and(blk: fn(T) -> bool) -> bool;
        fn is_err_and(blk: fn(U) -> bool) -> bool;
    }

    impl<T: Copy, U: Copy> Result<T, U>: ResultUtil<T, U> {
        fn is_ok_and(blk: fn(T) -> bool) -> bool {
            match self {
              Ok(t) => blk(t),
              Err(_) => false,
            }
        }

        fn is_err_and(blk: fn(U) -> bool) -> bool {
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
mod test {
    use test_util::*;

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

        let c = compile("(foo)bar", 0);
        let r = search(c, "foobar", 0);
        assert r.is_ok();

        let r = search("foo(", "foobar", 0);
        match r {
          Err(CompileErr(e)) => {
            assert e.code == 14;
            assert e.reason == @~"missing )";
            assert e.offset == 4u;
          }
          _ => { fail; }
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

    // Inline options supersedes search-time compile options
    #[test]
    fn test_search_options_2() {
        let r = search("(?-i)foobar", "FOOBAR", PCRE_CASELESS);
        assert r.is_err();
    }

    // Compile-time compile options supersedes search-time compile options
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
        assert r.is_ok_and(|m| m.group_count() == 0u);
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
