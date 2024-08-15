/// Chequea si se encuentra una pieza enemiga en el rango especificado, de forma horizontal y vertical.
/// Devuelve true si se encontro una pieza enemiga, y false si no.
pub fn chequear_rango_ver_hor(rango: isize, x1: isize, y1: isize, x2: isize, y2: isize) -> bool {
    if ((x1 + rango) == x2 || (x1 - rango) == x2) && y1 == y2
        || (x1 == x2 && (y1 - rango) == y2)
        || (x1 == x2 && (y1 + rango) == y2)
    {
        return true;
    }
    false
}

/// Chequea si se encuentra una pieza enemiga en el rango especificado, de forma diagonal.
/// Devuelve true si se encontro una pieza enemiga, y false si no.
pub fn chequear_rango_diagonal(rango: isize, x1: isize, y1: isize, x2: isize, y2: isize) -> bool {
    if ((x1 + rango) == x2 || (x1 - rango) == x2) && ((y1 + rango) == y2 || (y1 - rango) == y2) {
        return true;
    }
    false
}

/// Chequea si se encuentra una pieza enemiga en el movimiento del caballo.
/// dx y dy indican que movimiento del caballo en particular se realiza.
/// Devuelve true si se encontro una pieza enemiga, y false si no.
pub fn chequear_mov_caballo(
    x: isize,
    dx: isize,
    y: isize,
    dy: isize,
    x_enemigo: isize,
    y_enemigo: isize,
) -> bool {
    if (x + dx) == x_enemigo && (y + dy) == y_enemigo {
        return true;
    }
    false
}

#[test]
fn test_rango_ver_hor() {
    assert_eq!(chequear_rango_ver_hor(1, 0, 0, 0, 1), true);
    assert_eq!(chequear_rango_ver_hor(1, 0, 0, 1, 0), true);
    assert_eq!(chequear_rango_ver_hor(1, 0, 0, 1, 1), false);
    assert_eq!(chequear_rango_ver_hor(2, 0, 0, 1, 0), false);
    assert_eq!(chequear_rango_ver_hor(1, 4, 4, 3, 4), true);
    assert_eq!(chequear_rango_ver_hor(1, 4, 4, 5, 4), true);
    assert_eq!(chequear_rango_ver_hor(3, 4, 4, 4, 1), true);
    assert_eq!(chequear_rango_ver_hor(3, 4, 4, 4, 7), true);
    assert_eq!(chequear_rango_ver_hor(2, 4, 4, 4, 1), false);
    assert_eq!(chequear_rango_ver_hor(1, 4, 4, 3, 3), false);
}

#[test]
fn test_rango_diagonal() {
    assert_eq!(chequear_rango_diagonal(1, 4, 4, 3, 3), true);
    assert_eq!(chequear_rango_diagonal(1, 4, 4, 5, 5), true);
    assert_eq!(chequear_rango_diagonal(1, 4, 4, 5, 3), true);
    assert_eq!(chequear_rango_diagonal(1, 4, 4, 5, 4), false);
    assert_eq!(chequear_rango_diagonal(3, 4, 4, 4, 7), false);
    assert_eq!(chequear_rango_diagonal(1, 0, 0, 1, 1), true);
    assert_eq!(chequear_rango_diagonal(2, 0, 0, 2, 2), true);
    assert_eq!(chequear_rango_diagonal(3, 0, 0, 3, 3), true);
    assert_eq!(chequear_rango_diagonal(4, 0, 0, 4, 4), true);
    assert_eq!(chequear_rango_diagonal(3, 4, 4, 4, 1), false);
}

#[test]
fn test_mov_caballo() {
    assert_eq!(chequear_mov_caballo(4, -2, 4, -1, 2, 3), true);
    assert_eq!(chequear_mov_caballo(4, -2, 4, 1, 2, 5), true);
    assert_eq!(chequear_mov_caballo(4, 2, 4, -1, 6, 3), true);
    assert_eq!(chequear_mov_caballo(4, 2, 4, 1, 6, 5), true);
    assert_eq!(chequear_mov_caballo(4, -1, 4, -2, 3, 2), true);
    assert_eq!(chequear_mov_caballo(4, -1, 4, 2, 3, 6), true);
    assert_eq!(chequear_mov_caballo(4, 1, 4, -2, 5, 2), true);
    assert_eq!(chequear_mov_caballo(4, 1, 4, 2, 5, 6), true);
    assert_eq!(chequear_mov_caballo(4, -2, 4, -1, 2, 4), false);
    assert_eq!(chequear_mov_caballo(4, 2, 4, 1, 6, 4), false);
    assert_eq!(chequear_mov_caballo(4, -1, 4, 2, 4, 6), false);
}
