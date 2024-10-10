


fn main() {
    let new_file: &str = "hello.txt";
    let command_to_execute: String = format!("touch {}", new_file);
    print!ln("{}", command_to_execute);
}