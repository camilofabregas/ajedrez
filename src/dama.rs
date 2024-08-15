use super::misc::chequear_rango_diagonal;
use super::misc::chequear_rango_ver_hor;

pub struct Dama;
impl Dama {
    /// Chequea el rango de ataque de la Dama y devuelve true o false si encontro alguna pieza enemiga o no.
    pub fn puede_capturar_a(x: isize, y: isize, x_enemigo: isize, y_enemigo: isize) -> bool {
        for i in 1..8 {
            if chequear_rango_ver_hor(i, x, y, x_enemigo, y_enemigo)
                || chequear_rango_diagonal(i, x, y, x_enemigo, y_enemigo)
            {
                return true;
            }
        }
        false
    }
}

#[test]
fn test_dama_vertical() {
    assert_eq!(Dama::puede_capturar_a(3, 3, 3, 0), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 3, 1), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 3, 2), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 3, 4), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 3, 5), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 3, 6), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 3, 7), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 1, 2), false);
    assert_eq!(Dama::puede_capturar_a(3, 3, 1, 2), false);
    assert_eq!(Dama::puede_capturar_a(3, 3, 2, 5), false);
}

#[test]
fn test_dama_horizontal() {
    assert_eq!(Dama::puede_capturar_a(3, 3, 0, 3), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 1, 3), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 2, 3), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 4, 3), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 5, 3), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 6, 3), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 7, 3), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 2, 1), false);
    assert_eq!(Dama::puede_capturar_a(3, 3, 5, 2), false);
}

#[test]
fn test_dama_diagonal() {
    assert_eq!(Dama::puede_capturar_a(3, 3, 2, 2), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 1, 1), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 0, 0), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 4, 4), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 5, 5), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 6, 6), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 7, 7), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 4, 2), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 5, 1), true);
    assert_eq!(Dama::puede_capturar_a(3, 3, 6, 0), true);
}
