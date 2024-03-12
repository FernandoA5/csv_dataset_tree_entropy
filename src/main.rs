
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
    // for num_col in 0..headers.len()
    for num_col in 0..1{
        println!("───{}", headers[num_col]);

        //CALCULOS DE COLUMNA ROOT:
        let valores_unicos = valores_unicos_por_columna[num_col].clone();
        println!("Valores únicos de la columna {}: {:?}", headers[num_col], valores_unicos); //To-delete
        //Ahora buscamos todas las apariciones de cada valor único en la columna:
        //PARA ESO RECORREMOS LOS VALORES ÚNICOS Y POR CADA UNO IMPRIMIMOS LO SIGUIENTE:
        //"Apariciones de {nombre_columna} {valor}: {apariciones}"
        //BAJO NINGÚN CONCEPTO LOS IMPRIMAS COMO TUPLAS:
        let mut vec_apariciones: Vec<(String, usize)> = Vec::new();
        for valor in valores_unicos.iter(){
            let apariciones = vectores.iter().filter(|x| x.header == headers[num_col])
            .flat_map(|x| x.col.iter()).filter(|x| *x == valor).count();
            vec_apariciones.push((valor.to_string(), apariciones));
            println!("Apariciones de {} {}: {}", headers[num_col], valor, apariciones);
        }
        


        

        //Calculamos la probabilidad de cada valor único, lo imprimimos de la misma forma que las apariciones:
        let total_apariciones: usize = vec_apariciones.iter().map(|x| x.1).sum();
        let mut probabilidad: Vec<f64> = Vec::new();
        for aparicion in vec_apariciones.iter(){
            probabilidad.push(aparicion.1 as f64 / total_apariciones as f64);
            println!("Probabilidad de {} {}: {}", headers[num_col], aparicion.0, aparicion.1 as f64 / total_apariciones as f64);
        }
        

        //Calculamos la entropía de cada valor único, lo imprimimos de la misma forma que las apariciones:
        let mut entropia: Vec<f64> = Vec::new();
        for (i, prob) in probabilidad.iter().enumerate(){
            let entropia_i:f64;
            //VALIDAMOS QUE LA ENTROPIA NO SEA 0
            if *prob == 0.0 {
                entropia_i = 0.0;
                entropia.push(entropia_i);
            }
            else{
                entropia_i = -prob * prob.log10();
                entropia.push(entropia_i);
            }
            println!("Entropía de {} {}: {}", headers[num_col], vec_apariciones[i].0, entropia_i);
        }


        //Entropía de la columna:
        let entropia_columna: f64 = entropia.iter().sum();
        println!("Entropía de la columna {}: {}", headers[num_col], entropia_columna); //To-delete




        let mut columnas = headers.clone();
        //Eliminamos la columna actual que coincida con el valor de headers[num_col]:
        columnas = columnas.into_iter().filter(|x| *x != headers[num_col]).collect();

        for col_i in columnas.clone() {
            //"   └─── {padre}-{hija}", col_i
            println!("   └─── {}-{}", headers[num_col], col_i); //To-delete

            //CALCULOS DE COLUMNA I:
            let valores_unicos_i = valores_unicos_por_columna[headers.iter().position(|x| *x == col_i).unwrap()].clone();
            println!("   Valores únicos de la columna {}: {:?}", col_i, valores_unicos_i); //To-delete
            //COMBINACIONES DE LOS VALORES ÚNICOS DE LA COLUMNA I y la columna ROOT Vec::<String, String> = (root[0], col_i[0]), (root[0], col_i[1]), (root[1], col_i[0]), (root[1], col_i[1]):
            let mut combinaciones_valores_unicos: Vec<(String, String)> = Vec::new();
            let mut apariciones_combinaciones: Vec<usize> = Vec::new();
            for valor_root in valores_unicos.iter(){
                for valor_i in valores_unicos_i.iter(){
                    combinaciones_valores_unicos.push((valor_root.to_string(), valor_i.to_string()));
                    //BUSCAMOS LAS APARICIONES DE CADA COMBINACIÓN ROOT && I: Usize (Sin reiterar, con lo que ya tenemos)
                    let apariciones = vectores.iter().filter(|x| x.header == headers[num_col])
                    .flat_map(|x| x.col.iter()).zip(vectores.iter().filter(|x| x.header == col_i)
                    .flat_map(|x| x.col.iter())).filter(|(x, y)| *x == valor_root && *y == valor_i)
                    .count();
                    apariciones_combinaciones.push(apariciones);                    
                    //"   Apariciones de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i}: {apariciones}"
                    println!("   Apariciones de la combinación {} {} y {} {}: {}", headers[num_col], valor_root, col_i, valor_i, apariciones);
                }
            }
            
            //CALCULAMOS LA PROBABILIDAD DE CADA COMBINACIÓN ROOT && I.
            //APARIENCIA DE CADA COMBINACIÓN ROOT && I / APARICIONES DE CADA VALOR DE ROOT :
            let mut probabilidad_combinaciones: Vec<f64> = Vec::new();
            for (i, valor_root) in valores_unicos.iter().enumerate(){
                for (_j, valor_i) in valores_unicos_i.iter().enumerate(){
                    let apariciones = vectores.iter().filter(|x| x.header == headers[num_col])
                    .flat_map(|x| x.col.iter()).zip(vectores.iter().filter(|x| x.header == col_i)
                    .flat_map(|x| x.col.iter())).filter(|(x, y)| *x == valor_root && *y == valor_i)
                    .count();
                    probabilidad_combinaciones.push(apariciones as f64 / vec_apariciones[i].1 as f64);
                    //"   Probabilidad de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i}: {probabilidad}"
                    println!("   Probabilidad de la combinación {} {} y {} {}: {}", headers[num_col], valor_root, col_i, valor_i, apariciones as f64 / vec_apariciones[i].1 as f64);
                }
            }

            //ENTROPIA DE CADA COMBINACIÓN ROOT && I:
            let mut entropia_combinaciones: Vec<f64> = Vec::new();
            for (i, prob) in probabilidad_combinaciones.iter().enumerate(){
                //VALIDAMOS QUE LA ENTROPIA NO SEA 0
                let entropia: f64;
                if *prob == 0.0 {
                    entropia = 0.0;
                    entropia_combinaciones.push(entropia);
                }
                else{
                    entropia = -prob * prob.log10();
                    entropia_combinaciones.push(entropia);
                }

                //"   Entropía de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i}: {entropia}"
                println!("   Entropía de la combinación {} {} y {} {}: {}", headers[num_col], combinaciones_valores_unicos[i].0, col_i, combinaciones_valores_unicos[i].1, entropia);
            }
            //ENTROPIA TOTAL DE LA COMBINACIÓN ROOT && I:
            let entropia_total_combinaciones: f64 = entropia_combinaciones.iter().sum();
            //"   Entropía total de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i}: {entropia_total}"
            println!("   Entropía total de la combinación {} y {}: {}", headers[num_col], col_i, entropia_total_combinaciones);
            




            let columnas_i = columnas.clone();
            //Eliminamos la columna actual
            let columnas_i: Vec<String> = columnas_i.into_iter().filter(|x| *x != col_i).collect();

            for col_j in columnas_i.clone() {
                //"   └─── {padre}-{hija}-{nieta}", col_j
                println!("      └─── {}-{}-{}", headers[num_col], col_i, col_j); //To-delete
                

                //CALCULOS DE COLUMNA J:
                let valores_unicos_j = valores_unicos_por_columna[headers.iter().position(|x| *x == col_j).unwrap()].clone();
                println!("        Valores únicos de la columna {}: {:?}", col_j, valores_unicos_j); //To-delete                
                //COMBINACIONES DE LOS VALORES ÚNICOS DE LA COLUMNA ROOT, I y J Vec::<String, String, String> = (root[0], col_i[0], col_j[0]), (root[0], col_i[0], col_j[1]), (root[0], col_i[1], col_j[0]), (root[0], col_i[1], col_j[1]):
                let mut combinaciones_valores_unicos: Vec<(String, String, String)> = Vec::new();   
                for valor_root in valores_unicos.iter(){
                    for valor_i in valores_unicos_i.iter(){
                        for valor_j in valores_unicos_j.iter(){
                            combinaciones_valores_unicos.push((valor_root.to_string(), valor_i.to_string(), valor_j.to_string()));
                            //BUSCAMOS LAS APARICIONES DE CADA COMBINACIÓN ROOT && I && J: Usize (Sin reiterar, con lo que ya tenemos)
                            let apariciones = vectores.iter().filter(|x| x.header == headers[num_col])
                            .flat_map(|x| x.col.iter()).zip(vectores.iter().filter(|x| x.header == col_i)
                            .flat_map(|x| x.col.iter())).zip(vectores.iter().filter(|x| x.header == col_j)
                            .flat_map(|x| x.col.iter())).filter(|((x, y), z)| *x == valor_root && *y == valor_i && *z == valor_j)
                            .count();
                            //"      Apariciones de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i} y {nombre_columna} {valor_j}: {apariciones}"
                            println!("        Apariciones de la combinación {} {} y {} {} y {} {}: {}", headers[num_col], valor_root, col_i, valor_i, col_j, valor_j, apariciones);
                        }
                    }
                }              
                
                //CALCULAMOS LA PROBABILIDAD DE CADA COMBINACIÓN ROOT && I && J.
                //APARIENCIA DE CADA COMBINACIÓN ROOT && I && J / APARICIONES DE CADA VALOR DE ROOT && I:
                let mut indice_combinaciones: usize = 0;
                let mut probabilidad_combinaciones: Vec<f64> = Vec::new();
                for (_i, valor_root) in valores_unicos.iter().enumerate(){
                    for (_j, valor_i) in valores_unicos_i.iter().enumerate(){
                        for (_k, valor_j) in valores_unicos_j.iter().enumerate(){
                            let apariciones = vectores.iter().filter(|x| x.header == headers[num_col])
                            .flat_map(|x| x.col.iter()).zip(vectores.iter().filter(|x| x.header == col_i)
                            .flat_map(|x| x.col.iter())).zip(vectores.iter().filter(|x| x.header == col_j)
                            .flat_map(|x| x.col.iter())).filter(|((x, y), z)| *x == valor_root && *y == valor_i && *z == valor_j)
                            .count();

                            let div = apariciones_combinaciones[indice_combinaciones] as f64;                            
                            let probabilidad = apariciones as f64 / div;

                            probabilidad_combinaciones.push(probabilidad);                            

                            //"      Probabilidad de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i} y {nombre_columna} {valor_j}: {probabilidad}"
                            println!("        Probabilidad de la combinación {} {} y {} {} y {} {}: {}", headers[num_col], valor_root, col_i, valor_i, col_j, valor_j, probabilidad);
                            // println!("          └─ {} / {}[{}]", apariciones, div, indice_combinaciones);
                            // println!("          └─A: {:?}", apariciones_combinaciones); //To-delete
                        }
                        //EL índice de combinaciones se incrementa en 1 por cada ciclo completo de k
                        indice_combinaciones += 1;
                    }
                }

                //ENTROPIA DE CADA COMBINACIÓN ROOT && I && J:
                let mut entropia_combinaciones: Vec<f64> = Vec::new();
                for (i, prob) in probabilidad_combinaciones.iter().enumerate(){
                    //VALIDAMOS QUE LA ENTROPIA NO SEA 0
                    let entropia: f64;
                    if *prob == 0.0 {
                        entropia = 0.0;
                        entropia_combinaciones.push(entropia);
                    }
                    else{
                        entropia = -prob * prob.log10();
                        entropia_combinaciones.push(entropia);
                    }

                    //"      Entropía de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i} y {nombre_columna} {valor_j}: {entropia}"
                    println!("        Entropía de la combinación {} {} y {} {} y {} {}: {}", headers[num_col], combinaciones_valores_unicos[i].0, col_i, combinaciones_valores_unicos[i].1, col_j, combinaciones_valores_unicos[i].2, entropia);
                }
                




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
    // println!("\n########################----VERSIÓN DINÁMICA----########################");
    // let columnas: Vec<String> = headers.clone();
    // let mut contador_arboles = 0;
    // let n_headers = headers.len();
    // // arbol(headers.len(), headers, vectores, combinaciones, 0);
    // arbol_recursivo(n_headers, headers, &mut contador_arboles, 0, n_headers);
    


}

// fn arbol_recursivo(n_headers: usize, headers: Vec<String>, contador_arboles: &mut usize, depth: usize, n_headers_original: usize) {
//     if n_headers != 0 {
//         for i_header in 0..headers.len() {
//             if n_headers == n_headers_original{
//                 println!("───{}Columna: {}" , "    ".repeat(depth), headers[i_header]);
//             }
//             else if n_headers == 1 {
//                 *contador_arboles += 1;
//                 println!("{}└─Columna: {}.({})", "     ".repeat(depth), headers[i_header], *contador_arboles);
//             }
            
//             else{
//                 println!("{}└─Columna: {}", "     ".repeat(depth), headers[i_header]);
//             }


//             let mut new_headers = headers.clone();
//             //Eliminamos la columna actual que coincida con el valor de headers[i_header]:
//             new_headers = new_headers.into_iter().filter(|x| *x != headers[i_header]).collect();
            
//             arbol_recursivo(n_headers - 1, new_headers, contador_arboles, depth + 1, n_headers_original);
//         }
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