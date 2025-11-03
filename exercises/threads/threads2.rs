// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::{Arc,Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    //let status = Arc::new(JobStatus { jobs_completed: 0 });
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
// 使用 Arc + Mutex 包装共享数据，实现多线程安全访问
    // Arc: 提供原子引用计数，允许数据在多线程间共享所有权
    // Mutex: 提供互斥锁，保证同一时间只有一个线程访问数据
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            // 锁定 Mutex 以获取数据访问权（lock() 返回 Result，用 unwrap() 简化处理）
            let mut status = status_shared.lock().unwrap();
            // 安全更新共享数据（此时其他线程会被阻塞，直到释放锁）
            status.jobs_completed += 1;
            // 离开作用域后，MutexGuard 会自动释放锁
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        // 打印当前已完成的任务数（需要先锁定才能访问）
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
