use super::misc::chequear_rango_ver_hor;

pub struct Torre;
impl Torre {
    /// Chequea el rango de ataque de la Torre y devuelve true o false si encontro alguna pieza enemiga o no.
    pub fn puede_capturar_a(x: isize, y: isize, x_enemigo: isize, y_enemigo: isize) -> bool {
        for i in 1..8 {
            if chequear_rango_ver_hor(i, x, y, x_enemigo, y_enemigo) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test_torre_vertical() {
    assert_eq!(Torre::puede_capturar_a(0, 0, 0, 1), true);
    assert_eq!(Torre::puede_capturar_a(0, 0, 0, 2), true);
    assert_eq!(Torre::puede_capturar_a(0, 0, 0, 3), true);
    assert_eq!(Torre::puede_capturar_a(0, 0, 0, 4), true);
    assert_eq!(Torre::puede_capturar_a(0, 0, 0, 5), true);
    assert_eq!(Torre::puede_capturar_a(0, 0, 0, 6), true);
    assert_eq!(Torre::puede_capturar_a(0, 0, 0, 7), true);
    assert_eq!(Torre::puede_capturar_a(0, 0, 1, 1), false);
    assert_eq!(Torre::puede_capturar_a(0, 0, 3, 5), false);
}

#[test]
fn test_torre_horizontal() {
    assert_eq!(Torre::puede_capturar_a(0, 0, 1, 0), true);
    assert_eq!(Torre::puede_capturar_a(0, 0, 2, 0), true);
    assert_eq!(Torre::puede_capturar_a(0, 0, 3, 0), true);
    assert_eq!(Torre::puede_capturar_a(0, 0, 4, 0), true);
    assert_eq!(Torre::puede_capturar_a(0, 0, 5, 0), true);
    assert_eq!(Torre::puede_capturar_a(0, 0, 6, 0), true);
    assert_eq!(Torre::puede_capturar_a(0, 0, 7, 0), true);
    assert_eq!(Torre::puede_capturar_a(0, 0, 1, 1), false);
    assert_eq!(Torre::puede_capturar_a(0, 0, 3, 5), false);
}
