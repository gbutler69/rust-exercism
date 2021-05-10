pub struct CircularBuffer<T> {
    storage: Vec<T>,
    front: usize,
    back: usize,
    count: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Default> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            storage: Vec::with_capacity(capacity),
            front: 0,
            back: 0,
            count: 0,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.buffer_is_full() {
            return Err(Error::FullBuffer);
        }
        self.write_at_back(element);
        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buffer_is_empty() {
            return Err(Error::EmptyBuffer);
        }
        Ok(self.read_at_front())
    }

    pub fn clear(&mut self) {
        self.front = 0;
        self.back = 0;
        self.count = 0;
        self.storage.clear();
    }

    pub fn overwrite(&mut self, element: T) {
        if self.buffer_is_full() {
            self.increment_front();
        }
        self.write_at_back(element);
    }

    fn increment_back(&mut self) {
        self.back += 1;
        self.back %= self.storage.capacity();
        self.count += 1;
    }

    fn increment_front(&mut self) {
        self.front += 1;
        self.front %= self.storage.capacity();
        self.count -= 1;
    }

    fn buffer_is_full(&self) -> bool {
        self.count == self.storage.capacity()
    }

    fn buffer_is_empty(&self) -> bool {
        self.count == 0
    }

    fn write_at_back(&mut self, element: T) {
        match (self.storage.capacity(), self.storage.len(), self.back) {
            (cap, len, _) if len < cap => {
                self.storage.push(element);
                self.increment_back();
            }
            _ => {
                self.storage[self.back] = element;
                self.increment_back();
            }
        }
    }

    fn read_at_front(&mut self) -> T {
        let result = std::mem::take(&mut self.storage[self.front]);
        self.increment_front();
        result
    }
}
