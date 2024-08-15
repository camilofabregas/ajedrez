use super::pieza::Pieza;
use std::fs;

/// Recibe la ruta a un archivo y carga el contenido en un String.
/// Si se recibe una ruta invalida o una ruta default, se carga un tablero default.
pub fn cargar_archivo(ruta: &str) -> String {
    let input_default = "_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ D _ _ _ _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ t _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ _ _".to_owned();
    if ruta == "input_default" {
        return input_default;
    }
    let input = fs::read_to_string(ruta);
    match input {
        Ok(input) => input,
        Err(_) => {
            println!("ERROR: no se pudo leer o encontrar el archivo. Se cargara uno por default.");
            input_default
        }
    }
}

/// Recibe el string del input y genera el tablero como vector de 8 lineas x 8 columnas
/// Si el tablero no es de 8x8, se carga uno por default.
/// Finalmente, se llama a detectar_piezas.
pub fn generar_tablero(input: String) -> Vec<String> {
    let temp = input.replace(' ', "");
    let tablero: Vec<String> = temp.split("\r\n").map(str::to_string).collect();
    if chequear_largo(&tablero) {
        tablero
    } else {
        println!("ERROR: el tablero no es de 8x8. Se cargara uno por default.");
        let input_default = "_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ D _ _ _ _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ t _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ _ _".to_owned();
        let temp_default = input_default.replace(' ', "");
        let tablero_default: Vec<String> = temp_default.split("\r\n").map(str::to_string).collect();
        tablero_default
    }
}

/// Devuelve true si el tablero es de 8x8 y false si no.
fn chequear_largo(tablero: &Vec<String>) -> bool {
    if tablero.len() == 8
        && tablero[0].len() == 8
        && tablero[1].len() == 8
        && tablero[2].len() == 8
        && tablero[3].len() == 8
        && tablero[4].len() == 8
        && tablero[5].len() == 8
        && tablero[6].len() == 8
        && tablero[7].len() == 8
    {
        return true;
    }
    false
}

/// Recorre los simbolos del tablero y si es distinto de _, es una pieza.
/// Si es mayuscula, es negra. Si es minuscula, es blanca.
/// Finalmente detecta que se tenga exactamente una pieza blanca y una pieza negra.
/// En ese caso, se printea el error y se vuelve a main.
pub fn detectar_piezas(tablero: &[String]) -> Result<(Vec<Pieza>, Vec<Pieza>), String> {
    let mut pieza_negra = Vec::new();
    let mut pieza_blanca = Vec::new();
    for (y, fila) in tablero.iter().enumerate() {
        for (x, caracter) in fila.chars().enumerate() {
            if caracter != '_' {
                if caracter.is_uppercase() {
                    pieza_negra.push(super::pieza::asignar_pieza(
                        &caracter, x as isize, y as isize,
                    ));
                } else {
                    pieza_blanca.push(super::pieza::asignar_pieza(
                        &caracter, x as isize, y as isize,
                    ));
                }
            }
        }
    }
    if pieza_negra.len() == 1 && pieza_blanca.len() == 1 {
        Ok((pieza_negra, pieza_blanca))
    } else {
        Err("ERROR: la cantidad de piezas es incorrecta (una blanca y una negra).".to_owned())
    }
}

/// Realiza el intento de captura de la pieza negra a la blanca, y viceversa.
/// Se obtiene un booleano por cada pieza.
pub fn captura_inminente(pieza_negra: Vec<Pieza>, pieza_blanca: Vec<Pieza>) -> (bool, bool) {
    let resultado_negra = pieza_negra[0].puede_capturar_a(&pieza_blanca[0]);
    let resultado_blanca = pieza_blanca[0].puede_capturar_a(&pieza_negra[0]);
    //return generar_resultado(resultado_blanca, resultado_negra);
    (resultado_negra, resultado_blanca)
}

/// Imprime el resultado de la partida por pantalla.
pub fn generar_resultado(resultado_negra: bool, resultado_blanca: bool) {
    if resultado_blanca && !resultado_negra {
        println!("B");
    } else if !resultado_blanca && resultado_negra {
        println!("N");
    } else if resultado_blanca && resultado_negra {
        println!("E");
    } else {
        println!("P");
    }
}

#[test]
fn test_chequear_largo() {
    let input_test = "_ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ D _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ t _
            _ _ _ _ _ _ _ _
            _ _ _ _ _ _ _ _"
        .to_owned();
    let temp_test = input_test.replace(' ', "");
    let tablero_test: Vec<String> = temp_test.split("\n").map(str::to_string).collect();
    assert_eq!(chequear_largo(&tablero_test), true);

    let input_test = "_ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ D _ _ _ 
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ t _
    _ _ _ _ _ _ _ _
    _ _ _ _ _ _ _ _"
        .to_owned();
    let temp_test = input_test.replace(' ', "");
    let tablero_test: Vec<String> = temp_test.split("\n").map(str::to_string).collect();
    assert_eq!(chequear_largo(&tablero_test), false);
}
