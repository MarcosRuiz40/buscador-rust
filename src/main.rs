use std::fs; // LIBRERIA PARA LEER EL ARCHIVO
use std::io; // LIBRERIA PARA HACER LOS INPUTS 
fn main() {
    let mut cont = 0; // creamos el contador 
    let mut archivo_i =String::new(); // creamos la variable para guardar archivo
    let mut palabra_i  =String::new(); // creamos la variable para guardar la palabra

    //CREAMOS EL PRIMER INPUT PARA QUE GUARDE EL NOMBRE DEL ARCHIVO
    println!("Cual es el nombre del archivo: ");
    io::stdin()
        .read_line(&mut archivo_i)
        .expect("No se a encontrado el archivo");

    //CREAMOS EL SEGUNDO INPUT PARA QUE GUARDE LA PALABRA QUE QUIERE BUSCAR 
    println!("Cual es la palabra que desea buscar: ");
    io::stdin()
        .read_line(&mut palabra_i)
        .expect("No se a encontrado la palabra deseeada");

    let archivo_i = archivo_i.trim();
    let palabra_i = palabra_i.trim();
    
    //LEEMOS EL ARCHIVO QUE CREAMOS 
    let contenido: String = fs::read_to_string(archivo_i).expect("No se pudo leer el archivo");

    println!("El archivo es: {}",archivo_i); 
    println!("La palabra es: {}",palabra_i); 


    let palabra_lower = palabra_i.to_lowercase(); // PASAMOS LA PALABRA A MINÚSCULA

    //CREAMOS UN BUCLE FOR PARA QUE CUESTRE LAS LINEAS DEL TXT Y LA ENUMERACIÓN
    for (_numero, linea) in contenido.lines().enumerate() {
    if linea.to_lowercase().contains(&palabra_lower){
        println!("{}: {}", _numero + 1, linea);
        cont += 1; 
    } 
}
//MOSTRAMOS LAS COINCIDENCIAS AL BUSCAR LA PALABRA
println!("Se encontrarón {} coincidencias",cont);

if cont == 0{
    println!("No se encontrarón coincidencias")
}
}
