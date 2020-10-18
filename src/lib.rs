#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// TODO make generic
pub struct Graph {
    // might fail, if the parent of a nodes are greater than the nodes index
    nodes: Vec<Node>,
}

pub struct Node {
    parent: Vec<usize>,
}

struct PositionedNode {
    pub field: Node,
    pub value: usize,
}

impl Graph {
    fn position(self) -> Vec<PositionedNode> {
        let mut Graph {nodes} = self;

        for (i, node) in nodes.iter().enumerate() {
        }
    }
}
