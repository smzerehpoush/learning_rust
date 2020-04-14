fn main() {
    println!("Hello, world!");
    let x = 1;
    let y = "Ali";
    another_fucntion(x, y);
    println!("{}", x);
}

fn another_fucntion(x: i32, y: &str) {
//    in rust standard naming convention is snake_case like this word
    println!("another function x {}, y {}", x, y);
//    statements vs expressions  :
//    Statements are instructions that perform some action and do not return a value.
//    Expressions evaluate to a resulting value.
//    Expressions can be part of statements
//    Expressions do not include ending semicolons.
//    If you add a semicolon to the end of an expression, you turn it into a statement,
//    which will then not return a value.
    let y = {
        let x = 1;
        x + 1
    };
    //    functions with return values
    fn six() -> i32 {
        6
    }
//    we declare function return types after function signature with ->
}
