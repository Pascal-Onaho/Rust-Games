use std::io;

fn main() {

    let game_title: (&str, u64, &str, u64, &str, u64) = ("Elden_ring", 2022, "E33", 2025, "Baldur's gate", 2024);
    println!("The games and their release dates are {:?}", game_title);
    let mut user_year = String::new();
    println!("Input any year for comparison to the aforementioned:");
    io::stdin()
        .read_line(&mut user_year)
        .expect("This input was not recorded");

    let user_year: u64 = user_year.trim()
                                   .parse()
                                   .expect("Urmm...this is not going as planned!");

    // Comparison with Elden ring
    if user_year > game_title.1 {
        println!("The year you inputed is newer than when Elden ring was released!");
    } else if user_year < game_title.1 {
        println!("The year you inputed is older than when Elden ring was released");
    } else {
        println!("The year you inputed is the same as when Elden ring was released");
    }

    // Comparison with E33
     if user_year > game_title.3 {
        println!("The year you inputed is newer than when E33 was released!");
    } else if user_year < game_title.3 {
        println!("The year you inputed is older than when E33 was released");
    } else {
        println!("The year you inputed is the same as when E33 was released");
    }

     if user_year > game_title.5 {
        println!("The year you inputed is newer than when Baldurs Gate was released!");
    } else if user_year < game_title.5 {
        println!("The year you inputed is older than when Baldurs Gate was released");
    } else {
        println!("The year you inputed is the same as when Baldurs Gate was released");
    }

    
}