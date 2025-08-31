use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: Vec<char> = string
        .chars()
        .filter(|c| ['(', ')', '[', ']', '{', '}'].contains(c))
        .collect();

    let mut stack: Vec<char> = Vec::new();

    if brackets.len() % 2 != 0 {
        false
    } else {
        let mut pairs: HashMap<char, char> = HashMap::new();
        pairs.insert('}', '{');
        pairs.insert(')', '(');
        pairs.insert(']', '[');

        for _ in 0..brackets.len() {
            let popped = brackets.pop().unwrap();
            let last = stack.last();

            match last {
                Some(&last) => match pairs.get(&last) {
                    Some(&pair) => {
                        if pair == popped {
                            stack.pop();
                        } else {
                            stack.push(popped);
                        }
                    }
                    None => return false,
                },
                None => stack.push(popped),
            }
        }

        brackets.is_empty() && stack.is_empty()
    }
}
