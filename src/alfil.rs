use super::misc::chequear_rango_diagonal;

pub struct Alfil;
impl Alfil {
    /// Chequea el rango de ataque del Alfil y devuelve true o false si encontro alguna pieza enemiga o no.
    pub fn puede_capturar_a(x: isize, y: isize, x_enemigo: isize, y_enemigo: isize) -> bool {
        for i in 1..8 {
            if chequear_rango_diagonal(i, x, y, x_enemigo, y_enemigo) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test_alfil() {
    assert_eq!(Alfil::puede_capturar_a(3, 3, 2, 2), true);
    assert_eq!(Alfil::puede_capturar_a(3, 3, 1, 1), true);
    assert_eq!(Alfil::puede_capturar_a(3, 3, 0, 0), true);
    assert_eq!(Alfil::puede_capturar_a(3, 3, 4, 4), true);
    assert_eq!(Alfil::puede_capturar_a(3, 3, 5, 5), true);
    assert_eq!(Alfil::puede_capturar_a(3, 3, 6, 6), true);
    assert_eq!(Alfil::puede_capturar_a(3, 3, 7, 7), true);
    assert_eq!(Alfil::puede_capturar_a(3, 3, 4, 2), true);
    assert_eq!(Alfil::puede_capturar_a(3, 3, 5, 1), true);
    assert_eq!(Alfil::puede_capturar_a(3, 3, 6, 0), true);
}
