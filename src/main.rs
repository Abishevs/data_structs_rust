mod stack;
use std::io;
use stack::Stack;

fn matching(open: u8, close: u8) ->bool {
    if close < open {
        return false;
    }
    match close - open {
        2 | 1 => true,
        _ => false,
    }
}

fn main(){
    let mut stack = Stack::new();
    let mut buffer = String::new();
    match io::stdin().read_line(&mut buffer) {
        Ok(n) => {
            for byte in buffer.bytes() {
                // 40 = ( 91 = [ 123 = { 
                if byte == 40 || byte == 91 || byte == 123 { 
                    stack.push(byte)
                } else {
                    match stack.peek() {
                        Some(&peek) => {
                            // check peek against current byte if
                            // current byte - peek is 1 or 2 then they are matching 
                            // ) = 41 ] = 93 and } = 125
                            if matching(peek, byte) {
                                // so then remove it from the stack
                                stack.pop();
                            } 
                    }
                    None => {println!("Contains non bracket symbol byte: {}", byte );
                    break;
                    }

                }
            }
            }
            println!("{n} bytes read");
            println!("{}", buffer);
        }
        Err(error) => println!("error: {error}"),
    }
    // BUG: it has to have matching pairs else it will evalute to true
    if stack.is_empty() {
        // if they all where matching, then yeeeet
        println!("{}", true);
    }else {
        // else baaaad
        println!("{}", false);
    }

    println!("{:?}", stack)
}
