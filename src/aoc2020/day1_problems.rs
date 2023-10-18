mod day1 {
    #[allow(dead_code)]
    pub fn prob_2numbers(v: Vec<i32>) -> Result<i32, &'static str> {
        for x in 0..v.len() - 1 {
            for y in x..v.len() - 1 {
                let i = v[x];
                let j = v[y];
                if i + j == 2020 {
                    return Ok(i * j);
                }
            }
        }
        Err("not found")
    }

    #[allow(dead_code)]
    pub fn prob_3numbers(v: Vec<i32>) -> Result<i32, &'static str> {
        for x in 0..v.len() - 1 {
            for y in x..v.len() - 1 {
                for z in y..v.len() - 1 {
                    let i = v[x];
                    let j = v[y];
                    let k = v[z];
                    if i + j + k == 2020 {
                        return Ok(i * j * k);
                    }
                }
            }
        }
        Err("not found")
    }
}

#[cfg(test)]
mod day1_tests {
    use crate::aoc2020::aoc1::aocCommon::AocDay::Day1;
    use crate::aoc2020::aoc1::aocCommon::*;
    use crate::aoc2020::aoc1::aocCommonFile::*;
    use crate::aoc2020::day1_problems::day1::*;
    #[test]
    fn test_2_numbers_problem_with_sample() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let result = prob_2numbers(input);
        assert_eq!(result, Ok(514579));
    }

    #[test]
    fn test_3_numbers_problem_with_sample() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        let result = prob_3numbers(input);
        assert_eq!(result, Ok(241861950));
    }

    #[test]
    fn test_2_numbers_problem_with_data_file() {
        fn process_file() -> Result<i32, &'static str> {
            let input: Vec<_> = load_input_lines(AocYear::Aoc2020, Day1, DataFileType::Data)
                .lines()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            prob_2numbers(input)
        }

        let result = process_file();
        assert_eq!(result, Ok(877971));
    }
}

#[cfg(test)]
mod day1Async_tests {
    use crate::aoc2020::aoc1::aocCommon::AocDay::Day1;
    use crate::aoc2020::aoc1::aocCommon::*;
    use crate::aoc2020::aoc1::aocCommonFileAsync::*;
    use crate::aoc2020::day1_problems::day1::*;

    #[actix_rt::test]
    async fn test_2_numbers_problem_with_data_file_async() {
        async fn process_file() -> Result<i32, &'static str> {
            let file_content = load_input_lines_async(AocYear::Aoc2020, Day1, DataFileType::Data)
                .await
                .map_err(|_x| "file load failed")?;
            let input: Vec<_> = file_content
                .lines()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            prob_2numbers(input)
        }
        assert_eq!(process_file().await, Ok(877971));
    }
}
