use crate::Mamifero;

pub struct Gato;

impl Mamifero for Gato {
    fn falar(&self) {
        println!("Miau");
    }
}
