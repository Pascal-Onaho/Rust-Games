fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("{counter}");

        if counter == 10 {
            break counter;
        }
    };

    println!("The result is {result}");

    'outer: loop {
        println!("This is the outer loop");

        loop {
            println!("This is the inner loop");
            break 'outer
        }
    }
    println!("Done!");
}