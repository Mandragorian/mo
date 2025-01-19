#[derive(Debug, Default)]
pub struct RingBuffer {
    pub buf: Vec<bool>,
    start: usize,
    end: usize,
}

impl RingBuffer {
    pub fn new(points: usize) -> Self {
        Self {
            buf: vec![false; points],
            start: 0,
            end: points - 1,
        }
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn sample(&mut self, pressed: bool) {
        self.buf[self.end] = pressed;
        self.buf[self.start] = false;
        self.start = (self.start + 1) % self.buf.len();
        self.end = (self.end + 1) % self.len();
    }

    pub fn iter(&self) -> RingBufferIter {
        RingBufferIter {
            buf: self,
            //cur: (self.start + 1) % self.len(),
            cur: self.start,
        }
    }
}

impl std::ops::Index<usize> for RingBuffer {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buf[index]
    }
}

pub struct RingBufferWindowIter<'a> {
    buf: &'a RingBuffer,
    cur: usize,
    end: usize,
}

impl Iterator for RingBufferWindowIter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur != self.end {
            let item = self.buf[self.cur];
            self.cur = (self.cur + 1) % self.buf.len();
            Some(item)
        } else {
            None
        }
    }
}

pub struct RingBufferIter<'a> {
    buf: &'a RingBuffer,
    cur: usize,
}

impl Iterator for RingBufferIter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur != self.buf.end {
            let item = self.buf[self.cur];
            self.cur = (self.cur + 1) % self.buf.len();
            Some(item)
        } else {
            None
        }
    }
}
