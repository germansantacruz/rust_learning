fn main() {
    let x = 5;
    let y = x;
    
    /*
    Los tipos como los enteros que tienen un tamaño conocido en el momento de la compilación se almacenan completamente en la pila, por lo que copiar los valores reales es rápido. Eso significa que no hay razón para que queramos evitar que x sea válido después de crear la variable y. En otras palabras, no hay diferencia entre copiar superficial y profunda aquí.
     */

    println!("{x}");
    println!("{y}");
}