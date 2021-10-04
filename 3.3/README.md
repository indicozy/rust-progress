In function signatures, you **must** declare the type of each parameter. 
Statements do not return values. Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error.
Calling a function is an expression. Calling a macro is an expression. The block that we use to create new scopes, {}, is an expression, for example:
```
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```
Note the x + 1 line without a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value. Keep this in mind as you explore function return values and expressions next.

## Additional info
### Expression vs statement

An “expression” is a combination of values and functions that are combined and interpreted by the compiler to create a new value, as opposed to a “statement” which is just a standalone unit of execution and doesn't return anything.

    The result variable has to be set up outside the statement itself. What initial value should it be set to?
    What if I forget to assign to the result variable in the if statement? The purpose of the “if “statement is purely to have side effects (the assignment to the variables). This means that the statements are potentially buggy, because it would be easy to forget to do an assignment in one branch. And because the assignment was just a side effect, the compiler could not offer any warning. Since the result variable has already been defined in scope, I could easily use it, unaware that it was invalid.
    What is the value of the result variable in the “else” case? In this case, I haven’t specified a value. Did I forget? Is this a potential bug?
    Finally, the reliance on side-effects to get things done means that the statements are not easily usable in another context (for example, extracted for refactoring, or parallelizing) because they have a dependency on a variable that is not part of the statement itself.

Example:
``` c#
// statement-based code
public void IfThenElseStatement(bool aBool)
{
   int result;     //what is the value of result before it is used?
   if (aBool)
   {
      result = 42; //what is the result in the 'else' case?
   }
   Console.WriteLine("result={0}", result);
}
```

``` c#
// expression-based code
public void IfThenElseExpression(bool aBool)
{
    int result = aBool ? 42 : 0; // ? and : are expressions, they create a new value
    Console.WriteLine("result={0}", result);
}
```

## Notes
* TODO: Rust is expression-based language. Upd: I'm stupid
