pub fn solve(input: &[u64]) -> Option<u64> {
    let mut window_start = 0;
    let mut window_end = 25;
    let mut window = [0u64; 25];
    window.copy_from_slice(&input[window_start..window_end]);

    while window_end < input.len() {
        let search_key = input[window_end];

        // There are two approaches to handling this:
        // 1.) Copy the new window on every loop iteration and sort it.
        //     This is suboptimal because we are copying 25 elements,
        //     instead of just one. Also we loose the sorting we had.
        // 2.) Only replace the modified element, keeping the
        //     array almost sorted. Then an insertion sort could
        //     could sort it in O(N) time :) Unfortunately, rust's
        //     sort() implementation uses insertion sort only for
        //     arrays that are <= 20 elements long. So I use the
        //     unstable_sort(), which is not as fast, but at least
        //     does not allocate additional memory.
        if window_start > 0 {
            let to_remove = input[window_start - 1];
            let to_add = input[window_end - 1];

            let idx = window.binary_search(&to_remove).unwrap();
            window[idx] = to_add;
        }
        window.sort_unstable();

        let mut idx_left = 0;
        let mut idx_right = window.len() - 1;
        let mut sum = 0;

        while idx_left < idx_right {
            let smallest = window[idx_left];
            let largest = window[idx_right];

            // The smallest number in the range is larger than the expected sum,
            // thus the sum of any 2 numbers in the window will always be larger
            // than the expected value
            if smallest > search_key {
                return Some(search_key);
            }

            // The largest element is smaller than half of the expected sum,
            // thus the sum of any 2 numbers in the windows will always be
            // smaller than the expected sum
            if largest < (search_key >> 1) {
                return Some(search_key);
            }

            sum = smallest + largest;
            if sum == search_key {
                break;
            } else if sum > search_key {
                idx_right -= 1;
            } else {
                idx_left += 1;
            }
        }

        if search_key != sum {
            return Some(search_key);
        }

        window_start += 1;
        window_end += 1;
    }

    None
}
