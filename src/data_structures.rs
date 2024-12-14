use std::collections::HashMap;

#[derive(Debug)]
pub struct DisjointSets<T> {
    parent: HashMap<T, T>,
}

impl<T> DisjointSets<T>
where
    T: Eq + std::hash::Hash + Copy,
{
    pub fn new() -> Self {
        Self {
            parent: HashMap::new(),
        }
    }

    pub fn find_parent(&mut self, x: &T) -> T {
        let p = match self.parent.get(x) {
            Some(p) => *p,
            None => {
                self.parent.insert(*x, *x);
                *x
            }
        };
        if p == *x {
            p
        } else {
            let gp = self.find_parent(&p);
            self.parent.insert(*x, gp);
            gp
        }
    }

    pub fn union(&mut self, x: T, y: T) {
        let x_p = self.find_parent(&x);
        let y_p = self.find_parent(&y);
        if x_p == y_p {
            return;
        }
        self.parent.insert(x_p, y_p);
    }
}
