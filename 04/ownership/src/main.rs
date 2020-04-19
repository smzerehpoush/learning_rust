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
        println!("{}", length)
    //    & -> ampersand
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
