
#[derive(Debug, Clone)]
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
    let columnas: Vec<String> = headers.clone();
    // arbol(columnas, 
    //     vectores.clone(), 
    //     0, 
    //     combinaciones, 
    //     0, 
    //     vectores.clone(), 
    //     headers.clone(),
    //     0,
    //     0
    // );
    arbol(headers.len(), headers, vectores, 0);


}

fn arbol(n_headers: usize, headers: Vec<String>, vectores: Vec<Vector>, n_arboles: usize,){
    if n_headers == 1 {
        println!("Arbol: {}", n_arboles);
        // for i in 0..vectores.len() {
        //     println!("*{} {} - Columna: {}", "  ".repeat(i), n_headers, headers[i]);
        //     // for j in 0..vectores[i].col.len() {
        //     //     println!("Valor: {}", vectores[i].col[j]);
        //     // }
        // }
    }
    else {
        //Eliminamos la columna actual que coincida con el valor de headers[num_col]:
        // let mut vectores_i = vectores.clone();
        // vectores_i = vectores_i.into_iter().filter(|x| x.header != headers[n_headers-1]).collect();
        // let mut headers_i = headers.clone();    
        // headers_i = headers_i.into_iter().filter(|x| *x != headers[n_headers-1]).collect();
        // arbol(n_headers-1, headers_i.clone(), vectores_i.clone());
        for i in 0..vectores.len(){
            println!("-{} {} - Columna: {}", "  ".repeat(i), n_headers, headers[i]);
            arbol(n_headers-1, headers.clone(), vectores.clone(), n_arboles);
        }
    }
}


// fn arbol(columnas: Vec<String>, 
//     datos: Vec<Vector>,
//     contador: usize, 
//     combinaciones: usize, 
//     indice: usize, 
//     original_data: Vec<Vector>,
//     original_headers: Vec<String>,
//     profunidad: usize,
//     n_header: usize
// ){

//     if columnas.len() == 1 {
//         let tabulador = "   ".repeat(profunidad);
//         println!("*{} {}[{}] - Columna {}: {}", tabulador, indice, contador, columnas[0], datos[0].col[0]);
//         if contador == combinaciones-1 {
//             println!("Fin"); //To-delete
//             return;
//         }
//         else {
//             if indice == original_headers.len()-1 {
//                 //REINICIAMOS EL ÍNDICE: (NUEVO SET DE ARBOLES)
//                 println!("Reinicio"); //To-delete

//                 arbol(original_headers.clone(), 
//                     original_data.clone(), 
//                     contador+1, 
//                     combinaciones, 
//                     0, 
//                     original_data.clone(),
//                     original_headers.clone(),
//                     0,
//                     n_header+1
//                 );
                
//             }
//             else {
//                 //REINICIAMOS LA PROFUNDIDAD: (NUEVO ÁRBOL)
//                 arbol(original_headers.clone(), 
//                     original_data.clone(), 
//                     contador+1, 
//                     combinaciones, 
//                     indice+1, 
//                     original_data.clone(),
//                     original_headers.clone(),
//                     0,
//                     n_header
//                 );
//             }
//         }
//     }
//     else {
//         let mut columnas_i: Vec<String> = columnas.clone();
//         let mut datos_i: Vec<Vector> = datos.clone();
        
//         if columnas.len() == original_headers.len(){
//             let tabulador = "   ".repeat(profunidad);
//             println!("-{} {}[{}] - Columna {}: {}", tabulador, contador , indice, columnas[n_header], datos[n_header].col[0] ); //To-delete
//             columnas_i = columnas_i.into_iter().filter(|x| *x != columnas[n_header]).collect::<Vec<String>>();
//             datos_i = datos_i.into_iter().filter(|x| x.header != columnas[n_header]).collect::<Vec<Vector>>();

//         }
//         else{
//             let tabulador = "   ".repeat(profunidad);
//             println!("+{} {}[{}] - Columna {}: {}", tabulador, contador , indice, columnas[0], datos[0].col[0] ); //To-delete
    
//             columnas_i = columnas_i.into_iter().filter(|x| *x != columnas[0]).collect::<Vec<String>>();
//             datos_i = datos_i.into_iter().filter(|x| x.header != columnas[0]).collect::<Vec<Vector>>();
//         }
        
//         arbol(columnas_i, 
//             datos_i, 
//             contador, 
//             combinaciones, 
//             indice, 
//             original_data.clone(),
//             original_headers.clone(),
//             profunidad+1,
//             0
//         );

//     }
// }




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
