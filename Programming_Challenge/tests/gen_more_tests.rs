use std::fs;

fn main() {
    for x in 25..=50 {
        let out = std::process::Command::new("python")
            .args(["./generate_tests.py", "L"])
            .output()
            .expect("Should run just fine...")
            .stdout;
        fs::write(format!("test{}.in", x), out).expect("Should be just fine...")
    }
}
