use core::cmp::{Eq, PartialEq};
use core::fmt::Debug;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::file_node::FileNode;
use itertools::Itertools;

pub trait Sizeable {
    fn get_size(&self) -> usize;
}

#[derive(Debug, PartialEq, Eq)]
pub struct File {
    pub size: usize,
    pub name: String,
}

impl Sizeable for File {
    fn get_size(&self) -> usize {
        self.size
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Directory {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileSystem {
    nodes: FileNode,
}

pub trait HasParent {
    fn has_parent(&self, other: PathBuf) -> bool;
}

impl HasParent for Path {
    fn has_parent(&self, other: PathBuf) -> bool {
        if let Some(parent) = self.parent() {
            return parent.to_path_buf().eq(&other);
        }
        false
    }
}

impl FileSystem {
    pub fn new(inputs: Vec<&str>) -> FileSystem {
        let root = Rc::new(RefCell::new(FileNode::default()));
        let mut node = root.clone();
        for input in inputs {
            let parts = input.split(' ').collect_vec();
            match parts[0] {
                "$" => match parts[1] {
                    "cd" => match parts[2] {
                        ".." => {
                            let parent = node.borrow().parent.clone().unwrap();
                            node = parent;
                        }
                        "/" => (),
                        _ => {
                            let child = node
                                .borrow_mut()
                                .children
                                .entry(parts[2].to_string())
                                .or_default()
                                .clone();
                            node = child;
                        }
                    },
                    _ => (),
                },
                "dir" => {
                    let dir = node
                        .borrow_mut()
                        .children
                        .entry(parts[1].to_string())
                        .or_default()
                        .clone();
                    dir.borrow_mut().parent = Some(node.clone());
                }
                _ => {
                    let file = node
                        .borrow_mut()
                        .children
                        .entry(parts[1].to_string())
                        .or_default()
                        .clone();
                    file.borrow_mut().size = parts[0].parse().unwrap();
                    file.borrow_mut().parent = Some(node.clone());
                }
            }
        }
        FileSystem { nodes: root.take() }
    }

    pub fn find_dirs_by_max_size(&self, max: usize) -> usize {
        self.nodes
            .find_dirs()
            .iter()
            .filter(|node| node.get_size() < max)
            .fold(0 as usize, |acc, node| acc + node.get_size())
    }

    pub fn find_dir_by_min_size(&self, min: usize) -> usize {
        let mut nodes = self.nodes.find_dirs();
        nodes.sort_by_key(|d| d.get_size());
        nodes
            .iter()
            .find(|n| n.get_size() >= min)
            .unwrap()
            .get_size()
    }

	pub fn free_size(&self, total: usize, target: usize) -> usize {
		let unused = total - self.nodes.get_size();
		let diff = target - unused;
		self.find_dir_by_min_size(diff)
	}
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use itertools::Itertools;

    use super::HasParent;
    use super::FileSystem;

    #[test]
    fn it_should_assert_that_a_path_has_a_parent() {
        let inputs = [Path::new("/"), Path::new("/a"), Path::new("/a/b")];
        let expectations = [false, false, true];
        let actual = inputs.map(|i| i.has_parent(Path::new("/a").to_path_buf()));
        assert_eq!(expectations, actual);
    }

	#[test]
	fn it_should_free_the_right_size() {
        let binding = "$ cd /
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
		7214296 k"
            .replace('\t', "");

        let input = binding.split('\n').collect_vec();
        let files = FileSystem::new(input);

        let expected = 24933642;
        assert_eq!(expected, files.free_size(70000000, 30000000));
	}

    #[test]
    fn it_finds_size_of_smallest_directory_to_free_up_space() {
        let binding = "$ cd /
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
		7214296 k"
            .replace('\t', "");

        let input = binding.split('\n').collect_vec();
        let files = FileSystem::new(input);

        let expected = 24933642;
        assert_eq!(expected, files.find_dir_by_min_size(8_381_165));
    }

    #[test]
    fn it_finds_and_sums_sizes_of_dirs_smaller_than_x() {
        let binding = "$ cd /
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
		7214296 k"
            .replace('\t', "");

        let input = binding.split('\n').collect_vec();
        let files = FileSystem::new(input);

        let expected = 95437;
        assert_eq!(expected, files.find_dirs_by_max_size(100000));
    }
}
