fn main() {
    println!("Hello, world!");
    let x: u32 = "43".parse().expect("bad input");
    println!("{}", x);
//    rust is a static type language so it must know all types at compile time
//    rust has four scalar types : integers, floating-point numbers, booleans, characters
//    signed numbers stored in two's complement format
//    you can use underscore _ for visual separator like 1_000_000
//    integers have both signed and unsigned types like u32 or i64
    let x = 1_000_000;
    println!("{}", x);
//    default type for integers : i32


//  floating point numbers
    let x = 2.0;
    println!("{}", x);
    let x: f32 = 3.1;
    println!("{}", x);
//    default type for floating point : f64

//    booleans
    let x = true;
    println!("{}", x);
    let x: bool = false;
    println!("{}", x);


//    characters
    let x = 'z';
    println!("{}", x);
    let x = 'ℤ';
    println!("{}", x);
    let x = '😻';
    println!("{}", x);
//Rust’s char type is four bytes in size and represents a Unicode Scalar Value,
// which means it can represent a lot more than just ASCII.


// compound types
//    rust has two compound types : array and tuple
//
//    tuples
//     A tuple is a general way of grouping together a number of values
// with a variety of types into one compound type. Tuples have a fixed length: once declared,
// they cannot grow or shrink in size

    let tuple = (1, 2, 3);
    println!("{}", tuple.0);
    // println!("{}", tuple);
    let tuple: (i32, i32, char) = (1, 2, 'a');
    // println!("{}", tuple);

    let (x, y, z) = tuple;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
    println!("{}", tuple.0);
    println!("{}", tuple.1);
    println!("{}", tuple.2);


//    arrays
    let array = [1, 2, 3, 4, 5];
    println!("{:?}", array);
    let array: [i8; 3] = [127, 2, 3];
    let first = array[0];
    println!("{}", first);

//    an array with size and initial value  => let arrayy = [init_value; size]
    let array = [0; 5];
    let first = array[0];
    println!("{}", first);
    let second = array[1];
    println!("{}", second);
//    you can not access items with indexes that do not exists.
//    for example this code throws a runtime exception
//     let a = [1, 2, 3];
//     let index = 10;
//     let element = array[index];
//     println!("{}", element)
}
