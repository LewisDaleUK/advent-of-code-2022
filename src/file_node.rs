use std::{
    cell::RefCell,
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
    rc::Rc,
};

pub type NodeRef = Rc<RefCell<FileNode>>;
#[derive(PartialEq, Eq, Clone, Default)]
pub struct FileNode {
    pub children: HashMap<String, NodeRef>,
    pub size: usize,
    pub parent: Option<NodeRef>,
}

trait PathRoot {
    fn root(&self) -> Option<PathBuf>;
    fn tail(&self) -> Option<PathBuf>;
}

impl PathRoot for Path {
    fn root(&self) -> Option<PathBuf> {
        if let Some(root) = self.components().nth(0) {
            Some(Path::new(root.as_os_str()).to_path_buf())
        } else {
            None
        }
    }

    fn tail(&self) -> Option<PathBuf> {
        Some(
            self.components()
                .skip(1)
                .fold(Path::new("").to_path_buf(), |acc, c| {
                    acc.join(c.as_os_str())
                }),
        )
    }
}

impl FileNode {
    pub fn get_size(&self) -> usize {
        if self.children.len() > 0 {
            self.children
                .iter()
                .fold(0, |acc, (_, child)| acc + child.borrow_mut().get_size())
        } else {
            self.size
        }
    }

    pub fn find_dirs(&self) -> Vec<FileNode> {
        let mut dirs: Vec<FileNode> = vec![];
        if !self.children.is_empty() {
            dirs.push(self.clone());
            for child in self.children.values().clone() {
                dirs.append(&mut child.borrow_mut().find_dirs());
            }
        }
        dirs
    }
}

impl fmt::Debug for FileNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::FileNode;
    use std::collections::HashMap;

    #[test]
    fn can_create_a_file_node() {
        let expected = FileNode {
            children: HashMap::new(),
            size: 0,
            parent: None,
        };
        let actual = FileNode::default();
        assert_eq!(actual, expected);
    }
}