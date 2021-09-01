use std::collections::HashMap;

pub fn mean(nums : &Vec<i32>) -> f64 {
    let sum: i32 = nums.iter().sum();
    sum as f64 / nums.len() as f64
}


pub fn median(nums : &Vec<i32>) -> f64 {

    let mut nums_sort = nums.clone();
    nums_sort.sort();

    let length = nums_sort.len();
    let mid = length/2;

    if length % 2 == 0 {
        // 정의는 평균 값이라고 하는데.....
        mean(&vec![nums[mid-1], nums[mid]])
    } else {
        nums[mid] as f64
    }
}

pub fn mode(nums : &Vec<i32>) -> i32 {

    let mut map = HashMap::new();
    for num in nums.iter() {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut ck = -1;
    let mut cv = -1;

    for (k, v) in map.iter() {

        //println!("key: {}, val: {}", k, v);
        if cv < *v {
            //println!("cv>*v cv: {}, v: {}", cv, v);
            ck = **k;
            cv = *v;
        }
    }

    ck
}