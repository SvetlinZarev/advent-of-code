pub use direction::Direction;
pub use point::Point;

pub const DIR4: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

mod direction;
mod point;
