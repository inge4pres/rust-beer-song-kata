pub fn verse(n: u32) -> String {
    format!("{} of beer on the wall, {} of beer.
{}, {} of beer on the wall.
", bottles(n), bottles(n).to_lowercase(), next(n), end(n))
}

pub fn sing(start: u32, end: u32) -> String {
    let mut ret = String::new();
    for i in (end..=start).rev() {
        ret.push_str(&verse(i));
        if i > end {
            ret.push('\n');
        }
    }
    ret
}

fn bottles(beers: u32) -> String {
    match beers {
        0 => String::from("No more bottles"),
        1 => String::from("1 bottle"),
        _ => format!("{} bottles", beers)
    }
}

fn next(beers: u32) -> String {
    match beers {
        0 => String::from("Go to the store and buy some more"),
        1 => String::from("Take it down and pass it around"),
        _ => String::from("Take one down and pass it around")
    }
}

fn end(beers: u32) -> String {
    match beers {
        0 => bottles(99).to_lowercase(),
        _ => bottles(beers - 1).to_lowercase()
    }
}









