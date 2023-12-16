pub fn parse(input: &str) -> Vec<[[u16; 4]; 4]> {
    let mut blueprints = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_ascii_whitespace();

        // ore bots cost ore
        let ore_bot_costs = [iter.nth(6).unwrap().parse().unwrap(), 0, 0, 0];
        // clay bots cost ore
        let clay_bot_costs = [iter.nth(5).unwrap().parse().unwrap(), 0, 0, 0];
        // obsidian bots cost ore and clay
        let obsidian_bot_costs = [
            iter.nth(5).unwrap().parse().unwrap(),
            iter.nth(2).unwrap().parse().unwrap(),
            0,
            0,
        ];
        // geode bots cost ore and obsidian
        let geode_bot_costs = [
            iter.nth(5).unwrap().parse().unwrap(),
            0,
            iter.nth(2).unwrap().parse().unwrap(),
            0,
        ];

        let blueprint = [
            ore_bot_costs,
            clay_bot_costs,
            obsidian_bot_costs,
            geode_bot_costs,
        ];
        blueprints.push(blueprint);
    }

    blueprints
}
