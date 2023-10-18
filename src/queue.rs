#[derive(Debug)]
pub struct Queue<T> {
    // define an generic field vector, T stands for Type.
    // later it can be Stack<u32>
    items: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { items: Vec::new() }
    }

    pub fn push(&mut self, item:T) {
        self.items.push(item)
    }
    pub fn enqueue(&self) -> Option<T>{
       unimplemented!() 
    }

    pub fn peek(&self) -> Option<&T>{
        unimplemented!()
    }

    pub fn is_empty(&self) -> bool{
        self.items.is_empty()
    }
    
    pub fn size(&self) -> usize{
        self.items.len()
    }
    
}

pub struct Dequeue<T> {
    items: Vec<T>,
}

impl<T> Dequeue<T> {
    pub fn new() -> Self {
        Dequeue { items: Vec::new() }
    }

    pub fn push(&mut self, item:T) {
        self.items.push(item)
    }
    pub fn enqueue(&self) -> Option<T>{
       unimplemented!() 
    }

    pub fn peek(&self) -> Option<&T>{
        unimplemented!()
    }

    pub fn is_empty(&self) -> bool{
        self.items.is_empty()
    }
    
    pub fn size(&self) -> usize{
        self.items.len()
    }
    
}

