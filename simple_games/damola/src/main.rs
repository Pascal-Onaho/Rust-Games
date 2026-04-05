fn main() {
    let s = "Hello"; // String literal(Stored on stack)
    let p = String::from("Pascal"); // String (Stored on heap)
    println!("Here are the words: {} {}", s, p);
}
