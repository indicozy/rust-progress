# Main
## if-else
    It’s also worth noting that the condition in this code must be a bool. If the condition isn’t a bool, we’ll get an error.

There's no `elif` as in Python, use `else if`...

**Important**

    Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code. Chapter 6 describes a powerful Rust branching construct called match for these cases.


### if in a let statement
    Because if is an expression, we can use it on the right side of a let statement:
```
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

We can use `loops` too:
```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

## loops
We can use `loop` and `while index < 5` as usual. We can use for too:
```
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

    The safety and conciseness of for loops make them the most commonly used loop construct in Rust. 

We can use range in `for` loops:
```
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

## Exercises

* Convert temperatures between Fahrenheit and Celsius.
* Generate the nth Fibonacci number.
* Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.


