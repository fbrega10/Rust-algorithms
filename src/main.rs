//use algorithms_exercises::insertionsort::insertion_sort;
use algorithms_exercises::sortsofsorts::binary_search;
use std::vec;

fn main() {
    //let mut array = vec![879, 78, 65, 2, 7849, 4789];

    //println!("unordered array -> {:?}", array);
    //insertion_sort(&mut array);
    //for i in 1..array.len() {
    //if array[i] < array[i - 1] {
    //panic!("this was not expected, algorithm reordering error!");
    //}
    //}
    //println!("ordered array -> {:?}", array);

    let array = vec![
        String::from("Apple"),
        String::from("Bibi"),
        String::from("Circus"),
        String::from("Circuse"),
        String::from("Hahaha"),
        String::from("Haaha"),
        String::from("Hasha"),
        String::from("Joker"),
        String::from("Molly"),
        String::from("Tea"),
    ];

    let x: String = match binary_search(&array, &String::from("Haaha")) {
        Some(x) => {
            println!("Element found");
            x
        }
        None => {
            println!("not found");
            String::from("None")
        }
    };
    println!("{}", x);
}
