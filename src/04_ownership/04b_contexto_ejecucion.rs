fn main() {
    let s1 = String::from("hola");
    let s2 = s1;
    //let s2 = s1.clone();

    println!("{s2}");
    // Rust considera a s1 como no válido.
    // El valor de s1 se mueve a s2, ya que no implementa "Copy"
    println!("{s1}");
}