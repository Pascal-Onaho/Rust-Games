use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    age: u32,
    score: f32
}

fn main() {


//Student name input
let mut student_name = String::new();

println!("Please input your name:");

io::stdin().read_line(&mut student_name).expect("This input is invalid");

student_name = String::from(student_name.trim());


//Student age input
let mut student_age = String::new();

println!("Please input your age:");

io::stdin().read_line(&mut student_age).expect("This input is invalid");

let student_age: u32 = student_age.trim().parse().expect("Error");


//Student score input
let mut student_score = String::new();
println!("Please input your score:");

io::stdin().read_line(&mut student_score).expect("This input is invalid");

let student_score: f32 = student_score.trim().parse().expect("Error");

let student_1 = Student {
    name: student_name,
    age: student_age,
    score: student_score
};

println!("{:#?}", student_1);

if student_1.score >= 70.0 {
    println!("The grade is excellent");
} else if student_1.score >= 60.0 {
    println!("The grade is very good");
} else if student_1.score >= 50.0 {
    println!("The grade is okay");
} else {
    println!("Do better next time, {}", student_1.name);
}
}