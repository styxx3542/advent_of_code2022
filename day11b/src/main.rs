use nom::{combinator::all_consuming, Finish};
mod parse;
use parse::{parse_all_monkeys, Monkey};

fn turn(mut monkey: Monkey, monkeys: &mut Vec<Monkey>, divisor_product: u64) -> Monkey {
    for mut item in monkey.items.into_iter() {
        item %= divisor_product;
        monkey.items_inspected += 1;
        let val = monkey.operation.eval(item);
        if val % monkey.divisor == 0 {
            monkeys[monkey.receiver_if_true].items.push(val);
        } else {
            monkeys[monkey.receiver_if_false].items.push(val);
        }
    }
    monkey.items = Vec::new();
    monkey
}
fn main() {
    let mut monkeys = all_consuming(parse_all_monkeys)(include_str!("../input.txt"))
        .finish()
        .unwrap()
        .1;
    let divisor_product = monkeys.iter().map(|m| m.divisor).product::<u64>();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            monkeys[i] = turn(monkeys[i].clone(), &mut monkeys, divisor_product);
        }
    }
    monkeys.sort_by_key(|monkey| monkey.items_inspected);
    let val =
        monkeys[monkeys.len() - 1].items_inspected * monkeys[monkeys.len() - 2].items_inspected;
    println!("{val}");
}
