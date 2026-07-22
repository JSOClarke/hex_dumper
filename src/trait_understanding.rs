trait Speak{
    fn speak(&self);
}

pub struct Dog;


pub struct Cat;

impl Speak for Dog {
    fn speak(&self){
        println!("Bark")
    }
}

fn make_it_speak<T:Speak>(animal:&T){
    animal.speak();
}