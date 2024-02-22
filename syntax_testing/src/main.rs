fn main() {
    for i in 0..20 {
        println!("Double: {}, Triple: {}", double(i), triple(i));
    }

    let s: &str = "hi";
    let s: String = String::from("hi");
    
    // Only takes by ref, so ownership is still kept by local variable
    no_ownership(&s);
    println!("{}", s);

    // Takes ownership but returns it back via return value
    let s1 = take_and_return_ownership(s);
    println!("{}", s1);

    // Takes ownership but doesn't give it back (compiler will yell if you try to print s1 after this)
    take_ownership(s1);

    let world = String::from("Hello World");
    let first = slice_ownership(&world);
    println!("The first world is: {}", first);
}

fn double(cur:u32) -> u32 {
    cur * cur
}

fn triple(cur: u32) ->u32 {
    cur * cur * cur
}

fn take_ownership(s:String) {
    println!("{}", s);
}

fn take_and_return_ownership(s:String) -> String {
    println!("{}", s);
    s
}

fn no_ownership(s: &String) {
    println!("{}", s);
}

fn slice_ownership(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
