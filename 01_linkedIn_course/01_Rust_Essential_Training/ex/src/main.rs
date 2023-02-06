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

// Ex 8 part a and b
/*
fn main() {
    let a = true;
    let b = false;

    println!("a is {a} and b is {b}");
    println!("NOT a is {}", !a);
    println!("a AND b is {}", a & b);
    println!("a OR b is {}", a | b);
    println!("a XOR b is {}", a ^ b);

    let c = (a ^ b) && panic!("broke");
    println!("c is {c}");
}
 */
/*
fn main() {
    let a = 1;
    let b = 2;
    println!("a is {} and b is {}", a, b);
    println!("a EQUAL TO  b is {}", a == b);
    println!("a NOT EQUAL TO b is {}", a != b);
    println!("a GREATER THAN b is {}", a > b);
    println!("a GREATER THAN OR EQUAL TO b is {}", a >= b);
    println!("a LESS THAN b is {}", a < b);
    println!("a LESS THAN OR EQUAL TO b is {}", a <= b);
}
 */

// Ex 08
fn main() {
    let letter = 'a';
    let number = '1';
}
