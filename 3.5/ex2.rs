fn main(){
    let x = fibo(10);
    println!("{}", x);
}
fn fibo(x: i32) -> i32 {
    if x <= 1 {return x}
    fibo(x-1) + fibo(x-2)
}
