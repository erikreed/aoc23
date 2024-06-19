use std::collections::HashMap;
use lazy_regex::regex_captures;
use std::error::Error;
use std::fs;

type Item = HashMap<String, i32>;

#[derive(Debug)]
struct RuleSet {
    id: String,
    rules: Vec<Rule>,
    fallback_destination: String,
}

impl RuleSet {
    pub fn parse(line: &str) -> Self {
        let (id, p) = line.split_once('{').unwrap();
        let rules = p[..p.len() - 1].split(',').collect::<Vec<_>>();
        let fallback_destination = rules.last().unwrap();
        let rules = rules[..rules.len() - 1]
            .iter()
            .map(|t| Rule::parse(t).unwrap())
            .collect();

        Self {
            id: id.to_string(),
            rules,
            fallback_destination: fallback_destination.to_string(),
        }
    }
    pub fn item_destination(self: &Self, item: &Item) -> &str {
        for rule in &self.rules {
            let v = *item.get(rule.value_id.as_str()).unwrap();
            match rule.gt {
                true => {
                    if rule.value < v {
                        return rule.success_destination.as_str();
                    }
                }
                false => {
                    if rule.value > v {
                        return rule.success_destination.as_str();
                    }
                }
            }
        }
        return self.fallback_destination.as_str();
    }
}

#[derive(Debug)]
struct Rule {
    value_id: String,
    gt: bool,
    value: i32,
    success_destination: String,
}

impl Rule {
    fn parse(line: &str) -> Result<Self, Box<dyn Error>> {
        // example string: "a<2006:qkq"
        let (_, value_id, gt, value, success_destination)
            = regex_captures!(r"^(\w+)([<>])(\d+):(\w+)$", line).unwrap();
        let value: i32 = value.parse().unwrap();


        let (p1, p2) = line.split_once(':').unwrap();
        Ok(Self {
            value_id: value_id.to_string(),
            gt: gt == ">",
            value,
            success_destination: success_destination.to_string(),
        })
    }
}

fn num_combinations(rules: &HashMap<String, RuleSet>) -> u64 {
    let mut frontier = vec![
        (rules.get("in").unwrap(), HashMap::<&str, (i32, i32)>::new())
    ];
    let mut count = 0;
    while !frontier.is_empty() {
        let (rule, items) = frontier.pop().unwrap();
        for r in &rule.rules {
            match items.get(r.value_id.as_str()) {
                None => {}
                Some(t) => {}
            }
        }
    }
    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string("d19.txt")?;
    let (rules, items) = text.trim().split_once("\n\n").unwrap();

    let rules = rules.split_ascii_whitespace().map(|t| {
        RuleSet::parse(t)
    });

    let rule_lookup: HashMap<String, RuleSet> = HashMap::from_iter(
        rules.map(|r| (r.id.to_string(), r))
    );

    let items: Vec<Item> = items.split_ascii_whitespace().map(|t| {
        HashMap::from_iter(
            t[1..t.len() - 1].split(',').map(|v| {
                let (k, v) = v.split_once('=').unwrap();
                (k.to_string(), v.parse().unwrap())
            })
        )
    }).collect();


    let mut part1_sum = 0;
    let first_rule = rule_lookup.get("in").unwrap();
    for item in &items {
        let mut destination: &str = &first_rule.id;
        loop {
            if destination == "A" {
                part1_sum += item.values().sum::<i32>();
                break;
            }
            if destination == "R" {
                break;
            }
            let current_rule = rule_lookup.get(destination).unwrap();
            destination = current_rule.item_destination(item);
        }

    }

    println!("part1 sum: {part1_sum}");

    println!("part2 sum: {}", num_combinations(&rule_lookup));

    Ok(())
}
