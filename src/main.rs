use std::env;

mod alfil;
mod caballo;
mod dama;
mod misc;
mod partida;
mod peon;
mod pieza;
mod rey;
mod torre;

/// Recibe el archivo de entrada (si no se provee ninguno, se genera un tablero default).
/// Si hay algun error con la carga del archivo, se genera un tablero default.
/// El input queda guardado en un String, y se envia a generar_tablero pasarlo a un Vector.
fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() < 2 {
        println!("ERROR: no se proveyo un archivo. Se cargara uno por default.");
        partida::cargar_archivo("input_default")
    } else {
        partida::cargar_archivo(&args[1])
    };

    let tablero = partida::generar_tablero(input);
    let (pieza_negra, pieza_blanca) = match partida::detectar_piezas(&tablero) {
        Ok((pieza_negra, pieza_blanca)) => (pieza_negra, pieza_blanca),
        Err(mje_error) => {
            println!("{}", mje_error);
            return;
        }
    };
    let (res_negra, res_blanca) = partida::captura_inminente(pieza_negra, pieza_blanca);
    partida::generar_resultado(res_negra, res_blanca);
}
