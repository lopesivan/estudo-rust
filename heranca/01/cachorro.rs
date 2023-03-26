use crate::Mamifero;

pub struct Cachorro;

impl Mamifero for Cachorro {
    fn falar(&self) {
        println!("Au Au");
    }
}
