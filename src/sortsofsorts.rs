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

pub fn insertion_sort<T>(arr: &mut Vec<T>)
where
    T: Ord + Eq + Copy,
{
    for j in 1..arr.len() {
        let mut key = j;
        while key > 0 && arr[key - 1] > arr[key] {
            arr.swap(key - 1, key);
            key -= 1;
        }
    }
}

pub fn bubble_sort<T>(arr: &mut Vec<T>)
where
    T: Ord + Eq + Copy,
{
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
            //there hasn't been any swap
            break;
        }
    }
}

pub fn binary_search(arr: &Vec<String>, element: &String) -> Option<String> {
    //O(log n) execution time in the worst case
    //performs well with large data
    assert!(arr.len() > 0);
    let mut low: usize = 0;
    let mut high: usize = arr.len();
    let mut mid: usize = (high - low) / 2;

    while mid > 0 {
        mid = (high - low) / 2;
        if *element == *arr[mid] {
            //the element is the guessed String
            return Some(arr[mid].to_string());
        } else if *element > arr[mid] {
            low = mid - 1;
        } else {
            high = mid + 1;
        }
    }
    None
}

pub fn factorial_recursion(x: i64) -> i64 {
    //Using recursion to calculate the factorial
    //Each time you calculate a number, a new call to the factorial_recursion is pushed to the
    //stack
    assert!(x > 0);
    if x == 1 {
        return 1;
    } else {
        x * factorial_recursion(x - 1)
    }
}

pub fn pancake_sort(arr: &mut Vec<i32>) {
    //very similar to the selection sort (same execution time -> O(n^2))
    //selects the maximum element of the array and puts it on the right side
    //of the array, while reducing the array length.
    //when the size goes down to 1, all the array is sorted ouwhen the size goes down to 1, all the
    //array is sorted out
    let mut size = arr.len() - 1;
    let mut max = 0;
    let mut index = 0;

    for _ in 0..size {
        for x in 0..size {
            if arr[x] > max {
                max = arr[x];
                index = x;
            }
        }

        arr.swap(index, size);
        max = 0;
        if size == 1 {
            break;
        }
        size -= 1;
    }
}
