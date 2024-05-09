fn main() {
    let x = 2;
    let y: &i32; 
    // {
    //     let x_squared = x * x;
    //     let x_cube = x_squared * x;
    //     let y_value = x_cube + x_squared + x;
    //     let y = 0;
    //     y = &y_value;
    // };
    y = &0;
    println!("Y = {}", *y);
}
