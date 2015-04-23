use std::str::FromStr;
// Rust has no exceptions

fn main(){
    println!("Enter a number between one and ten!");
    let mut user_input = String::new();
    
    // This will warn by default, because we're disregarding any possible errors.
    //std::io::stdin().read_line(&mut user_input);
    let input_result = std::io::stdin().read_line(&mut user_input);
    match input_result {
        Ok(bytes_read) => println!("I read {} bytes from stdin.",bytes_read),
        Err(e) => panic!("I ran into an error reading from stdin: {:?}", e),
    };

    // Ok, we've read one line from the keyboard now.
    // First, let's make sure we don't write to user_input any more:
    let user_input = user_input;
    // Now let's turn it into a float:
    let float_input = match f32::from_str(user_input.trim()) {
        Ok(num) => num,
        Err(_) => panic!("Couldn't parse a float out of your input!"),
    };

    let doubled_input = float_input * 2.0;
    println!("Double {} is {}",float_input, doubled_input);
}
