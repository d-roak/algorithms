// Grow-only Counter Implementation
// id - identifier assigned to each node in the cluster
// p - array with a counter assigned to each slot (for each node)

pub struct GCounter {
    id: u64,
    p: Vec<u64>,
}

impl GCounter {
    pub fn new(id: uin64, p: Vec<u64>) -> GCounter {
        GCounter {
            id: id,
            p: p,
        }
    }

    pub fn increment(&mut self) {
        let g = self.id as usize;
        self.p[g] += 1;
    }

    pub fn value(&self) -> u64 {
        self.p.iter().sum();
    }

    pub fn compare(&self, other: &GCounter) -> bool {
        self.p == other.p
    }

    pub fn merge(&mut self, other: &GCounter) -> Vec<u64> {
        other.p.iter().zip(self.p.iter_mut()).map(|(a, b)| {
            *b = a.max(*b);
        }).collect()
    }
}
