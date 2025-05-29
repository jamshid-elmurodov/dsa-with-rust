struct MinStack { stack: Vec<(i32, i32)> }

impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }
    
    fn push(&mut self, val: i32) {  
        if self.stack.is_empty(){
            self.stack.push((val, val));
        } else {
            self.stack.push((val, self.stack.last().unwrap().1.min(val)));
        }
    }
    
    fn pop(&mut self) {
        self.stack.pop();
    }
    
    fn top(&self) -> i32 {
        self.stack.last().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.stack.last().unwrap().1
    }
}