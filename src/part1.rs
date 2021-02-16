/*
    CIS198 Homework 1
    Part 1: Implementing functions

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
// This will result in useful warnings if you missed something.
//#![allow(dead_code)]
//#![allow(unused_variables)]

/*
    Problem 1: Double

    Implement the function that doubles an integer in three different ways.

    What are some differences between them? Can you write unit tests
    which fail (or fail to compile) for some but not others?
    References require derefs in order to use and rewrite their values, otherwise it's a type error.

    Which of the three do you prefer?
    I prefer the in-place unless I need a new value.
*/

pub fn double_v1(n: i32) -> i32 {
    n * 2
}

pub fn double_v2(n: &i32) -> i32 {
    (*n) * 2
}

pub fn double_v3(n: &mut i32) {
    // double n in place
    *n = (*n)*2;
}

// Example unit test (so you can recall the syntax)
#[test]
fn test_double_v1() {
    assert_eq!(double_v1(2), 4);
    assert_eq!(double_v1(-3), -6);
}
#[test]
fn test_double_v2() {
    assert_eq!(double_v2(&2), 4);
    assert_eq!(double_v2(&-3), -6);
}
#[test]
fn test_double_v3() {
    let mut n1 = 2;
    double_v3(&mut n1);
    assert_eq!(n1, 4);
    let mut n2 = -3;
    double_v3(&mut n2);
    assert_eq!(n2, -6);
}

/*
    Problem 2: Integer square root

    Implement the integer square root function: sqrt(n) should return the
    largest m such that m * m <= n. For a 'harder' version, try to do it more
    efficiently than trying every possibility.
*/
pub fn sqrt(n: usize) -> usize {
    if n==0 { return 0 }
    let highest_possible = n/2;
    let mut current_best = 0;
    for i in 1..=highest_possible {
        if i*i <= n {
            current_best = i;
        } else {
            break;
        }
    }
    current_best
}


#[test]
fn test_sqrt() {
    assert!(sqrt(4) == 2);
    assert!(sqrt(3) == 1);
    assert!(sqrt(9) == 3);
    assert!(sqrt(24) == 4);
    assert!(sqrt(26) == 5);
}

// Remember to write unit tests here (and on all future functions)

/*
    Problem 3: Slice sum

    Implement the sum function on slices in two different ways
    (using different for loop patterns).
    Do not use the predefined sum function.
    Also, try to do it without an unnecessary `return` statement at the end --
    Clippy should detect if you mess this up.

    Which of the two ways do you prefer?
    I prefer the first so I don't have to do a deref inside the loop.
*/
pub fn sum_v1(slice: &[i32]) -> i32 {
    // do some initialization...
    let mut sum = 0;
    for &v in slice {
        sum += v;
    }
    sum
}

pub fn sum_v2(slice: &[i32]) -> i32 {
    // do some initialization...
    let mut sum = 0;
    for v in slice {
        sum += *v;
    }
    sum
}

/*
    Problem 4: Unique

    Make unique. Create a new vector which contains each item in the vector
    only once! Much like a set would.
    This doesn't need to be efficient; you can use a for loop.
*/

pub fn unique(slice: &[i32]) -> Vec<i32> {
    let mut new_vec = vec!();
    for value in slice {
        let mut seen = false;
        for old_value in &new_vec {
            if old_value == value {
                seen = true;
                break;
            }
        }
        if seen {
            continue;
        }
        new_vec.push(*value)
    }
    new_vec
}

/*
    Problem 5: Filter

    Return a new vector containing only elements that satisfy `pred`.
    This uses some unfamiliar syntax for the type of pred -- all you need
    to know is that pred is a function from i32 to bool.
*/
pub fn filter(slice: &[i32], pred: impl Fn(i32) -> bool) -> Vec<i32> {
    let mut new_vec = vec!();
    for &value in slice {
        if pred(value) {
            new_vec.push(value)
        }
    }
    new_vec
}

#[test]
fn test_filter() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    assert_eq!(filter(&vec![1, 2, 3, 4, 5, 6], &is_even), vec![2, 4, 6]);
}

/*
    Problem 6: Fibonacci

    Given starting fibonacci numbers n1 and n2, compute a vector of
    length 'out_size'
    where v[i] is the ith fibonacci number.
*/
pub fn fibonacci(n1: i32, n2: i32, out_size: usize) -> Vec<i32> {
    let mut new_vec = vec![n1, n2];
    let mut one_before = n2;
    let mut two_before = n1;
    let mut current;
    for _i in 1..=out_size-2{
        current = one_before+two_before;
        new_vec.push(current);
        two_before = one_before;
        one_before = current;
    }
    new_vec
}

/*
    Problem 7: String concatenation

    Create a function which concats 2 &strs and returns a String,
    and a function which concats 2 Strings and returns a String.

    You may use any standard library function you wish.

    What are some reasons the second function is not efficient?
    It requires two different Strings, which need space in memory to grow, and then makes a whole new String. The first function just makes the one String and is also more general.
*/
pub fn str_concat(s1: &str, s2: &str) -> String {
    let mut b = String::new();
    b += s1;
    b += s2;
    b
}

pub fn string_concat(s1: String, s2: String) -> String {
    s1 + &s2
}

/*
    Problem 8: String concatenation continued

    Convert a Vec<String> into a String.
    Your answer to the previous part may help.
*/

pub fn concat_all(v: Vec<String>) -> String {
    let mut b = String::new();
    for s in v {
        b += &s;
    }
    b
}

/*
    Problem 9: Parsing

    Convert a Vec<String> into a Vec<i32> and vice versa.

    Assume all strings are correct numbers! We will do error handling later.
    Use `.expect("ignoring error")` to ignore Result from parse()
    See https://doc.rust-lang.org/std/primitive.str.html#method.parse

    The unit tests check if your functions are inverses of each other.

    A useful macro: format! is like println! but returns a String.
*/

pub fn parse_all(v: Vec<String>) -> Vec<i32> {
    let mut new_vec = vec![]; 
    for val in v {
        new_vec.push(val.parse().expect("Didn't parse the string correctly"));
    }
    new_vec
}

pub fn print_all(v: Vec<i32>) -> Vec<String> {
    let mut new_vec = vec![]; 
    for val in v {
        new_vec.push(format!("{}", val));
    }
    new_vec
}

#[test]
fn test_print_parse() {
    assert_eq!(parse_all(print_all(vec![1, 2])), vec![1, 2]);
}

#[test]
fn test_parse_print() {
    let v = vec!["1".to_string(), "2".to_string()];
    assert_eq!(print_all(parse_all(v.clone())), v);
}

/*
    Problem 10: Composing functions

    Implement a function which concatenates the even Fibonacci
    numbers out of the first n Fibonacci numbers.

    For example: if n = 6, the first 5 Fibonacci numbers are 1, 1, 2, 3, 5, 8,
    so the function should return the String "28".

    Don't use a for loop! Your previous functions should be sufficient.
*/

pub fn concat_even_fibonaccis(n: usize) -> String {
    let fib_nums = fibonacci(1, 1, n);
    let is_even = |i| i%2 == 0;
    concat_all(print_all(filter(&fib_nums, is_even)))
}

#[test]
fn test_concat_even_fibonaccis() {
    assert_eq!(&concat_even_fibonaccis(6), "28");
    assert_eq!(&concat_even_fibonaccis(9), "2834");
}
