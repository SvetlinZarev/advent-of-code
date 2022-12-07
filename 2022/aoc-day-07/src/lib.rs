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

pub mod v1 {
    use std::collections::{HashMap, HashSet};

    use super::*;

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
}

pub mod v2 {
    use std::collections::HashMap;

    use super::*;

    #[derive(Default, Debug)]
    struct Folder<'l> {
        content: HashMap<&'l str, Folder<'l>>,
        size: u32,
    }

    pub fn part_one(entries: &[Entry]) -> u32 {
        let root = disk_usage(entries);
        solve_part_one(&root)
    }

    fn solve_part_one(folder: &Folder) -> u32 {
        let mut answer = 0;
        if folder.size <= 100_000 {
            answer += folder.size;
        }

        for folder in folder.content.values() {
            answer += solve_part_one(folder);
        }

        answer
    }

    pub fn part_two(entries: &[Entry]) -> u32 {
        let root = disk_usage(entries);
        let available_space = TOTAL_DISK_SPACE - root.size;
        let to_free = if available_space >= REQUIRED_SPACE {
            0
        } else {
            REQUIRED_SPACE - available_space
        };

        solve_part_two(&root, to_free)
    }

    fn solve_part_two(folder: &Folder, to_free: u32) -> u32 {
        let mut answer = u32::MAX;
        if folder.size >= to_free {
            answer = folder.size;
        }

        for folder in folder.content.values() {
            answer = answer.min(solve_part_two(folder, to_free));
        }

        answer
    }

    fn disk_usage<'l>(entries: &'l [Entry]) -> Folder<'l> {
        let mut root = Folder::default();

        collect_file_sizes(entries, &mut 0, &mut root, 0);
        merge_directory_size(&mut root);

        root
    }

    fn collect_file_sizes<'l>(
        entries: &'l [Entry],
        idx: &mut usize,
        current: &mut Folder<'l>,
        depth: u32,
    ) -> u32 {
        let mut skip = false;

        while *idx < entries.len() {
            *idx += 1;

            match entries[*idx - 1] {
                Entry::Root => {
                    if depth != 0 {
                        return depth - 1;
                    }
                }

                Entry::Return => {
                    if depth == 0 {
                        panic!("cannot go before root");
                    }

                    return 0;
                }

                Entry::Enter(name) => {
                    let folder = current
                        .content
                        .entry(name)
                        .or_insert_with(|| Default::default());

                    let to_return = collect_file_sizes(entries, idx, folder, depth + 1);
                    if to_return > 0 {
                        return to_return - 1;
                    }
                }

                Entry::Dir(name) => {
                    if skip {
                        continue;
                    }

                    current
                        .content
                        .entry(name)
                        .or_insert_with(|| Default::default());
                }

                Entry::File(_name, size) => {
                    if skip {
                        continue;
                    }

                    current.size += size;
                }

                Entry::List => {
                    if current.content.len() > 0 || current.size > 0 {
                        skip = true;
                    }
                }
            }
        }

        0
    }

    fn merge_directory_size(current: &mut Folder) -> u32 {
        let mut children = 0;
        for folder in current.content.values_mut() {
            children += merge_directory_size(folder);
        }
        current.size += children;
        current.size
    }
}

#[cfg(test)]
mod tests {
    use aoc_shared::input::load_text_input_from_file;

    use crate::{parse_input, v1, v2};

    #[test]
    fn test_part_one_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let entries = parse_input(&input);
        let answer = v1::part_one(&entries);
        assert_eq!(1297683, answer);
    }

    #[test]
    fn test_part_two_v1() {
        let input = load_text_input_from_file("inputs/input.txt");
        let entries = parse_input(&input);
        let answer = v1::part_two(&entries);
        assert_eq!(5756764, answer);
    }

    #[test]
    fn test_part_one_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let entries = parse_input(&input);
        let answer = v2::part_one(&entries);
        assert_eq!(1297683, answer);
    }

    #[test]
    fn test_part_two_v2() {
        let input = load_text_input_from_file("inputs/input.txt");
        let entries = parse_input(&input);
        let answer = v2::part_two(&entries);
        assert_eq!(5756764, answer);
    }
}
