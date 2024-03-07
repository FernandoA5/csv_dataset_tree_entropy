
#[derive(Debug)]
struct Vector{
    header: String,
    col: Vec<String>,

}
fn main() {
    //Leemos el CSV:
    let mut headers: Vec<String> = Vec::new();
    let mut vectores: Vec<Vector> = Vec::new();
    read_csv_vector(&mut vectores, &mut headers);
    
    //Obtenemos la combinatoria de las columnas:
    let combinaciones: usize= (1..=headers.len()).product();
    println!("Arboles: {}", combinaciones); //To-delete

    //Obtenemos los valores únicos de cada columna:
    let mut valores_unicos_por_columna: Vec<Vec<String>> = Vec::new();
    obtener_valores_unicos_por_columna(&vectores, &headers, &mut valores_unicos_por_columna);
    

    //Imprimimos los valores únicos de cada columna:
    for (i, valores_unicos) in valores_unicos_por_columna.iter().enumerate() {
        println!("Valores únicos de la columna {}: {:?}", headers[i], valores_unicos); //To-delete
    }
}


fn obtener_valores_unicos_por_columna(vectores: &Vec<Vector>, headers: &Vec<String>, valores_unicos_por_columna: &mut Vec<Vec<String>>){
    for (_i, header) in headers.iter().enumerate() {
        let mut valores_unicos: Vec<String> = Vec::new();
        for vector  in vectores.iter() {
            if vector.header == *header {
                for valor in vector.col.iter() {
                    if !valores_unicos.contains(&valor.to_string()){
                        valores_unicos.push(valor.to_string());
                    }
                }
            }
        }
        valores_unicos_por_columna.push(valores_unicos);
    }
}

fn read_csv_vector(vector: &mut Vec<Vector>, headers: &mut Vec<String>){
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path("src/titanic.csv")
        .unwrap();

    //Obtenemos los headers:
    let _headers = rdr.headers().unwrap();
    for header in _headers.iter(){
        headers.push(header.to_string());
    }
    println!("{:?}", headers);

    //Implementación correcta:
    for (i, result) in rdr.records().enumerate() {
        let record = result.unwrap();
        for (j, col) in record.iter().enumerate() {
            if i == 0 {
                vector.push(Vector{header: headers[j].to_string(), col: Vec::new() });
            }
            vector[j].col.push(col.to_string());
        }
    }

}
