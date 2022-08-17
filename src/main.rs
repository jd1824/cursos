fn main() {
    let s = String::from("hola mundo");

    let longitud = calcula_longitud(&s);
    println!("la longitud de la cadena ({}) es: {}", s, longitud);
}

fn calcula_longitud(cadena: &String) -> usize{
    let aux: usize = cadena.len();
    aux
}

