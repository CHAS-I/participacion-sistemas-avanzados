fn main(){
    let x = 5;
    
    let mut y =10;
    y = 20;
    let x = 5;
    {
        let x = x + 1;
        println!("El valor de x es : {}", x);
    }
    println!("El valor de x es : {}", x);
    
    let entero: i32 = 42;
    let flotante: f64 = 3.1416;
    let booleano: bool = true;
    let caracter: char = 'a';
    let firulais: (i32, f64, char) = (42, 3.1416, 'a');
    let array: [i32; 3] = [1, 2, 3];
    
    println!("Tupla(firulais) forma1: {:?}", firulais);
    println!("Tupla(firulais) forma2: {} {} {}", firulais.0, firulais.1, firulais.2);


    println!("hello, world!")
}
