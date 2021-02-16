/*
    CIS198 Homework 1
    Part 3: Ownership, move semantics, and lifetimes

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
// #![allow(dead_code)]
// #![allow(unused_variables)]

/*
    Problem 1: Swap ints

    Implement the function that swaps two integers, and write unit tests.

    Make sure you write a test for when x1 and x2 are the same!

    The Rust borrow checker may help avoid some possible bugs.
*/
pub fn swap_ints(x1: &mut i32, x2: &mut i32) {
    if *x1 != *x2 {
        let temp : i32 = *x1;
        *x1 = *x2;
        *x2 = temp;
    }
}

#[test]
fn swap_int_test() {
    let i1 = &mut 1;
    let i2 = &mut 2;
    let i3 = &mut 1;
    swap_ints(i1, i2);
    assert!(*i1 == 2);
    swap_ints(i1, i2);
    assert!(*i1 == 1);
    swap_ints(i1, i3);
    assert!(*i1 == 1);
}

/*
    Problem 2: String duplication
*/
#[test]
fn copy_string_test() {
    let str1 = String::from("foo");
    let str2 = str1.clone();
    assert_eq!(str1, str2);
}
// This test doesn't work. Fix it by copying strings properly.
// Q1. What went wrong?
// They're two different owned values for the same String. The second one shadows the first. This can be changed by copying the value.

// Q2. How come it works fine here?
// Integer types in Rust are Copy by default so the value is copied instead of making the second variable shadow the first.
#[test]
fn copy_int_test() {
    let i1 = 1;
    let i2 = i1;
    assert_eq!(i1, i2);
}

// Now implement the following function that duplicates a string n times.
fn duplicate_string(s: &str, times: usize) -> Vec<String> {
    let mut result = vec![];
    for _i in 1..=times{
        result.push(String::from(s))
    }
    result
}

/*
    Problem 3: String duplication continued

    These two don't work either. Fix by changing the type of "string" in the
    function copy_me ONLY, and by adjusting the parameter to "copy_me" where
    it's called.
*/

fn copy_me(string: /* Change in here only*/ &String) -> String {
    string.clone()
}

#[test]
fn copy_me_test() {
    let str1 = String::from("foo");
    assert_eq!(str1, copy_me(/* Change in here only*/ &str1));
}

#[test]
fn copy_me_test2() {
    let str1 = String::from("foo");
    let str2 = copy_me(&str1 /* Change in here only*/);
    assert_eq!(str1, str2);
}

/*
    Problem 4: Lifetime specifiers

    For each of the following three functions, either implement it by adding
    lifetime specifiers, or explain why this is not possible.

    (It's not truly impossible -- we will see later on that advanced features
    such as "unsafe code" can be used to turn off Rust's safety and lifetime
    checks.)
*/
fn new_ref_string<'a>() -> &'a String {
    unimplemented!();
    // let initial = Box::new(String::new());
    // let result: &'a String = initial.as_ref();
    // result
}

fn new_ref_str<'a>() -> &'a str {
    unimplemented!();
    // let result: &'a String = &String::new();
    // result
}

use part2;
// The same function from part2
fn pick_longest2<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    part2::pick_longest2(s1, s2)
}

/*
    Problem 5: Using functions with lifetimes

    Write two versions of a function which returns the longest string in a
    vector, using pick_longest2 as a helper function.

    You can add lifetime specifiers if necessary.
    What are the pros and cons of v1 and v2?
*/

fn pick_longest_in_v1(v: Vec<String>) -> String {
    if v.len() == 0 {
        return String::new();
    }

    let mut partial_result: &str = "unreachable value";
    let mut started = false;
    let mut temp_string: String;
    for s in &v {
        if !started {
            started = true;
            partial_result = s;
        } else {
            temp_string = part2::pick_longest(partial_result, s);
            partial_result = &temp_string;
        }
    }

    String::from(partial_result)
}

fn pick_longest_in_v2<'a>(v: Vec<&'a str>) -> &'a str {
    if v.len() == 0 {
        unreachable!();
    }

    let mut partial_result: &str = "unreachable value";
    let mut started = false;
    for s in v {
        if !started {
            started = true;
            partial_result = s;
        } else {
            partial_result = part2::pick_longest2(partial_result, s);
        }
    }

    partial_result
}

/*
    Problem 6: Move semantics

    Write three versions of a function that pads a vector with zeros.
    Fail if the vector is larger than the desired length.

    Use .clone() as necessary to make the unit tests compile.

    Which of these functions do you prefer? Which is the most efficient?
*/

