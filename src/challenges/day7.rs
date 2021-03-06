use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::ops::Deref;
use std::str::FromStr;

use regex::Regex;

use crate::Result;

pub fn day7_1() -> Result<()> {
    debug();

    let inputs: Vec<RulePair> = crate::utils::get_inputs(7);

    let rules = Rules::new(&inputs);
    rules.verify();

    let mut steps: Vec<char> = rules.keys().cloned().collect();
    steps.sort_by(|s1, s2| rules[&s2][&s1]);

    let answer: String = steps.iter().collect();

    println!("7-1: {}", answer);

    // Not BFGIKLNRTXHPUMWQVZOYJACDSE

    Ok(())
}

pub fn day7_2() -> Result<()> {
    let answer = 0;
    println!("7-2: {}", answer);

    Ok(())
}

#[derive(Debug)]
struct RulePair(char, char);

impl FromStr for RulePair {
    type Err = std::char::ParseCharError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let re = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
        let captures = re.captures(s).unwrap();

        let first = captures[1].parse::<char>()?;
        let second = captures[2].parse::<char>()?;

        Ok(RulePair(first, second))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rules {
    rules: HashMap<char, HashMap<char, Ordering>>,
}

impl Deref for Rules {
    type Target = HashMap<char, HashMap<char, Ordering>>;

    fn deref(&self) -> &HashMap<char, HashMap<char, Ordering>> {
        &self.rules
    }
}

impl Rules {
    pub fn new(rule_pairs: &[RulePair]) -> Self {
        let mut rules = HashMap::new();
        let mut steps: HashSet<char> = HashSet::new();

        // Fill based on inputs
        for rule in rule_pairs {
            steps.insert(rule.0);
            steps.insert(rule.1);
            let entry = rules.entry(rule.0).or_insert(HashMap::new());
            entry.insert(rule.1, Ordering::Greater);
        }

        for step in &steps {
            if !rules.contains_key(step) {
                rules.insert(*step, HashMap::new());
            }
        }

        // Fill based on existing orderings
        loop {
            let mut new_entries: HashSet<(char, char, Ordering)> = HashSet::new();
            for (step1, step1_rules) in &rules {
                for (step2, cmp12) in step1_rules {
                    if rules.contains_key(&step2) {
                        let step2_rules = &rules[step2];
                        for step3 in step2_rules.keys() {
                            if step1_rules[step2] == step2_rules[step3] {
                                if step1_rules.contains_key(step3)
                                    && step1_rules[step3] != step1_rules[step2]
                                {
                                    new_entries.insert((*step1, *step3, step1_rules[step2]));
                                }
                            }
                        }
                    }
                }
            }

            if new_entries.is_empty() {
                break;
            } else {
                for rule in new_entries {
                    let entry = rules.entry(rule.0).or_insert(HashMap::new());
                    entry.insert(rule.1, Ordering::Greater);
                }
            }
        }

        // Fill based on incomparables
        let mut new_entries: HashSet<(char, char, Ordering)> = HashSet::new();
        for (current_step, children) in &rules {
            for step in &steps {
                if !children.contains_key(step) {
                    if rules[step].contains_key(current_step) {
                        new_entries.insert((*current_step, *step, Ordering::Less));
                    } else {
                        let ordering = step.cmp(current_step);
                        new_entries.insert((*current_step, *step, ordering));
                    }
                }
            }
        }
        println!("{:?}", new_entries);
        for rule in new_entries {
            let entry = rules.entry(rule.0).or_insert(HashMap::new());
            entry.insert(rule.1, rule.2);
        }

        Self { rules }
    }

    fn verify(&self) {
        let mut steps: Vec<char> = self.rules.keys().cloned().collect();
        steps.sort();

        // Verify reflexivity and antisymmetry
        for i in 0..steps.len() {
            let step1 = &steps[i];
            let first_rules = &self.rules[step1];
            for step2 in steps.iter().skip(i) {
                let second_rules = &self.rules[step2];

                let cmp12 = first_rules[step2];
                let cmp21 = second_rules[step1];

                if step1 == step2 {
                    assert_eq!(
                        cmp12, cmp21,
                        "Reflexivity violated\n1:{} 2:{} 12:{:?} 21:{:?}",
                        step1, step2, cmp12, cmp21
                    );
                    assert_eq!(
                        cmp12,
                        Ordering::Equal,
                        "Reflexivity violated\n1:{} 1:{} 11:{:?}",
                        step1,
                        step1,
                        cmp12
                    );
                } else {
                    assert_ne!(
                        cmp12, cmp21,
                        "Antisymmetry violated\n1:{} rules: {:?}\n2:{} rules: {:?}",
                        step1, first_rules, step2, second_rules
                    );
                }

                for step3 in second_rules.keys() {
                    let cmp13 = first_rules[step3];
                    let cmp23 = second_rules[step3];

                    if cmp12 == Ordering::Less && cmp23 == Ordering::Less {
                        assert_eq!(
                            cmp13,
                            Ordering::Less,
                            "Transitivity violated\n 1:{} 2:{} 3:{} -- 12:{:?} 23:{:?} 13:{:?}",
                            step1,
                            step2,
                            step3,
                            cmp12,
                            cmp23,
                            cmp13,
                        );
                    }
                }
            }
        }

        // Verify total ordering
        for step_rules in self.rules.values() {
            assert_eq!(step_rules.len(), steps.len());
        }
    }
}

fn debug() {
    let inputs: Vec<RulePair> = crate::utils::get_inputs(7);

    let rules1 = Rules::new(&inputs);
    rules1.verify();

    let rules2 = Rules::new(&inputs);
    rules1.verify();

    let rules3 = Rules::new(&inputs);
    rules1.verify();

    assert_eq!(rules1, rules2);
    assert_eq!(rules2, rules3);
    assert_eq!(rules1, rules3);
}
