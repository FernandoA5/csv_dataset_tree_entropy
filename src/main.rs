#[derive(Debug, Clone)]
struct Vector{
    header: String,
    col: Vec<String>,

}

//DEFINIMOS EL PATH DEL CSV COMO UNA CONSTANTE:
const PATH: &str = "src/crash_data_tadeo.csv";
const VERBOSE: bool = false;

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

    //ESTO ES POCO ORTODOXO, ATENTO:
    let mut vec_arboles_entropias: Vec<(Vec<String>, f64)> = Vec::new();

    //Esto recorre de 0 a N columnas:
    for num_col in 0..headers.len(){
    // for num_col in 0..1{
        let mut vec_ramas_arboles: Vec<String> = Vec::new();
        println!("───{}", headers[num_col]);
        vec_ramas_arboles.push(format!("───{}\n", headers[num_col]));

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

            
            


            if VERBOSE{
                vec_ramas_arboles.push(format!("Apariciones de {} {}: {}\n", headers[num_col], valor, apariciones));
                println!("Apariciones de {} {}: {}", headers[num_col], valor, apariciones);  
            }
        }
        


        

        //Calculamos la probabilidad de cada valor único, lo imprimimos de la misma forma que las apariciones:
        let total_apariciones: usize = vec_apariciones.iter().map(|x| x.1).sum();
        let mut probabilidad: Vec<f64> = Vec::new();
        for aparicion in vec_apariciones.iter(){

            let probabilidad_i: f64;
            if total_apariciones == 0 {
                probabilidad_i = 0.0;
                
            }
            else{
                probabilidad_i = aparicion.1 as f64 / total_apariciones as f64;
            }
            probabilidad.push(probabilidad_i);

            

            if VERBOSE{
                vec_ramas_arboles.push(format!("Probabilidad de {} {}: {}\n", headers[num_col], aparicion.0, probabilidad_i));
                println!("Probabilidad de {} {}: {}", headers[num_col], aparicion.0, aparicion.1 as f64 / total_apariciones as f64);
            }
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

            

            if VERBOSE{
                vec_ramas_arboles.push(format!("Entropía de {} {}: {}\n", headers[num_col], vec_apariciones[i].0, entropia_i));
                println!("Entropía de {} {}: {}", headers[num_col], vec_apariciones[i].0, entropia_i);
            }
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
            vec_ramas_arboles.push(format!("   └─── {}-{}\n", headers[num_col], col_i));

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

                    

                    if VERBOSE{
                        vec_ramas_arboles.push(format!("   Apariciones de la combinación {} {} y {} {}: {}\n", headers[num_col], valor_root, col_i, valor_i, apariciones));
                        println!("   Apariciones de la combinación {} {} y {} {}: {}", headers[num_col], valor_root, col_i, valor_i, apariciones);
                    }
                }
            }
            
            //CALCULAMOS LA PROBABILIDAD DE CADA COMBINACIÓN ROOT && I.
            //APARIENCIA DE CADA COMBINACIÓN ROOT && I / APARICIONES DE CADA VALOR DE ROOT :
            let mut probabilidad_combinaciones: Vec<f64> = Vec::new();
            // let mut indice_combinaciones: usize = 0; //EN ESTE CASO ESPECÍFICO USAMOS i PARA EL ÍNDICE DE COMBINACIONES
            for (i, valor_root) in valores_unicos.iter().enumerate(){
                for (_j, valor_i) in valores_unicos_i.iter().enumerate(){
                    let apariciones = vectores.iter().filter(|x| x.header == headers[num_col])
                    .flat_map(|x| x.col.iter()).zip(vectores.iter().filter(|x| x.header == col_i)
                    .flat_map(|x| x.col.iter())).filter(|(x, y)| *x == valor_root && *y == valor_i)
                    .count();

                    let div = vec_apariciones[i].1 as f64;
                    let probabilidad: f64;
                    //VALIDAMOS QUE EL DIVISOR NO SEA 0
                    if div == 0.0 {
                        probabilidad = 0.0;
                    }
                    else{
                        probabilidad = apariciones as f64 / div;
                    }

                    probabilidad_combinaciones.push(probabilidad);
                    //"   Probabilidad de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i}: {probabilidad}"

                    

                    if VERBOSE{
                        vec_ramas_arboles.push(format!("   Probabilidad de la combinación {} {} y {} {}: {} \n", headers[num_col], valor_root, col_i, valor_i, probabilidad));
                        println!("   Probabilidad de la combinación {} {} y {} {}: {}", headers[num_col], valor_root, col_i, valor_i, probabilidad);
                    }
                    // println!("          └─ {} / {}[{}] = {}", apariciones, div, indice_combinaciones, probabilidad);
                    // println!("          └─A: {:?}", vec_apariciones); //To-delete
                }
                //EL índice de combinaciones se incrementa en 1 por cada ciclo completo de j
                // indice_combinaciones += 1;
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

                

                if VERBOSE{
                    vec_ramas_arboles.push(format!("   Entropía de la combinación {} {} y {} {}: {}\n", headers[num_col], combinaciones_valores_unicos[i].0, col_i, combinaciones_valores_unicos[i].1, entropia));
                    println!("   Entropía de la combinación {} {} y {} {}: {}", headers[num_col], combinaciones_valores_unicos[i].0, col_i, combinaciones_valores_unicos[i].1, entropia);
                }
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
                vec_ramas_arboles.push(format!("      └─── {}-{}-{}\n", headers[num_col], col_i, col_j));
                

                //CALCULOS DE COLUMNA J:
                let valores_unicos_j = valores_unicos_por_columna[headers.iter().position(|x| *x == col_j).unwrap()].clone();

                println!("        Valores únicos de la columna {}: {:?}", col_j, valores_unicos_j); //To-delete                
                //COMBINACIONES DE LOS VALORES ÚNICOS DE LA COLUMNA ROOT, I y J Vec::<String, String, String> = (root[0], col_i[0], col_j[0]), (root[0], col_i[0], col_j[1]), (root[0], col_i[1], col_j[0]), (root[0], col_i[1], col_j[1]):
                let mut combinaciones_valores_unicos: Vec<(String, String, String)> = Vec::new();   
                let mut apariciones_combinaciones_i: Vec<usize> = Vec::new();
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
                            apariciones_combinaciones_i.push(apariciones);
                            //"      Apariciones de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i} y {nombre_columna} {valor_j}: {apariciones}"

                            

                            if VERBOSE{
                                vec_ramas_arboles.push(format!("      Apariciones de la combinación {} {} y {} {} y {} {}: {}\n", headers[num_col], valor_root, col_i, valor_i, col_j, valor_j, apariciones));
                                println!("        Apariciones de la combinación {} {} y {} {} y {} {}: {}", headers[num_col], valor_root, col_i, valor_i, col_j, valor_j, apariciones);
                            }
                        }
                    }
                }              
                
                //PROBABILIDAD.
                //CALCULAMOS LA PROBABILIDAD DE CADA COMBINACIÓN ROOT && I && J.
                //APARIENCIA DE CADA COMBINACIÓN ROOT && I && J / APARICIONES DE CADA VALOR DE ROOT && I:
                let mut indice_combinaciones: usize = 0;
                let mut probabilidad_combinaciones: Vec<f64> = Vec::new();
                // println!("          A: {:?}", apariciones_combinaciones); //To-delete

                for (_i, valor_root) in valores_unicos.iter().enumerate(){
                    for (_j, valor_i) in valores_unicos_i.iter().enumerate(){
                        for (_k, valor_j) in valores_unicos_j.iter().enumerate(){
                            let apariciones = vectores.iter().filter(|x| x.header == headers[num_col])
                            .flat_map(|x| x.col.iter()).zip(vectores.iter().filter(|x| x.header == col_i)
                            .flat_map(|x| x.col.iter())).zip(vectores.iter().filter(|x| x.header == col_j)
                            .flat_map(|x| x.col.iter())).filter(|((x, y), z)| *x == valor_root && *y == valor_i && *z == valor_j)
                            .count();

                            // println!("TAMAÑO: {}", apariciones_combinaciones.len()); //To-delete
                            // println!("apariciones_combinaciones: {:?}", apariciones_combinaciones); //To-delete
                            let div = apariciones_combinaciones[indice_combinaciones] as f64;                            

                            let probabilidad: f64;
                            //VALIDAMOS QUE EL DIVISOR NO SEA 0
                            if div == 0.0 {
                                probabilidad = 0.0;
                            }
                            else{
                                probabilidad = apariciones as f64 / div;
                            }
                            probabilidad_combinaciones.push(probabilidad);                            

                            //"      Probabilidad de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i} y {nombre_columna} {valor_j}: {probabilidad}"

                            

                            if VERBOSE {
                                vec_ramas_arboles.push(format!("      Probabilidad de la combinación {} {} y {} {} y {} {}: {}\n", headers[num_col], valor_root, col_i, valor_i, col_j, valor_j, probabilidad));
                                println!("        Probabilidad de la combinación {} {} y {} {} y {} {}: {}", headers[num_col], valor_root, col_i, valor_i, col_j, valor_j, probabilidad);
                            }
                            // println!("          └─ {} / {}[{}]", apariciones, div, indice_combinaciones);
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
                    if *prob == 0.0 || *prob == 1.0{ 
                        entropia = 0.0;
                        entropia_combinaciones.push(entropia);
                    }
                    else{
                        entropia = -prob * prob.log10();
                        entropia_combinaciones.push(entropia);
                    }
                    // println!("          - {} * log10({}) = {}", prob, prob, entropia); //To-delete

                    //"      Entropía de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i} y {nombre_columna} {valor_j}: {entropia}"

                    

                    if VERBOSE{
                        vec_ramas_arboles.push(format!("      Entropía de la combinación {} {} y {} {} y {} {}: {}\n", headers[num_col], combinaciones_valores_unicos[i].0, col_i, combinaciones_valores_unicos[i].1, col_j, combinaciones_valores_unicos[i].2, entropia));
                        println!("        Entropía de la combinación {} {} y {} {} y {} {}: {}", headers[num_col], combinaciones_valores_unicos[i].0, col_i, combinaciones_valores_unicos[i].1, col_j, combinaciones_valores_unicos[i].2, entropia);
                    }
                    
                }

                //ENTROPIA TOTAL DE LA COMBINACIÓN ROOT && I && J:
                let entropia_total_combinaciones: f64 = entropia_combinaciones.iter().sum();
                //"      Entropía total de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i} y {nombre_columna} {valor_j}: {entropia_total}"
                println!("        Entropía total de la combinación {} y {} y {}: {}", headers[num_col], col_i, col_j, entropia_total_combinaciones);
                




                let columnas_j = columnas_i.clone();
                //Eliminamos la columna actual
                let columnas_j: Vec<String> = columnas_j.into_iter().filter(|x| *x != col_j).collect();






                for col_k in columnas_j.clone() {
                    contador += 1;  
                    //"           └─ {padre}-{hija}-{nieta}-{bisnieta}", col_k
                    println!("           └─ {}-{}-{}-{}", headers[num_col], col_i, col_j, col_k); //To-delete
                    vec_ramas_arboles.push(format!("           └─ {}-{}-{}-{}\n", headers[num_col], col_i, col_j, col_k));

                    //CALCULOS DE COLUMNA K:
                    let valores_unicos_k = valores_unicos_por_columna[headers.iter().position(|x| *x == col_k).unwrap()].clone();
                    println!("             Valores únicos de la columna {}: {:?}", col_k, valores_unicos_k); //To-delete
                    //COMBINACIONES DE LOS VALORES ÚNICOS DE LA COLUMNA ROOT, I, J y K Vec::<String, String, String, String> = (root[0], col_i[0], col_j[0], col_k[0]), (root[0], col_i[0], col_j[0], col_k[1]), (root[0], col_i[0], col_j[1], col_k[0]), (root[0], col_i[0], col_j[1], col_k[1]):
                    let mut combinaciones_valores_unicos: Vec<(String, String, String, String)> = Vec::new();
                    for valor_root in valores_unicos.iter(){
                        for valor_i in valores_unicos_i.iter(){
                            for valor_j in valores_unicos_j.iter(){
                                for valor_k in valores_unicos_k.iter(){
                                    combinaciones_valores_unicos.push((valor_root.to_string(), valor_i.to_string(), valor_j.to_string(), valor_k.to_string()));
                                    //BUSCAMOS LAS APARICIONES DE CADA COMBINACIÓN ROOT && I && J && K: Usize (Sin reiterar, con lo que ya tenemos)
                                    let apariciones = vectores.iter().filter(|x| x.header == headers[num_col])
                                    .flat_map(|x| x.col.iter()).zip(vectores.iter().filter(|x| x.header == col_i)
                                    .flat_map(|x| x.col.iter())).zip(vectores.iter().filter(|x| x.header == col_j)
                                    .flat_map(|x| x.col.iter())).zip(vectores.iter().filter(|x| x.header == col_k)
                                    .flat_map(|x| x.col.iter())).filter(|(((x, y), z), w)| *x == valor_root && *y == valor_i && *z == valor_j && *w == valor_k)
                                    .count();
                                    //"          Apariciones de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i} y {nombre_columna} {valor_j} y {nombre_columna} {valor_k}: {apariciones}"

                                    

                                    if VERBOSE {
                                        vec_ramas_arboles.push(format!("             Apariciones de la combinación {} {} y {} {} y {} {} y {} {}: {}\n", headers[num_col], valor_root, col_i, valor_i, col_j, valor_j, col_k, valor_k, apariciones));
                                        println!("             Apariciones de la combinación {} {} y {} {} y {} {} y {} {}: {}", headers[num_col], valor_root, col_i, valor_i, col_j, valor_j, col_k, valor_k, apariciones);
                                    }
                                }
                            }
                        }
                    }

                    //CALCULAMOS LA PROBABILIDAD DE CADA COMBINACIÓN ROOT && I && J && K.
                    //APARIENCIA DE CADA COMBINACIÓN ROOT && I && J && K / APARICIONES DE CADA VALOR DE ROOT && I && J:
                    let mut indice_combinaciones: usize = 0;
                    let mut probabilidad_combinaciones: Vec<f64> = Vec::new();
                    // println!("          A: {:?}", apariciones_combinaciones_i); //To-delete
                    for (_i, valor_root) in valores_unicos.iter().enumerate(){
                        for (_j, valor_i) in valores_unicos_i.iter().enumerate(){
                            for (_k, valor_j) in valores_unicos_j.iter().enumerate(){
                                for (_l, valor_k) in valores_unicos_k.iter().enumerate(){
                                    let apariciones = vectores.iter().filter(|x| x.header == headers[num_col])
                                    .flat_map(|x| x.col.iter()).zip(vectores.iter().filter(|x| x.header == col_i)
                                    .flat_map(|x| x.col.iter())).zip(vectores.iter().filter(|x| x.header == col_j)
                                    .flat_map(|x| x.col.iter())).zip(vectores.iter().filter(|x| x.header == col_k)
                                    .flat_map(|x| x.col.iter())).filter(|(((x, y), z), w)| *x == valor_root && *y == valor_i && *z == valor_j && *w == valor_k)
                                    .count();

                                    let div = apariciones_combinaciones_i[indice_combinaciones] as f64;
                                    let probabilidad: f64;                                    
                                    //VALIDAMOS QUE EL DIVISOR NO SEA 0
                                    if div == 0.0 {
                                        probabilidad = 0.0;
                                    }
                                    else{
                                        probabilidad = apariciones as f64 / div;
                                    }

                                    probabilidad_combinaciones.push(probabilidad);                            

                                    //"          Probabilidad de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i} y {nombre_columna} {valor_j} y {nombre_columna} {valor_k}: {probabilidad}"

                                    

                                    if VERBOSE {
                                        vec_ramas_arboles.push(format!("             Probabilidad de la combinación {} {} y {} {} y {} {} y {} {}: {}\n", headers[num_col], valor_root, col_i, valor_i, col_j, valor_j, col_k, valor_k, probabilidad));
                                        println!("             Probabilidad de la combinación {} {} y {} {} y {} {} y {} {}: {}", headers[num_col], valor_root, col_i, valor_i, col_j, valor_j, col_k, valor_k, probabilidad);
                                    }
                                    // println!("               └─ {} / {}[{}] = {}", apariciones, div, indice_combinaciones, probabilidad);
                                }
                                //EL índice de combinaciones se incrementa en 1 por cada ciclo completo de l
                                indice_combinaciones += 1;
                            }
                        }
                    }

                    //ENTROPIA DE CADA COMBINACIÓN ROOT && I && J && K:
                    let mut entropia_combinaciones: Vec<f64> = Vec::new();
                    for (i, prob) in probabilidad_combinaciones.iter().enumerate(){
                        //VALIDAMOS QUE LA ENTROPIA NO SEA 0
                        let entropia: f64;
                        if *prob == 0.0 || *prob == 1.0{ 
                            entropia = 0.0;
                            entropia_combinaciones.push(entropia);
                        }
                        else{
                            entropia = -prob * prob.log10();
                            entropia_combinaciones.push(entropia);
                        }
                        // println!("          - {} * log10({}) = {}", prob, prob, entropia); //To-delete

                        //"          Entropía de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i} y {nombre_columna} {valor_j} y {nombre_columna} {valor_k}: {entropia}"

                        

                        if VERBOSE {
                            vec_ramas_arboles.push(format!("             Entropía de la combinación {} {} y {} {} y {} {} y {} {}: {}\n", headers[num_col], combinaciones_valores_unicos[i].0, col_i, combinaciones_valores_unicos[i].1, col_j, combinaciones_valores_unicos[i].2, col_k, combinaciones_valores_unicos[i].3, entropia));
                            println!("             Entropía de la combinación {} {} y {} {} y {} {} y {} {}: {}", headers[num_col], combinaciones_valores_unicos[i].0, col_i, combinaciones_valores_unicos[i].1, col_j, combinaciones_valores_unicos[i].2, col_k, combinaciones_valores_unicos[i].3, entropia);
                        }
                        
                    }

                    //ENTROPIA TOTAL DE LA COMBINACIÓN ROOT && I && J && K:
                    let entropia_total_combinaciones: f64 = entropia_combinaciones.iter().sum();

                    //"          Entropía total de la combinación {nombre_columna} {valor_root} y {nombre_columna} {valor_i} y {nombre_columna} {valor_j} y {nombre_columna} {valor_k}: {entropia_total}"
                    println!("             Entropía total de la combinación {} y {} y {} y {}: {}", headers[num_col], col_i, col_j, col_k, entropia_total_combinaciones);

                    vec_ramas_arboles.push(format!("             Entropia: {}\n", entropia_total_combinaciones));

                    //GUARDAMOS LA ENTROPIA TOTAL DE LA COMBINACIÓN ROOT && I && J && K:
                    vec_arboles_entropias.push((vec_ramas_arboles.clone(), entropia_total_combinaciones));


                }
            }
        }
    }

    println!("\n\n");
    //BUSCAMOS LA RAMA CON LA MENOR ENTROPIA:
    let mut menor_entropia: f64 = 1000000.0;
    let mut rama_menor_entropia: Vec<String> = Vec::new();
    for (rama, entropia) in vec_arboles_entropias.iter(){
        if *entropia < menor_entropia && *entropia != 0.0{
            menor_entropia = *entropia;
            rama_menor_entropia = rama.clone();
        }
    }

    let mut rama_string = String::new();

    for rama in rama_menor_entropia.iter(){
        rama_string.push_str(rama);
        //SI ENCONTRAMOS LA SECUENCIA "Entropia: {menor_entropia}" ENTONCES CORTAMOS EL STRING (BREAK)
        if rama.contains(&format!("Entropia: {}", menor_entropia)){
            break;
        }
    }

    println!("Rama con menor entropía: \n {}", rama_string);

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