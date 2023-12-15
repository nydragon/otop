/// Send an interrupt signal to a specified process.
pub fn _kill_process(pid: i32, sig: i32) {
    unsafe {
        libc::kill(pid, sig);
    };
}
