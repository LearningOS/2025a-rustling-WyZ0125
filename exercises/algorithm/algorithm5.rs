/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/


use std::collections::VecDeque;

// 图结构体：用邻接表（Vec<Vec<usize>>）存储图结构
struct Graph {
    adj: Vec<Vec<usize>>, // adj[i] 表示节点 i 的所有邻接节点
}

impl Graph {
    // 创建含 n 个节点的空图（初始无任何边）
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n], // 初始化 n 个空向量，对应 n 个节点的邻接表
        }
    }

    // 给图添加无向边（src 和 dest 互相成为邻接节点）easy
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // BFS 实现：从 start 节点开始遍历，返回节点访问顺序
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // 1. 初始化：记录节点是否已访问（长度=节点数，初始全为 false）
        let node_count = self.adj.len();
        let mut visited = vec![false; node_count];
        
        // 2. 初始化队列：存储待访问的节点（BFS 核心数据结构，用 VecDeque 实现队列）
        let mut queue = VecDeque::new();
        
        // 3. 初始化访问顺序列表：记录最终的访问顺序
        let mut visit_order = vec![];

        // 4. 启动 BFS：处理起始节点
        visited[start] = true; // 标记起始节点为已访问
        queue.push_back(start); // 将起始节点加入队列
        visit_order.push(start); // 起始节点加入访问顺序

        // 5. 循环处理队列：直到队列为空（所有可达节点都已访问）
        while let Some(current_node) = queue.pop_front() {
            // 遍历当前节点的所有邻接节点
            for &neighbor in &self.adj[current_node] {
                // 如果邻接节点未被访问
                if !visited[neighbor] {
                    visited[neighbor] = true; // 标记为已访问
                    queue.push_back(neighbor); // 加入队列，等待后续处理
                    visit_order.push(neighbor); // 加入访问顺序
                }
            }
        }

        visit_order // 返回最终的访问顺序
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

