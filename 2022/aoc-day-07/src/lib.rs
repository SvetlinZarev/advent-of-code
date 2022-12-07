use std::collections::{HashMap, HashSet};

const ROOT_DIR: &str = "/";
const TOTAL_DISK_SPACE: u32 = 70_000_000;
const REQUIRED_SPACE: u32 = 30_000_000;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Entry<'l> {
    List,
    Root,
    Return,
    Enter(&'l str),
    File(&'l str, u32),
    Dir(&'l str),
}

impl<'l> Entry<'l> {
    fn from_str(s: &'l str) -> Result<Self, String> {
        if s.starts_with("$") {
            let cmd_line = &s[2..];
            return match cmd_line.split_once(' ') {
                None => Ok(Entry::List),
                Some((cmd, param)) if cmd == "cd" => match param {
                    "/" => Ok(Entry::Root),
                    ".." => Ok(Entry::Return),
                    name => Ok(Entry::Enter(name)),
                },
                Some(_) => Err(format!("unknown command: {}", cmd_line)),
            };
        }

        if s.starts_with("dir ") {
            return Ok(Entry::Dir(&s[4..]));
        }

        match s.split_once(' ') {
            None => Err(format!("unexpected input: {}", s)),
            Some((size, name)) => Ok(Entry::File(
                name,
                size.parse()
                    .map_err(|_| format!("unexpected file entry: {}", s))?,
            )),
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Entry> {
    let mut entries = vec![];

    for line in input.lines().map(|l| l.trim()) {
        if line.is_empty() {
            continue;
        }

        match Entry::from_str(line).unwrap() {
            Entry::Dir(_) => {
                // The DIR entries are not used in the
                // current implementation, so drop them
                // in order to save some cycles
            }
            entry => entries.push(entry),
        }
    }

    entries
}

pub fn part_one(entries: &[Entry]) -> u32 {
    let directories = disk_usage(entries);

    directories
        .into_values()
        .filter(|&size| size <= 100_000)
        .sum()
}

pub fn part_two(entries: &[Entry]) -> u32 {
    let directories = disk_usage(entries);
    let &total_size = directories.get(&vec![ROOT_DIR]).unwrap();
    let available_space = TOTAL_DISK_SPACE - total_size;
    let to_free = if available_space >= REQUIRED_SPACE {
        0
    } else {
        REQUIRED_SPACE - available_space
    };

    directories
        .into_values()
        .filter(|&size| size >= to_free)
        .min()
        .unwrap()
}

fn disk_usage<'l>(entries: &'l [Entry]) -> HashMap<Vec<&'l str>, u32> {
    let mut fs = HashMap::new();
    let mut seen = HashSet::new();

    let mut path = vec![];
    let mut listed = vec![];
    let mut directory_processed = false;

    for entry in entries.iter().copied() {
        match entry {
            Entry::List => {
                seen.insert(listed);
                listed = path.clone();

                // check if we have already processed that directory
                directory_processed = seen.contains(&listed);

                // Register the path in the tracker
                fs.entry(path.clone()).or_insert(0);
            }

            Entry::Root => {
                path.clear();
                path.push(ROOT_DIR);
            }

            Entry::Return => {
                path.pop().unwrap();
            }

            Entry::Enter(dir) => {
                path.push(dir);
            }

            Entry::File(_name, size) => {
                // skip entries in already processed directories
                // in order to avoid double counting
                if directory_processed {
                    continue;
                }

                // update the current directory
                match fs.get_mut(&listed) {
                    None => panic!("unknown path: {:?}", listed),
                    Some(total) => *total += size,
                }
            }

            Entry::Dir(_) => {}
        }
    }

    let mut directories = HashMap::new();
    for (path, size) in fs.into_iter() {
        for to in 1..=path.len() {
            match directories.get_mut(&path[..to]) {
                Some(total) => {
                    *total += size;
                }
                None => {
                    directories.insert(path[..to].to_vec(), size);
                }
            }
        }
    }

    directories
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part_one, part_two};
    use aoc_shared::input::load_text_input_from_file;

    #[test]
    fn test_part_one() {
        let input = load_text_input_from_file("inputs/input.txt");
        let entries = parse_input(&input);
        let answer = part_one(&entries);
        assert_eq!(1297683, answer);
    }

    #[test]
    fn test_part_two() {
        let input = load_text_input_from_file("inputs/input.txt");
        let entries = parse_input(&input);
        let answer = part_two(&entries);
        assert_eq!(5756764, answer);
    }
}
