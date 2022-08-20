fn main() {
    let cadena = String::from("hola mundo del rust");
    println!("la primera palabra de la cadena es: {}", primera_palabra(&cadena));

}

fn primera_palabra(cadena: &str) -> &str {
    let bytes = cadena.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &cadena[0..i];
        }
    }
    &cadena[..]
}





