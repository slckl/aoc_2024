use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq)]
struct Rule {
    before: u32,
    after: u32,
}

#[derive(Debug, PartialEq)]
struct Update {
    pages: Vec<u32>,
}

fn parse(i: &str) -> (Vec<Rule>, Vec<Update>) {
    // Document starts with rules, with a rule per line.
    // Followed by empty line, indicating a new section.
    // Finished by a list of updates, with each update being on its own line.
    let lines = i.lines();
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    let mut first_section = true;
    for line in lines {
        let line = line.trim();
        // Empty line indicates the second section.
        if line.is_empty() {
            first_section = false;
        } else if first_section {
            // First section - rules.
            let mut split = line.split('|');
            let before = split.next().unwrap().parse().unwrap();
            let after = split.next().unwrap().parse().unwrap();
            rules.push(Rule { before, after });
        } else {
            // Second section - updates.
            let split = line.split(',');
            let mut update = Update { pages: Vec::new() };
            for page in split {
                update.pages.push(page.parse().unwrap());
            }
            updates.push(update);
        }
    }

    (rules, updates)
}

#[cfg(test)]
const TEST_DOC: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

#[test]
fn test_parse() {
    let (rules, updates) = parse(TEST_DOC);
    assert_eq!(
        rules[0],
        Rule {
            before: 47,
            after: 53
        }
    );
    assert_eq!(
        rules[20],
        Rule {
            before: 53,
            after: 13
        }
    );
    assert_eq!(
        updates[0],
        Update {
            pages: vec![75, 47, 61, 53, 29]
        }
    );
    assert_eq!(
        updates[5],
        Update {
            pages: vec![97, 13, 75, 29, 47]
        }
    );
}

impl Rule {
    fn check(&self, update: &Update) -> bool {
        let mut seen_before = false;
        let mut seen_after = false;
        for p in &update.pages {
            let p = *p;
            if p == self.before {
                // Check if we have seen the after page already...
                if seen_after {
                    return false;
                }
                seen_before = true;
            } else if p == self.after {
                // Check whether we have seen the before page *before* this page, 'cause this is after page.
                // But if we only have the after page, but no before page, then this rule does not apply.
                if seen_before {
                    return true;
                } else {
                    seen_after = true;
                }
            }
        }
        // No pages or only one of the pages were present in this update.
        true
    }
}

impl Update {
    /// Checks whether this update is in the correct order, given `rules`.
    fn check(&self, rules: &[Rule]) -> bool {
        for rule in rules {
            if !rule.check(self) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test_check() {
    let (rules, updates) = parse(TEST_DOC);
    assert!(updates[0].check(&rules));
    assert!(updates[1].check(&rules));
    assert!(updates[2].check(&rules));
    assert!(!updates[3].check(&rules));
    assert!(!updates[4].check(&rules));
    assert!(!updates[5].check(&rules));
}

fn check_and_sum_middle_pages(i: &str) -> u32 {
    let (rules, updates) = parse(i);
    updates
        .into_iter()
        .filter(|update| update.check(&rules))
        .map(|update| {
            let middle = update.pages.len() / 2;
            update.pages[middle]
        })
        .sum()
}

#[test]
fn test_sum() {
    let sum = check_and_sum_middle_pages(TEST_DOC);
    assert_eq!(sum, 143);
}

impl Update {
    /// Corrects this update to have correct page order
    /// according to the given `rules`.
    ///
    /// Claude cooked this one, I had heard of these things
    /// but didn't know you can apply them like this.
    fn correct(&mut self, rules: &[Rule]) {
        // Create adjacency list representation of the graph
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut in_degree: HashMap<usize, usize> = HashMap::new();

        // Initialize in_degree for all pages
        for idx in 0..self.pages.len() {
            in_degree.entry(idx).or_insert(0);
        }

        // Build the graph from applicable rules
        for (i, &page_i) in self.pages.iter().enumerate() {
            for (j, &page_j) in self.pages.iter().enumerate() {
                if i == j {
                    continue;
                }

                // Check if there's a rule saying page_i must come before page_j
                if rules
                    .iter()
                    .any(|r| r.before == page_i && r.after == page_j)
                {
                    graph.entry(i).or_default().push(j);
                    *in_degree.entry(j).or_insert(0) += 1;
                }
            }
        }

        // Perform topological sort using Kahn's algorithm
        let mut sorted_indices = Vec::new();
        let mut queue: VecDeque<usize> = in_degree
            .iter()
            .filter(|(_, &degree)| degree == 0)
            .map(|(&node, _)| node)
            .collect();

        while let Some(node) = queue.pop_front() {
            sorted_indices.push(node);

            if let Some(neighbors) = graph.get(&node) {
                for &next in neighbors {
                    *in_degree.get_mut(&next).unwrap() -= 1;
                    if in_degree[&next] == 0 {
                        queue.push_back(next);
                    }
                }
            }
        }

        // Create new pages array in correct order
        let old_pages = self.pages.clone();
        for (new_idx, &old_idx) in sorted_indices.iter().enumerate() {
            self.pages[new_idx] = old_pages[old_idx];
        }
    }
}

#[test]
fn test_correct() {
    let (rules, mut updates) = parse(TEST_DOC);
    updates[3].correct(&rules);
    assert_eq!(updates[3].pages, vec![97, 75, 47, 61, 53]);
}

fn correct_and_sum_middle_pages_of_corrected_updates(i: &str) -> u32 {
    let (rules, updates) = parse(i);
    updates
        .into_iter()
        .filter(|update| !update.check(&rules))
        .map(|mut update| {
            update.correct(&rules);
            let middle = update.pages.len() / 2;
            update.pages[middle]
        })
        .sum()
}

#[test]
fn test_correct_and_sum() {
    let sum = correct_and_sum_middle_pages_of_corrected_updates(TEST_DOC);
    assert_eq!(sum, 123);
}

fn main() {
    let input = std::fs::read_to_string("day5/input.txt").unwrap();
    // Part 1.
    let sum = check_and_sum_middle_pages(&input);
    println!("p1: {sum}");
    // Part 2.
    let sum = correct_and_sum_middle_pages_of_corrected_updates(&input);
    println!("p2: {sum}");
}
