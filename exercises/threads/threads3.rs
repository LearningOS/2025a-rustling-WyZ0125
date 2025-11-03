// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

/*一、整体功能概述
本代码通过 MPSC（多生产者单消费者）通道 和 Arc（原子引用计数） 实现多线程协作：
主线程创建一个包含 10 个数字的 Queue，并初始化 MPSC 通道；
启动两个子线程，通过 Arc 共享 Queue，分别发送 first_half（1-5）和 second_half（6-10）的数字；
主线程作为消费者，接收所有子线程发送的数据，最终验证接收总数是否等于 Queue.length（10），确保数据无丢失。 */

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);// 用 Arc 包装 Queue，生成共享指针
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    // 关键1：给第二个子线程克隆发送端 tx（避免重复 move）
    let tx2 = tx.clone();
//核心问题：mpsc::Sender（发送端）不实现 Copy 特性，只能 move 一次；若两个子线程都用同一个 tx，会导致编译错误；

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    // 关键2：克隆发送端 tx_clone 传入 send_tx，原 tx 留在主线程关闭
    let tx_clone = tx.clone();
    send_tx(queue, tx_clone);

    // 关键3：关闭主线程的原 tx，确保所有发送端关闭后通道能通知接收端退出
    drop(tx);
/*核心原理：MPSC 通道的接收端（rx）会一直等待数据，只有当所有发送端都被关闭后，for received in rx 循环才会退出；
关闭逻辑：
主线程通过 drop(tx) 手动关闭原发送端；
子线程发送完数据后，其持有的 tx/tx2 会随线程结束自动 drop（关闭）；
所有发送端关闭 → 接收端循环退出 → 主线程继续执行后续断言。 */

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
