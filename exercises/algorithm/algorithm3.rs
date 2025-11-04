/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where
    T: Ord, // 注意该格式:约束T必须实现Ord trait，支持比较操作（<、>等）
{
    let len = array.len();
    // 外层循环：控制已排序部分的长度（0..len-1，最后一个元素自动有序）
    for i in 0..len - 1 {
        // 假设当前索引i是未排序部分的最小值索引
        let mut min_index = i;
        // 内层循环：从i+1开始，找到未排序部分的最小值索引 不进行交换；
        for j in i + 1..len {
            if array[j] < array[min_index] {
                min_index = j; // 更新最小值索引
            }
        }
        // 将找到的最小值与当前i位置的元素交换（放到已排序部分末尾）
        //交换操作 array.swap(i, min_index) 仅在每轮内层循环结束后执行 1 次，将最小值放到指定位置。
        array.swap(i, min_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}