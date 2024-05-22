fn main() {
    let nombre = "David";

    {
        let pais = "Bolivia";

        println!("País: {pais}");
        println!("Nombre: {nombre}");

    }   // pais ya no es válido

    println!("País: {pais}");
    println!("Nombre: {nombre}");
}