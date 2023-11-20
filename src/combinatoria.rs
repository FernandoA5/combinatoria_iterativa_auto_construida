pub fn combinatoria(n: usize, elementos: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut combinaciones: Vec<Vec<String>> = Vec::new();
    for i0 in elementos[0].iter() {
        for i1 in elementos[1].iter() {
            for i2 in elementos[2].iter() {
                let mut combinacion_actual: Vec<String> = Vec::new();
                combinacion_actual.push(i0.clone());
                combinacion_actual.push(i1.clone());
                combinacion_actual.push(i2.clone());
                combinaciones.push(combinacion_actual);
            }
        }
    }

    println!("Combinaciones: {:?}", combinaciones);
    combinaciones

}