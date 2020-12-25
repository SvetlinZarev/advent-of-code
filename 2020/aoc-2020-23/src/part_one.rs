const MOVES: usize = 100;

pub fn solve(input: &[usize]) -> String {
    assert!(input.len() > 4);

    let min = input.iter().copied().min().unwrap();
    let max = input.iter().copied().max().unwrap();
    assert!(min > 0);

    let mut cups = input.iter().copied().collect::<Vec<_>>();

    for _ in 0..MOVES {
        let a = cups.remove(1);
        let b = cups.remove(1);
        let c = cups.remove(1);

        let mut target = cups[0] - 1;
        while target == a || target == b || target == c || target < min {
            if target <= min {
                target = max;
            } else {
                target -= 1;
            }
        }

        let idx = cups.iter().copied().position(|x| x == target).unwrap();
        cups.insert(idx + 1, a);
        cups.insert(idx + 2, b);
        cups.insert(idx + 3, c);
        cups.rotate_left(1);
    }

    let idx = cups.iter().copied().position(|x| x == 1).unwrap();
    cups.rotate_left(idx);

    cups[1..]
        .iter()
        .copied()
        .fold(String::with_capacity(9), |mut result, v| {
            result.push((v + b'0' as usize) as u8 as char);
            result
        })
}
