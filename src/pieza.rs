use super::alfil::Alfil;
use super::caballo::Caballo;
use super::dama::Dama;
use super::peon::Peon;
use super::rey::Rey;
use super::torre::Torre;

/// Instancia el tipo de pieza segun el caracter, y la asigna sus coordenadas.
/// Devuelve la Pieza instanciada.
pub fn asignar_pieza(caracter: &char, fil: isize, col: isize) -> Pieza {
    match caracter {
        'R' | 'r' => Pieza {
            x: fil,
            y: col,
            tipo: TipoPieza::Rey,
        },
        'D' | 'd' => Pieza {
            x: fil,
            y: col,
            tipo: TipoPieza::Dama,
        },
        'A' | 'a' => Pieza {
            x: fil,
            y: col,
            tipo: TipoPieza::Alfil,
        },
        'C' | 'c' => Pieza {
            x: fil,
            y: col,
            tipo: TipoPieza::Caballo,
        },
        'T' | 't' => Pieza {
            x: fil,
            y: col,
            tipo: TipoPieza::Torre,
        },
        'P' => Pieza {
            x: fil,
            y: col,
            tipo: TipoPieza::Peon { color: 'P' },
        },
        'p' => Pieza {
            x: fil,
            y: col,
            tipo: TipoPieza::Peon { color: 'p' },
        },
        _ => Pieza {
            x: fil,
            y: col,
            tipo: TipoPieza::Rey,
        }, //Se asigna pieza Rey en caso de error.
    }
}

pub struct Pieza {
    x: isize,
    y: isize,
    tipo: TipoPieza,
}
/// Delega la captura a su tipo de pieza (TipoPieza).
impl Pieza {
    pub fn puede_capturar_a(&self, otra_pieza: &Pieza) -> bool {
        self.tipo
            .puede_capturar_a(self.x, self.y, otra_pieza.x, otra_pieza.y)
    }
}

/// Los tipos de pieza que alberga la clase Pieza.
enum TipoPieza {
    Rey,
    Dama,
    Alfil,
    Caballo,
    Torre,
    Peon { color: char },
}
impl TipoPieza {
    fn puede_capturar_a(&self, x: isize, y: isize, x_enemigo: isize, y_enemigo: isize) -> bool {
        match self {
            TipoPieza::Rey => Rey::puede_capturar_a(x, y, x_enemigo, y_enemigo),
            TipoPieza::Dama => Dama::puede_capturar_a(x, y, x_enemigo, y_enemigo),
            TipoPieza::Alfil => Alfil::puede_capturar_a(x, y, x_enemigo, y_enemigo),
            TipoPieza::Caballo => Caballo::puede_capturar_a(x, y, x_enemigo, y_enemigo),
            TipoPieza::Torre => Torre::puede_capturar_a(x, y, x_enemigo, y_enemigo),
            TipoPieza::Peon { color } => Peon::puede_capturar_a(x, y, x_enemigo, y_enemigo, color),
        }
    }
}
