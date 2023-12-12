mod parse;
use nom::{combinator::all_consuming, Finish};
use parse::{parse_input, Coord};

fn main() {
    let v = all_consuming(parse_input)(include_str!("../input.txt"))
        .finish()
        .unwrap()
        .1;
    let mut set = Coord::make_set(v);

    for i in 0..1000 {
        let mut init = Coord { x: 0, y: 0 };
        loop {
            let next = init.get_next_coord(&set);
            if next == init {
                set.insert(next);
                println!("{:?}", next);
                break;
            }
            init = next;
            if init.is_out_of_bounds() {
                break;
            }
        }
        if init.is_out_of_bounds() {
            println!("{}", i);
            break;
        }
    }
}
