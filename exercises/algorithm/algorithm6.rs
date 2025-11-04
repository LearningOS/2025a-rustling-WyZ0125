/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/
//DFS 的核心是 **“尽可能深地遍历图的分支”**，当无法继续前进时，回溯到上一个节点，选择另一条未探索的路径。
//通常使用递归（或栈）实现，同时需要记录已访问节点避免重复遍历。

use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, 
    //双 Vec（Vec<Vec<usize>>）的原因：邻接表的标准存储结构
//adj: Vec<Vec<usize>> 是图的 邻接表（Adjacency List） 存储方式，
//外层 Vec 和内层 Vec 各司其职，对应 “节点” 和 “节点的邻接关系”，是处理图结构的高效且直观的设计。
}

impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src); 
    }

    // DFS 辅助函数：递归实现深度优先遍历
    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        // 1. 标记当前节点为已访问
        visited.insert(v);
        // 2. 将当前节点加入访问顺序
        visit_order.push(v);

        // 3. 递归遍历所有未访问的邻接节点
        // 遍历当前节点的所有邻接节点（注意：邻接表可能无序，此处按存储顺序遍历）
        for &neighbor in &self.adj[v] {
            // 如果邻接节点未被访问，则递归访问
            if !visited.contains(&neighbor) {
                self.dfs_util(neighbor, visited, visit_order);
            }
        }
    }

    // Perform a depth-first search on the graph, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new(); // 记录已访问节点
        let mut visit_order = Vec::new(); // 记录访问顺序
        self.dfs_util(start, &mut visited, &mut visit_order); // 从 start 节点开始遍历
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); 

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); 
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); 
    }
}

