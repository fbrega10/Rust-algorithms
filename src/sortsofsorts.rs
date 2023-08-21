//MIT License

//Copyright (c) 2023 fbrega10

//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.

pub fn insertion_sort(arr: &mut Vec<i32>) {
    for j in 1..arr.len() {
        let mut key = j;
        while key > 0 && arr[key - 1] > arr[key] {
            arr.swap(key - 1, key);
            key -= 1;
        }
    }
}

pub fn bubble_sort(arr: &mut Vec<i32>) {
    assert!(arr.len() > 0);
    for _ in 0..arr.len() {
        let mut flag = false;
        for j in 0..(arr.len() - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                flag = true;
            }
        }
        if flag == false {
            //there has been no reordering
            break;
        }
    }
}

pub fn binary_search(arr: &Vec<String>, element: &String) -> Option<String> {
    assert!(arr.len() > 0);
    let mut low: usize = 0;
    let mut high: usize = arr.len();
    let mut mid: usize = (high - low) / 2;

    while mid > 0 {
        mid = (high - low) / 2;
        if *element == *arr[mid] {
            println!("found the element guessed: {}", arr[mid]);
            return Some(arr[mid].to_string());
        } else if *element > arr[mid] {
            low = mid - 1;
        } else {
            high = mid + 1;
        }
    }
    None
}
