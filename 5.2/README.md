# Main

Pretty interesting.

In the struct Rectangle:

``` rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

We can see that it cannot display information `to println!()`. However, we can create our own function `std::fmt::Display`, learn that later. 

Also, we can explicitly make debugging with structs if we add annotation `#[derive(Debug)]` just before the struct definition:

``` rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

## Notes
* TODO: Learn how we can take information from tuple
* TODO: Learn how to make Display function
