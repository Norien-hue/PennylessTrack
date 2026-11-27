fn main() {
    let fake_ghp: &str = "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";
    println!("Hello, world!, {}", fake_ghp);
}

fn very_unsafe_fn() {
    let pid = unsafe { libc::getppid() };
}