use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::{ptr, mem, slice, str};
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::ffi::CString;
use std::fmt::Write as FmtWrite;

// Token constants
pub const NUM: i64 = 128;
pub const FUN: i64 = 129;
pub const SYS: i64 = 130;
pub const GLO: i64 = 131;
pub const LOC: i64 = 132;
pub const ID: i64 = 133;
pub const CHAR: i64 = 134;
pub const ELSE: i64 = 135;
pub const ENUM: i64 = 136;
pub const IF: i64 = 137;
pub const INT: i64 = 138;
pub const RETURN: i64 = 139;
pub const SIZEOF: i64 = 140;
pub const WHILE: i64 = 141;
pub const ASSIGN: i64 = 142;
pub const COND: i64 = 143;
pub const LOR: i64 = 144;
pub const LAN: i64 = 145;
pub const OR: i64 = 146;
pub const XOR: i64 = 147;
pub const AND: i64 = 148;
pub const EQ: i64 = 149;
pub const NE: i64 = 150;
pub const LT: i64 = 151;
pub const GT: i64 = 152;
pub const LE: i64 = 153;
pub const GE: i64 = 154;
pub const SHL: i64 = 155;
pub const SHR: i64 = 156;
pub const ADD: i64 = 157;
pub const SUB: i64 = 158;
pub const MUL: i64 = 159;
pub const DIV: i64 = 160;
pub const MOD: i64 = 161;
pub const INC: i64 = 162;
pub const DEC: i64 = 163;
pub const BRAK: i64 = 164;

// VM opcodes
pub const LEA: i64 = 0;
pub const IMM: i64 = 1;
pub const JMP: i64 = 2;
pub const JSR: i64 = 3;
pub const BZ: i64 = 4;
pub const BNZ: i64 = 5;
pub const ENT: i64 = 6;
pub const ADJ: i64 = 7;
pub const LEV: i64 = 8;
pub const LI: i64 = 9;
pub const LC: i64 = 10;
pub const SI: i64 = 11;
pub const SC: i64 = 12;
pub const PSH: i64 = 13;
pub const OR_: i64 = 14;
pub const XOR_: i64 = 15;
pub const AND_: i64 = 16;
pub const EQ_: i64 = 17;
pub const NE_: i64 = 18;
pub const LT_: i64 = 19;
pub const GT_: i64 = 20;
pub const LE_: i64 = 21;
pub const GE_: i64 = 22;
pub const SHL_: i64 = 23;
pub const SHR_: i64 = 24;
pub const ADD_: i64 = 25;
pub const SUB_: i64 = 26;
pub const MUL_: i64 = 27;
pub const DIV_: i64 = 28;
pub const MOD_: i64 = 29;
pub const OPEN: i64 = 30;
pub const READ: i64 = 31;
pub const CLOS: i64 = 32;
pub const PRTF: i64 = 33;
pub const MALC: i64 = 34;
pub const FREE: i64 = 35;
pub const MSET: i64 = 36;
pub const MCMP: i64 = 37;
pub const EXIT: i64 = 38;

// Types
pub const T_CHAR: i64 = 0;
pub const T_INT: i64 = 1;
pub const T_PTR: i64 = 2;

// Identifier offsets
pub const TK: i64 = 0;
pub const HASH: i64 = 1;
pub const NAME: i64 = 2;
pub const CLASS: i64 = 3;
pub const TYPE: i64 = 4;
pub const VAL: i64 = 5;
pub const HCLASS: i64 = 6;
pub const HTYPE: i64 = 7;
pub const HVAL: i64 = 8;
pub const IDSZ: i64 = 9;

pub struct C4 {
    // Current position in source code
    p: *mut u8,
    lp: *mut u8,
    // Data/bss pointer
    data: *mut u8,
    
    // Current position in emitted code
    e: *mut i64,
    le: *mut i64,
    // Currently parsed identifier
    id: *mut i64,
    // Symbol table (simple list of identifiers)
    sym: *mut i64,
    // Current token
    tk: i64,
    // Current token value
    ival: i64,
    // Current expression type
    ty: i64,
    // Local variable offset
    loc: i64,
    // Current line number
    line: i64,
    // Print source and assembly flag
    src: i64,
    // Print executed instructions
    debug: i64,
    
    // Original source code
    source: Vec<u8>,
    
    // Stack for VM
    sp: *mut i64,
    bp: *mut i64,
}

