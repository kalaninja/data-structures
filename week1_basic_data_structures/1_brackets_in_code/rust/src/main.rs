fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let result = match wrong_bracket_index(input) {
        Some(i) => (i + 1).to_string(),
        None => "Success".to_string()
    };

    println!("{}", result);
}

fn wrong_bracket_index(str: String) -> Option<usize> {
    let mut stack = Vec::new();

    for (i, c) in str.char_indices() {
        if c == '(' {
            stack.push((i, ')'))
        } else if c == '[' {
            stack.push((i, ']'))
        } else if c == '{' {
            stack.push((i, '}'));
        } else if c == ')' || c == ']' || c == '}' {
            match stack.pop() {
                Some((_, b)) if b == c => continue,
                _ => return Some(i)
            }
        }
    }

    stack.first().map(|(i, _)| *i)
}

#[test]
fn test_wrong_bracket_index() {
    let tests: Vec<(&str, Option<usize>)> = vec![
        ("[]", None),
        ("{[}", Some(2)),
        ("foo(bar[i);", Some(9))
    ];

    tests.iter()
        .for_each(|(d, e)| {
            assert_eq!(wrong_bracket_index(d.to_string()), *e);
        });
}