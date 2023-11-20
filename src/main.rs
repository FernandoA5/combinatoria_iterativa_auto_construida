use std::io::Write;
mod combinatorias;

fn main() {
    let mut lista1: Vec<String> = Vec::new();
    let mut lista2: Vec<String> = Vec::new();
    let mut lista3: Vec<String> = Vec::new();

    //Agregamos los elementos a la lista1
    lista1.push("0".to_string());
    lista1.push("1".to_string());

    //Agregamos los elementos a la lista2
    lista2.push("2".to_string());
    lista2.push("3".to_string());
    lista2.push("1".to_string());

    //Agregamos los elementos a la lista3
    lista3.push("S".to_string());
    lista3.push("C".to_string());
    lista3.push("Q".to_string());

    let mut elementos: Vec<Vec<String>> = Vec::new();
    elementos.push(lista1.clone());
    elementos.push(lista2.clone());
    elementos.push(lista3.clone());

    println!("Lista 1: {:?}", lista1.clone());
    println!("Lista 2: {:?}", lista2.clone());
    println!("Lista 3: {:?}", lista3.clone());
    
    escribir_programa(3, elementos.clone());
    combinatorias::combinatoria(3, elementos.clone());
}

fn escribir_programa(n: i32, elementos: Vec<Vec<String>>){

    let mut archivo = std::fs::File::create("src/combinatorias.rs").unwrap();
    let mut contenido = String::new();

    //Escribimos la función de entrada de la librería
    contenido.push_str("pub fn combinatoria(n: usize, elementos: Vec<Vec<String>>) -> Vec<Vec<String>> {\n");
    contenido.push_str("\tlet mut combinaciones: Vec<Vec<String>> = Vec::new();\n");

    //Escribimos el cuerpo de la función iterativa (N FOR) //Quien se encarga de abrir los ciclos for
    for(i, lista) in elementos.iter().enumerate()
    {
        let tabulado = "\t".repeat(i + 1);
        //Abrimos los ciclos for
        contenido.push_str(&format!("{}for i{} in elementos[{}].iter() {{\n", tabulado, i, i));

        
    }

    //Escribimos el cuerpo de la función iterativa (N FOR) //Quien se encarga de agregar la combinación a la lista de combinaciones
    let tabulado = "\t".repeat(elementos.len() + 1);
    contenido.push_str(&format!("{}let mut combinacion_actual: Vec<String> = Vec::new();\n", tabulado));
    
    for i in 0..elementos.len()
    {
        // indices.push_str(&format!("[i{}]", i));
        contenido.push_str(&format!("{}combinacion_actual.push(i{}.clone());\n", tabulado, i));
    }
    

    contenido.push_str(&format!("{}combinaciones.push(combinacion_actual);\n", tabulado));


    //Escribimos el pie de la función iterativa (N FOR) //Quien se encarga de cerrar los ciclos for
    for(i, lista) in elementos.iter().enumerate()
    {
        let tabulado = "\t".repeat(elementos.len() - i);
        //Cerramos los ciclos for
        contenido.push_str(&format!("{}}}\n", tabulado));
    }
    //imprimir la lista de combinaciones
    contenido.push_str("\tprintln!(\"Combinaciones: {:?}\", combinaciones);\n");

    //Retornamos la lista de combinaciones
    contenido.push_str("\tcombinaciones\n");

    //Escribimos el final de la función recursiva
    contenido.push_str("}");

    //guardar el contenido en el archivo
    archivo.write_all(contenido.as_bytes()).unwrap();
}
