fn divide(whole: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let size = whole.len();
    let halfway = size / 2;

    let first_half = whole[0..halfway].to_vec();
    let second_half = whole[halfway..size].to_vec();

    (first_half, second_half)
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut index_left = 0;
    let mut index_right = 0;

    while index_left < left.len() && index_right < right.len() {
        if left[index_left] < right[index_right] {
            result.push(left[index_left]);
            index_left = index_left + 1;
        } else {
            result.push(right[index_right]);
            index_right = index_right + 1;
        }
    }

    if index_left < left.len() {
        while index_left < left.len() {
            result.push(left[index_left]);
            index_left += index_left + 1;
        }
    }

    if index_right < right.len() {
        while index_right < right.len() {
            result.push(right[index_right]);
            index_right = index_right + 1;
        }
    }

    result
}

pub fn merge_sort(input: &Vec<i32>) -> Vec<i32> {
    if input.len() == 1 {
        return input.to_owned();
    }

    let (first_half, second_half) = divide(&input);

    let first_half = merge_sort(&first_half);
    let second_half = merge_sort(&second_half);

    merge(first_half, second_half)
}
