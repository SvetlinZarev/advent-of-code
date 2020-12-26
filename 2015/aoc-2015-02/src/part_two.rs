use crate::Cuboid;

pub fn solve(input: &[Cuboid]) -> u32 {
    let mut ribbon_length = 0;

    for cuboid in input.iter().copied() {
        ribbon_length += cuboid.volume();
        ribbon_length += cuboid.min_side_perimeter();
    }

    ribbon_length
}
