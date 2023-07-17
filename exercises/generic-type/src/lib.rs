use std::cmp::Ordering;

// Exercise 1 
// Implement struct Point to make it work.
// Make it compile
fn exercise1() {
    let integer: Point<i32,i32> = Point { x: 5, y: 10 };
    let float: Point<f64,f64> = Point { x: 1.0, y: 4.0 };
}



// Exercise 2
// Modify this struct to make the code work
// Make it compile
struct Point<T,U> {
    x: T,
    y: U,
}

fn exercise2() {
    // DON'T modify this code.
    let p = Point{x: 5, y : "hello".to_string()};
}



// Exercise 3
// Make it compile
// Add generic for Val to make the code work, DON'T modify the code in `main`.
struct Val <T> {
    val: T,
}

impl Val<f64> {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl Val<String> {
    fn value(&self) -> &String {
        &self.val
    }
}


fn exercise3() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}

// Exercise 4
// Find the maximum value in a collection
// Make it compile
// Implementing logic
// Run tests
fn find_max<T:Ord>(collection: &[T]) -> Option<&T> {
    if collection.len() == 0 {  return None; };
    let mut max: &T = &collection[0];
    for i in 1..=collection.len() - 1 {
            if collection[i-1].cmp(&collection[i]) == Ordering::Greater {
            max = &collection[i-1];
            }
            else {
                max = &collection[i];
            }
    };
    return Some(max);
}

// Exercise 5 
// Reverse the elements in a collection
// Make it compile 
// Run tests 
fn reverse_collection<T>(collection: &mut [T]) {
    collection.reverse()    
}


// Exercise 6
// Function to check if a collection contains a specific value
fn contains_value<T:PartialEq>(collection: &[T], value: &T) -> bool {
    collection.contains(value)
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 4
    #[test]
    fn test_find_max_with_numbers() {
        let numbers = vec![1, 5, 3, 8, 2];
        assert_eq!(find_max(&numbers), Some(&8));
    }

    // Test for exercise 4
    #[test]
    fn test_find_max_with_strings() {
        let strings = vec!["apple", "banana", "cherry", "durian"];
        assert_eq!(find_max(&strings), Some(&"durian"));
    }

    // Test for exercise 4
    #[test]
    fn test_find_max_with_empty_collection() {
        let empty: Vec<i32> = Vec::new();
        assert_eq!(find_max(&empty), None);
    }

    // Test for exercise 5
    #[test]
    fn test_reverse_collection_with_numbers() {
        let mut numbers = vec![1, 2, 3, 4, 5];
        reverse_collection(&mut numbers);
        assert_eq!(numbers, vec![5, 4, 3, 2, 1]);
    }

    // Test for exercise 5
    #[test]
    fn test_reverse_collection_with_strings() {
        let mut strings = vec!["apple", "banana", "cherry", "durian"];
        reverse_collection(&mut strings);
        assert_eq!(strings, vec!["durian", "cherry", "banana", "apple"]);
    }

    // Test for exercise 5
    #[test]
    fn test_reverse_collection_with_empty_collection() {
        let mut empty: Vec<i32> = Vec::new();
        reverse_collection(&mut empty);
        assert_eq!(empty, Vec::<i32>::new());
    }

    // Test for exercise 6
    #[test]
    fn test_contains_value_with_numbers() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(contains_value(&numbers, &3), true);
        assert_eq!(contains_value(&numbers, &6), false);
    }

    // Test for exercise 6
    #[test]
    fn test_contains_value_with_strings() {
        let strings = vec!["apple", "banana", "cherry", "durian"];
        assert_eq!(contains_value(&strings, &"banana"), true);
        assert_eq!(contains_value(&strings, &"grape"), false);
    }

    // Test for exercise 6
    #[test]
    fn test_contains_value_with_empty_collection() {
        let empty: Vec<i32> = Vec::new();
        assert_eq!(contains_value(&empty, &5), false);
    }

}
