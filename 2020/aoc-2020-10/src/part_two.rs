pub fn solve_v1(input: &mut [usize]) -> Option<usize> {
    input.sort_unstable();
    let input = &*input;

    let mut cache = vec![0usize; input[input.len() - 1] + 1];

    for &i in input.iter() {
        if i <= 3 {
            cache[i] += 1;

            for x in 1..i {
                cache[i] += cache[x];
            }

            continue;
        }

        cache[i] += cache[i - 3];
        cache[i] += cache[i - 2];
        cache[i] += cache[i - 1];
    }

    Some(cache[cache.len() - 1])
}

pub fn solve_v2_const_mem(input: &mut [usize]) -> Option<usize> {
    input.sort_unstable();
    let input = &*input;

    let mut cache = [0; 3];
    let mut prev = 0;

    for &i in input.iter() {
        let mut combinations = cache[0];

        if i <= 3 {
            combinations += 1;
        }

        let diff = i - prev;
        match diff {
            1 => {
                combinations += cache[1] + cache[2];
                cache[2] = cache[1];
                cache[1] = cache[0];
            }
            2 => {
                combinations += cache[1];
                cache[2] = cache[0];
                cache[1] = 0;
            }
            3 => {
                cache[2] = 0;
                cache[1] = 0;
            }
            _ => return None,
        }

        cache[0] = combinations;
        prev = i;
    }

    Some(cache[0])
}
