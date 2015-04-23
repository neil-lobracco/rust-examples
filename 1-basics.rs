// We declare a function with the `fn` keyword
fn main(){

    // This is a macro that will print out what we give it.
    println!("Hello, world!");
    // This will work:
    println!("Life, the universe, and everything is: {}",42);
    // This won't compile because we didn't pass it enough parameters:
    // println!("This {} takes {} many {}","function","parameters");


    // Variables are bound with the `let` keyword.
    let my_int = 30;
    
    // By default, variables are immutable. The following will not compile:
    //my_int += 7;

    
    // However, we can define a mutable variable with `let mut`:
    let mut my_mutable_int = 30;
    my_mutable_int += 7;

    // In Rust, constructs like if, while, and match are expressions, not statements.
    let arg_description = match std::env::args().count() {
        0 => "That's not many args at all.",
        1 => "Just one argument?",
        2...4 => "Alright, not bad.",
        4...10 => "Come on, that's too many",
        _ => "Just stop it. Stop it.",
    };
    println!("{}",arg_description);

    // for loops are always "foreach" style
    for i in 0..10 {
        print!("{},",i);
    }
    println!("Ok, done");

    // But we could also write that in the functional style:
    let counting_string = (0..10)
        .map(|i| {
            format!("{}",i)
        })
        .fold("".to_string(), |acc, item| {
            acc + item.as_ref() + ","
        });
    println!("{} OK, Done!",counting_string);

    // Function calls use the familar syntax
    my_func(12);
    my_func(11);
    my_func(0);
}

// Types of parameters and return values must be specified and will not be inferred.
fn my_func(num: u32) {
    let response = match num {
        0 => "You passed in zero.",
        _ => {
            if num % 2 == 0 {
                "You passed in an even number"
            }
            else {
                "You passed in an odd number"
            }
        },
    };
    println!("{}",response);
}
