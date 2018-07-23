/*
Given a list of integers,
use a vector and return the mean (the average value),
median (when sorted, the value in the middle position),
and mode (the value that occurs most often; a hash map will be helpful here) of the list.



Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/

pub fn get_mean(vector: &Vec<i64>) -> f64 {
    let sum: i64 = vector.iter().sum();
    return sum as f64 / vector.len() as f64;
}

fn is_odd(n: &u64) -> bool {
    if *n == 0 { return false; };
    return *n % 2 == 1;
}

pub fn get_median(vector: &Vec<i64>) -> f64 {
    if vector.len() == 1 { return vector[0] as f64; }
    let _vector_length = vector.len() as u64;
    let mut new_vector = vector.to_vec();
    new_vector.sort();

    if is_odd(&_vector_length) {
        return new_vector[(vector.len() / 2) as usize] as f64;
    } else {
        let sum = (new_vector[(vector.len() / 2 - 1) as usize] + new_vector[vector.len() / 2 as usize]) as f64;
        return (sum / 2.0) as f64;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mean() {
        assert_eq!(get_mean(&vec![10, 4]), 7.0);
        assert_eq!(get_mean(&vec![2, 2]), 2.0);
        assert_eq!(get_mean(&vec![2, 2, 3, 4, 5, 6, 7, 100]), 16.125);
    }

    #[test]
    fn test_get_is_odd() {
        assert_eq!(is_odd(&1), true);
        assert_eq!(is_odd(&0), false);
        assert_eq!(is_odd(&9), true);
        assert_eq!(is_odd(&90), false);
    }

    #[test]
    fn test_get_median() {
        assert_eq!(get_median(&vec![10, 4, 3]), 4.0);
        assert_eq!(get_median(&vec![10, 4, 3, 5, 6]), 5.0);

        assert_eq!(get_median(&vec![10]), 10.0);
        assert_eq!(get_median(&vec![10, 4]), 7.0);
        assert_eq!(get_median(&vec![10, 4, 22, 3]), 7.0);
        assert_eq!(get_median(&vec![9, 22, 23, 1]), 15.5);
    }
}
