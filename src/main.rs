//area rectangulo solucion 3
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

fn main() {
    let rect1 = Rectangulo{
        ancho: 15,
        alto: 24,
    };
    println!("el area del rectangulo es: {}", area(&rect1));
}


fn area(rectangulo: &Rectangulo) -> u32 {
    rectangulo.ancho * rectangulo.alto
}


