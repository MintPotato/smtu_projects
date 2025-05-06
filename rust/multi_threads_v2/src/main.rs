use multi_threads_v2::ThreadPool;
use std::sync::{Arc, Mutex};

fn main() {
    let nums: Vec<i32> = (1..=1000).collect();
    let cluster_size = 100;

    let pool = ThreadPool::build(5).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(1);
    });
    let sum = Arc::new(Mutex::new(0));

    for cluster in nums.chunks(cluster_size) {
        let sum = Arc::clone(&sum);
        let cluster = cluster.to_vec();

        pool.execute(move || {
            let cluster_sum: i32 = cluster.iter().sum();
            
            *sum.lock().unwrap() += cluster_sum;
            println!("----\n{:#?}\n----", cluster_sum);
        });
    }
    
    // println!("----\n{:#?}\n----", sum);
    drop(pool);
    println!("----\n{:#?}\n----", sum);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_wo_drop() {
        let nums: Vec<i32> = (1..=1000).collect();
        let cluster_size = 100;

        let pool = ThreadPool::build(5).unwrap();
        let sum = Arc::new(Mutex::new(0));

        for cluster in nums.chunks(cluster_size) {
            let sum = Arc::clone(&sum);
            let cluster = cluster.to_vec();

            pool.execute(move || {
                let cluster_sum: i32 = cluster.iter().sum();
                
                *sum.lock().unwrap() += cluster_sum;
                println!("----\n{:#?}\n----", cluster_sum);
            });
        }
        
        assert_ne!(*sum.lock().unwrap(), 500500);
    }

    #[test]
    pub fn test_w_drop() {
        let nums: Vec<i32> = (1..=1000).collect();
        let cluster_size = 100;

        let pool = ThreadPool::build(5).unwrap();
        let sum = Arc::new(Mutex::new(0));

        for cluster in nums.chunks(cluster_size) {
            let sum = Arc::clone(&sum);
            let cluster = cluster.to_vec();

            pool.execute(move || {
                let cluster_sum: i32 = cluster.iter().sum();
                
                *sum.lock().unwrap() += cluster_sum;
                println!("----\n{:#?}\n----", cluster_sum);
            });
        }
        drop(pool);
        assert_eq!(*sum.lock().unwrap(), 500500);
    }

    #[test]
    pub fn test_w_zero_size() {
        let pool = ThreadPool::build(0).unwrap_or_else(|e| {
            println!("Goida");
            return ThreadPool::new(1); // так надо
        });
    }

    #[test]
    pub fn test_w_time() {
        let time = std::time::Instant::now();

        let nums: Vec<i32> = (1..=1000).collect();
        let cluster_size = 100;

        let pool = ThreadPool::build(5).unwrap();
        let sum = Arc::new(Mutex::new(0));

        for cluster in nums.chunks(cluster_size) {
            let sum = Arc::clone(&sum);
            let cluster = cluster.to_vec();

            pool.execute(move || {
                let cluster_sum: i32 = cluster.iter().sum();
                
                *sum.lock().unwrap() += cluster_sum;
            });
        }
        drop(pool);

        let time = time.elapsed();
        println!("Time elapsed {:#?}", time);

    }
}