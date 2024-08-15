use super::misc::chequear_rango_diagonal;
use super::misc::chequear_rango_ver_hor;

pub struct Rey;
impl Rey {
    /// Chequea el rango de ataque del Rey y devuelve true o false si encontro alguna pieza enemiga o no.
    pub fn puede_capturar_a(x: isize, y: isize, x_enemigo: isize, y_enemigo: isize) -> bool {
        if chequear_rango_ver_hor(1, x, y, x_enemigo, y_enemigo)
            || chequear_rango_diagonal(1, x, y, x_enemigo, y_enemigo)
        {
            return true;
        }
        false
    }
}

#[test]
fn test_rey() {
    assert_eq!(Rey::puede_capturar_a(3, 3, 3, 4), true);
    assert_eq!(Rey::puede_capturar_a(3, 3, 3, 2), true);
    assert_eq!(Rey::puede_capturar_a(3, 3, 4, 3), true);
    assert_eq!(Rey::puede_capturar_a(3, 3, 2, 3), true);
    assert_eq!(Rey::puede_capturar_a(3, 3, 4, 4), true);
    assert_eq!(Rey::puede_capturar_a(3, 3, 2, 2), true);
    assert_eq!(Rey::puede_capturar_a(3, 3, 2, 4), true);
    assert_eq!(Rey::puede_capturar_a(3, 3, 4, 2), true);
    assert_eq!(Rey::puede_capturar_a(3, 3, 3, 1), false);
    assert_eq!(Rey::puede_capturar_a(3, 3, 3, 5), false);
    assert_eq!(Rey::puede_capturar_a(3, 3, 1, 3), false);
}
