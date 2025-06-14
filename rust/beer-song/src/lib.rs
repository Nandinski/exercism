pub fn verse(n: u32) -> String {
    match n {
        n if n > 2 => format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.", n-1),
        2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.".to_string(), 
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.".to_string(), 
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.".to_string(), 
        _ => unreachable!()
    }
}

pub fn sing(start: u32, end: u32) -> String {
    assert!(
        start >= end,
        "Can only sing from higher number of beers to a lower number"
    );
    let verses: Vec<String> = (end..=start).rev().map(verse).collect();
    verses.join("\n\n")
}
