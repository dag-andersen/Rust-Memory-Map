use core::fmt;

pub mod NodeToMem;
pub mod Tree;
pub mod TreePrinter;

const NODE_SIZE : usize = std::mem::size_of::<Node>();

pub struct Node {
    pub min_ip: u32,
    pub max_ip: u32,
    pub left: usize,
    pub right: usize,
    pub name: [u8; 32],
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{:p}, n: {}, min: {}, max: {}, l: {}, r: {}", &self, std::str::from_utf8(&self.name).unwrap(), self.min_ip, self.max_ip, self.left, self.right)
    }
}