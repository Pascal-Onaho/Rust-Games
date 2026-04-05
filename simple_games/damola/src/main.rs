fn main() {
    let width: i32 = 40;
    let height: i32 = 70;

    let soln = area(width, height);

    println!("{soln}");
}

fn area(width: i32, length: i32) -> i32 {

    println!("The area of the shape is:");
    let shape_area = width * length;
    return shape_area

}