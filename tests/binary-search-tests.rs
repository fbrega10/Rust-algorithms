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
    assert_eq!(String::from("Bibi"), x)
}

pub fn binary_search_test2() {
    let mut array = vec![
        String::from("Kicking"),
        String::from("Unquestionable"),
        String::from("Joker-hey"),
        String::from("Molly"),
        String::from("Tea"),
        String::from("Memory-a"),
        String::from("Uncomfortable"),
        String::from("Verysure"),
        String::from("Hahaha"),
        String::from("Haaha"),
        String::from("Hasha"),
        String::from("Hasha"),
        String::from("Questionable"),
    ];

    array.sort();
    let array = array;

    let x: Option<String> = match binary_search(&array, &String::from("Verysure")) {
        Some(x) => Some(x),
        None => None,
    };
    assert_eq!(String::from("Verysure"), x.unwrap());

    let x: Option<String> = match binary_search(&array, &String::from("Kicking")) {
        Some(x) => Some(x),
        None => None,
    };

    assert_eq!(String::from("Kicking"), x.unwrap());

    let x: Option<String> = match binary_search(&array, &String::from("Not present yet")) {
        Some(x) => Some(x),
        None => None,
    };
    assert_ne!(x.unwrap(), String::from("Not present yet"))
}
