fn main() {
    //rust ownership is for keeping memory safe without GC
    //rust manages memory as a approach with a set of rules that compiler check at compile time
    //note that ownership does not slow down app during runtime

    //Ownership Rules
    //
    //
    // Each value in Rust has a variable that’s called its owner.
    // There can only be one owner at a time.
    // When the owner goes out of scope, the value will be dropped.
    println!("Hello, world!");
    {
        let string = "salam";
        println!("{}", string);
        let mut string = String::from("salam");
        string.push_str(" ali:)");
        println!("{}", string);

        let str = String::from("Ali");
        let str2 = str;
        // println!("{}", str); this leads to compile time problem. because str is moved to str2
        println!("{}", str2);

        let s1 = String::from("testing clone.");
        let s2 = s1.clone();
        println!("{} - {}", s1, s2);
        // The ownership of a variable follows the same pattern every time:
        // assigning a value to another variable moves it. When a variable that includes data
        // on the heap goes out of scope, the value will be cleaned up by drop unless the data
        // has been moved to be owned by another variable.

        let str = String::from("this is test");
        take_ownership(str);
        // println!("{}", str); this leads to compile time problem because function takes ownership
        // of argument

        //    now what if we want take ownership back again
        //    here you are
        let str: String = String::from("this is test");
        let str = take_ownership_and_give_it_back(str);
        println!("{}", str);
        //    but it is very bad because two time variable swapped
        //    what if just send pointer of variable
        //    this is the solution
        let str = String::from("pointer base argument passing");
        //    assume we want to find length of string
        let length = compute_length(&str);
        println!("{}", length);
        //    & -> ampersand
        //    you can not change variable passed with reference ..
        //    you need to do this
        let mut str = String::from("aliz");
        change_mutable_reference(&mut str);
        println!("{}", str);
        //But mutable references have one big restriction: you can have only one mutable reference
        // to a particular piece of data in a particular scope
        //this code does not compile for upper reason

        // let mut string = String::from("ali");
        // let p1 = &mut string;
        // let p2 = &mut string;
        // println!("{} - {}", p1, p2); this crashes because of multiple mutable reference in scope

        //    This restriction allows for mutation but in a very controlled fashion.
        // It’s something that new Rustaceans struggle with, because most languages let you
        // mutate whenever you’d like.
        //
        // The benefit of having this restriction is that Rust can prevent data races at
        // compile time. A data race is similar to a race condition and happens when
        // these three behaviors occur:
        //
        // Two or more pointers access the same data at the same time.
        // At least one of the pointers is being used to write to the data.
        // There’s no mechanism being used to synchronize access to the data.
        // Data races cause undefined behavior and can be difficult to diagnose
        // and fix when you’re trying to track them down at runtime;
        // Rust prevents this problem from happening because it won’t even compile code
        // with data races!

        //    and also you can not have both mutable and immutable references
        //    Whew! We also cannot have a mutable reference while we have an immutable one.
        // Users of an immutable reference don’t expect the values to suddenly change out
        // from under them! However, multiple immutable references are okay because no one
        // who is just reading the data has the ability to affect anyone else’s reading of the data.

        // let reference_to_nothing = dangle();
        let mut s = no_dangle();
        s.push_str("alizzzz");
        println!("{}", s);
        let mut string = String::from("This Is Test");
        println!("{}", first_word(&string));
        // you can use string.clear() to empty string and equal to ""
        string.clear();
        //    because of this you can not find world anymore...

        let string = String::from("testing slices :))");
        let testing = &string[0..7];
        let slices = &string[8..14];
        println!("{}", testing);
        println!("{}", slices);
        // you can drop start-end values like this bellow
        let s1 = &string[..3]; // start from 0 to 3
        let s2 = &string[3..]; // start from 3 to end of string
        println!("{}", s1);
        println!("{}", s2);
        let string = String::from("This Is Test");
        println!("{}", first_word_with_slice(&string));
    //    we have array slices to . here u are
        let arr = [1,2,3,];
        let arr_slice = &arr[1..3];
    }
    fn first_word_with_slice(s: &str) -> &str {
        for (i, &item) in s.as_bytes().iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        s
    }
    fn first_word(s: &String) -> usize {
        for (i, &item) in s.as_bytes().iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        s.len()
    }
    fn no_dangle() -> String {
        let s = String::from("alix");
        s
    }
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //
    //     &s
    // }
    fn change_mutable_reference(str: &mut String) {
        str.push_str("Alizzz");
    }
    //this takes argument ownership and can not use it later ...
    fn take_ownership(str: String) {
        println!("{}", str);
    }

    //this takes argument ownership and give it back
    fn take_ownership_and_give_it_back(str: String) -> String {
        println!("{}", str);
        str
    }

    fn compute_length(str: &String) -> usize {
        str.len()
    }
}
