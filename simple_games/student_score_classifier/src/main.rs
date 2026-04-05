use std::io;

fn main() {
    let student_scores: [u32; 5] = [87, 73, 62, 79, 81];

    println!("Thes are the students' scores: {:?}", student_scores);

    let mut user_score = String::new();

    println!("Please enter a score number:");

    io::stdin()
        .read_line(&mut user_score)
        .expect("This input was not recorded");

    let user_score: u32 = user_score.trim()
                                    .parse()
                                    .expect("This is not a number!");
    
    if user_score == student_scores[0] {
        println!("Found the number!");
    } else if user_score == student_scores[1] {
        println!("Found the number!");
    } else if user_score == student_scores[2] {
        println!("Found the number!");
    } else if user_score == student_scores[3] {
        println!("Found the number!");
    } else if user_score == student_scores[4] {
        println!("Found the number!");
    } else {
        println!("Number not found!");
    }
}