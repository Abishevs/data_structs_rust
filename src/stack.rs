#[derive(Debug)]
pub struct Stack<T> {
    // define an generic field vector, T stands for Type.
    // later it can be Stack<u32>
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self{
        Stack { items: Vec::new()}
    }

    pub fn push(&mut self, item:T){
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T>{
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T>{
        if self.is_empty(){
            None
        }else {
            Some(&self.items[&self.size() - 1])
        }
    }

    pub fn is_empty(&self) -> bool{
        self.items.is_empty()
    }
    
    pub fn size(&self) -> usize{
        // vector returns usize type which is platform specific, 64 on 64 bit-platform
        self.items.len()
    }

}
