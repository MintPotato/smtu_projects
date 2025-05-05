use multi_threads_v2::ThreadPool;
use std::{thread};

fn main() {
    let fibbonaci_vec: Vec<u32> = vec![1, 1, 2];
    let pool = ThreadPool::new(3);

}

fn fibbonaci(mut prev_nums: Vec<u32>) -> Vec<u32>{
    return prev_nums
}