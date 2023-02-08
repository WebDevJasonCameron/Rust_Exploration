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
/*
fn main() {
    let letter = 'a';
    let number = '1';
    let finger = '\u{261d}';
    println!("{letter}\n{number}\n{finger}")
}
 */

// Challange
/*
fn main() {
    let a = 13.0;
    let b = 2.3;
    let c: f64 = 120.0;
    let average: f64 = (a as f64 + b as f64 + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed!");
}
*/

// Ex 09
/*
fn main() {
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    let first_letter = letters[0];
    println!("first_letter is {first_letter}");

    let numbers: [i32; 5];
    numbers = [0; 5];
    let index: usize = numbers.len();
    println!("last number is {}", numbers[index]);
}
*/

// Ex 10  Arrays
/*
fn main() {
    let parking_lot = [[1, 2, 3], [4, 5, 6]];

    let number = parking_lot[0][1];

    println!("number is {number}")

    let garage = [[[0; 100]; 20]; 5];
}
*/

// Ex 11 Tuple
/*
fn main() {
    let mut stuff: (u8, f32, char) = (10, 3.14, 'x'); // don't need to use the colon and assignments
    stuff.0 += 3;
    let first_item = stuff.0;
    println!("first_item is {first_item}");

    let (a, b, c) = stuff;
    println!("b is {b}");
}
*/

// Ex 12 Functions
/*
fn main() {
    say_hello();
    say_number(13);
    let x = 1;
    let y = 2;
    say_the_sum(x, y);
    say_number(x as i32);
}

fn say_hello() {
    println!("Hello!");
}

fn say_number(number: i32) {
    println!("numbers is {number}");
}

fn say_the_sum(a: u8, b: u8) {
    let sum = a + b;
    println!("sum is {sum}");
}

*/

// Ex 12
/*
fn main() {
    let result = square(13);
    println!("result is {:?}", result);
    println!("result is {} and {}", result.0, result.1);
}

fn square(x: i32) -> (i32, i32) {
    println!("squaring {x}");
    return (x, x * x);
}
*/

// Challenge
/*
fn main() {
    let celsius_temp = 23.0;
    let fahrenheit_temp = clesius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!")
}

fn clesius_to_fahrenheit(temp: f64) -> f64 {
    temp * 1.8 + 32.0
}
*/

// Ex 13
/*
fn main() {
    let x = 4;

    if x + 1 != 3 {
        println!("x + 1 is NOT 3!")
    }

    let a = 3;
    let b = 5;

    if a > b {
        println!("a is greater than b");
    } else {
        if a < b {
            println!("a is less than b");
        } else {
            println!("a is equal to b");
        }
    }

    if a > b {
        println!("a is greater than b");
    } else if a < b {
        println!("a is less than b");
    } else {
        println!("a is equal to b");
    }
}
 */

// Ex 14    If statements cont
/*
fn main() {
    let make_x_odd = true;
    let x = if make_x_odd { 1 } else { 2 };

    /*
    if make_x_odd {
        x = 1;
    } else {
        x = 2;
    } */

    println!("x is {x}");
}
*/

// Ex 15     loops

/*
fn main() {
    let mut count = 0;

    let result = loop {
        if count >= 10 {
            break count * 10;
        }
        count += 1;
        println!("count is {count}")
    };

    println!("The result is {result}")
}
 */

// while loop
/*
fn main() {
    let mut count = 0;
    let letters = ['a', 'b', 'c'];

    while count < letters.len() {
        println!("letter is {}", letters[count]);
        count += 1;
    }
}
*/

// for loop
/*
fn main() {
    let message = ['h', 'e', 'l', 'l', 'o'];

    for item in message {
        println!("item is {item}")
    }

    let message2 = ['h', 'e', 'l', 'l', 'o'];

    for (index, &item) in message2.iter().enumerate() {
        println!("item is {item}, and index is {index}");
        if item == 'e' {
            break;
        }
    }

    for number in 0..5 {
        println!("number is {number}");
    }
}
*/

// Ex 16   loops inside loops
/*
fn main() {
    let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
            print!("{num}\t");
        }
        println!();
    }
}
*/

// Challenge
/*
fn main() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    max = numbers[0];
    min = numbers[0];
    mean = 0.0;

    /* My Code */
    for num in numbers {
        if num > max {
            max = num;
        } else if num < min {
            min = num;
        }
        mean += num as f64;
    }

    mean /= numbers.len() as f64;

    println!("max is {max}");
    println!("min is {min}");
    println!("mean is {mean}");

    /*assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    */
}
*/

// Ex 17        Scope
/*
fn main() {

    let planet = "Earth";
    if true {
        println!("planet is {planet}");
    }
    println!("planet is {planet}");


    // Shadowing a var

    let planet = "Earth";
    println!("planet is {planet}");
    let planet = "Mars";
    println!("planet is {planet}");

}
 */
fn main() {
    let mut message = String::from("Earth");
    println!("message is {message}");
    message.push_str(" is home");
    println!("message {message}");
}
