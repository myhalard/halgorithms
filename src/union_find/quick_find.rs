
pub struct UF {
    len: usize,
    vec: Vec<usize>,
}

impl UF {
    pub fn new(len: usize) -> UF {
        let mut vec = Vec::with_capacity(len);
        for i in 0..len {
            vec.push(i);
        }
        
        UF {
            len,
            vec,
        }
    }

    pub fn union(&mut self, left: usize, right: usize) {
        let left = self.vec[left];
        let right = self.vec[right];

        if left == right {
            return;
        }

        for i in 0..self.len {
            if self.vec[i] == left {
                self.vec[i] = right;
            }
        }
    }

    pub fn find(&self, left:usize, right: usize) -> bool {
        self.vec[left] == self.vec[right]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_starts_unconnected() {
        let uf = UF::new(2);
        assert!(!uf.find(0, 1));
    }
    #[test]
    fn it_is_connected() {
        let mut uf = UF::new(2);
        uf.union(0, 1);
        assert!(uf.find(0, 1));
    }
}
