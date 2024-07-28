pub struct UF {
    nodes: Vec<usize>,
    sizes: Vec<usize>,
}

impl UF {
    pub fn new(len: usize) -> UF {
        let mut nodes = Vec::with_capacity(len);
        let mut sizes = Vec::with_capacity(len);
        for i in 0..len {
            nodes.push(i);
            sizes.push(1);
        }
        
        UF {
            nodes,
            sizes,
        }
    }

    pub fn union(&mut self, left: usize, right: usize) {
        let left = self.root(left);
        let right = self.root(right);
        if left == right {
            return;
        }
        if self.sizes[left] < self.sizes[right] {
            self.nodes[left] = right;
            self.sizes[right] += self.sizes[left]; 
        }
        else {
            self.nodes[right] = left;
            self.sizes[left] += self.sizes[right];
        }
    }

    pub fn find(&mut self, left: usize, right: usize) -> bool {
        self.root(left) == self.root(right)
    }

    fn root(&mut self, node: usize) -> usize {
        let mut node = node;
        while self.nodes[node] != node {
            self.nodes[node] = self.nodes[self.nodes[node]];
            node = self.nodes[node];
        }
        node
    }


}