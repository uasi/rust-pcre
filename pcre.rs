extern mod std;

use core::libc::{c_char, c_int, c_void};
use core::option::{Some, None};
use core::result::{Ok, Err};
use core::result::Result;
use consts::*;

enum Pcre {}
enum PcreExtra {}
struct PcreRes {
    p: *Pcre,
    drop { unsafe { c::free(self.p as *c_void); } }
}


/// The result type of `compile`
pub type CompileResult = Result<Pattern, CompileErr>;

/// The result type of `exec`
pub type ExecResult = Result<Match, ExecErr>;


/// The result type of `search`
pub type SearchResult = Result<Match, RegexErr>;

// The result type of `replace`
pub type ReplaceResult = Result<@~str, RegexErr>;

/// The type that represents compile error
pub struct CompileErr {
    code: int,
    reason: @~str,
    offset: uint,
}

/// The type that represents exec error
pub type ExecErr = int;

/// Either compile or exec error
pub enum RegexErr {
    CompileErr(CompileErr),
    ExecErr(ExecErr),
}

/// Compiled regular expression
pub struct Pattern {
    str: @~str,
    priv pcre_res: @PcreRes,
}

/// Match
pub struct Match {
    subject: @~str,
    pattern: Pattern,
    priv captures: @~[int],
}

#[nolink]
#[abi = "cdecl"]
extern mod c {
    fn free(p: *c_void);
}

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

pub trait PatternUtil {
    fn info_capture_count(self) -> uint;
    fn info_name_count(self) -> uint;
    fn info_name_entry_size(self) -> uint;
    fn with_name_table(self, blk: &fn(*u8));
    fn group_count(self) -> uint;
    fn group_names(self) -> ~[~str];
}

impl PatternUtil for Pattern {
    fn info_capture_count(self) -> uint {
        let count = -1 as c_int;
        unsafe {
            pcre::pcre_fullinfo(self.pcre_res.p, ptr::null(),
                                PCRE_INFO_CAPTURECOUNT as c_int,
                                ptr::addr_of(&count) as *c_void);
        }
        assert!(count >= 0 as c_int);
        return count as uint;
    }

    fn info_name_count(self) -> uint {
        let count = -1 as c_int;
        unsafe {
            pcre::pcre_fullinfo(self.pcre_res.p, ptr::null(),
                                PCRE_INFO_NAMECOUNT as c_int,
                                ptr::addr_of(&count) as *c_void);
        }
        assert!(count >= 0 as c_int);
        return count as uint;
    }

    fn info_name_entry_size(self) -> uint {
        let size = -1 as c_int;
        unsafe {
            pcre::pcre_fullinfo(self.pcre_res.p, ptr::null(),
                                PCRE_INFO_NAMEENTRYSIZE as c_int,
                                ptr::addr_of(&size) as *c_void);
        }
        assert!(size >= 0 as c_int);
        return size as uint;
    }

    fn with_name_table(self, blk: &fn(*u8)) {
        let table = ptr::null::<u8>();
        unsafe {
            pcre::pcre_fullinfo(self.pcre_res.p, ptr::null(),
                                PCRE_INFO_NAMETABLE as c_int,
                                ptr::addr_of(&table) as *c_void);
        }
        assert!(table != ptr::null::<u8>());
        blk(table);
    }

    fn group_count(self) -> uint {
        return self.info_capture_count();
    }

    fn group_names(self) -> ~[~str] {
        let count = self.info_name_count();
        if count == 0u { return ~[]; }
        let size = self.info_name_entry_size();
        let mut names: ~[~str] = ~[];
        unsafe {
            do self.with_name_table |table| {
                for uint::range(0u, count) |i| {
                    let p = ptr::offset(table, size * i + 2u);
                    let s = str::raw::from_c_str(p as *c_char);
                    vec::push(&mut names, s);
                }
            }
        }
        return names;
    }
}

pub trait PatternLike {
    fn compile(&self, options: int) -> CompileResult;
}

impl<'self> PatternLike for &'self str {
    fn compile(&self, options: int) -> CompileResult {
        compile(*self, options)
    }
}

impl PatternLike for ~str {
    fn compile(&self, options: int) -> CompileResult {
        compile(*self, options)
    }
}

impl PatternLike for @str {
    fn compile(&self, options: int) -> CompileResult {
        compile(*self, options)
    }
}

impl PatternLike for Pattern {
    fn compile(&self, _options: int) -> CompileResult {
        Ok(*self)
    }
}

