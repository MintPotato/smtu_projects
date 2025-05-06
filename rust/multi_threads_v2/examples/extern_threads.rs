extern crate threads_pool;
use std::sync::{Arc, Mutex};

fn main() {
    let time = std::time::Instant::now();

        let nums: Vec<i32> = (1..=1000).collect();
        let cluster_size = 100;

        let pool = threads_pool::ThreadPool::new(5);
        let sum = Arc::new(Mutex::new(0));

        for cluster in nums.chunks(cluster_size) {
            let sum = Arc::clone(&sum);
            let cluster = cluster.to_vec();

            let _ = pool.execute(move || {
                let cluster_sum: i32 = cluster.iter().sum();
                
                *sum.lock().unwrap() += cluster_sum;
            });
        }
        drop(pool);

        let time = time.elapsed();
        println!("Time elapsed {:#?}", time);
}