mod heap;

use heap::Heap;
use std::io::{self, Write};

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn get_number_input<T: std::str::FromStr>(prompt: &str) -> Option<T> {
    let input = get_user_input(prompt);
    input.parse::<T>().ok()
}

fn display_menu() -> Option<u32> {
    println!("\nD-Heap Operations:");
    println!("1. Build heap");
    println!("2. Change D");
    println!("3. Extract Max");
    println!("4. Insert");
    println!("5. Print heap");
    println!("6. Exit");

    get_number_input("Enter your choice: ")
}

fn build_heap() -> Option<Heap> {
    match get_number_input::<u32>("Enter D value: ") {
        Some(d) if d >= 2 => {
            let input = get_user_input("Enter numbers separated by spaces: ");
            let numbers: Vec<i32> = input
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            let heap = Heap::new(d, &numbers);
            println!("Heap built successfully!");
            heap.print();
            Some(heap)
        }
        Some(_) => {
            println!("D must be at least 2.");
            None
        }
        None => {
            println!("Invalid input for D.");
            None
        }
    }
}

fn change_d(heap: &mut Heap) {
    match get_number_input::<u32>("Enter new D value: ") {
        Some(d) if d >= 1 => {
            heap.change_d(d);
            println!("D value changed successfully!");
            println!("New heap: ");
            heap.print()
        }
        Some(_) => println!("D must be at least 1."),
        None => println!("Invalid input for D."),
    }
}

fn extract_max(heap: &mut Heap) {
    match heap.extract_max() {
        Ok(max) => {
            println!("Maximum value: {}", max);
            println!("New heap: ");
            heap.print()
        }
        Err(e) => println!("Error extracting max: {:?}", e),
    }
}

fn insert_value(heap: &mut Heap) {
    match get_number_input::<i32>("Enter a number to insert: ") {
        Some(num) => match heap.insert(num) {
            Ok(_) => {
                println!("Successfully inserted {}", num);
                println!("New heap: ");
                heap.print()
            }
            Err(e) => println!("Failed to insert: {:?}", e),
        },
        None => println!("Invalid number."),
    }
}

fn print_heap(heap: &Heap) {
    heap.print();
}

fn main() {
    let mut heap: Option<Heap> = None;

    loop {
        match display_menu() {
            Some(1) => {
                heap = build_heap();
            }
            Some(2) => {
                if let Some(ref mut h) = heap {
                    change_d(h);
                } else {
                    println!("No heap exists. Please build a heap first.");
                }
            }
            Some(3) => {
                if let Some(ref mut h) = heap {
                    extract_max(h);
                } else {
                    println!("No heap exists. Please build a heap first.");
                }
            }
            Some(4) => {
                if let Some(ref mut h) = heap {
                    insert_value(h);
                } else {
                    println!("No heap exists. Please build a heap first.");
                }
            }
            Some(5) => {
                if let Some(ref h) = heap {
                    print_heap(h);
                } else {
                    println!("No heap exists. Please build a heap first.");
                }
            }
            Some(6) => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please enter a number between 1 and 6."),
        }
    }
}
