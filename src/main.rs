use chrono::{offset::Local, Timelike};
use dynfmt::{Format, SimpleCurlyFormat};

fn main() {
    // get local time
    let now = Local::now();

    // debug: show time
    #[cfg(debug_assertions)]
    println!("{now}");

    // get hms
    let hms = [now.hour(), now.minute(), now.second()];

    // map hms to braille
    let hms = hms.iter().map(|i| {
        #[allow(clippy::cast_possible_truncation)]
        to_braille(*i as u8)
    }).collect::<Vec<char>>();

    // debug: show time
    #[cfg(debug_assertions)]
    println!("{hms:?}");

    // format hms
    let s = SimpleCurlyFormat.format(
        "\x1b[37;41m{}\x1b[37;42m{}\x1b[37;44m{}\x1b[0m",
        hms)
        .unwrap();

    // print
    println!("{s}");

    #[cfg(debug_assertions)]
    demo();
}

#[allow(dead_code)]
fn demo() {
    let mut s = String::new();
    for i in 0..16 {
        for j in 0..16 {
            let n: u8 = i*16+j;
            let ch = to_braille(n);
            s.push_str(&format!(
                    "{n:02X} \x1b[37;40m{ch}\x1b[0m "
            ));
        }
        s.push_str("\n\n");
    }
    println!("{s}");
}

fn to_braille(byte: u8) -> char {
    // braille initial character
    const START: u32  = 0x2800;
    // braille pips are not ordered the way we want
    const MAP: [u8;8] = [7,5,4,3,6,2,1,0];

    let mut braille_bytes = START;

    // mutate each of the lower 8 bits individually
    for i in 0..8 {
        let bit: bool = 0 != (byte & (1 << i));
        let map: u32 = 1 << MAP[i as usize];
        braille_bytes |= u32::from(bit) * (map);
    }

    // cast it back into a char
    char::from_u32(braille_bytes)
        .expect("we really hope this works lol")
    // '⣿'
}
