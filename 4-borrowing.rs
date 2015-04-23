#[derive(Debug)]
struct Foo {
    a: i32
}

fn main(){
    let f = Foo{a: 21};

    // Borrow f into b
    let b = &f;
    println!("b is {:?}", b);

    // We can borrow multiple times:
    let b2 = &f;
    
    // All are still accesible:
    println!("f, b, and b2 are {:?}, {:?}, and {:?}",f,b,b2);

    // This is still fine, just another borrow
    just_borrows(&f);

    // This is not allowed, because we still have borrows open.
    //takes_ownership(f);
}

fn takes_ownership(f: Foo){
    println!("Took ownership of {:?}",f);
}

fn just_borrows(f: &Foo){
    println!("I just borrowed {:?}",f);
}
