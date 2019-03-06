/// # System Callable Interrupt Handler
pub fn interrupt() {
    loop {
        super::halt();
    }
}
