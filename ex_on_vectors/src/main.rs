// I wanto to solve the following problem that is mention in
// The Rust Progamming Language Book, chapter 8
// Given a list of integers, use a vector and return
// the median (when sorted, the value inthe middle position) and 
//mode (the value that occurs most often; a hash map will behelpful here) of the list.

use std::collections::{HashMap, HashSet};

fn main() {
    let mut numbers = vec![1,1,1,2,2,2,3,3,8,8];
    let median = compute_median(&mut numbers);
    match median {
        Some(value) => println!("The median is: {}", value),
        None => println!("The  vector is empty, so there is no median."),
    }
    let modes=compute_mode(&mut numbers);
    match modes {
        Some(value) => println!("The modes are {:?}",value),
        None => println!("The vector is empty,  so therre is no mode"),        
    }
}



// THe next function, given a vector gives us the median.

fn compute_median(vec: &mut Vec<i32>) -> Option<f64> {
    vec.sort();
    let len = vec.len();
    if len == 0 {
        None
    } else if len % 2 ==0 {
        let median_1 = vec[len/2-1];
        let median_2 = vec[len/2];
        Some((median_1 as f64 + median_2 as f64) / 2.0)
    } else {
        Some(vec[(len-1)/2] as f64)
    }

}


// Code to determine the mode of a list of numbers

fn compute_mode(vec: &Vec<i32>) -> Option<HashSet<i32>> {
    let len = vec.len();
    if len == 0 {
        None
    } else {
        let mut frequency = HashMap::new();
    
        //count the frequency of each number
        for &i in vec.iter() {
            let count = frequency.entry(i).or_insert(0);
            *count += 1;
        }
        println!("{:?}",frequency);

        let mut m=0; //current maximum frequency
        let mut modes =  HashSet::new();

        for (&key, &value) in &frequency {
            if value > m {
                m = value;
                modes.clear(); //clear the previous modes
                modes.insert(key);
            } else if value ==m {
                modes.insert(key);          
            }
        }
        Some(modes)
    }
}

