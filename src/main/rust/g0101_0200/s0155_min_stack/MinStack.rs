// #Medium #Top_100_Liked_Questions #Top_Interview_Questions #Stack #Design
// #Data_Structure_II_Day_14_Stack_Queue #Programming_Skills_II_Day_18 #Level_2_Day_16_Design
// #Udemy_Design #Big_O_Time_O(1)_Space_O(N) #2024_09_10_Time_0_ms_(100.00%)_Space_5.9_MB_(71.62%)

pub struct MinStack {
    current_node: Option<Box<Node>>,
}


/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
struct Node {
    min: i32,
    data: i32,
    next_node: Option<Box<Node>>,
    previous_node: Option<Box<Node>>,
}

impl Node {
    fn new(min: i32, data: i32, previous_node: Option<Box<Node>>, next_node: Option<Box<Node>>) -> Self {
        Node {
            min,
            data,
            next_node,
            previous_node,
        }
    }
}

impl MinStack {
    pub fn new() -> Self {
        MinStack { current_node: None }
    }

    pub fn push(&mut self, val: i32) {
        match &self.current_node {
            None => {
                self.current_node = Some(Box::new(Node::new(val, val, None, None)));
            }
            Some(current) => {
                let min = std::cmp::min(current.min, val);
                let previous_node = self.current_node.take();
                self.current_node = Some(Box::new(Node::new(min, val, previous_node, None)));
            }
        }
    }

    pub fn pop(&mut self) {
        if let Some(current) = self.current_node.take() {
            self.current_node = current.previous_node;
        }
    }

    pub fn top(&self) -> i32 {
        if let Some(current) = &self.current_node {
            return current.data;
        }
        panic!("Stack is empty!");
    }

    pub fn get_min(&self) -> i32 {
        if let Some(current) = &self.current_node {
            return current.min;
        }
        panic!("Stack is empty!");
    }
}

/*
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
