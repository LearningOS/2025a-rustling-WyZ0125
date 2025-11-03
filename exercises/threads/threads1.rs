// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.


use std::thread;
use std::time::{Duration, Instant};

//thread::spawn创建子进程后不会等待在for i in 0..10 {代码块，直接handles.push(handle); 
//然后在let thread_result = handle.join().unwrap();再等

fn main() {
    let mut handles = vec![];
    // 2. 循环创建 10 个线程（i 从 0 到 9）
    for i in 0..10 {
        // thread::spawn: 创建并启动一个新线程，参数是一个闭包（线程要执行的逻辑）
        // move 关键字：将循环变量 i 的所有权“转移”到闭包中（线程需要独立拥有 i，否则主线程循环会提前释放 i）
        let handle = thread::spawn(move || {
            // 记录线程开始执行的时间点（用于后续计算运行时长）
            let start = Instant::now();
            
            // 让线程休眠 250 毫秒（模拟线程执行任务的耗时，确保每个线程至少运行 250ms）
            thread::sleep(Duration::from_millis(250));
            
            // 打印线程完成的提示（线程调度由操作系统决定，打印顺序可能和创建顺序不一致）
            println!("thread {} is complete", i);
            
            // 计算线程从启动到当前的耗时（as_millis() 转为毫秒数，返回 u128 类型）
            // 这个值会作为线程的“返回值”，传递给主线程
            start.elapsed().as_millis()
        });
        // 将当前线程的句柄（JoinHandle）存入 handles 向量，方便后续管理
        handles.push(handle);
    }

/*你的理解存在一个关键偏差：handles.push(handle) 存入的是 “线程句柄”，而非 “线程的执行结果”—— 子线程在 thread::spawn 被调用时就已启动，
但它的执行速度与主线程无关，主线程循环创建完 10 个线程后，子线程大概率还在休眠（250ms），并未完成。join() 阻塞的核心目的，就是等待这些 “未完成的子线程” 执行结束，再获取结果。
先理清一个关键时间线：主线程与子线程的执行顺序
假设主线程执行速度极快，我们拆解整个流程的时间线（忽略代码执行的微小耗时）：

主线程循环创建线程（0~1ms）：主线程用 for i in 0..10 循环 10 次，每次调用 thread::spawn：
子线程被创建并立即启动（开始执行闭包内的逻辑：记录 start → 休眠 250ms → 打印 → 返回时间）；
主线程拿到子线程的 JoinHandle（句柄），立刻存入 handles 向量，继续下一次循环。
循环结束时（约 1ms 后），10 个子线程都处于 “休眠中”（还在执行 thread::sleep(Duration::from_millis(250))），完全没完成。 

主线程开始遍历 handles（1ms 时）：此时 handles 里存的是 10 个 “正在运行的子线程的句柄”，而非 “已完成的结果”。主线程执行 for handle in handles 时，子线程还在休眠，根本没生成返回值。

join() 阻塞的必要性（1ms ~ 251ms）：当主线程执行 handle.join() 时，发现对应的子线程还在运行（休眠中），于是 主线程暂停执行（阻塞），直到这个子线程完成休眠、打印信息、生成返回值。
只有等子线程完全执行结束后，join() 才能从子线程中拿到返回值（运行时间），再通过 unwrap() 提取，存入 results。 */

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        // 调用 join() 等待线程完成，获取返回值（unwrap() 处理 Result，假设线程不恐慌）
        let thread_result = handle.join().unwrap();
        // 将返回值存入 results 向量
        results.push(thread_result);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
