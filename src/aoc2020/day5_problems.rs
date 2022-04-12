
#[allow(dead_code)]
mod day5 {

    pub fn char_to_binary_digit(c: &char) -> i32 {
        match c {
            'B' => 1,
            'R' => 1,
            'F' => 0,
            'L' => 0,
            _ => panic!("badData"),
        }
    }

    pub fn process_line(s: Vec<char>) -> i32 {
        s.iter()
            .map(|x| char_to_binary_digit(x))
            .fold(0, |acc, digit| (acc << 1) + digit)
    }

    pub fn prob1_from_string(s: &str) -> Option<i32> {
        s.lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .map(|x| process_line(x.to_vec()))
            .max()
    }

    pub fn prob2_from_string(s: &str) -> Option<i32> {
        let xx =s.lines()
            .map(|x| x.chars().collect::<Vec<char>>())
            .map(|x| process_line(x.to_vec()))
            .collect::<Vec<i32>>();
        
        for n in 8 .. (1 << 10)-8
        {
            let i = n as i32;            
            if xx.contains(&(i-1)) && !xx.contains(&i) && xx.contains(&(i+1)) {
                return Some(n as i32);
            }
        }             
        None
    }
}

#[cfg(test)]
mod day5_tests {
    use crate::aoc2020::aoc1::aocCommon::*;
    use crate::aoc2020::day5_problems::day5::*;
    use crate::aoc2020::aoc1::aocCommonFile::*;

    #[test]
    fn test_char_with_sample() {
        let input = vec!['B', 'F', 'F', 'F', 'B', 'B', 'F', 'R', 'R', 'R'];
        let result = process_line(input);
        assert_eq!(result, 567);
    }

    #[test]
    fn test_single_char_b_with_sample() {
        let input = 'B';
        let result = char_to_binary_digit(&input);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_single_char_r_with_sample() {
        let input = 'R';
        let result = char_to_binary_digit(&input);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_single_char_f_with_sample() {
        let input = 'F';
        let result = char_to_binary_digit(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_single_char_l_with_sample() {
        let input = 'L';
        let result = char_to_binary_digit(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_char_with_sample_string() {
        let input: Vec<_> = "BFFFBBFRRR".chars().collect();
        let result = process_line(input);
        assert_eq!(result, 567);
    }

    #[test]
    fn test_char_with_sample_line2() {
        let input = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";
        let result = prob1_from_string(input);
        assert_eq!(result, Some(820));
    }

    #[test]
    fn test_prob1_with_sample_file() {
        let input = load_input_lines(AocYear::Aoc2020,AocDay::Day5, DataFileType::SampleData);
        let result = prob1_from_string(&input);
        assert_eq!(result, Some(820));
    }

    #[test]
    fn test_prob1_with_data_file() {
        let input = load_input_lines(AocYear::Aoc2020,AocDay::Day5, DataFileType::Data);
        let result = prob1_from_string(&input);
        assert_eq!(result, Some(998));
    }

    #[test]
    fn test_prob2_with_data_file() {
        let input = load_input_lines(AocYear::Aoc2020,AocDay::Day5, DataFileType::Data);
        let result = prob2_from_string(&input);
        assert_eq!(result, Some(676));
    }
}

#[cfg(test)]
mod day5_prob2_tests {
    use crate::aoc2020::aoc1::aocCommon::*;
    use crate::aoc2020::day5_problems::day5::*;
    use crate::aoc2020::aoc1::aocCommonFile::*;

    #[test]
    fn test_prob2_with_data_file() {
        let input = load_input_lines(AocYear::Aoc2020,AocDay::Day5, DataFileType::Data);
        let result = prob2_from_string(&input);
        assert_eq!(result, Some(676));
    }
}
