pub const PCRE_CASELESS: int          = 0x00000001; // Compile
pub const PCRE_MULTILINE: int         = 0x00000002; // Compile
pub const PCRE_DOTALL: int            = 0x00000004; // Compile
pub const PCRE_EXTENDED: int          = 0x00000008; // Compile
pub const PCRE_ANCHORED: int          = 0x00000010; // Compile, exec, DFA exec
pub const PCRE_DOLLAR_ENDONLY: int    = 0x00000020; // Compile
pub const PCRE_EXTRA: int             = 0x00000040; // Compile
pub const PCRE_NOTBOL: int            = 0x00000080; // Exec, DFA exec
pub const PCRE_NOTEOL: int            = 0x00000100; // Exec, DFA exec
pub const PCRE_UNGREEDY: int          = 0x00000200; // Compile
pub const PCRE_NOTEMPTY: int          = 0x00000400; // Exec, DFA exec
pub const PCRE_UTF8: int              = 0x00000800; // Compile
pub const PCRE_NO_AUTO_CAPTURE: int   = 0x00001000; // Compile
pub const PCRE_NO_UTF8_CHECK: int     = 0x00002000; // Compile, exec, DFA exec
pub const PCRE_AUTO_CALLOUT: int      = 0x00004000; // Compile
pub const PCRE_PARTIAL_SOFT: int      = 0x00008000; // Exec, DFA exec
pub const PCRE_PARTIAL: int           = 0x00008000; // Backwards compatible synonym
pub const PCRE_DFA_SHORTEST: int      = 0x00010000; // DFA exec
pub const PCRE_DFA_RESTART: int       = 0x00020000; // DFA exec
pub const PCRE_FIRSTLINE: int         = 0x00040000; // Compile
pub const PCRE_DUPNAMES: int          = 0x00080000; // Compile
pub const PCRE_NEWLINE_CR: int        = 0x00100000; // Compile, exec, DFA exec
pub const PCRE_NEWLINE_LF: int        = 0x00200000; // Compile, exec, DFA exec
pub const PCRE_NEWLINE_CRLF: int      = 0x00300000; // Compile, exec, DFA exec
pub const PCRE_NEWLINE_ANY: int       = 0x00400000; // Compile, exec, DFA exec
pub const PCRE_NEWLINE_ANYCRLF: int   = 0x00500000; // Compile, exec, DFA exec
pub const PCRE_BSR_ANYCRLF: int       = 0x00800000; // Compile, exec, DFA exec
pub const PCRE_BSR_UNICODE: int       = 0x01000000; // Compile, exec, DFA exec
pub const PCRE_JAVASCRIPT_COMPAT: int = 0x02000000; // Compile
pub const PCRE_NO_START_OPTIMIZE: int = 0x04000000; // Compile, exec, DFA exec
pub const PCRE_NO_START_OPTIMISE: int = 0x04000000; // Synonym
pub const PCRE_PARTIAL_HARD: int      = 0x08000000; // Exec, DFA exec
pub const PCRE_NOTEMPTY_ATSTART: int  = 0x10000000; // Exec, DFA exec
pub const PCRE_UCP: int               = 0x20000000; // Compile

pub const COMPILE_OPTIONS: int        = 0x27fc7a7f;
pub const EXEC_OPTIONS: int           = 0x1df0a590;

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

pub const PCRE_ERROR_NOMATCH: int        =  -1;
pub const PCRE_ERROR_NULL: int           =  -2;
pub const PCRE_ERROR_BADOPTION: int      =  -3;
pub const PCRE_ERROR_BADMAGIC: int       =  -4;
pub const PCRE_ERROR_UNKNOWN_OPCODE: int =  -5;
pub const PCRE_ERROR_UNKNOWN_NODE: int   =  -5;  // For backward compatibility
pub const PCRE_ERROR_NOMEMORY: int       =  -6;
pub const PCRE_ERROR_NOSUBSTRING: int    =  -7;
pub const PCRE_ERROR_MATCHLIMIT: int     =  -8;
pub const PCRE_ERROR_CALLOUT: int        =  -9;  // Never used by PCRE itself
pub const PCRE_ERROR_BADUTF8: int        = -10;
pub const PCRE_ERROR_BADUTF8_OFFSET: int = -11;
pub const PCRE_ERROR_PARTIAL: int        = -12;
pub const PCRE_ERROR_BADPARTIAL: int     = -13;
pub const PCRE_ERROR_INTERNAL: int       = -14;
pub const PCRE_ERROR_BADCOUNT: int       = -15;
pub const PCRE_ERROR_DFA_UITEM: int      = -16;
pub const PCRE_ERROR_DFA_UCOND: int      = -17;
pub const PCRE_ERROR_DFA_UMLIMIT: int    = -18;
pub const PCRE_ERROR_DFA_WSSIZE: int     = -19;
pub const PCRE_ERROR_DFA_RECURSE: int    = -20;
pub const PCRE_ERROR_RECURSIONLIMIT: int = -21;
pub const PCRE_ERROR_NULLWSLIMIT: int    = -22;  // No longer actually used
pub const PCRE_ERROR_BADNEWLINE: int     = -23;
pub const PCRE_ERROR_BADOFFSET: int      = -24;
pub const PCRE_ERROR_SHORTUTF8: int      = -25;

pub const PCRE_INFO_OPTIONS: int         =   0;
pub const PCRE_INFO_SIZE: int            =   1;
pub const PCRE_INFO_CAPTURECOUNT: int    =   2;
pub const PCRE_INFO_BACKREFMAX: int      =   3;
pub const PCRE_INFO_FIRSTBYTE: int       =   4;
pub const PCRE_INFO_FIRSTCHAR: int       =   4; // For backwards compatibility
pub const PCRE_INFO_FIRSTTABLE: int      =   5;
pub const PCRE_INFO_LASTLITERAL: int     =   6;
pub const PCRE_INFO_NAMEENTRYSIZE: int   =   7;
pub const PCRE_INFO_NAMECOUNT: int       =   8;
pub const PCRE_INFO_NAMETABLE: int       =   9;
pub const PCRE_INFO_STUDYSIZE: int       =  10;
pub const PCRE_INFO_DEFAULT_TABLES: int  =  11;
pub const PCRE_INFO_OKPARTIAL: int       =  12;
pub const PCRE_INFO_JCHANGED: int        =  13;
pub const PCRE_INFO_HASCRORLF: int       =  14;
pub const PCRE_INFO_MINLENGTH: int       =  15;
