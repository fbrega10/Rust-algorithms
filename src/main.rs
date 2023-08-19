use algorithms_exercises::insertionsort::insertion_sort;
use std::vec;

fn main() {
    let mut array = vec![879, 78, 65, 2, 7849, 4789];

    println!("unordered array -> {:?}", array);
    insertion_sort(&mut array);
    for i in 1..array.len() {
        if array[i] < array[i - 1] {
            panic!("this was not expected, algorithm reordering error!");
        }
    }
    println!("ordered array -> {:?}", array);
}
