//use algorithms_exercises::insertionsort::insertion_sort;
use algorithms_exercises::sortsofsorts::binary_search;
use std::vec;

fn main() {
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
