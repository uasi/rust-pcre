extern mod std;
use std::ptr;
use std::vec;
use std::c_str::{CString};
use std::libc::{c_char, c_int, c_void};
use std::option::{Some, None};
use std::result::{Ok, Err};
use std::result::Result;
use std::libc::funcs::c95::stdlib::free;
use consts::{PCRE_INFO_CAPTURECOUNT, PCRE_INFO_NAMECOUNT, 
            PCRE_INFO_NAMEENTRYSIZE, PCRE_INFO_NAMETABLE,
            PCRE_NO_UTF8_CHECK, PCRE_ERROR_NOMATCH,
            COMPILE_OPTIONS, EXEC_OPTIONS};

mod pcre {
    use std::libc::{c_char, c_int, c_void};
    pub enum Pcre {}
    pub enum PcreExtra {}

    extern {
        pub fn pcre_compile2(pattern: *c_char, options: c_int,
                         errorcodeptr: *c_int,
                         errptr: **c_char, erroffset: *c_int,
                         tableptr: *c_char) -> *Pcre;
        pub fn pcre_exec(code: *Pcre, extra: *PcreExtra,
                     subject: *c_char, length: c_int, startoffset: c_int,
                     options: c_int, ovector: * c_int, ovecsize: c_int) -> c_int;
        pub fn pcre_fullinfo(code: *Pcre, extra: *PcreExtra,
                         what: c_int, where: *c_void) -> c_int;
        pub fn pcre_get_stringnumber(code: *Pcre, name: *c_char) -> c_int;
    }
}

struct PcreRes {
    p: *pcre::Pcre,
}

impl Drop for PcreRes {
    #[fixed_stack_segment]
    fn drop(&mut self) { unsafe { free(self.p as *c_void); } }
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
#[deriving(Eq,Clone,ToStr)]
pub struct CompileErr {
    code: int,
    reason: ~str,
    offset: uint,
}

/// The type that represents exec error
pub type ExecErr = int;

/// Either compile or exec error
#[deriving(Eq,Clone,ToStr)]
pub enum RegexErr {
    CompileErr(CompileErr),
    ExecErr(ExecErr),
}

/// Compiled regular expression
#[deriving(Clone)]
pub struct Pattern {
    str: @~str,
    options: int,
    priv pcre_res: @PcreRes,
}

impl Eq for Pattern {
    fn eq(&self, other: &Pattern) -> bool {
        self.str == other.str && self.options == other.options
     }

     fn ne(&self, other: &Pattern) -> bool {
        !self.eq(other)
     }
}

/// Match
pub struct Match {
    subject: @~str,
    pattern: Pattern,
    priv captures: @~[int],
}

impl Eq for Match {
    fn eq(&self, other: &Match) -> bool {
        self.subject == other.subject && self.pattern == other.pattern
    }

    fn ne(&self, other: &Match) -> bool {
        !self.eq(other)
    }
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
    #[fixed_stack_segment]
    fn info_capture_count(self) -> uint {
        let count = -1 as c_int;
        unsafe {
            pcre::pcre_fullinfo(self.pcre_res.p, ptr::null(),
                                PCRE_INFO_CAPTURECOUNT as c_int,
                                ptr::to_unsafe_ptr(&count) as *c_void);
        }
        assert!(count >= 0 as c_int);
        return count as uint;
    }

    #[fixed_stack_segment]
    fn info_name_count(self) -> uint {
        let count = -1 as c_int;
        unsafe {
            pcre::pcre_fullinfo(self.pcre_res.p, ptr::null(),
                                PCRE_INFO_NAMECOUNT as c_int,
                                ptr::to_unsafe_ptr(&count) as *c_void);
        }
        assert!(count >= 0 as c_int);
        return count as uint;
    }

