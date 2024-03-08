
#[derive(Debug)]
struct Vector{
    header: String,
    col: Vec<String>,

}

//DEFINIMOS EL PATH DEL CSV COMO UNA CONSTANTE:
const PATH: &str = "src/titanic.csv";

fn main() {
    //Leemos el CSV:
    let mut headers: Vec<String> = Vec::new();
    let mut vectores: Vec<Vector> = Vec::new();
    read_csv_vector(&mut vectores, &mut headers, PATH);
    
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


    print!("---------------------------------ARBOL DE DECISIÓN---------------------------------\n");
    //VERSIÓN ESTÁTICA:
    println!("########################----VERSIÓN ESTÁTICA----########################");
    let mut contador = 0;
    //Esto recorre de 0 a N columnas:
    for num_col in 0..headers.len(){
        println!("───Columna: {}", headers[num_col]);
        let mut columnas = headers.clone();
        //Eliminamos la columna actual que coincida con el valor de headers[num_col]:
        columnas = columnas.into_iter().filter(|x| *x != headers[num_col]).collect();
        //CARACTERES PARA DIBUJAR EL ÁRBOL:
        //┌───
        //│
        //└───
        //EL CARACTER PARA CONECTAR ┌ y └ ES  en uno solo:
        //├───



        for col_i in columnas.clone() {
            println!("   └─── Columna: {}", col_i); //To-delete
            let columnas_i = columnas.clone();
            //Eliminamos la columna actual
            let columnas_i: Vec<String> = columnas_i.into_iter().filter(|x| *x != col_i).collect();

            for col_j in columnas_i.clone() {
                

                println!("        └─ Columna: {}", col_j); //To-delete


                let columnas_j = columnas_i.clone();
                //Eliminamos la columna actual
                let columnas_j: Vec<String> = columnas_j.into_iter().filter(|x| *x != col_j).collect();

                for col_k in columnas_j.clone() {
                    contador += 1;  
 
                    println!("           └─ Columna: {} - {}", col_k, contador); //To-delete    
                }
            }
        }
    }


    //VERSIÓN DINÁMICA:
    println!("\n########################----VERSIÓN DINÁMICA----########################");
    let mut columnas: Vec<String> = headers.clone();


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

fn read_csv_vector(vector: &mut Vec<Vector>, headers: &mut Vec<String>, path: &str){
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
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
