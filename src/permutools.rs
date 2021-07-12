#[derive(Debug)]
pub struct Permutation3<A, B, C> {
    av: Vec<A>,
    bv: Vec<B>,
    cv: Vec<C>,
    last: (usize, usize, usize),
}

#[derive(Debug)]
pub struct P3Inner<A, B, C> {
    ai: usize,
    bi: usize,
    ci: usize,
    asize: usize,
    bsize: usize,
    csize: usize,
    finished: bool,
    parent: *mut Permutation3<A, B, C>,
}

impl<A, B, C> Permutation3<A, B, C> {
    pub fn new(av: Vec<A>, bv: Vec<B>, cv: Vec<C>,) -> Self {
        Self { av, bv, cv, last: (0, 0, 0), }
    }

    pub fn reset(&mut self) -> () {
        self.last = (0, 0, 0);
    }

    pub fn size(&self) -> usize {
        self.av.len() * self.bv.len() * self.cv.len()
    }

    pub fn iter(&mut self) -> P3Inner<A, B, C> {
        P3Inner {
            ai: 0,
            bi: 0,
            ci: 0,
            asize: self.av.len(),
            bsize: self.bv.len(),
            csize: self.cv.len(),
            finished: false,
            parent: self,
        }
    }

    fn poll(&mut self, ai: usize, bi: usize, ci: usize) -> (A, B, C) {
        self.last = (ai, bi, ci);
        (self.av.remove(ai), self.bv.remove(bi), self.cv.remove(ci), )
    }

    pub fn back(&mut self, item: (A, B, C)) -> () {
        let (ai, bi, ci) = self.last;
        let (a, b, c) = item;
        self.av.insert(ai, a);
        self.bv.insert(bi, b);
        self.cv.insert(ci, c);
    }
}

impl<A, B, C> Iterator for P3Inner<A, B, C> {
    type Item = (A, B, C);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None
        }
        let ai = self.ai;
        let bi = self.bi;
        let ci = self.ci;
        match (self.ai + 1 == self.asize, self.bi + 1 == self.bsize, self.ci + 1 == self.csize, ) {
            (true, true, true)  => self.finished = true,
            (false, true, true) => { self.ci = 0; self.bi = 0; self.ai += 1; },
            (_, false, true)    => { self.ci = 0; self.bi += 1; },
            (_, _, false)       => self.ci += 1,
        };
        let p = unsafe { &mut *self.parent };
        Some(p.poll(ai, bi, ci))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation3() {
        let mut permu = Permutation3::new(
            vec![1,2,3], vec!['a', 'b'], vec![true, false]
        );
        let mut i = permu.iter();
        let mut item: Option<(usize, char, bool)> = i.next();
        assert_eq!(item, Some((1, 'a', true)));
        permu.back(item.unwrap());
        item = i.next();
        assert_eq!(item, Some((1, 'a', false)));
        permu.back(item.unwrap());
        item = i.next();
        assert_eq!(item, Some((1, 'b', true)));
        permu.back(item.unwrap());
        item = i.next();
        assert_eq!(item, Some((1, 'b', false)));
        permu.back(item.unwrap());
        item = i.next();
        assert_eq!(item, Some((2, 'a', true)));
        permu.back(item.unwrap());
        item = i.next();
        assert_eq!(item, Some((2, 'a', false)));
        permu.back(item.unwrap());
        item = i.next();
        assert_eq!(item, Some((2, 'b', true)));
        permu.back(item.unwrap());
        item = i.next();
        assert_eq!(item, Some((2, 'b', false)));
        permu.back(item.unwrap());
        item = i.next();
        assert_eq!(item, Some((3, 'a', true)));
        permu.back(item.unwrap());
        item = i.next();
        assert_eq!(item, Some((3, 'a', false)));
        permu.back(item.unwrap());
        item = i.next();
        assert_eq!(item, Some((3, 'b', true)));
        permu.back(item.unwrap());
        item = i.next();
        assert_eq!(item, Some((3, 'b', false)));
        permu.back(item.unwrap());
    }
}
