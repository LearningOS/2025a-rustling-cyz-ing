use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self 
    where
        T: PartialOrd,
    {
        let mut merged_list = LinkedList::new();
        
        // 使用两个指针分别遍历两个链表
        let mut a_ptr = list_a.start;
        let mut b_ptr = list_b.start;
        
        // 当两个链表都还有元素时
        while let (Some(a_node), Some(b_node)) = (a_ptr, b_ptr) {
            let a_val = unsafe { &(*a_node.as_ptr()).val };
            let b_val = unsafe { &(*b_node.as_ptr()).val };
            
            // 比较两个链表的当前节点值，选择较小的节点
            if a_val <= b_val {
                // 从list_a中取出节点
                a_ptr = unsafe { (*a_node.as_ptr()).next };
                list_a.length -= 1;
                // 将节点添加到合并后的链表
                add_node_to_list(&mut merged_list, a_node);
            } else {
                // 从list_b中取出节点
                b_ptr = unsafe { (*b_node.as_ptr()).next };
                list_b.length -= 1;
                // 将节点添加到合并后的链表
                add_node_to_list(&mut merged_list, b_node);
            }
        }
        
        // 如果list_a还有剩余节点
        while let Some(a_node) = a_ptr {
            a_ptr = unsafe { (*a_node.as_ptr()).next };
            list_a.length -= 1;
            add_node_to_list(&mut merged_list, a_node);
        }
        
        // 如果list_b还有剩余节点
        while let Some(b_node) = b_ptr {
            b_ptr = unsafe { (*b_node.as_ptr()).next };
            list_b.length -= 1;
            add_node_to_list(&mut merged_list, b_node);
        }
        
        // 清空原始链表，避免double free
        list_a.start = None;
        list_a.end = None;
        list_a.length = 0;
        list_b.start = None;
        list_b.end = None;
        list_b.length = 0;
        
        merged_list
    }
}

// 辅助函数：将节点添加到链表的末尾
fn add_node_to_list<T>(list: &mut LinkedList<T>, mut node_ptr: NonNull<Node<T>>) {
    unsafe {
        // 将节点的next设为None，因为它是链表的新末尾
        (*node_ptr.as_ptr()).next = None;
        
        match list.end {
            None => {
                // 如果链表为空，设置start和end都指向这个节点
                list.start = Some(node_ptr);
                list.end = Some(node_ptr);
            }
            Some(end_ptr) => {
                // 如果链表不为空，将当前末尾节点的next指向新节点
                (*end_ptr.as_ptr()).next = Some(node_ptr);
                // 更新end指针
                list.end = Some(node_ptr);
            }
        }
        list.length += 1;
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1,3,5,7];
        let vec_b = vec![2,4,6,8];
        let target_vec = vec![1,2,3,4,5,6,7,8];
        
        for i in 0..vec_a.len(){
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len(){
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a,list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len(){
            assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
        }
    }
    
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11,33,44,88,89,90,100];
        let vec_b = vec![1,22,30,45];
        let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

        for i in 0..vec_a.len(){
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len(){
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a,list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len(){
            assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
        }
    }
}