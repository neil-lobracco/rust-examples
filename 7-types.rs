use std::marker::PhantomData;

struct Inch;
struct Meter;
struct Furlong;
struct Length<Unit> {
    v: f64,
    _m: PhantomData<Unit>,
}
impl<Unit> Length<Unit> {
    fn get(self) -> f64 {
        self.v
    }
    fn new(v: f64) -> Length<Unit> {
        Length { v: v, _m: PhantomData }
    }
}

fn main(){
    // Works fine - we're passing in the right type
    expects_inches(Length::<Inch>::new(30.1));

    // Won't compile - passing in wrong type:
    //expects_inches(Length::<Furlong>::new(0.00392));
}

fn expects_inches(inches: Length<Inch>){
    println!("You passed in {} inches",inches.get());
}
