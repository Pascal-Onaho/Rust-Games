use std::io;

fn main() {

    let arr: [char; 4] = ['A', 'C', 'G', 'T'];

    println!("The DNA bases are {:?}", arr);

    let mut index = String::new();

    println!("Input an index from 0 to 3 in order to see the element at that index:");


    io::stdin()
        .read_line(&mut index)
        .expect("The input was not recorded");

    let index: usize = index.trim().parse().expect("This is not a number");

    let element = arr[index];

    println!("The element at the index is {}", element);

}