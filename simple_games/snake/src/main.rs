fn main() {

    let number: i32 = 25;

    if number % 10 == 0 {
        println!("The number is divisible by 10!");
    } else if number % 15 == 0 {
        println!("The number is divisible by 15!");
    } else if number % 5 == 0 {
        println!("This number is divisible by 5!");
    } else {
        println!("This number is not divisble by 10, 15, or 5. GGs!");
    }

    if number != 0 {
        println!("The number is something other than zero!");
    }


    let test: i32 = if number > 10 {
        number + 25
    } else {
        0
    };

    println!("The test is {}", test);

   let condition  = true;

   let numb = if condition {5} else {6}; 

   println!("The number is {}", numb);
   


}