/// System Callable Interrupt Handler
pub fn interrupt() {
    println!("this is a syscall bro");
    loop {
        super::halt();
    }
}
