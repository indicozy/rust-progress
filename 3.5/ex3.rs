fn main() {
    let main = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree",
        ];
    let head  = "On the ";
    let tail = " day of Christmas, my true love sent to me";
    let days = [
        "first",
        "second",
        "third",
        "forth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelvth",
        ];
    for i in 0..12{
        println!("{}{}{}", head, days[i], tail);
        for j in 12-i-1..12{
            println!("{}", main[j]);
        }
        println!("");
    }
}
