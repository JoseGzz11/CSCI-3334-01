// Implements a function is_even(n: i32) -> bool
// that returns true if a number is even, false otherwise.
fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        true
    } else {
        false
    }
}

fn main() {
    // Creates an array of 10 integer numbers of your choice
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Uses a for loop to iterate through the array and for each number
    for element in numbers.iter() {
        // Prints whether it's even or odd using your is_even function
        if is_even(*element) == true {
            println!("{} Even", element);
        }
        // If the number is divisible by 3, print "Fizz" instead
        else if element % 3 == 0 {
            println!("{} Fizz", element);
        }
        // If the number is divisible by 5, print "Buzz" instead
        else if element % 5 == 0 {
            println!("{} Buzz", element);
        }
        // If it's divisible by both 3 and 5, print "FizzBuzz"
        else if element % 5 == 0 && element % 3 == 0 {
            println!("{} FizzBuzz", element);
        }
    }

    // Use a while loop to find and print the sum of all numbers in the array.
    let mut index = 0;
    let mut sum = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Addition of all the number in the array: {}", sum);

    // Use a loop to find and print the largest number in the array.
    let mut largestnum = numbers[0];
    for &number in numbers.iter() {
        if largestnum < number {
            largestnum = number;
        }
    }
    println!("The largest number in the array is: {}", largestnum);
}
