# Main
just use slices as references when needed bruh

Knowing that you can take slices of literals and String values leads us to one more improvement on first_word, and that’s its signature:

```
fn first_word(s: &String) -> &str {
```

A more experienced Rustacean would write the signature shown in Listing 4-9 instead because it allows us to use the same function on both &String values and &str values.

```
fn first_word(s: &str) -> &str {
```
