use core::str;
use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline, not_line_ending, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

enum Command {
    Cd(Cd),
    Ls(Vec<Files>),
}

enum Cd {
    Root,
    Up,
    Down(String),
}

enum Files {
    File { name: String, size: u32 },
    Directory(String),
}

fn parse_directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Directory(name.chars().collect())))
}

fn parse_file(input: &str) -> IResult<&str, Files> {
    let (input, (size, name)) = separated_pair(
        complete::u32,
        tag(" "),
        many1(one_of("abcdefghijklmnopqrstuvwxyz.")),
    )(input)?;
    Ok((
        input,
        Files::File {
            name: name.iter().collect(),
            size,
        },
    ))
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, elements) = separated_list1(newline, alt((parse_file, parse_directory)))(input)?;

    Ok((input, Command::Ls(elements)))
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, name) = not_line_ending(input)?;

    let command = match name {
        "/" => Command::Cd(Cd::Root),
        ".." => Command::Cd(Cd::Up),
        n => Command::Cd(Cd::Down(n.to_string())),
    };

    Ok((input, command))
}

fn parse_commands(input: &str) -> IResult<&str, Vec<Command>> {
    let (input, cmd) = separated_list1(newline, alt((parse_cd, parse_ls)))(input)?;
    Ok((input, cmd))
}

struct File {
    name: String,
    size: u32,
}

fn part2(input: &str) -> String {
    let (_, cmds) = parse_commands(input).unwrap();
    let mut directories: BTreeMap<String, Vec<File>> = BTreeMap::new();
    let mut ctx: Vec<String> = Vec::new();

    for cmd in &cmds {
        match cmd {
            Command::Cd(Cd::Root) => {
                ctx.push("".to_string());
            }
            Command::Cd(Cd::Up) => {
                ctx.pop();
            }
            Command::Cd(Cd::Down(name)) => {
                ctx.push(format!("/{}", name));
            }

            Command::Ls(files) => {
                for file in files {
                    match file {
                        Files::File { name, size } => {
                            directories
                                .entry(ctx.iter().cloned().collect())
                                .or_insert_with(Vec::new)
                                .push(File {
                                    name: name.to_string(),
                                    size: *size,
                                });
                        }
                        Files::Directory(_) => (),
                    };
                }
            }
        }
    }

    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();
    for (path, files) in directories.iter() {
        let dirs = path
            .split("/")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let size = files.iter().map(|File { size, .. }| size).sum::<u32>();

        for i in 0..dirs.len() {
            sizes
                .entry((&dirs[0..=i]).iter().cloned().collect())
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }
    let res: u32 = *sizes
        .iter()
        .filter(|(_, &size)| size >= 30_000_000 + sizes[""] - 70_000_000)
        .map(|(_, size)| size)
        .min()
        .unwrap();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(
            "$ cd /
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
7214296 k
",
        );
        assert_eq!(result, "24933642");
    }
}
