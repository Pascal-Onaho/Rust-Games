fn main() {
    println!("Hello, world!");
    let y = {
        let x = 6;
        x + 1
    };

    let a = five(); // let a: f64 = five().into();  (This converts the i32 (five()) into an f64)

    another_function(5.3, 'h');
    println!("This is {y}");
    println!("The value of a is: {}", a);
}

fn another_function(x: f64, measurement_label: char) {

    println!("The time it took you to code is {}{}", x, measurement_label);
    println!("Another Function!");
}

fn five() -> i32 {
    5
}
