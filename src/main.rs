mod stack;
mod queue;
use stack::Stack;
use queue::Queue;

fn main() {
    let mut test_stack = Stack::new();
    let mut test_queue = Queue::new();
    test_queue.push("BRW");
    test_stack.push(2);
    test_stack.push(5);
    test_stack.push(3);
    // neat way to print out the structure if attribute debbug is set of course 
    println!("intial {:?}", test_stack);
    println!("size of the stack: {}",test_stack.size());
    match test_stack.peek(){
        Some(value) => println!("Peek stack of stack: {:?}. peek:{}",test_stack, value),
        None => println!("Stack is empty")
    }
    test_stack.pop();
    match test_stack.peek(){
        Some(value) => println!("Peek stack after poping an element {}", value),
        None => println!("Stack is empty")
    }
    println!("size of the stack after poping one element: {}",test_stack.size());
    test_stack.pop();
    test_stack.pop();
    println!("size of the stack after popping all elements: {}",test_stack.size());
    test_stack.push(69);
    match test_stack.peek(){
        Some(value) => println!("Peek stack after adding an element {}", value),
        None => println!("Stack is empty")
    }
    test_stack.pop();
    match test_stack.peek(){
        Some(value) => println!("Peek an empty stack {}", value),
        None => println!("PEEKED Stack should be empty and None match is runeed")
    }

    test_stack.peek();
    println!("Last state of the stack(should be empty by now) {:?}", test_stack);

}
