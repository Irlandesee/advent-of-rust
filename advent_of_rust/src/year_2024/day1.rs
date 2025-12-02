use std::fs;
use std::path::PathBuf;
pub fn solve_day1(input_file: PathBuf) -> (i32, i32) {
    //Part 1
    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let mut left_vector:Vec<i32> = Vec::new();
    let mut right_vector:Vec<i32> = Vec::new();

    contents.iter()
        .for_each(|vec|
            {
                left_vector.push(vec[0]);
                right_vector.push(vec[1]);
            });

    // contents.iter()
    //     .map(|vec|
    //         {
    //             left_vector.push(vec[0]);
    //             right_vector.push(vec[1]);
    //         })
    //     .collect::<Vec<_>>();

    // for content in contents {
    //     left_vector.push(content[0]);
    //     right_vector.push(content[1]);
    // }


    left_vector.sort();
    right_vector.sort();

    let mut  part_one_sum= 0;
    //We assume both vectors are the same length
    for (index, smallest_left) in left_vector.iter().enumerate() {
        let delta = smallest_left - right_vector[index];
        part_one_sum += delta.abs()
    }

    // Part 2
    let mut similarity_score = 0;

    left_vector.iter().for_each(|x|
        {
            let x_times = right_vector.iter()
                .filter(|y| x == *y) //or x.eq(y)
                .count()
                as i32;

            similarity_score += x * x_times

        });

    (part_one_sum, similarity_score)
}