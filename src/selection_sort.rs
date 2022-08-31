pub fn selection_sort(input: &Vec<i32>) -> Vec<i32> {
    let mut result = input.clone();
    let size = result.len();
    let mut minimum = 0;

    for step in 0..size {
        for index in step + 1..size {
            if result[index] < result[minimum] {
                minimum = index;
            }
        }
        result.swap(step, minimum);
    }

    result
}
