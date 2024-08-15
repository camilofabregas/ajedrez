use super::misc::chequear_mov_caballo;

pub struct Caballo;
impl Caballo {
    /// Chequea el rango de ataque del Caballo y devuelve true o false si encontro alguna pieza enemiga o no.
    pub fn puede_capturar_a(x: isize, y: isize, x_enemigo: isize, y_enemigo: isize) -> bool {
        if chequear_mov_caballo(x, -2, y, -1, x_enemigo, y_enemigo)
            || chequear_mov_caballo(x, -2, y, 1, x_enemigo, y_enemigo)
            || chequear_mov_caballo(x, 2, y, -1, x_enemigo, y_enemigo)
            || chequear_mov_caballo(x, 2, y, 1, x_enemigo, y_enemigo)
            || chequear_mov_caballo(x, -1, y, -2, x_enemigo, y_enemigo)
            || chequear_mov_caballo(x, -1, y, 2, x_enemigo, y_enemigo)
            || chequear_mov_caballo(x, 1, y, -2, x_enemigo, y_enemigo)
            || chequear_mov_caballo(x, 1, y, 2, x_enemigo, y_enemigo)
        {
            return true;
        }
        false
    }
}

#[test]
fn test_caballo() {
    assert_eq!(Caballo::puede_capturar_a(4, 4, 3, 2), true);
    assert_eq!(Caballo::puede_capturar_a(4, 4, 5, 2), true);
    assert_eq!(Caballo::puede_capturar_a(4, 4, 2, 3), true);
    assert_eq!(Caballo::puede_capturar_a(4, 4, 2, 5), true);
    assert_eq!(Caballo::puede_capturar_a(4, 4, 3, 6), true);
    assert_eq!(Caballo::puede_capturar_a(4, 4, 5, 6), true);
    assert_eq!(Caballo::puede_capturar_a(4, 4, 4, 2), false);
    assert_eq!(Caballo::puede_capturar_a(4, 4, 3, 4), false);
}
