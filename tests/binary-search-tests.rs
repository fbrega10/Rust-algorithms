use algorithms_exercises::sortsofsorts::binary_search;

#[test]
pub fn binary_search_test() {
    let mut array = vec![
        String::from("Apple"),
        String::from("Joker"),
        String::from("Tea"),
        String::from("Molly"),
        String::from("Bibi"),
        String::from("Circus"),
        String::from("Circuse"),
        String::from("Hahaha"),
        String::from("Haaha"),
        String::from("Hasha"),
    ];

    array.sort();
    let array = array;
    let x: String = match binary_search(&array, &String::from("Bibi")) {
        Some(x) => {
            println!("Element found");
            x
        }
        None => {
            println!("not found");
            String::from("None")
        }
    };
    assert_eq!(String::from("Bibi"), x);
}
