fn main() {
    println!("Hello, world!");

    let mut v = Vec::new();
    let s = vec![2, 4, 6, 8];

    for value in &s {
        println!("{}", value);
    }

    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    v.push(50);

    for value in &mut v {
        *value += 10;
        println!("{}", value);
    }

    println!("These are the vectors: {:?}, {:?}", v, s);

    let mut reads: Vec<String> = Vec::new();

    reads.push("ATCG". to_string());

    println!("{:?}", reads);

}
