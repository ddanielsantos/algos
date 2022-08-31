mod bubble_sort;
mod selection_sort;

fn main() {
    let original_array = [4, 9, 2, 1, 7, 8];
    let bubble_result = bubble_sort::bubble_sort(original_array);
    let selection_result = selection_sort::selection_sort(original_array);

    println!("Original array: {:?}", original_array);
    println!("Bubble sort:    {:?}", bubble_result);
    println!("Selection sort: {:?}", selection_result);
}