    #[fixed_stack_segment]
    fn info_name_entry_size(self) -> uint {
        let size = -1 as c_int;
        unsafe {
            pcre::pcre_fullinfo(self.pcre_res.p, ptr::null(),
                                PCRE_INFO_NAMEENTRYSIZE as c_int,
                                ptr::to_unsafe_ptr(&size) as *c_void);
        }
        assert!(size >= 0 as c_int);
        return size as uint;
    }

    #[fixed_stack_segment]
    fn with_name_table(self, blk: &fn(*u8)) {
        let table = ptr::null::<u8>();
        unsafe {
            pcre::pcre_fullinfo(self.pcre_res.p, ptr::null(),
                                PCRE_INFO_NAMETABLE as c_int,
                                ptr::to_unsafe_ptr(&table) as *c_void);
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
                for i in range(0u, count) {
                    let p = ptr::offset(table, (size * i + 2u) as int);
                    let c_string = CString::new(p as *c_char, false);
                    match c_string.as_str() {
                      Some(string) => { names.push(string.to_owned()) },
                      None => { fail!("Name in nametable was null!") }
                    }
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
        self.clone()
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
        return self.subject.slice(self.begin(), self.end()).to_owned();
    }

    fn prematch(self) -> ~str {
        return self.subject.slice(0u, self.begin()).to_owned();
    }

    fn postmatch(self) -> ~str {
        return self.subject.slice(self.end(),
                                    (*self.subject).len()).to_owned();
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
        return Some(@(self.subject.slice(i1 as uint, i2 as uint).to_owned()));
    }

    #[fixed_stack_segment]
    fn named_group(self, name: &str) -> Option<@~str> {
        let i =  unsafe {
            do name.with_c_str |s| {
                pcre::pcre_get_stringnumber(self.pattern.pcre_res.p, s)
            }
        };
        if i <= 0 as c_int { return None; }
        return self.group(i as uint);
    }

    fn subgroups(self) -> ~[~str] {
        let mut v = vec::with_capacity(self.group_count());
        do self.subgroups_iter |subgroup| {
            v.push(subgroup.to_owned());
        }
        return v;
    }

    fn subgroups_iter(self, blk: &fn(&str)) {
        for i in range(1u, self.group_count() + 1u) {
            match self.group(i) {
              Some(s) => blk(*s),
              None => fail!(),
            }
        }
    }

    fn group_count(self) -> uint {
        return (*self.captures).len() / 2u - 1u;
    }

    fn group_names(self) -> ~[~str] {
        return self.pattern.group_names();
    }
}

#[fixed_stack_segment]
pub fn compile(pattern: &str, options: int) -> CompileResult {
    if options | COMPILE_OPTIONS != COMPILE_OPTIONS {
        warn!("unrecognized option bit(s) are set");
    }

    let options = options | PCRE_NO_UTF8_CHECK; // str is always valid
    let errcode = 0 as c_int;
    let errreason: *c_char = ptr::null();
    let erroffset = 0 as c_int;
    let p = unsafe {
        do pattern.with_c_str |pat| {
            pcre::pcre_compile2(pat as *c_char,
                                options as c_int,
                                ptr::to_unsafe_ptr(&errcode),
                                ptr::to_unsafe_ptr(&errreason),
                                ptr::to_unsafe_ptr(&erroffset),
                                ptr::null())
        }
    };
    if p == ptr::null() {
        let reason = unsafe { CString::new(errreason, false) };
        match reason.as_str() {
            Some(string) => {
                return Err(CompileErr {code: errcode as int,
                                       reason: string.to_owned(),
                                       offset: erroffset as uint});
            },
            None => { fail!("errreason was NULL!")}
        }

    }
    return Ok(Pattern {str: @pattern.to_owned(),
                       options: options,
                       pcre_res: @PcreRes {p: p}});
}

#[fixed_stack_segment]
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
        do subject.with_c_str |s| {
            pcre::pcre_exec(pattern.pcre_res.p, ptr::null(),
                            s as *c_char, subject.len() as c_int,
                            offset as c_int, options as c_int,
                            vec::raw::to_ptr(ovec) as *c_int,
                            count * (3 as c_int)) as int
        }
    };

