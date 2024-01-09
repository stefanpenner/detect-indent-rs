use regex::Regex;
use std::{collections::HashMap, sync::OnceLock};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IndentKind {
    Space,
    Tab,
}

impl IndentKind {
    pub fn repeat(&self, times: usize) -> String {
        match *self {
            IndentKind::Space => " ".repeat(times),
            IndentKind::Tab => "\t".repeat(times),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Indent {
    amount: usize,
    indent: String,
    kind: Option<IndentKind>,
}

impl Indent {
    pub fn amount(&self) -> usize {
        self.amount
    }
    pub fn indent(&self) -> &str {
        &self.indent
    }
    pub fn kind(&self) -> Option<IndentKind> {
        self.kind
    }
}

#[derive(Debug)]
struct Usage {
    used: isize,
    weight: isize,
}

fn most_used(indents: &HashMap<isize, Usage>) -> usize {
    let mut result = 0;
    let mut max_used = 0;
    let mut max_weight = 0;

    for (&key, usage) in indents.iter() {
        if usage.used > max_used || (usage.used == max_used && usage.weight > max_weight) {
            max_used = usage.used;
            max_weight = usage.weight;
            result = key;
        }
    }

    assert!(
        result >= 0,
        "detect-irdent::most_used cannot return a negative"
    );

    result as usize
}

static INDENT_REGEX: OnceLock<Regex> = OnceLock::new();
fn get_indent_regex() -> &'static Regex {
    INDENT_REGEX.get_or_init(|| Regex::new(r"^(?:( )+|\t+)").unwrap())
}

pub fn detect_indent(string: &str) -> Indent {
    let mut spaces = 0;
    let mut tabs = 0;
    let mut indents: HashMap<isize, Usage> = HashMap::new();

    let mut prev = 0;
    let mut current: Option<isize> = None;
    let mut key;

    for line in string.lines() {
        if line.is_empty() {
            continue;
        }
        let mut indent = 0;

        match get_indent_regex().captures(line) {
            Some(captures) => {
                if let Some(capture) = captures.get(0) {
                    let string = capture.as_str();
                    indent = string.len();

                    match string.chars().next().unwrap() {
                        ' ' => spaces += 1,
                        _ => tabs += 1,
                    }
                };
            }

            None => indent = 0,
        }

        assert!(
            indent <= (std::isize::MAX as usize),
            "indent greater than std::isize::MAX"
        );
        let iindent = indent as isize;

        let diff = iindent - prev;
        prev = iindent;

        if diff != 0 {
            key = diff.abs();
            current = Some(key);

            indents
                .entry(key)
                .or_insert(Usage { used: 0, weight: 0 })
                .used += 1;
        } else if let Some(key) = current {
            indents.get_mut(&key).unwrap().used += 1;
        }
    }

    let amount = most_used(&indents);

    let (kind, indent) = if amount == 0 {
        (None, "".to_string())
    } else if spaces >= tabs {
        (Some(IndentKind::Space), IndentKind::Space.repeat(amount))
    } else {
        (Some(IndentKind::Tab), IndentKind::Tab.repeat(amount))
    };

    Indent {
        amount,
        indent,
        kind,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn indent_from_file(filepath: &str) -> Indent {
        let contents = std::fs::read_to_string(filepath)
            .unwrap_or_else(|e| panic!("Could not read file {filepath}: {e:?}"));
        detect_indent(&contents)
    }

    #[test]
    fn mixed_space() {
        assert_eq!(
            indent_from_file("fixture/mixed-space.js"),
            Indent {
                amount: 4,
                indent: "    ".to_string(),
                kind: Some(IndentKind::Space)
            }
        );
    }

    #[test]
    fn mixed_tab() {
        assert_eq!(
            indent_from_file("fixture/mixed-tab.js"),
            Indent {
                amount: 1,
                indent: "\t".to_string(),
                kind: Some(IndentKind::Tab)
            }
        );
    }

    #[test]
    fn space() {
        assert_eq!(
            indent_from_file("fixture/space.js"),
            Indent {
                amount: 4,
                indent: "    ".to_string(),
                kind: Some(IndentKind::Space)
            }
        );
    }

    #[test]
    fn tab_four() {
        assert_eq!(
            indent_from_file("fixture/tab-four.js"),
            Indent {
                amount: 4,
                indent: "\t\t\t\t".to_string(),
                kind: Some(IndentKind::Tab)
            }
        );
    }

    #[test]
    fn tab() {
        assert_eq!(
            indent_from_file("fixture/tab.js"),
            Indent {
                amount: 1,
                indent: "\t".to_string(),
                kind: Some(IndentKind::Tab)
            }
        );
    }

    #[test]
    fn vendor_prefixed_css() {
        assert_eq!(
            indent_from_file("fixture/vendor-prefixed-css.css"),
            Indent {
                amount: 4,
                indent: "    ".to_string(),
                kind: Some(IndentKind::Space)
            }
        );
    }

    #[test]
    fn test_get_most_used() {
        let mut map = HashMap::new();
        assert_eq!(most_used(&map), 0);
        map.insert(1, Usage { used: 1, weight: 1 });
        assert_eq!(most_used(&map), 1);
        map.insert(2, Usage { used: 2, weight: 2 });
        assert_eq!(most_used(&map), 2);
        map.insert(3, Usage { used: 1, weight: 1 });
        assert_eq!(most_used(&map), 2);
        map.insert(4, Usage { used: 1, weight: 1 });
        assert_eq!(most_used(&map), 2);
        map.insert(5, Usage { used: 4, weight: 4 });
        assert_eq!(most_used(&map), 5);
        map.insert(
            1,
            Usage {
                used: 10,
                weight: 10,
            },
        );
        assert_eq!(most_used(&map), 1);
    }

    #[test]
    fn indent_kind_repeat() {
        assert_eq!(IndentKind::Space.repeat(0), "");
        assert_eq!(IndentKind::Space.repeat(1), " ");
        assert_eq!(IndentKind::Space.repeat(10), "          ");

        assert_eq!(IndentKind::Tab.repeat(0), "");
        assert_eq!(IndentKind::Tab.repeat(1), "\t");
        assert_eq!(IndentKind::Tab.repeat(10), "\t\t\t\t\t\t\t\t\t\t");
    }
}