impl PatternLike for CompileResult {
    fn compile(&self, _options: int) -> CompileResult {
        *self
    }
}

pub trait MatchExtensions {
    fn matched(self) -> ~str;
    fn prematch(self) -> ~str;
    fn postmatch(self) -> ~str;
    fn begin(self) -> uint;
    fn end(self) -> uint;
    fn group(self, i: uint) -> Option<@~str>;
    fn named_group(self, name: &str) -> Option<@~str>;
    fn subgroups(self) -> ~[~str];
    fn subgroups_iter(self, blk: &fn(&str));
    fn group_count(self) -> uint;
    fn group_names(self) -> ~[~str];
}

impl MatchExtensions for Match {
    fn matched(self) -> ~str {
        return str::slice(*self.subject, self.begin(), self.end());
    }

    fn prematch(self) -> ~str {
        return str::slice(*self.subject, 0u, self.begin());
    }

    fn postmatch(self) -> ~str {
        return str::slice(*self.subject ,self.end(),
                          str::char_len(*self.subject));
    }

    fn begin(self) -> uint {
        return self.captures[0] as uint;
    }

    fn end(self) -> uint {
        return self.captures[1] as uint;
    }

    fn group(self, i: uint) -> Option<@~str> {
        if i > self.group_count() {
            return None;
        }
        let i1 = self.captures[i * 2u];
        let i2 = self.captures[i * 2u + 1u];
        if(i1 < 0 || i2 < 0) {
            return None;
        }
        return Some(@str::slice(*self.subject, i1 as uint, i2 as uint));
    }

    fn named_group(self, name: &str) -> Option<@~str> {
        let i =  unsafe {
            str::as_buf(name, |s, _n| {
                pcre::pcre_get_stringnumber(self.pattern.pcre_res.p,
                                            s as *c_char)
            })
        };
        if i <= 0 as c_int { return None; }
        return self.group(i as uint);
    }

    fn subgroups(self) -> ~[~str] {
        let mut v = ~[];
        unsafe {
            vec::reserve(&mut v, self.group_count());
            do self.subgroups_iter |subgroup| {
                vec::push(&mut v, str::from_slice(subgroup));
            }
        }
        return v;
    }

    fn subgroups_iter(self, blk: &fn(&str)) {
        for uint::range(1u, self.group_count() + 1u) |i| {
            match self.group(i) {
              Some(s) => blk(*s),
              None => fail!(),
            }
        }
    }

    fn group_count(self) -> uint {
        return vec::len(*self.captures) / 2u - 1u;
    }

    fn group_names(self) -> ~[~str] {
        return self.pattern.group_names();
    }
}

pub fn compile(pattern: &str, options: int) -> CompileResult {
    if options | COMPILE_OPTIONS != COMPILE_OPTIONS {
        warn!("unrecognized option bit(s) are set");
    }

    let options = options | PCRE_NO_UTF8_CHECK; // str is always valid
    let errcode = 0 as c_int;
    let errreason: *c_char = ptr::null();
    let erroffset = 0 as c_int;
    let p = unsafe {
        str::as_buf(pattern, |pat, _n| {
            pcre::pcre_compile2(pat as *c_char,
                                options as c_int,
                                ptr::addr_of(&errcode),
                                ptr::addr_of(&errreason),
                                ptr::addr_of(&erroffset),
                                ptr::null())
        })
    };
    if p == ptr::null() {
        let reason = unsafe { @str::raw::from_c_str(errreason) };
        return Err(CompileErr {code: errcode as int,
                               reason: reason,
                               offset: erroffset as uint});
    }
    return Ok(Pattern {str: @str::from_slice(pattern),
                       pcre_res: @PcreRes {p: p}});
}

