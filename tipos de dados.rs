Scalar Types

    Length	    Signed	Unsigned
    8-bit	    i8	    u8
    16-bit	    i16	    u16
    32-bit	    i32	    u32
    64-bit	    i64	    u64
    128-bit	    i128	u128
    arch	    isize	usize


    Number literals	    Example
    Decimal	            98_222
    Hex     	        0xff
    Octal	            0o77
    Binary	            0b1111_0000
    Byte                (u8 only)	b'A'            

    fn main() {
        let x = 2.0; // f64
    
        let y: f32 = 3.0; // f32
    }
    

    fn main() {
        // addition
        let sum = 5 + 10;
    
        // subtraction
        let difference = 95.5 - 4.3;
    
        // multiplication
        let product = 4 * 30;
    
        // division
        let quotient = 56.7 / 32.2;
        let floored = 2 / 3; // Results in 0
    
        // remainder
        let remainder = 43 % 5;
    }
    

BOOLEAN

    fn main() {
        let t = true;
    
        let f: bool = false; // with explicit type annotation
    }

    
    fn main() {
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';
    }
    

COMPUND TYPES

    fn main() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    }
    

    fn main() {
        let tup = (500, 6.4, 1);
    
        let (x, y, z) = tup;
    
        println!("The value of y is: {y}");
    }

    fn main() {
        let x: (i32, f64, u8) = (500, 6.4, 1);
    
        let five_hundred = x.0;
    
        let six_point_four = x.1;
    
        let one = x.2;
    }
    
    The Array Type

    fn main() {
        let a = [1, 2, 3, 4, 5];
    }
    
    #![allow(unused)]
    fn main() {
    let months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];
    }


    use std::io;

    fn main() {
        let a = [1, 2, 3, 4, 5];

        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!("The value of the element at index {index} is: {element}");
    }
