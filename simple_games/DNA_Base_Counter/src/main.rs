use std::io;

fn main() {
    let dna_bases: [char; 4] = ['A', 'C', 'T', 'G'];

    // Stack data → simple, Copy type (no complex ownership behavior)

    println!("These are the four DNA bases in biology: {:?}", dna_bases);


    let mut user_char = String::new();
     // user_char OWNS a String (heap data starts here)

    println!("Enter a character:");

    io::stdin()
        .read_line(&mut user_char)
        .expect("The input was not recorded");

    let user_char: char = user_char.trim()
                                   .parse()
                                   .expect("This is not a CHAR type!");
    
    if user_char == dna_bases[0] {
        println!("This is a valid DNA base");
    } else if user_char == dna_bases[1] {
        println!("This is a valid DNA base");
    } else if user_char == dna_bases[2] {
        println!("This is a valid DNA base");
    } else if user_char == dna_bases[3] {
        println!("This is a valid DNA base");
    } else {
        println!("THIS IS NOT A VALID DNA BASE. BOY, BYE!");
    }
}