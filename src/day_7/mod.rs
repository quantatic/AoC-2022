use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::{complete::newline, streaming::u32},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

pub const INPUT: &str = include_str!("./input");

#[derive(Debug)]
struct ProblemStatement {
    commands: Vec<CommandExecution>,
}

#[derive(Debug)]
enum CommandExecution {
    Cd(String),
    Ls(Vec<FileInfo>),
}

#[derive(Debug)]
enum FileInfo {
    Directory(String),
    File { name: String, size: u32 },
}

fn file_info(input: &str) -> IResult<&str, FileInfo> {
    fn dir(input: &str) -> IResult<&str, FileInfo> {
        let (rest, name) = preceded(tag("dir "), is_not("\n"))(input)?;

        let info = FileInfo::Directory(name.to_string());
        Ok((rest, info))
    }

    fn file(input: &str) -> IResult<&str, FileInfo> {
        let (rest, (size, name)) = separated_pair(u32, tag(" "), is_not("\n"))(input)?;

        let info = FileInfo::File {
            name: name.to_string(),
            size,
        };

        Ok((rest, info))
    }

    alt((dir, file))(input)
}

fn command_execution(input: &str) -> IResult<&str, CommandExecution> {
    fn cd(input: &str) -> IResult<&str, CommandExecution> {
        let (rest, directory) = preceded(tag("$ cd "), is_not("\n"))(input)?;

        let execution = CommandExecution::Cd(directory.to_string());

        Ok((rest, execution))
    }

    fn ls(input: &str) -> IResult<&str, CommandExecution> {
        let (rest, files) = preceded(tag("$ ls\n"), separated_list1(tag("\n"), file_info))(input)?;

        let execution = CommandExecution::Ls(files);

        Ok((rest, execution))
    }

    alt((cd, ls))(input)
}

fn problem_statement(input: &str) -> IResult<&str, ProblemStatement> {
    let (rest, commands) = separated_list1(newline, command_execution)(input)?;

    let problem_statement = ProblemStatement { commands };

    Ok((rest, problem_statement))
}

fn parse_problem_statement(input: &str) -> Result<ProblemStatement> {
    let (rest, problem_statement) =
        problem_statement(input).map_err(|err| err.map_input(str::to_string))?;

    if !rest.is_empty() {
        return Err(anyhow!(
            "expected full input stream to be parsed, but got {:?} left over",
            rest
        ));
    }

    Ok(problem_statement)
}

#[derive(Debug)]
struct DirNode {
    name: String,
    children_idx: BTreeMap<String, usize>,
    parent_idx: usize,
}

#[derive(Debug)]
struct FileNode {
    name: String,
    size: u32,
}

#[derive(Debug)]
enum FsNode {
    Dir(DirNode),
    File(FileNode),
}

impl FsNode {
    fn name(&self) -> &str {
        match &self {
            FsNode::Dir(DirNode { name, .. }) => name,
            FsNode::File(FileNode { name, .. }) => name,
        }
    }
}

#[derive(Debug)]
struct Fs {
    nodes: Vec<FsNode>,
    current_directory_idx: usize,
}

impl Fs {
    fn new() -> Self {
        let root_node = FsNode::Dir(DirNode {
            name: "/".to_string(),
            children_idx: BTreeMap::new(),
            parent_idx: 0,
        });

        Self {
            nodes: vec![root_node],
            current_directory_idx: 0,
        }
    }

    fn cd(&mut self, path: &str) {
        if path == "/" {
            self.current_directory_idx = 0;
            return;
        }

        let FsNode::Dir(current_directory) = &self.nodes[self.current_directory_idx] else {
            unreachable!("current directory was a somehow not a directory: {:?}", self);
        };

        if path == ".." {
            self.current_directory_idx = current_directory.parent_idx;
            return;
        }

        if let Some(&new_directory_idx) = current_directory.children_idx.get(path) {
            self.current_directory_idx = new_directory_idx;
        } else {
            unreachable!(
                "tried to cd to directory that didn't exist: {:?}, {:?}",
                path, self
            );
        }
    }

    fn create_file(&mut self, file_info: FileInfo) {
        let new_idx = self.nodes.len();

        let FsNode::Dir(current_directory) = &mut self.nodes[self.current_directory_idx] else {
            unreachable!("current directory was a somehow not a directory: {:?}", self);
        };

        let new_file = match file_info {
            FileInfo::Directory(name) => FsNode::Dir(DirNode {
                name,
                children_idx: BTreeMap::new(),
                parent_idx: self.current_directory_idx,
            }),
            FileInfo::File { name, size } => FsNode::File(FileNode { name, size }),
        };
        let new_filename = new_file.name().to_string();

        current_directory.children_idx.insert(new_filename, new_idx);
        self.nodes.push(new_file);
    }

    fn calculate_size(&self, file_idx: usize) -> u32 {
        match &self.nodes[file_idx] {
            FsNode::File(FileNode { size, .. }) => *size,
            FsNode::Dir(DirNode { children_idx, .. }) => children_idx
                .values()
                .map(|child_idx| self.calculate_size(*child_idx))
                .sum(),
        }
    }
}

pub fn part_one(input: &str) -> Result<u32> {
    const MAXIMUM_DIRECTORY_SIZE: u32 = 100_000;

    let problem_statement = parse_problem_statement(input)?;
    let mut fs = Fs::new();
    for command in problem_statement.commands {
        match command {
            CommandExecution::Cd(path) => fs.cd(&path),
            CommandExecution::Ls(files) => {
                for file_info in files {
                    fs.create_file(file_info);
                }
            }
        }
    }

    let total = fs
        .nodes
        .iter()
        .enumerate()
        .filter_map(|(i, node)| match node {
            FsNode::File(_) => None,
            FsNode::Dir(_) => Some(fs.calculate_size(i)),
        })
        .filter(|&dir_size| dir_size <= MAXIMUM_DIRECTORY_SIZE)
        .sum();

    Ok(total)
}

pub fn part_two(input: &str) -> Result<u32> {
    const FS_SIZE: u32 = 70_000_000;
    const MINIMUM_FREE_SPACE: u32 = 30_000_000;
    const MAXIMUM_USED_SPACE: u32 = FS_SIZE - MINIMUM_FREE_SPACE;

    let problem_statement = parse_problem_statement(input)?;
    let mut fs = Fs::new();
    for command in problem_statement.commands {
        match command {
            CommandExecution::Cd(path) => fs.cd(&path),
            CommandExecution::Ls(files) => {
                for file_info in files {
                    fs.create_file(file_info);
                }
            }
        }
    }

    let currently_used_space = fs.calculate_size(0);
    let minimum_deletion_size = currently_used_space - MAXIMUM_USED_SPACE;

    fs.nodes
        .iter()
        .enumerate()
        .filter_map(|(i, node)| match node {
            FsNode::File(_) => None,
            FsNode::Dir(_) => Some(fs.calculate_size(i)),
        })
        .filter(|&dir_size| dir_size >= minimum_deletion_size)
        .min()
        .ok_or_else(|| {
            anyhow!("couldn't find any files meeting the necessary deletion requirements")
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_one() {
        let result = part_one(TEST_INPUT).unwrap();
        assert_eq!(result, 95_437);
    }

    #[test]
    fn solution_part_one() {
        let result = part_one(INPUT).unwrap();
        assert_eq!(result, 1_723_892);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT).unwrap();
        assert_eq!(result, 24_933_642);
    }

    #[test]
    fn solution_part_two() {
        let result = part_two(INPUT).unwrap();
        assert_eq!(result, 8_474_158);
    }
}
