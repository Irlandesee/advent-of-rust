use std::fs;
fn main() {
    //let test_file_path= String::from("inputs/test_input");
    let file_input = String::from("inputs/puzzle_input");
    let contents = fs::read_to_string(file_input)
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

    let mut sum = 0;
    //We assume both vectors are the same length
    for (index, smallest_left) in left_vector.iter().enumerate() {
        let delta = smallest_left - right_vector[index];
        sum += delta.abs()
    }

    println!("Part 1: {}", sum);

}
