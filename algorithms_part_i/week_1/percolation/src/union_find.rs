pub struct UF {
    pub count: usize,
    ids: Vec<usize>,
}

impl UF {
    pub fn new(size: &usize) -> Self {
        let count = size.clone();
        UF {
            ids: (0..count).collect(),
            count: count,
        }
    }

    pub fn union(&mut self, left: usize, right: usize) {
        let left_id = self.find(left);
        // println!("found left {}", left_id);
        let right_id = self.find(right);
        // println!("found right {}", right_id);
        if left_id != right_id {
            self.ids[left_id] = right_id.clone();
            self.count -= 1
        }
    }

    pub fn find(&mut self, id: usize) -> usize {
        let mut index = id.clone();
        let mut p_id = id.clone();
        while index != self.ids[index] {
            index = self.ids[index];
        }
        while p_id != index {
            let temp = self.ids[p_id];
            self.ids[p_id] = index;
            p_id = temp;
        }
        // println!("finding {} {}", id, self.ids.len());
        index
    }
    pub fn connected(&mut self, left: usize, right: usize) -> bool {
        let x = self.find(left) == self.find(right);
        // println!("connected {}", x);
        x
    }
}
