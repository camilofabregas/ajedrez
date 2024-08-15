pub struct Peon;
impl Peon {
    /// Chequea el rango de ataque del Peon y devuelve true o false si encontro alguna pieza enemiga o no.
    pub fn puede_capturar_a(
        x: isize,
        y: isize,
        x_enemigo: isize,
        y_enemigo: isize,
        color: &char,
    ) -> bool {
        if ((x - 1) == x_enemigo || (x + 1) == x_enemigo)
            && ((color.is_uppercase() && (y + 1) == y_enemigo)
                || (!color.is_uppercase() && (y - 1) == y_enemigo))
        {
            return true;
        }
        false
    }
}

#[test]
fn test_peon_negro() {
    assert_eq!(Peon::puede_capturar_a(0, 0, 2, 1, &'P'), false);
    assert_eq!(Peon::puede_capturar_a(0, 0, 0, 1, &'P'), false);
    assert_eq!(Peon::puede_capturar_a(0, 0, 1, 1, &'P'), true);
}

#[test]
fn test_peon_blanco() {
    assert_eq!(Peon::puede_capturar_a(3, 2, 3, 1, &'p'), false);
    assert_eq!(Peon::puede_capturar_a(3, 2, 2, 1, &'p'), true);
    assert_eq!(Peon::puede_capturar_a(3, 2, 4, 1, &'p'), true);
}
