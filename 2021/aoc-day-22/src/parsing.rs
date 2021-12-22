use crate::core::{Command, Cuboid, Int, Operation};
use std::str::FromStr;

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rest, z) = s
            .rsplit_once(',')
            .ok_or_else(|| format!("cannot extract the Z range: {:?}", s))?;
        let (rest, y) = rest
            .rsplit_once(',')
            .ok_or_else(|| format!("cannot extract the Y range: {:?}", s))?;
        let (op, x) = rest
            .split_once(' ')
            .ok_or_else(|| format!("cannot extract the X range: {:?}", s))?;

        let (z0, z1) = parse_range(&z[2..])?;
        let (y0, y1) = parse_range(&y[2..])?;
        let (x0, x1) = parse_range(&x[2..])?;

        let cuboid = Cuboid::new((x0, y0, z0), (x1 + 1, y1 + 1, z1 + 1)).ok_or_else(|| {
            format!(
                "invalid cuboid dimensions: x={}..{}; y={}..{}, z={}..{}",
                x0, x1, y0, y1, z0, z1
            )
        })?;

        let op = match op {
            "on" => Operation::On,
            "off" => Operation::Off,
            _ => return Err(format!("unsupported operation: {:?}", op)),
        };

        Ok(Command { op, cuboid })
    }
}

fn parse_range(s: &str) -> Result<(Int, Int), String> {
    let (from, to) = s
        .split_once("..")
        .ok_or_else(|| format!("cannot split range: {:?}", s))?;

    let from = from
        .parse()
        .map_err(|e| format!("failed to parse number: {:?} -> {:?}", from, e))?;
    let to = to
        .parse()
        .map_err(|e| format!("failed to parse number: {:?} -> {:?}", from, e))?;

    Ok((from, to))
}