fn pad_with_zeros_v1(v: Vec<usize>, desired_len: usize) -> Vec<usize> {
    let mut result = vec![];
    let zeroes_to_pad = desired_len - v.len();
    for i in &v {
        result.push(i.clone())
    }
    for _i in 1..=zeroes_to_pad {
        result.push(0)
    }
    println!("the arr is {:?}", result);
    debug_assert_eq!(result.len(), desired_len);
    result
}

fn pad_with_zeros_v2(slice: &[usize], desired_len: usize) -> Vec<usize> {
    let mut result = vec![];
    let zeroes_to_pad = desired_len - slice.len();
    for i in slice {
        result.push(i.clone())
    }
    for _i in 1..=zeroes_to_pad {
        result.push(0)
    }
    debug_assert_eq!(result.len(), desired_len);
    result
}

fn pad_with_zeros_v3(v: &mut Vec<usize>, desired_len: usize) {
    let zeroes_to_pad = desired_len - v.len();
    for _i in 1..=zeroes_to_pad {
        v.push(0)
    }
    debug_assert_eq!(v.len(), desired_len);
}

#[test]
fn test_pad_twice_v1() {
    let v = vec![1];
    let v = pad_with_zeros_v1(v, 2);
    let v = pad_with_zeros_v1(v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v2() {
    let v = vec![1];
    let v = pad_with_zeros_v2(&v, 2);
    let v = pad_with_zeros_v2(&v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

#[test]
fn test_pad_twice_v3() {
    let mut v = vec![1];
    pad_with_zeros_v3(&mut v, 2);
    pad_with_zeros_v3(&mut v, 4);
    assert_eq!(v, vec![1, 0, 0, 0]);
}

/*
    Problem 7: Move semantics continued

    Write a function which appends a row to a vector of vectors.
    Notice that it takes ownership over the row.
    You shouldn't need to use .clone().

    Why is this more general than being passed a &[bool]
    and cloning it?

    Second, write a function which returns whether
    a row equals the first row in the vector of vectors.
    Notice that it does not take ownership over the row.

    Why is this more general than being passed a Vec<bool>?
*/

fn append_row(grid: &mut Vec<Vec<bool>>, row: Vec<bool>) {
    grid.push(row);
}

fn is_first_row(grid: &[Vec<bool>], row: &[bool]) -> bool {
    // Check if row is the first row in grid
    // Remember to handle the case when grid is empty
    if grid.len() == 0{
        return false;
    }

    let r1 = &grid[0];
    if row.len() != r1.len(){
        return false;
    }
    for (idx, val1) in r1.iter().enumerate() {
        if row[idx] != *val1 {
            return false;
        }
    }
    return true;
}

/*
    Problem 8: Modifying while iterating

    In C and C++, you run into subtle bugs if you try to modify a data
    structure while iterating over it. Rust's move semantics prevents that.
*/

use std::{collections::HashMap, ops::Add, unimplemented};

// To familiarize yourself with HashMaps,
// implement the following function which converts pairs from a slice
// into key-value pairs in a hashmap.
// Documentation:
// https://doc.rust-lang.org/std/collections/struct.HashMap.html

fn vector_to_hashmap(v: &[(i32, String)]) -> HashMap<i32, String> {
    let mut result = HashMap::new();
    for (key, val) in v {
        result.insert(key.clone(), String::from(val));
    }
    return result
}

// Now rewrite this function to delete all entries in hashmap where the keys
// are negative.
fn delete_negative_keys(h: &mut HashMap<i32, i32>) {
    // This fails, uncomment to see error.
    let a = h.keys().map(|k|k.clone()).collect::<Vec<i32>>();
    for k in a {
        if k < 0 {
            h.remove(&k);
        }
    }
}

/*
    Problem 9: The Entry API

    Move semantics present interesting API design choices not found in other
    languages.
    HashMap is an example of such a API.
    Specifically, the Entry API:
    https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html

    This allows for efficient HashMap access because we only access
    the entry in the map (computing an expensive hash function) once.

    Implement a function which does the following:
        For all entries in `add`: (k, v)
        If `k` exists in `merged`, append `v` to the value of `merged[k]`.
        If that `k` doesn't exist in `merged`, add the (k, v) to `merged`.
    Use `or_insert` and `and_modify`.
*/

fn merge_maps(
    merged: &mut HashMap<String, String>,
    add: HashMap<String,String>
) {
    for (k, v) in add {
        if let Some(current) = merged.get_mut(&k){
            for ch in v.chars() {
                current.push(ch);
            }
        } else {
            merged.insert(k, v);
        }
    }
}
