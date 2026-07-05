fn main() {
    use std::io::Read;
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);
    println!("{}", claude_statusline::run(&buf));
}
