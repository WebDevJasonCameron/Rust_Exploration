// EX 04
/*
fn main() {
    let x: f32 = 10.1234567891234567;
    println!("x is {}", x);
}
*/

// Ex 05
/*
// Odd ways to numbers...
fn main() {
    let a = 10;
    let b = 3;
    let c = a / b;
    println!("c is {}", c)
}
*/

// Ex 06
/*
fn main() {
    let a = 10.0;
    let b = 3.0;
    let c = a / b;
    println!("c is {0:08.3}\na is {1}\nonce again, c is {0}", c, a);
    println!("We can also do this...");
    println!("c is {c:08.3}\na is {a}");
}
*/

// Ex 07
/*
fn main() {
    let mut value = 0b1111_0101u8;
    println!("value is {value}");
    println!("value is {value:08b}");

    value = !value;
    println!("value is {value:08b}");
    println!("value is {value}");

    value = value & 0b1111_0111;
    println!("value is {value:08b}");
    println!("bit 6 is {}", value & 0b0100_0000);

    value = value | 0b0100_0000;
    println!("value is {value:08b}");

    value = value ^ 0b0101_0101;
    println!("value is {value:08b}");

    value = value << 4;
    println!("value is {value:08b}");

    value = value >> 2;
    println!("value is {value:08b}");
}
*/
