pub fn selection_sort(input: [i32; 6]) -> [i32; 6] {
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