impl C4 {
    pub fn new() -> Self {
        C4 {
            p: ptr::null_mut(),
            lp: ptr::null_mut(),
            data: ptr::null_mut(),
            e: ptr::null_mut(),
            le: ptr::null_mut(),
            id: ptr::null_mut(),
            sym: ptr::null_mut(),
            tk: 0,
            ival: 0,
            ty: 0,
            loc: 0,
            line: 0,
            src: 0,
            debug: 0,
            source: Vec::new(),
            sp: ptr::null_mut(),
            bp: ptr::null_mut(),
        }
    }
    
    // Lexer - tokenize input 
    unsafe fn next(&mut self) {
        let mut pp: *mut u8;
        
        loop {
            self.tk = *self.p as i64;
            if self.tk == 0 {
                return;
            }
            
            self.p = self.p.add(1);
            
            if self.tk == '\n' as i64 {
                if self.src != 0 {
                    print!("{}: ", self.line);
                    let len = self.p.offset_from(self.lp) as usize;
                    let slice = slice::from_raw_parts(self.lp, len);
                    print!("{}", str::from_utf8_unchecked(slice));
                    
                    self.lp = self.p;
                    while self.le < self.e {
                        self.le = self.le.add(1);
                        let op = *self.le;
                        
                        // Get opcode string
                        let op_names = "LEA ,IMM ,JMP ,JSR ,BZ  ,BNZ ,ENT ,ADJ ,LEV ,LI  ,LC  ,SI  ,SC  ,PSH ,\
                                      OR  ,XOR ,AND ,EQ  ,NE  ,LT  ,GT  ,LE  ,GE  ,SHL ,SHR ,ADD ,SUB ,MUL ,DIV ,MOD ,\
                                      OPEN,READ,CLOS,PRTF,MALC,FREE,MSET,MCMP,EXIT,";
                                      
                        let op_idx = (op * 5) as usize;
                        if op_idx + 5 <= op_names.len() {
                            print!("        {}", &op_names[op_idx..op_idx+4]);
                        }
                        
                        self.le = self.le.add(1);
                        if op <= ADJ {
                            println!(" {}", *self.le);
                        } else {
                            println!();
                        }
                    }
                }
                self.line += 1;
            }
            else if self.tk == '#' as i64 {
                // Skip macro or preprocessor
                while *self.p != 0 && *self.p != '\n' as u8 {
                    self.p = self.p.add(1);
                }
            }
            else if (self.tk >= 'a' as i64 && self.tk <= 'z' as i64) || 
                   (self.tk >= 'A' as i64 && self.tk <= 'Z' as i64) || 
                   self.tk == '_' as i64 {
                // Parse identifier
                pp = self.p.sub(1);
                
                while (*self.p >= 'a' as u8 && *self.p <= 'z' as u8) || 
                      (*self.p >= 'A' as u8 && *self.p <= 'Z' as u8) || 
                      (*self.p >= '0' as u8 && *self.p <= '9' as u8) || 
                      *self.p == '_' as u8 {
                    self.tk = self.tk * 147 + *self.p as i64;
                    self.p = self.p.add(1);
                }
                
                self.tk = (self.tk << 6) + (self.p.offset_from(pp) as i64);
                self.id = self.sym;
                
                while *self.id.offset(TK as isize) != 0 {
                    if self.tk == *self.id.offset(HASH as isize) &&
                       self.memcmp(*self.id.offset(NAME as isize) as *const u8, 
                              pp as *const u8, 
                              self.p.offset_from(pp) as usize) == 0 {
                        self.tk = *self.id.offset(TK as isize);
                        return;
                    }
                    self.id = self.id.add(IDSZ as usize);
                }
                
                *self.id.offset(NAME as isize) = pp as i64;
                *self.id.offset(HASH as isize) = self.tk;
                self.tk = *self.id.offset(TK as isize) = ID;
                return;
            }
            else if self.tk >= '0' as i64 && self.tk <= '9' as i64 {
                // Parse number
                if self.tk != '0' as i64 {
                    self.ival = self.tk - '0' as i64;
                    while *self.p >= '0' as u8 && *self.p <= '9' as u8 {
                        self.ival = self.ival * 10 + *self.p as i64 - '0' as i64;
                        self.p = self.p.add(1);
                    }
                }
                else if *self.p == 'x' as u8 || *self.p == 'X' as u8 {
                    self.ival = 0;
                    self.p = self.p.add(1);
                    while true {
                        self.tk = *self.p as i64;
                        if !((self.tk >= '0' as i64 && self.tk <= '9' as i64) || 
                             (self.tk >= 'a' as i64 && self.tk <= 'f' as i64) || 
                             (self.tk >= 'A' as i64 && self.tk <= 'F' as i64)) {
                            break;
                        }
                        self.ival = self.ival * 16 + (self.tk & 15) + 
                                   (if self.tk >= 'A' as i64 { 9 } else { 0 });
                        self.p = self.p.add(1);
                    }
                }
                else {
                    self.ival = 0;
                    while *self.p >= '0' as u8 && *self.p <= '7' as u8 {
                        self.ival = self.ival * 8 + *self.p as i64 - '0' as i64;
                        self.p = self.p.add(1);
                    }
                }
                
                self.tk = NUM;
                return;
            }
            else if self.tk == '/' as i64 {
                if *self.p == '/' as u8 {
                    // Line comment
                    self.p = self.p.add(1);
                    while *self.p != 0 && *self.p != '\n' as u8 {
                        self.p = self.p.add(1);
                    }
                }
                else {
                    // Division operator
                    self.tk = DIV;
                    return;
                }
            }
            else if self.tk == '\'' as i64 || self.tk == '"' as i64 {
                // String or character literal
                pp = self.data;
                
                while *self.p != 0 && *self.p as i64 != self.tk {
                    self.ival = *self.p as i64;
                    self.p = self.p.add(1);
                    
                    if self.ival == '\\' as i64 {
                        self.ival = *self.p as i64;
                        self.p = self.p.add(1);
                        
                        if self.ival == 'n' as i64 {
                            self.ival = '\n' as i64;
                        }
                    }
                    
                    if self.tk == '"' as i64 {
                        *self.data = self.ival as u8;
                        self.data = self.data.add(1);
                    }
                }
                
                self.p = self.p.add(1);
                
                if self.tk == '"' as i64 {
                    self.ival = pp as i64;
                } else {
                    self.tk = NUM;
                }
                
                return;
            }
            else if self.tk == '=' as i64 {
                if *self.p == '=' as u8 {
                    self.p = self.p.add(1);
                    self.tk = EQ;
                } else {
                    self.tk = ASSIGN;
                }
                return;
            }
            else if self.tk == '+' as i64 {
                if *self.p == '+' as u8 {
                    self.p = self.p.add(1);
                    self.tk = INC;
                } else {
                    self.tk = ADD;
                }
                return;
            }
            else if self.tk == '-' as i64 {
                if *self.p == '-' as u8 {
                    self.p = self.p.add(1);
                    self.tk = DEC;
                } else {
                    self.tk = SUB;
                }
                return;
            }
            else if self.tk == '!' as i64 {
                if *self.p == '=' as u8 {
                    self.p = self.p.add(1);
                    self.tk = NE;
                }
                return;
            }
            else if self.tk == '<' as i64 {
                if *self.p == '=' as u8 {
                    self.p = self.p.add(1);
                    self.tk = LE;
                } else if *self.p == '<' as u8 {
                    self.p = self.p.add(1);
                    self.tk = SHL;
                } else {
                    self.tk = LT;
                }
                return;
            }
            else if self.tk == '>' as i64 {
                if *self.p == '=' as u8 {
                    self.p = self.p.add(1);
                    self.tk = GE;
                } else if *self.p == '>' as u8 {
                    self.p = self.p.add(1);
                    self.tk = SHR;
                } else {
                    self.tk = GT;
                }
                return;
            }
            else if self.tk == '|' as i64 {
                if *self.p == '|' as u8 {
                    self.p = self.p.add(1);
                    self.tk = LOR;
                } else {
                    self.tk = OR;
                }
                return;
            }
            else if self.tk == '&' as i64 {
                if *self.p == '&' as u8 {
                    self.p = self.p.add(1);
                    self.tk = LAN;
                } else {
                    self.tk = AND;
                }
                return;
            }
            else if self.tk == '^' as i64 {
                self.tk = XOR;
                return;
            }
            else if self.tk == '%' as i64 {
                self.tk = MOD;
                return;
            }
            else if self.tk == '*' as i64 {
                self.tk = MUL;
                return;
            }
            else if self.tk == '[' as i64 {
                self.tk = BRAK;
                return;
            }
            else if self.tk == '?' as i64 {
                self.tk = COND;
                return;
            }
            else if self.tk == '~' as i64 || self.tk == ';' as i64 || 
                     self.tk == '{' as i64 || self.tk == '}' as i64 || 
                     self.tk == '(' as i64 || self.tk == ')' as i64 || 
                     self.tk == ']' as i64 || self.tk == ',' as i64 || 
                     self.tk == ':' as i64 {
                return;
            }
        }
    }

