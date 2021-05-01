pub fn verse(n: u32) -> String {
    // Hm, how do one pass the "format-string" to format!, so that we can apply some beautification?!
    match n {
        0 => {return format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");},
        1 => {return format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");},
        2 => {return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle{} of beer on the wall.\n", n, n, n-1, "");}
        n => {return format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle{} of beer on the wall.\n", n, n, n-1, "s");}
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut s = String::new();
    for n in (end..start+1).rev() {
        // Add next verse at the end of the String
        s.push_str(&verse(n));

        // Add a newline between verses
        if n > end {
            s.push_str("\n")
        }
    }
    s
}
