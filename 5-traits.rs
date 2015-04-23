trait Speak {
    fn speak(&self);
}

struct Cat {
    lives_left: u8,
}

impl Speak for Cat {
    fn speak(&self){
        println!("I have {} lives left.",self.lives_left);
    }
}

#[derive(Debug)]
enum Dogidicity {
    SlightlyDoggy,
    QuiteDoggy,
    OutlandishlyDoggy,
}

struct Dog {
    dogidicity: Dogidicity,
}

impl Speak for Dog {
    fn speak(&self){
        println!("I am feeling {:?}",self.dogidicity);
    }
}

fn main(){
    let c = Cat { lives_left: 9 };
    let d = Dog { dogidicity: Dogidicity::OutlandishlyDoggy };

    ask_to_speak(&c);
    ask_to_speak(&d);
}

//fn ask_to_speak<T: Speak>(speaker: &T){
fn ask_to_speak(speaker: &Speak){
    speaker.speak();
}
