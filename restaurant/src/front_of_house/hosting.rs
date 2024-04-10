pub fn add_to_waitlist() {
    unsafe {
        static mut COUNT: u8 = 0;
        COUNT += 1;
        dbg!("add {}", COUNT);
    }
}