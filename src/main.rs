//area rectangulo solucion 1
fn main() {
    let ancho = 15;
    let alto = 24;
    println!("el area del rectangulo es: {}", area(ancho, alto));
}


fn area(ancho: u32, alto: u32) -> u32 {
    ancho * alto
}


