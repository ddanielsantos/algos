pub fn bubble_sort(input: [i32; 6]) -> [i32; 6] {
    let mut result = input.clone();

    for _ in 0..result.len() - 1 {
        for index in 0..(result.len() - 1) {
            if result[index] > result[index + 1] {
                result.swap(index, index + 1);
            }
        }
    }

    result
}
