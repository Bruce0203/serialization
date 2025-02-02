fn main() {
    let a = [0, 1, 1];
    match [0, 1, 2] {
        v if v == a => {
            println!("123")
        }
        _ => {}
    }
}
