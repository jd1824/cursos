//area rectangulo solucion 2
fn main() {
    let rect1 = (15, 24);
    println!("el area del rectangulo es: {}", area(rect1));
}


fn area(rectangulo: (u32, u32)) -> u32 {
    rectangulo.0 * rectangulo.1
}


