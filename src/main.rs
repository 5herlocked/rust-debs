use std::sync::mpsc;
use std::thread;

fn main() {

    let (tx, rx) = mpsc::channel();

    let find = 4;
    let array = [1, 2, 3, 4, 6, 7, 8, 9, 10];

    // Use Thread::spawn to spawn a bunch of workers
    // move array into closure
    // open a channel of communication that returns a struct
    // index, found before mid index, OR found after mid index
    // switch case to print which one is which

    thread::spawn(move || {

    });
}

// Return a tuple of first and last occurrences or none
fn binary_search(array: &[isize], key: isize, mut low: usize, mut high: usize) -> Option<(usize, usize)> {
    while low <= high {
        let middle = (low + high) / 2;
        if array[middle] > key {
            high = middle - 1;
        }
        else if array[middle] < key {
            low = middle - 1;
        }
        else if array[middle] == key {
            let last_occurrence = find_last_index(array, key, middle, high);
            let first_occurrence = find_first_index(array, key, middle, low);

            [last_occurrence, first_occurrence]
        }
    }

    None
}

// Return the index or none
fn find_last_index(array: &[isize], key: isize, mut middle: usize, mut high: usize) -> Option<usize> {
    let mut low = middle;
    if array[high] == key {
        high
    }
    while low <= high {
        middle = (low + high) / 2;
        if array[middle] > key && A[middle + 1] == key {
            middle - 1
        }
        else if array[middle] == key {
            low = middle + 1;
        }
        else if array[middle] > key && array[middle - 1] != key {
            high = middle - 1;
        }
    }

    None
}

// Return the index or none
fn find_first_index(array: &[isize], key: isize, mut middle: usize, mut low: usize) -> Option<usize> {
    let mut high = middle - 1;
    if array[low] == key {
        low
    }
    while low <= high {
        middle = (low + high) / 2;
        if array[middle] < key && array[middle + 1] == key {
            middle + 1
        }
        else if array[middle] == key {
            high = middle - 1;
        }
        else if array[middle] < key && array[middle] != key {
            low = middle + 1;
        }
    }

    None
}