    // Stubs for the remaining core functionality
    unsafe fn expr(&mut self, lev: i64) {
        println!("expr() called with level {}", lev);
    }
    
    unsafe fn stmt(&mut self) {
        println!("stmt() called");
    }
    
    // Various helper functions
    unsafe fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
        let s1_slice = slice::from_raw_parts(s1, n);
        let s2_slice = slice::from_raw_parts(s2, n);
        
        for i in 0..n {
            if s1_slice[i] != s2_slice[i] {
                return (s1_slice[i] as i32) - (s2_slice[i] as i32);
            }
        }
        
        0
    }
    
    // Simple runner to compile and execute code
    pub unsafe fn run(&mut self, args: Vec<String>) -> i32 {
        let mut i = 1;
        self.src = 0;
        self.debug = 0;
        
        if args.len() > 1 && args[i].starts_with("-s") {
            self.src = 1;
            i += 1;
        }
        
        if args.len() > i && args[i].starts_with("-d") {
            self.debug = 1;
            i += 1;
        }
        
        if i >= args.len() {
            println!("usage: c4 [-s] [-d] file ...");
            return -1;
        }
        
        // Open input file
        let mut file = match File::open(&args[i]) {
            Ok(file) => file,
            Err(_) => {
                println!("could not open({})", args[i]);
                return -1;
            }
        };
        
        // Read the file
        self.source.clear();
        match file.read_to_end(&mut self.source) {
            Ok(_) => (),
            Err(_) => {
                println!("could not read({})", args[i]);
                return -1;
            }
        };
        
        // Add null terminator
        self.source.push(0);
        
        // Allocate memory buffers
        let poolsz = 256 * 1024; // arbitrary size
        
        let sym = vec![0i64; poolsz].into_boxed_slice();
        let sym_ptr = Box::into_raw(sym) as *mut i64;
        
        let e = vec![0i64; poolsz].into_boxed_slice();
        let e_ptr = Box::into_raw(e) as *mut i64;
        
        let data = vec![0u8; poolsz].into_boxed_slice();
        let data_ptr = Box::into_raw(data) as *mut u8;
        
        let stack = vec![0i64; poolsz].into_boxed_slice();
        let stack_ptr = Box::into_raw(stack) as *mut i64;
        
        self.sym = sym_ptr;
        self.e = e_ptr;
        self.le = e_ptr;
        self.data = data_ptr;
        
        self.p = self.source.as_mut_ptr();
        self.lp = self.p;
        
        // Initialize symbol table
        let keywords = "char else enum if int return sizeof while \
                       open read close printf malloc free memset memcmp exit void main";
        let keywords_cstr = CString::new(keywords).unwrap();
        self.p = keywords_cstr.as_ptr() as *mut u8;
        
        // Add keywords to symbol table
        i = CHAR;
        while i <= WHILE {
            self.next();
            *self.id.offset(TK as isize) = i;
            i += 1;
        }
        
        // Add library functions to symbol table
        i = OPEN;
        while i <= EXIT {
            self.next();
            *self.id.offset(CLASS as isize) = SYS;
            *self.id.offset(TYPE as isize) = T_INT;
            *self.id.offset(VAL as isize) = i;
            i += 1;
        }
        
        // Void type
        self.next();
        *self.id.offset(TK as isize) = CHAR;
        
        // Keep track of main
        self.next();
        let idmain = self.id;
        
        // Reset source pointer to the actual source
        self.p = self.source.as_mut_ptr();
        self.lp = self.p;
        
        // Parse declarations
        self.line = 1;
        self.next();
        
        while self.tk != 0 {
            // This is where the parsing phase would come 
            // Will translate this logic next
            self.next();
        }
        
        // No main?
        if *idmain.offset(VAL as isize) == 0 {
            println!("main() not defined");
            return -1;
        }
        
        if self.src != 0 {
            return 0;
        }
        
        // Setup VM
        self.bp = stack_ptr.add(poolsz);
        self.sp = self.bp;
        
        // Call exit if main returns
        self.sp = self.sp.sub(1);
        *self.sp = EXIT;
        
        let pc = *idmain.offset(VAL as isize) as *mut i64;
        
        // VM execution (simplified for now)
        println!("Executing main()...");
        
        // VM logic would be here
        
        0 // success
    }
}

// C memcmp implementation
fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    unsafe {
        let s1_slice = slice::from_raw_parts(s1, n);
        let s2_slice = slice::from_raw_parts(s2, n);
        
        for i in 0..n {
            if s1_slice[i] != s2_slice[i] {
                return (s1_slice[i] as i32) - (s2_slice[i] as i32);
            }
        }
        
        0
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut c4 = C4::new();
    
    std::process::exit(unsafe { c4.run(args) as i32 });
} 