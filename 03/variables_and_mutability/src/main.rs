fn main() {
    //by default variables are immutable. u can mutable them by adding mut after let ...
    let mut x = 5;
    println!("x : {}", x);
    x = 6;
    println!("x : {}", x);
    // there are several differences between const and immutable
// const for constants vs let for immutable variables
// type must be annotated
//    constants can be declared in any scope and you can use it everywhere
//    last difference is that you can only assign constant values to const
//    in other words cost values can not be initialized in runtime.
//    all values must be known at compile time
//    naming convention : all uppercase with _ underscore
    const Y: u16 = 1;
    println!("y : {}", Y);
    let spaces = "          ";
    println!("spaces : {}", spaces);
    //shadowing spaces...
    let spaces = spaces.len();
    println!("spaces : {}", spaces)
//    but you can not shadow const
//    const spaces= 1;
}
