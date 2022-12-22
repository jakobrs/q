use slotmap::{new_key_type, Key, SlotMap};

new_key_type! {
    pub struct NodeKey;
}

extern "C" {
    fn rand() -> std::ffi::c_int;
}

/// totally random. not seeded
fn randint() -> i32 {
    unsafe { rand() }
}

pub type KeyType = i64;

#[derive(Debug)]
pub struct Node {
    pub value: KeyType,
    pub priority: i32,
    pub count: usize,
    pub sum: KeyType,
    pub left: NodeKey,
    pub right: NodeKey,
    pub parent: NodeKey,
}

#[derive(Default)]
pub struct Treap {
    nodes: SlotMap<NodeKey, Node>,
    root: NodeKey,
}

impl Node {
    pub fn new(value: KeyType) -> Self {
        Self::new_with_priority(value, randint())
    }

    pub fn new_with_priority(value: KeyType, priority: i32) -> Self {
        Self {
            value,
            priority,
            count: 1,
            sum: value,
            left: NodeKey::null(),
            right: NodeKey::null(),
            parent: NodeKey::null(),
        }
    }
}

impl Treap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, node_key: NodeKey) -> Option<&Node> {
        self.nodes.get(node_key)
    }

    pub fn pull(&mut self, node_key: NodeKey) {
        let Node {
            value, left, right, ..
        } = self.nodes[node_key];

        let mut count = 1;
        let mut sum = value;
        if let Some(left) = self.nodes.get_mut(left) {
            count += left.count;
            sum += left.sum;
            left.parent = node_key;
        }
        if let Some(right) = self.nodes.get_mut(right) {
            count += right.count;
            sum += right.sum;
            right.parent = node_key;
        }

        let node = &mut self.nodes[node_key];
        node.count = count;
        node.sum = sum;
    }

    pub fn merge(&mut self, left: NodeKey, right: NodeKey) -> NodeKey {
        if !self.nodes.contains_key(left) {
            right
        } else if !self.nodes.contains_key(right) {
            left
        } else {
            let ref l = self.nodes[left];
            let ref r = self.nodes[right];

            if l.priority > r.priority {
                self.nodes[left].right = self.merge(l.right, right);
                self.pull(left);
                left
            } else {
                self.nodes[right].left = self.merge(left, r.left);
                self.pull(right);
                right
            }
        }
    }

    /// NOTE: does not clean up all parent links. prooobably fine
    pub fn split_node_at_value(&mut self, node_key: NodeKey, at: KeyType) -> (NodeKey, NodeKey) {
        if let Some(node) = self.nodes.get(node_key) {
            if node.value < at {
                let (rl, rr) = self.split_node_at_value(node.right, at);
                self.nodes[node_key].right = rl;
                self.pull(node_key);
                (node_key, rr)
            } else {
                let (ll, lr) = self.split_node_at_value(node.left, at);
                self.nodes[node_key].left = lr;
                self.pull(node_key);
                (ll, node_key)
            }
        } else {
            (NodeKey::null(), NodeKey::null())
        }
    }

    pub fn split_at_value(&mut self, at: KeyType) -> (NodeKey, NodeKey) {
        self.split_node_at_value(self.root, at)
    }

    pub fn insert_value(&mut self, value: KeyType) -> NodeKey {
        let new_node = self.nodes.insert(Node::new(value));
        let (left, right) = self.split_at_value(value);
        self.root = self.merge(left, new_node);
        self.root = self.merge(self.root, right);
        new_node
    }

    /// Returns NodeKey::null() if the value is not found.
    fn find_value_in_node(&self, node_key: NodeKey, value: i64) -> NodeKey {
        if let Some(node) = self.nodes.get(node_key) {
            if node.value == value {
                node_key
            } else if node.value > value {
                self.find_value_in_node(node.left, value)
            } else {
                self.find_value_in_node(node.right, value)
            }
        } else {
            NodeKey::null()
        }
    }

    pub fn find_value(&self, value: KeyType) -> NodeKey {
        self.find_value_in_node(self.root, value)
    }

    pub fn remove_value(&mut self, value: KeyType) {
        let node = self.find_value(value);
        if node.is_null() {
            panic!("aaaaa");
        }

        let mut cur = self.nodes[node].parent;
        while let Some(node) = self.nodes.get_mut(cur) {
            node.count -= 1;
            node.sum -= value;  
            cur = node.parent;
        }

        let Node { left, right, parent, .. } = self.nodes.remove(node).unwrap();

        let merged = self.merge(left, right);
        if let Some(merged) = self.nodes.get_mut(merged) {
            merged.parent = parent;
        }
        if let Some(parent) = self.nodes.get_mut(parent) {
            if parent.left == node {
                parent.left = merged;
            } else {
                parent.right = merged;
            }
        } else {
            self.root = merged;
        }
    }

    pub fn sum(&self) -> KeyType {
        if let Some(root) = self.nodes.get(self.root) {
            root.sum
        } else {
            0
        }
    }

    pub fn count(&self, node: NodeKey) -> usize {
        if let Some(node) = self.nodes.get(node) {
            node.count
        } else {
            0
        }
    }

    pub fn sum_of_n_greatest_in_node(&self, node_key: NodeKey, mut n: usize) -> KeyType {
        if let Some(node) = self.nodes.get(node_key) {
            if n == 0 {
                return 0;
            } else if n >= node.count {
                return node.sum;
            }

            let mut total = self.sum_of_n_greatest_in_node(node.right, n);
            n = n.saturating_sub(self.count(node.right));
            if n <= 0 {
                return total;
            }
            total += node.value;
            n = n.saturating_sub(1);
            if n <= 0 {
                return total;
            }
            total += self.sum_of_n_greatest_in_node(node.left, n);
            total
        } else {
            0
        }
    }

    pub fn sum_of_n_greatest(&self, n: usize) -> KeyType {
        self.sum_of_n_greatest_in_node(self.root, n)
    }

    pub fn iter(&self) -> Iter {
        Iter {
            stack: vec![],
            here: self.root,
            treap: self,
        }
    }

    pub fn visualise(&self) {
        pub fn go(nodes: &SlotMap<NodeKey, Node>, node: NodeKey, prefix: String) {
            if let Some(node) = nodes.get(node) {
                println!("{prefix}value: {}", node.value);
                println!("{prefix}count: {}", node.count);
                println!("{prefix}sum: {}", node.sum);
                println!("{prefix}priority: {}", node.priority);
                println!("{prefix}left:");
                go(nodes, node.left, prefix.clone() + "  ");
                println!("{prefix}right:");
                go(nodes, node.right, prefix + "  ");
            }
        }

        go(&self.nodes, self.root, "    ".into());
    }
}

pub struct Iter<'a> {
    stack: Vec<NodeKey>,
    here: NodeKey,
    treap: &'a Treap,
}

impl Iterator for Iter<'_> {
    type Item = NodeKey;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.treap.nodes.contains_key(self.here) && self.stack.is_empty() {
            return None;
        }

        while let Some(node) = self.treap.nodes.get(self.here) {
            self.stack.push(self.here);
            self.here = node.left;
        }

        let next = self.stack.pop().unwrap();
        self.here = self.treap.nodes[next].right;

        Some(next)
    }
}