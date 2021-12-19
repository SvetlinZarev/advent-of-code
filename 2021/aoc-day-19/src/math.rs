use crate::{Int, Point};

pub(crate) const fn distance_sq(a: Point, b: Point) -> Int {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2) + (a.2 - b.2).pow(2)
}

pub(crate) const fn is_same_vector(a: (Point, Point), b: (Point, Point)) -> bool {
    let p1 = subtract(a.1, a.0);
    let p2 = subtract(b.1, b.0);

    p1.0 == p2.0 && p1.1 == p2.1 && p1.2 == p2.2
}

pub(crate) const fn subtract(a: Point, b: Point) -> Point {
    (a.0 - b.0, a.1 - b.1, a.2 - b.2)
}

pub(crate) const fn rotate(point: Point, rot: u8) -> Point {
    match rot {
        0 => (point.0, point.1, point.2),     //[x, y, z]
        1 => (point.0, point.2, -point.1),    //[x, z, -y],
        2 => (point.0, -point.1, -point.2),   //[x, -y, -z],
        3 => (point.0, -point.2, point.1),    //[x, -z, y],
        4 => (point.1, point.0, -point.2),    //[y, x, -z],
        5 => (point.1, point.2, point.0),     //[y, z, x],
        6 => (point.1, -point.0, point.2),    //[y, -x, z],
        7 => (point.1, -point.2, -point.0),   //[y, -z, -x],
        8 => (point.2, point.0, point.1),     //[z, x, y],
        9 => (point.2, point.1, -point.0),    //[z, y, -x],
        10 => (point.2, -point.0, -point.1),  //[z, -x, -y],
        11 => (point.2, -point.1, point.0),   //[z, -y, x],
        12 => (-point.0, point.1, -point.2),  //[-x, y, -z],
        13 => (-point.0, point.2, point.1),   //[-x, z, y],
        14 => (-point.0, -point.1, point.2),  //[-x, -y, z],
        15 => (-point.0, -point.2, -point.1), //[-x, -z, -y],
        16 => (-point.1, point.0, point.2),   //[-y, x, z],
        17 => (-point.1, point.2, -point.0),  //[-y, z, -x],
        18 => (-point.1, -point.0, -point.2), //[-y, -x, -z],
        19 => (-point.1, -point.2, point.0),  //[-y, -z, x],
        20 => (-point.2, point.0, -point.1),  //[-z, x, -y],
        21 => (-point.2, point.1, point.0),   //[-z, y, x],
        22 => (-point.2, -point.0, point.1),  //[-z, -x, y],
        23 => (-point.2, -point.1, -point.0), //[-z, -y, -x],
        _ => unreachable!(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_1() {
        let a = (0, 0, 0);
        let b = (1, 2, 3);

        let d = distance_sq(a, b);
        assert_eq!(14, d);
    }
}
