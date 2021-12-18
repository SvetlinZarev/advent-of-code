use crate::{Number, Numeric};

pub fn part_one(input: &[Number]) -> Numeric {
    assert!(!input.is_empty());
    sum_numbers(input).magnitude()
}

fn sum_numbers(numbers: &[Number]) -> Number {
    assert!(!numbers.is_empty());

    let mut sum = numbers[0].clone();

    for n in numbers.iter().skip(1) {
        sum.add(n.clone());
        sum.reduce();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_shared::input::load_text_input_from_file;
    use aoc_shared::parsing::parse_line_delimited;
    use std::str::FromStr;

    const SAMPLE_1: &str = r#"
        [1,1]
        [2,2]
        [3,3]
        [4,4]
    "#;
    const SAMPLE_1_SUM: &'static str = "[[[[1,1],[2,2]],[3,3]],[4,4]]";

    const SAMPLE_2: &str = r#"
        [1,1]
        [2,2]
        [3,3]
        [4,4]
        [5,5]
    "#;
    const SAMPLE_2_SUM: &'static str = "[[[[3,0],[5,3]],[4,4]],[5,5]]";

    const SAMPLE_3: &str = r#"
        [1,1]
        [2,2]
        [3,3]
        [4,4]
        [5,5]
    "#;
    const SAMPLE_3_SUM: &'static str = "[[[[3,0],[5,3]],[4,4]],[5,5]]";

    const SAMPLE_4: &str = r#"
        [1,1]
        [2,2]
        [3,3]
        [4,4]
        [5,5]
        [6,6]
    "#;
    const SAMPLE_4_SUM: &'static str = "[[[[5,0],[7,4]],[5,5]],[6,6]]";

    const SAMPLE_5: &str = r#"
        [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
        [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
        [7,[5,[[3,8],[1,4]]]]
        [[2,[2,2]],[8,[8,1]]]
        [2,9]
        [1,[[[9,3],9],[[9,0],[0,7]]]]
        [[[5,[7,4]],7],1]
        [[[[4,2],2],6],[8,7]]
    "#;
    const SAMPLE_5_SUM: &'static str = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

    const SAMPLE_6: &str = r#"
        [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
        [[[5,[2,8]],4],[5,[[9,9],0]]]
        [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
        [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
        [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
        [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
        [[[[5,4],[7,7]],8],[[8,3],8]]
        [[9,3],[[9,9],[6,[4,9]]]]
        [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
        [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    "#;
    const SAMPLE_6_SUM: &'static str =
        "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]";

    #[test]
    fn test_sum_1() {
        verify_sum(SAMPLE_1, SAMPLE_1_SUM);
    }

    #[test]
    fn test_sum_2() {
        verify_sum(SAMPLE_2, SAMPLE_2_SUM);
    }

    #[test]
    fn test_sum_3() {
        verify_sum(SAMPLE_3, SAMPLE_3_SUM);
    }

    #[test]
    fn test_sum_4() {
        verify_sum(SAMPLE_4, SAMPLE_4_SUM);
    }

    #[test]
    fn test_sum_5() {
        verify_sum(SAMPLE_5, SAMPLE_5_SUM);
    }

    #[test]
    fn test_sum_6() {
        verify_sum(SAMPLE_6, SAMPLE_6_SUM);
    }

    fn verify_sum(nums: &str, sum: &str) {
        let input = parse_line_delimited(nums);
        let actual = sum_numbers(&input);
        let expected = Number::from_str(sum).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let input = parse_line_delimited(load_text_input_from_file("inputs/input.txt"));
        let answer = part_one(&input);
        assert_eq!(3486, answer);
    }
}
