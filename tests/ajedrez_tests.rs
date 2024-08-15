extern crate ajedrez;

use ajedrez::partida;

/// Probando tablero 1 (ejemplo catedra).
#[test]
fn test_ajedrez_negras() {
    let input = partida::cargar_archivo("ej1.txt");

    let tablero = partida::generar_tablero(input);
    let (pieza_negra, pieza_blanca) = match partida::detectar_piezas(&tablero) {
        Ok((pieza_negra, pieza_blanca)) => (pieza_negra, pieza_blanca),
        Err(mje_error) => {
            println!("{}", mje_error);
            return;
        }
    };
    let (res_negra, res_blanca) = partida::captura_inminente(pieza_negra, pieza_blanca);
    assert!(res_negra && !res_blanca); // Ganan negras ('N')
}

/// Probando tablero 2 (ejemplo catedra).
#[test]
fn test_ajedrez_blancas() {
    let input = partida::cargar_archivo("ej2.txt");

    let tablero = partida::generar_tablero(input);
    let (pieza_negra, pieza_blanca) = match partida::detectar_piezas(&tablero) {
        Ok((pieza_negra, pieza_blanca)) => (pieza_negra, pieza_blanca),
        Err(mje_error) => {
            println!("{}", mje_error);
            return;
        }
    };
    let (res_negra, res_blanca) = partida::captura_inminente(pieza_negra, pieza_blanca);
    assert!(res_blanca && !res_negra); // Ganan blancas ('B')
}

/// Probando tablero 3 (ejemplo catedra).
#[test]
fn test_ajedrez_empate() {
    let input = partida::cargar_archivo("ej3.txt");

    let tablero = partida::generar_tablero(input);
    let (pieza_negra, pieza_blanca) = match partida::detectar_piezas(&tablero) {
        Ok((pieza_negra, pieza_blanca)) => (pieza_negra, pieza_blanca),
        Err(mje_error) => {
            println!("{}", mje_error);
            return;
        }
    };
    let (res_negra, res_blanca) = partida::captura_inminente(pieza_negra, pieza_blanca);
    assert!(res_blanca && res_negra); // Empate (ambas capturan) ('E')
}

/// Probando tablero 4 (ejemplo catedra).
#[test]
fn test_ajedrez_derrota() {
    let input = partida::cargar_archivo("ej4.txt");

    let tablero = partida::generar_tablero(input);
    let (pieza_negra, pieza_blanca) = match partida::detectar_piezas(&tablero) {
        Ok((pieza_negra, pieza_blanca)) => (pieza_negra, pieza_blanca),
        Err(mje_error) => {
            println!("{}", mje_error);
            return;
        }
    };
    let (res_negra, res_blanca) = partida::captura_inminente(pieza_negra, pieza_blanca);
    assert!(!res_negra && !res_blanca); // No gana nadie ('P')
}

/// Probando tablero default (se pasa ruta que no existe).
#[test]
fn test_ajedrez_ruta_inexistente() {
    let input = partida::cargar_archivo("noexisto.txt");

    let tablero = partida::generar_tablero(input);
    let (pieza_negra, pieza_blanca) = match partida::detectar_piezas(&tablero) {
        Ok((pieza_negra, pieza_blanca)) => (pieza_negra, pieza_blanca),
        Err(mje_error) => {
            println!("{}", mje_error);
            return;
        }
    };
    let (res_negra, res_blanca) = partida::captura_inminente(pieza_negra, pieza_blanca);
    assert!(res_negra && !res_blanca); // Ganan negras ('N')
}

/// Probando tablero con 3 piezas.
#[test]
fn test_ajedrez_tres_piezas() {
    let input = partida::cargar_archivo("ej6.txt");

    let tablero = partida::generar_tablero(input);

    assert!(partida::detectar_piezas(&tablero).is_err());
}

/// Probando tablero con dos piezas del mismo color.
#[test]
fn test_ajedrez_piezas_mismo_color() {
    let input = partida::cargar_archivo("ej7.txt");

    let tablero = partida::generar_tablero(input);

    assert!(partida::detectar_piezas(&tablero).is_err());
}

/// Probando tablero que no respeta dimensiones de 8x8.
#[test]
fn test_ajedrez_fila_extra() {
    let input = partida::cargar_archivo("ej8.txt");

    let tablero = partida::generar_tablero(input);
    let tablero_default = partida::generar_tablero("_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ D _ _ _ _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ t _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ _ _".to_owned());

    assert_eq!(tablero, tablero_default);
}

/// Probando tablero que no respeta dimensiones de 8x8.
#[test]
fn test_ajedrez_columna_extra() {
    let input = partida::cargar_archivo("ej9.txt");

    let tablero = partida::generar_tablero(input);
    let tablero_default = partida::generar_tablero("_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ D _ _ _ _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ t _\r\n_ _ _ _ _ _ _ _\r\n_ _ _ _ _ _ _ _".to_owned());

    assert_eq!(tablero, tablero_default);
}
