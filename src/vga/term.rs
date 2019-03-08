//use lazy_static::lazy_static;
//use spin::Mutex;
//
//lazy_static! {
//    pub static ref TERM: Mutex<Term> = Mutex::new(Term::new());
//}
//
//pub struct Term {
//    buffer: [char; 81],
//    i: usize,
//}
//
//impl Term {
//    pub fn new() -> Term {
//        let level_4_table_pointer = 0xffff_ffff_ffff_f000 as *const u64;
//        Term {
//            buffer: unsafe { *(level_4_table_pointer as *mut [char; 81]) },
//            i: 0,
//        }
//    }
//
//    pub fn read(&mut self, c: char) {
//        match c {
//            '\x08' => self.erase(),
//            '\n' => self.newline(),
//            _ => self.consume(c),
//        }
//    }
//
//    pub fn init(&mut self) {
//        self.flush();
//        self.prompt();
//    }
//
//    fn newline(&mut self) {
//        self.eval();
//        self.flush();
//        self.prompt();
//    }
//
//    fn erase(&mut self) {
//        if self.i <= 0 {
//            return;
//        }
//        self.i -= 1;
//        self.buffer[self.i] = ' ';
//    }
//
//    fn eval(&mut self) {
//        for j in 0..self.i {
//            crate::print!("{}", self.buffer[j]);
//        }
//        crate::println!("\n");
//    }
//
//    fn flush(&mut self) {
//        self.i = 80;
//        while self.i > 0 {
//            self.buffer[self.i - 1] = ' ';
//            self.i -= 1;
//        }
//    }
//
//    fn prompt(&mut self) {
//        crate::print!("root@tetanos: ");
//    }
//
//    fn consume(&mut self, c: char) {
//        self.buffer[self.i] = c;
//        self.i = (self.i + 1) % 81;;
//    }
//}
