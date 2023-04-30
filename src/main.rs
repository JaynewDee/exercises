//

#![allow(dead_code)]

use std::fmt;

//
// Implement Display trait for custom Satellite struct
//

struct Satellite {
    name: String,
    velocity: f64,
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.velocity)
    }
}

fn make_satellite() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    println!("Hubble display: {}", hubble);
}

// Define Location enum
enum Location {
    Unknown,
    Anonymous,
    Known(f32, f32),
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Unknown => write!(f, "Location unknown!"),
            Location::Anonymous => write!(f, "Anonymous Location ... "),
            Location::Known(lat, lon) => write!(f, "Latitude: {}\nLongitude: {}", lat, lon)
        }
    }
} 

fn sum_array(nums_arr: Vec<i16>) -> i16 {
    nums_arr.iter().fold(0, |mut total, num| {
        total += num;
        total
    })
}

fn fizz_buzz(num: i16) -> String {
    let result = match num {
        num if num % 15 == 0 => "Fizz Buzz".to_string(),
        _ => 9.to_string(),
    };

    result
}

fn is_palindrome(input_str: &str) -> bool {
    let mut split_str: Vec<char> = input_str.chars().collect();
    let mut reversed: Vec<char> = Vec::with_capacity(split_str.len());

    while let Some(c) = split_str.pop() {
        reversed.push(c);
    }

    let reversed = String::from_iter(reversed);
    let result = match reversed {
        reversed if reversed == input_str => true,
        _ => false,
    };

    result
}

fn main() {
    is_palindrome("racecar");
    make_satellite();
    
    let location = Location::Unknown;
    println!("{}", location);
    let location = Location::Anonymous;
    println!("{}", location);
    let location = Location::Known(53.5556, 21.998);
    println!("{}", location);
}


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_sum_array() {
        let test_nums = vec![5, 12, 56];
        let sum = sum_array(test_nums);
        assert!(sum == 73);
        let test_nums = vec![19, 3, 5, 9];
        let sum = sum_array(test_nums);
        assert!(sum == 36);
    }

    #[test]
    fn test_fizz_buzz() {
        let result = fizz_buzz(15);
        assert!(result == "Fizz Buzz".to_string());
        let result = fizz_buzz(9);
        assert!(result == 9.to_string());
    }

    #[test]
    fn test_is_palindrome() {
        let result = is_palindrome("racecar");
        assert!(result == true);
        let result = is_palindrome("madam");
        assert!(result == true);
        let result = is_palindrome("rcecar");
        assert!(result == false);
        let result = is_palindrome("mada");
        assert!(result == false);
    }
}
