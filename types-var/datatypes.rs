/*
  Fundamental Datatypes and Variables.
*/

use std::mem; // Import statement. std::mem is imported to use size_of_val in this example.

fn main() { // The main function is the entry to our code.

  // let is used to bind variable(memory location) with value.
  
  let a:u8 = 123; // 8bit value. variable = a & u = unsigned (0 ... 2555).
  println!("a = {}", a); // We use {} just like we use & in C. println!() is the function used to print.
  
  // Variable binding is immutable by default. To make it mutable, use keyword 'mut' keyword.
  
  {
    let mut b:i8 = 0; // b is mutable.
    println!("b = {}", b);
    b = 43;
    println!("b = {}", b);
  }

  let mut c = 123456789; // 32-bit signed i32
  println!("c = {}, size = {} bytes", c, mem::size_of_val(&c)); // size_of_val() is used to find the size of a variable.
  
  // Different datatypes: i8 -> signed integer(8bit), u8 -> unsigned integer(8bit). Bits can be increased to 16, 32, 64..

  let z:isize = 123; // (isize/usize) -> itegeral datatypes. Size of the memory address.
  // 32-bit systems have 32-bit variables & 64-bit systems have 64-bit variables.
  // If you have an unused variable you can prefix the variable name with a '_'.

  // Characters

  let d:char = 'a'; // character variable. We don't have to use char keyword.
  println!("d = {}, size = {} bytes", d, mem::size_of_val(&d)); // size of a char is 4 bytes in rust.
  
  // Floating point numbers.

  let e:f32 = 2.5; // double-precision, f64 type by default.
  println!("e = {}, size = {} bytes", e, mem::size_of_val(&e)); // size of e = 8 bytes.
  
  // Boolean types. (true/false)
  {
    let g = false; // g is false.
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g)); // size of g = 1 byte.
    let f:bool = 4 > 0; // different way to assign boolean value to a variable. bool keyword is optional. Here f = true.
    println!("f = {}", f);
  }
  
}