pub fn exec(pattern: Pattern,
            subject: &str, offset: uint,
            options: int) -> ExecResult {

    if (options | EXEC_OPTIONS) != EXEC_OPTIONS {
        warn!("unrecognized option bit(s) are set");
    }

    let options = options | PCRE_NO_UTF8_CHECK; // str is always valid

    let count = (pattern.info_capture_count() + 1u) as c_int;
    let mut ovec = vec::from_elem((count as uint) * 3u, 0u as c_int);

    let ret_code = unsafe {
        str::as_buf(subject, |s, _n| {
            pcre::pcre_exec(pattern.pcre_res.p, ptr::null(),
                            s as *c_char, str::len(subject) as c_int,
                            offset as c_int, options as c_int,
                            vec::raw::to_ptr(ovec) as *c_int,
                            count * (3 as c_int)) as int
        })
    };

    if ret_code < 0 { return Err(ret_code as ExecErr); }

    // Truncate the working space.
    unsafe { vec::raw::set_len(&mut ovec, count as uint * 2u) }

    let mut captures: ~[int] = ~[];
    unsafe { vec::reserve(&mut captures, vec::len(ovec)); }
    for ovec.each |o| {
        unsafe { vec::push(&mut captures, *o as int); }
    }
    assert!(vec::len(captures) % 2u == 0u);

    return Ok(Match {subject: @str::from_slice(subject),
                     pattern: pattern,
                     captures: @captures});
}

pub fn search<T: PatternLike>(pattern: T, subject: &str,
                              options: int)
                              -> SearchResult {
    return search_from(pattern, subject, 0u, options);
}

pub fn search_from<T: PatternLike>(pattern: T, subject: &str,
                                   offset: uint, options: int)
                                   -> SearchResult {
    assert!(offset <= str::len(subject));

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
                Err(e_err) => {
                    return Err(ExecErr(e_err));
                }
            }
        }
        Err(c_err) => {
            return Err(CompileErr(c_err));
        }
    }
}

pub fn replace<T: PatternLike + Copy>(pattern: T, subject: &str,
                                      repl: &str, options: int)
                                      -> ReplaceResult {
    return replace_fn_from(pattern, subject,
                           |_m| { str::from_slice(repl) }, 0u, options);
}

pub fn replace_from<T: PatternLike + Copy>(pattern: T, subject: &str,
                                           repl: &str,  offset: uint,
                                           options: int)
                                           -> ReplaceResult {
    return replace_fn_from(pattern, subject,
                           |_m| { str::from_slice(repl) }, offset, options);
}

pub fn replace_fn<T: PatternLike + Copy>(pattern: T, subject: &str,
                                         repl_fn: &fn(Match) -> ~str,
                                         options: int)
                                         -> ReplaceResult {
    return replace_fn_from(pattern, subject, repl_fn, 0u, options);
}

pub fn replace_fn_from<T: PatternLike + Copy>(pattern: T, subject: &str,
                                              repl_fn: &fn(Match) -> ~str,
                                              offset: uint,
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

pub fn replace_all<T: PatternLike + Copy>(pattern: T, subject: &str,
                                          repl: &str,
                                          options: int)
                                          -> ReplaceResult {
    return replace_all_fn_from(pattern, subject,
                               |_m| { str::from_slice(repl) }, 0u, options);
}

pub fn replace_all_fn<T: PatternLike + Copy>(pattern: T, subject: &str,
                                             repl_fn: &fn(Match) -> ~str,
                                             options: int)
                                             -> ReplaceResult {
    return replace_all_fn_from(pattern, subject, repl_fn, 0u, options);
}

pub fn replace_all_from<T: PatternLike + Copy>(pattern: T, subject: &str,
                                               repl: &str,
                                               offset: uint,
                                               options: int)
                                               -> ReplaceResult {
    return replace_all_fn_from(pattern, subject,
                               |_m| { str::from_slice(repl) }, offset, options);
}

pub fn replace_all_fn_from<T: PatternLike + Copy>(pattern: T, subject: &str,
                                                  repl_fn: &fn(Match) -> ~str,
                                                  offset: uint,
                                                  options: int)
                                                  -> ReplaceResult {
    let mut offset = offset;
    let subject_len = str::len(subject);
    assert!(offset <= subject_len);

    let mut s = str::slice(subject, 0, offset);
    loop {
        let r = search_from(pattern, subject, offset, options);
        match r {
            Ok(m) => {
                s += str::slice(subject, offset, m.begin());
                s += repl_fn(m);
                offset = m.end();
            }
            Err(ExecErr(e)) if e == PCRE_ERROR_NOMATCH => {
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

pub fn fmt_compile_err(e: CompileErr) -> ~str {
    return fmt!("error %d: %s at offset %u", e.code, *e.reason, e.offset);
}


/// Return true iff `sr` indicates that the subject did not match the pattern
pub fn is_nomatch(sr: SearchResult) -> bool {
    match sr {
        Err(ExecErr(e)) if e == PCRE_ERROR_NOMATCH => true,
        _ => false,
    }
}
