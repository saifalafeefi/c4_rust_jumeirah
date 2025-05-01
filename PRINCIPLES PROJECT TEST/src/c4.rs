use std::fs::File;
use std::io::Read;
use std::ptr;
use std::slice;
use std::str;
use std::ffi::CString;

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
    
    // Source buffer
    source: Vec<u8>,
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
        }
    }
    
    // Basic lexer - partial implementation
    unsafe fn next(&mut self) {
        let pp: *mut u8;
        
        loop {
            self.tk = *self.p as i64;
            if self.tk == 0 {
                return;
            }
            
            self.p = self.p.add(1);
            
            if self.tk == '\n' as i64 {
                self.line += 1;
            }
            else if self.tk == '#' as i64 {
                // Skip preprocessor
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
                *self.id.offset(TK as isize) = ID;
                self.tk = ID;
                return;
            }
            else if self.tk >= '0' as i64 && self.tk <= '9' as i64 {
                // Parse number
                self.ival = 0;
                if self.tk != '0' as i64 {
                    self.ival = self.tk - '0' as i64;
                    while *self.p >= '0' as u8 && *self.p <= '9' as u8 {
                        self.ival = self.ival * 10 + *self.p as i64 - '0' as i64;
                        self.p = self.p.add(1);
                    }
                }
                else if *self.p == 'x' as u8 || *self.p == 'X' as u8 {
                    self.p = self.p.add(1);
                    loop {
                        let digit = *self.p as i64;
                        if !((digit >= '0' as i64 && digit <= '9' as i64) || 
                             (digit >= 'a' as i64 && digit <= 'f' as i64) ||
                             (digit >= 'A' as i64 && digit <= 'F' as i64)) {
                            break;
                        }
                        self.ival = self.ival * 16 + (digit & 15) + 
                                   (if digit >= 'A' as i64 { 9 } else { 0 });
                        self.p = self.p.add(1);
                    }
                }
                else {
                    while *self.p >= '0' as u8 && *self.p <= '7' as u8 {
                        self.ival = self.ival * 8 + *self.p as i64 - '0' as i64;
                        self.p = self.p.add(1);
                    }
                }
                
                self.tk = NUM;
                return;
            }
            // Just enough to parse a "Hello, World" program
            else if self.tk == '(' as i64 || self.tk == ')' as i64 || 
                    self.tk == ';' as i64 || self.tk == '}' as i64 || 
                    self.tk == '{' as i64 {
                return;
            }
            else if self.tk == '"' as i64 {
                // String literal
                pp = self.data;
                
                while *self.p != 0 && *self.p != '"' as u8 {
                    self.ival = *self.p as i64;
                    self.p = self.p.add(1);
                    
                    if self.ival == '\\' as i64 {
                        self.ival = *self.p as i64;
                        self.p = self.p.add(1);
                        
                        if self.ival == 'n' as i64 {
                            self.ival = '\n' as i64;
                        }
                    }
                    
                    *self.data = self.ival as u8;
                    self.data = self.data.add(1);
                }
                
                self.p = self.p.add(1);
                self.ival = pp as i64;
                return;
            }
        }
    }
    
    // Simple memcmp implementation
    unsafe fn memcmp(&self, s1: *const u8, s2: *const u8, n: usize) -> i32 {
        for i in 0..n {
            let b1 = *s1.add(i);
            let b2 = *s2.add(i);
            if b1 != b2 {
                return (b1 as i32) - (b2 as i32);
            }
        }
        0
    }
    
    // Very simplified expr function - just enough to parse simple function calls
    unsafe fn expr(&mut self, _lev: i64) {
        if self.tk == ID {
            // Handle function call (printf)
            let d = self.id;
            self.next();
            
            if self.tk == '(' as i64 {
                self.next();
                let mut t = 0;
                
                // Parse arguments
                while self.tk != ')' as i64 {
                    // Simplified handling for now
                    if self.tk == '"' as i64 {
                        *self.e.add(1) = IMM;
                        *self.e.add(2) = self.ival;
                        self.e = self.e.add(2);
                        self.next();
                    }
                    *self.e.add(1) = PSH;
                    self.e = self.e.add(1);
                    t += 1;
                    
                    if self.tk == ',' as i64 {
                        self.next();
                    }
                }
                
                self.next();
                
                // Function call
                if *d.offset(CLASS as isize) == SYS {
                    *self.e.add(1) = *d.offset(VAL as isize);
                    self.e = self.e.add(1);
                }
                
                // Adjust stack
                if t > 0 {
                    *self.e.add(1) = ADJ;
                    *self.e.add(2) = t;
                    self.e = self.e.add(2);
                }
            }
        }
    }
    
    // Very simplified statement function
    unsafe fn stmt(&mut self) {
        // Basic statement parsing
        if self.tk == '{' as i64 {
            self.next();
            while self.tk != '}' as i64 {
                self.stmt();
            }
            self.next();
        }
        else if self.tk == RETURN {
            self.next();
            if self.tk != ';' as i64 {
                self.expr(ASSIGN);
            }
            self.next();
            
            // Return from function
            *self.e.add(1) = LEV;
            self.e = self.e.add(1);
        }
        else if self.tk == ';' as i64 {
            self.next();
        }
        else {
            self.expr(ASSIGN);
            if self.tk == ';' as i64 {
                self.next();
            }
        }
    }
    
    // Simple runner to compile and execute code
    pub unsafe fn run(&mut self, args: Vec<String>) -> i32 {
        if args.len() < 2 {
            println!("usage: c4 [-s] [-d] file ...");
            return -1;
        }
        
        let mut i = 1;
        self.src = 0;
        self.debug = 0;
        
        if i < args.len() && args[i].starts_with("-s") {
            self.src = 1;
            i += 1;
        }
        
        if i < args.len() && args[i].starts_with("-d") {
            self.debug = 1;
            i += 1;
        }
        
        if i >= args.len() {
            println!("usage: c4 [-s] [-d] file ...");
            return -1;
        }
        
        // Open and read input file
        let mut file = match File::open(&args[i]) {
            Ok(file) => file,
            Err(_) => {
                println!("could not open({})", args[i]);
                return -1;
            }
        };
        
        self.source.clear();
        match file.read_to_end(&mut self.source) {
            Ok(_) => (),
            Err(_) => {
                println!("read() failed");
                return -1;
            }
        };
        self.source.push(0); // Null terminator
        
        // Allocate memory
        let poolsz = 256 * 1024; // arbitrary size
        
        let sym = vec![0i64; poolsz].into_boxed_slice();
        self.sym = Box::into_raw(sym) as *mut i64;
        
        let e = vec![0i64; poolsz].into_boxed_slice();
        self.e = Box::into_raw(e) as *mut i64;
        self.le = self.e;
        
        let data = vec![0u8; poolsz].into_boxed_slice();
        self.data = Box::into_raw(data) as *mut u8;
        
        let stack = vec![0i64; poolsz].into_boxed_slice();
        let mut sp = Box::into_raw(stack) as *mut i64;
        sp = sp.add(poolsz);
        
        // Initialize symbol table with keywords
        let keywords = "char else enum if int return sizeof while open read close printf malloc free memset memcmp exit void main";
        let keywords_cstr = CString::new(keywords).unwrap();
        self.p = keywords_cstr.as_ptr() as *mut u8;
        
        // Add keywords to symbol table
        let mut i: i64 = CHAR;
        while i <= WHILE {
            self.next();
            *self.id.offset(TK as isize) = i;
            i += 1;
        }
        
        // Add system calls to symbol table
        i = OPEN;
        while i <= EXIT {
            self.next();
            *self.id.offset(CLASS as isize) = SYS;
            *self.id.offset(TYPE as isize) = T_INT;
            *self.id.offset(VAL as isize) = i;
            i += 1;
        }
        
        // Handle void type
        self.next();
        *self.id.offset(TK as isize) = CHAR;
        
        // Keep track of main
        self.next();
        let idmain = self.id;
        
        // Reset source pointer
        self.p = self.source.as_mut_ptr();
        self.lp = self.p;
        
        // Minimal parsing - just look for main() and some basic function calls
        self.line = 1;
        self.next();
        
        while self.tk != 0 {
            // Very simplified parsing - just enough to handle basic programs
            if self.tk == INT {
                // Found a type declaration
                self.next();
                if self.tk == ID && self.memcmp((*self.id.offset(NAME as isize)) as *const u8, 
                                         "main\0".as_ptr(), 5) == 0 {
                    // Found main function
                    self.next();
                    if self.tk == '(' as i64 {
                        self.next();
                        if self.tk == ')' as i64 {
                            self.next();
                            if self.tk == '{' as i64 {
                                *idmain.offset(CLASS as isize) = FUN;
                                *idmain.offset(VAL as isize) = (self.e as i64) + 8; // Point to function body
                                *self.e.add(1) = ENT;
                                *self.e.add(2) = 0; // Local vars
                                self.e = self.e.add(2);
                                
                                // Parse function body
                                self.stmt();
                                
                                // Add return if needed
                                *self.e.add(1) = LEV;
                                self.e = self.e.add(1);
                            }
                        }
                    }
                }
            }
            self.next();
        }
        
        // Check if main() was defined
        if *idmain.offset(VAL as isize) == 0 {
            println!("main() not defined");
            return -1;
        }
        
        if self.src != 0 {
            // Source-only mode
            return 0;
        }
        
        // Set up VM
        let pc = *idmain.offset(VAL as isize) as *mut i64;
        let mut bp = sp;
        
        // Call exit if main returns
        sp = sp.sub(1);
        *sp = EXIT;
        
        // Setup main() arguments (simplified)
        sp = sp.sub(1);
        *sp = PSH;
        
        let t = sp;
        sp = sp.sub(1);
        *sp = 0; // argc
        
        sp = sp.sub(1);
        *sp = 0; // argv
        
        sp = sp.sub(1);
        *sp = t as i64;
        
        // Run VM (simplified)
        println!("Running main():");
        let mut pc = pc;
        let mut a: i64 = 0;
        let mut cycle: i64 = 0;
        
        loop {
            let i = *pc;
            pc = pc.add(1);
            cycle += 1;
            
            if self.debug != 0 {
                self.print_opcode(i, *pc, cycle);
            }
            
            match i {
                IMM => { a = *pc; pc = pc.add(1); }
                LEA => { a = (bp as i64) + *pc; pc = pc.add(1); }
                JMP => { pc = *pc as *mut i64; }
                JSR => { sp = sp.sub(1); *sp = (pc.add(1)) as i64; pc = *pc as *mut i64; }
                BZ => { pc = if a == 0 { *pc as *mut i64 } else { pc.add(1) }; }
                BNZ => { pc = if a != 0 { *pc as *mut i64 } else { pc.add(1) }; }
                ENT => { sp = sp.sub(1); *sp = bp as i64; bp = sp; sp = sp.sub(*pc as usize); pc = pc.add(1); }
                ADJ => { sp = sp.add(*pc as usize); pc = pc.add(1); }
                LEV => { sp = bp; bp = *sp as *mut i64; sp = sp.add(1); pc = *sp as *mut i64; sp = sp.add(1); }
                LI => { a = *(a as *mut i64); }
                LC => { a = *(a as *mut u8) as i64; }
                SI => { *((*sp) as *mut i64) = a; sp = sp.add(1); }
                SC => { *((*sp) as *mut u8) = a as u8; sp = sp.add(1); a = a & 0xff; }
                PSH => { sp = sp.sub(1); *sp = a; }
                
                OR_ => { a = *sp | a; sp = sp.add(1); }
                XOR_ => { a = *sp ^ a; sp = sp.add(1); }
                AND_ => { a = *sp & a; sp = sp.add(1); }
                EQ_ => { a = (*sp == a) as i64; sp = sp.add(1); }
                NE_ => { a = (*sp != a) as i64; sp = sp.add(1); }
                LT_ => { a = (*sp < a) as i64; sp = sp.add(1); }
                GT_ => { a = (*sp > a) as i64; sp = sp.add(1); }
                LE_ => { a = (*sp <= a) as i64; sp = sp.add(1); }
                GE_ => { a = (*sp >= a) as i64; sp = sp.add(1); }
                SHL_ => { a = *sp << a; sp = sp.add(1); }
                SHR_ => { a = *sp >> a; sp = sp.add(1); }
                ADD_ => { a = *sp + a; sp = sp.add(1); }
                SUB_ => { a = *sp - a; sp = sp.add(1); }
                MUL_ => { a = *sp * a; sp = sp.add(1); }
                DIV_ => { a = *sp / a; sp = sp.add(1); }
                MOD_ => { a = *sp % a; sp = sp.add(1); }
                
                PRTF => {
                    let t = sp.add(*pc as usize);
                    pc = pc.add(1);
                    
                    let fmt = (*t.sub(1)) as *const u8;
                    
                    // Super simple printf implementation - just enough for "Hello, World"
                    let s = slice::from_raw_parts(fmt, 100);
                    let mut i = 0;
                    
                    while i < s.len() && s[i] != 0 {
                        print!("{}", s[i] as char);
                        i += 1;
                    }
                }
                
                EXIT => {
                    println!("\nExit program with code {}", *sp);
                    return *sp as i32;
                }
                
                _ => {
                    println!("\nUnhandled opcode {}!", i);
                    return -1;
                }
            }
        }
    }
    
    unsafe fn print_opcode(&self, op: i64, arg: i64, cycle: i64) {
        let op_names = [
            "LEA ", "IMM ", "JMP ", "JSR ", "BZ  ", "BNZ ", "ENT ", "ADJ ",
            "LEV ", "LI  ", "LC  ", "SI  ", "SC  ", "PSH ", "OR  ", "XOR ",
            "AND ", "EQ  ", "NE  ", "LT  ", "GT  ", "LE  ", "GE  ", "SHL ",
            "SHR ", "ADD ", "SUB ", "MUL ", "DIV ", "MOD ", "OPEN", "READ",
            "CLOS", "PRTF", "MALC", "FREE", "MSET", "MCMP", "EXIT"
        ];
        
        if op < 0 || op as usize >= op_names.len() {
            println!("{}> UNKNOWN({})", cycle, op);
        } else {
            if op <= ADJ {
                println!("{}> {} {}", cycle, op_names[op as usize], arg);
            } else {
                println!("{}> {}", cycle, op_names[op as usize]);
            }
        }
    }
} 