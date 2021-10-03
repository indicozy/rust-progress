# Notes
## Integers
So, 98_222 is 98222

Signed numbers are stored using (two’s complement)[https://en.wikipedia.org/wiki/Two%27s_complement] representation.

Decimal	        98_222
Hex	        0xff
Octal	        0o77
Binary	        0b1111_0000
Byte (u8 only)	b'A'

## Integer Overflow
When debugging, compiler can panic, but in release it doesnt.

To explicitly handle the possibility of overflow, you can use these families of methods that the standard library provides on primitive numeric types:

* Wrap in all modes with the wrapping_\* methods, such as wrapping_add
* Return the None value if there is overflow with the checked_\* methods
* Return the value and a boolean indicating whether there was overflow with the overflowing_\* methods
Saturate at the value's minimum or maximum values with saturating_\* methods

## Floating
Usually it stores as f64, that's it

## Operations
Better to look at the [link](https://doc.rust-lang.org/book/appendix-02-operators.html)

## Chars
``` Rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```
Rust can store Unicode Scalar values, more in the [Chapter 8](https://doc.rust-lang.org/book/ch08-02-strings.html#storing-utf-8-encoded-text-with-strings)

## Compound Types
### Tuple
Just look the code:
``` Rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

### Array
* Arrays are useful when you want your data allocated on the stack rather than the heap.
* If you’re unsure whether to use an array or a vector, you should probably use a vector.

```
let a: [i32; 5] = [1, 2, 3, 4, 5]; // for different values
let a = [3; 5]; // for same values
let a = [3, 3, 3, 3, 3]; // same as previous
```

## Note
* TODO Learn on how floating numbers work
* "Arrays are useful when you want your data allocated on the stack rather than the heap" - Learn what that means
