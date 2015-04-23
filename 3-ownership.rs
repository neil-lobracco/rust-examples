#[derive(Debug)]
struct Foo {
    a: i32,
}

#[derive(Debug)]
struct Bar {
    q: i32,
    f: Foo,
}

fn main(){
    let foo = Foo {a: 25};
    let bar = Bar { q: 25, f: foo};
    println!("bar is: {:?}",bar);

    // This won't compile, because foo has been moved into bar
    //println!("foo is {:?}", foo);

    steals_ownership(bar);

    // Now bar is inaccesible, too:
    //println!("bar is: {:?}",bar);
}

fn steals_ownership(b: Bar){
    println!("Ha! I stole {:?}",b);
}
