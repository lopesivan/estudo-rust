mod mamifero;
mod gato;
mod cachorro;

use crate::mamifero::Mamifero;
use crate::gato::Gato;
use crate::cachorro::Cachorro;

fn main() {
    let gato = Gato;
    let cachorro = Cachorro;

    gato.falar();     // Saída: Miau
    cachorro.falar(); // Saída: Au Au
}