    if ret_code < 0 { return Err(ret_code as ExecErr); }

    // Truncate the working space.
    unsafe { vec::raw::set_len(&mut ovec, count as uint * 2u) }

    let mut captures: ~[int] = vec::with_capacity(ovec.len());

    for o in ovec.iter() {
        captures.push(*o as int);
    }
    assert!(captures.len() % 2u == 0u);

    return Ok(Match {subject: @subject.to_owned(),
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
    assert!(offset <= subject.len());

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

pub fn replace<T: PatternLike + Clone>(pattern: T, subject: &str,
                                      repl: &str, options: int)
                                      -> ReplaceResult {
    return replace_fn_from(pattern, subject,
                           |_m| { repl.to_owned() }, 0u, options);
}

pub fn replace_from<T: PatternLike + Clone>(pattern: T, subject: &str,
                                           repl: &str,  offset: uint,
                                           options: int)
                                           -> ReplaceResult {
    return replace_fn_from(pattern, subject,
                           |_m| { repl.to_owned() }, offset, options);
}

pub fn replace_fn<T: PatternLike + Clone>(pattern: T, subject: &str,
                                         repl_fn: &fn(Match) -> ~str,
                                         options: int)
                                         -> ReplaceResult {
    return replace_fn_from(pattern, subject, repl_fn, 0u, options);
}

pub fn replace_fn_from<T: PatternLike + Clone>(pattern: T, subject: &str,
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

pub fn replace_all<T: PatternLike + Clone>(pattern: T, subject: &str,
                                          repl: &str,
                                          options: int)
                                          -> ReplaceResult {
    return replace_all_fn_from(pattern, subject,
                               |_m| { repl.to_owned() }, 0u, options);
}

pub fn replace_all_fn<T: PatternLike + Clone>(pattern: T, subject: &str,
                                             repl_fn: &fn(Match) -> ~str,
                                             options: int)
                                             -> ReplaceResult {
    return replace_all_fn_from(pattern, subject, repl_fn, 0u, options);
}

pub fn replace_all_from<T: PatternLike + Clone>(pattern: T, subject: &str,
                                               repl: &str,
                                               offset: uint,
                                               options: int)
                                               -> ReplaceResult {
    return replace_all_fn_from(pattern, subject,
                               |_m| { repl.to_owned() }, offset, options);
}

pub fn replace_all_fn_from<T: PatternLike + Clone>(pattern: T, subject: &str,
                                                  repl_fn: &fn(Match) -> ~str,
                                                  offset: uint,
                                                  options: int)
                                                  -> ReplaceResult {
    let mut offset = offset;
    let subject_len = subject.len();
    assert!(offset <= subject_len);

    let mut strings = ~[];
    strings.push(subject.slice(0, offset).to_owned());
    loop {
        let r = search_from(pattern.clone(), subject, offset, options);
        match r {
            Ok(m) => {
                strings.push(subject.slice(offset, m.begin()).to_owned());
                strings.push(repl_fn(m));
                offset = m.end();
            }
            Err(ExecErr(e)) if e == PCRE_ERROR_NOMATCH => {
                if offset != subject_len {
                    strings.push(subject.slice(offset, subject_len).to_owned());
                }
                break;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    return Ok(@strings.concat());
}

pub fn fmt_compile_err(e: CompileErr) -> ~str {
    return format!("error {:d}: {:s} at offset {:u}", e.code, e.reason, e.offset);
}


/// Return true iff `sr` indicates that the subject did not match the pattern
pub fn is_nomatch(sr: SearchResult) -> bool {
    match sr {
        Err(ExecErr(e)) if e == PCRE_ERROR_NOMATCH => true,
        _ => false,
    }
}
