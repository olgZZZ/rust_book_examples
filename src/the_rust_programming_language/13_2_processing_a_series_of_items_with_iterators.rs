// Processing a Series of Items with Iterators
// https://doc.rust-lang.org/stable/book/ch13-02-iterators.html



// Listing 13-10: Creating an iterator
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
}





// Listing 13-11: Using an iterator in a for loop
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}





// The Iterator Trait and the next Method

// All iterators implement a trait named Iterator that is defined in the standard 
// library. The definition of the trait looks like this:
#![allow(unused)]
fn main() {
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
}




// Listing 13-12: Calling the next method on an iterator
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
}






// Methods that Consume the Iterator

// Listing 13-13: Calling the sum method to get the total of all items in the iterator
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}






// Methods that Produce Other Iterators

// Listing 13-14: Calling the iterator adaptor map to create a new iterator
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
}





/*
However, this code produces a warning:

$ cargo run
   Compiling iterators v0.1.0 (file:///projects/iterators)
warning: unused `Map` that must be used
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: iterators are lazy and do nothing unless consumed

warning: `iterators` (bin "iterators") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/iterators`

*/





// Listing 13-15: Calling the map method to create a new iterator and then calling 
// the collect method to consume the new iterator and create a vector
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}








// Using Closures that Capture Their Environment

/*
In Listing 13-16, we use filter with a closure that captures the shoe_size variable from its environment to iterate over a collection of Shoe struct instances. It will return only shoes that are the specified size.

Filename: src/lib.rs
Listing 13-16: Using the filter method with a closure that captures shoe_size
*/
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}






