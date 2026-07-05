fn main() {
    use std::io::Read;
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);
    let _ = claude_statusline::input::StatusInput::parse(&buf);
    println!("claude-statusline: (wip)");
}
