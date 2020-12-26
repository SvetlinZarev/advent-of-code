use crate::Cuboid;

pub fn solve(input: &[Cuboid]) -> u32 {
    let mut area = 0;

    for cuboid in input.iter().copied() {
        area += cuboid.area();
        area += cuboid.min_side_area();
    }

    area
}
