mod const_strings;
use const_strings::{LESS_20_NAMES, TENS_NAMES};

fn string_composer (num: usize) -> Option<String> {
    
    let ret_string =match num {
        // If is up to one hundred return None
        101.. => return None,
        // If num divisor of 3 and 5 fizzbuzz
        x if x%3 == 0 && x%5 == 0 => "fizzbuzz".to_string(),
        // If divisor of 3 fizz
        x if x%3 == 0 => "fizz".to_string(),
        // If divisor of 5 buzz, this manage too the tens, because 
        // all are divisors of 5
        x if x%5 == 0 => "buzz".to_string(),
        // If less than 20 take the string from the array
        1..=19 => LESS_20_NAMES[num-1].to_string(),
        // If greater than 20 compose the appropiate ten with the less than 20 name
        20..=99 => format!("{} {}", TENS_NAMES[num/10-1], LESS_20_NAMES[num%10-1]),
        // If something is not matching return None
        _ => return None
    };
    // Put the ! sign at the end
    Some(ret_string+"!")
}

// Imperative way to loop
fn imperative_concat(num: usize, to_num: usize) -> Option<String> {
    // Initialize a mutable and empty string
    let mut acum_string = "".to_string();
    
    for i in num..=to_num {
        // In each iteration concatenate the acumulated_string with the actual
        acum_string = format!("{}\n{} is {}", acum_string, i, string_composer(i)?);
    }
    Some(acum_string)
}

// Recursive way to loop
fn recursive_concat(num: usize, to_num: usize) -> Option<String> {
    // If num reached to_num then return the value of num and end the loop
    if num >= to_num {
        return Some(format!("{} is {}", num, string_composer(num)?));
    }
    // If not concatenate the num result with the next one
    Some(format!("{} is {}\n{}", num, string_composer(num)?, recursive_concat(num+1, to_num)?))
}

fn main() {

    println!("{}", recursive_concat(1, 100).unwrap_or("ERROR".to_string()));
    println!("{}", imperative_concat(1, 100).unwrap_or("ERROR".to_string()));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_string_composer_out_of_range() {
        string_composer(300).unwrap();
    }

    #[test]
    fn test_string_composer_test_gt_eq_than_20() {
        for i in 20..=100 {
            let return_string = string_composer(i).unwrap();
            let expected_string = match i {
                x if x%3==0 && x%5==0 => format!("fizzbuzz"),
                x if x%3==0 => format!("fizz"),
                x if x%5==0 => format!("buzz"),
                _ => format!("{} {}", TENS_NAMES[i/10-1], LESS_20_NAMES[i%10-1]),
            };

            println!("{} is {}", i, return_string);
            assert_eq!(return_string, format!("{}!", expected_string));
        }
    }

    #[test]
    fn test_string_composer_less_than_20 () {
        for i in 1..=19 {
            let return_string = string_composer(i).unwrap();
            let expected_string = match i {
                x if x%3==0 && x%5==0 => "fizzbuzz",
                x if x%3==0 => "fizz",
                x if x%5==0 => "buzz",
                _ => LESS_20_NAMES[i-1]
            }.to_string();

            println!("{} is {}", i, return_string);
            assert_eq!(return_string, format!("{}!", expected_string));
        }
    }

    #[test]
    fn test_recursive_concat () {
        assert_eq!(recursive_concat(90, 91), Some(String::from("90 is fizzbuzz!\n91 is ninety one!")));
    }

    #[test]
    fn test_imperative_concat () {
        assert_eq!(recursive_concat(90, 91), Some(String::from("90 is fizzbuzz!\n91 is ninety one!")));
    }
}