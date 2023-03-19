#![no_std]

pub struct Cirq<'a, T: Copy> {
    head: usize,
    tail: usize,
    num_elements: usize,
    buffer: &'a mut [T],
}

#[derive(Debug, PartialEq)]
pub enum Error {
    FullBuffer,
}

impl<'a, T: Copy> Cirq<'a, T> {
    pub fn from_slice(slice: &'a mut [T]) -> Self {
        Cirq {
            head: 0,
            tail: 0,
            num_elements: 0,
            buffer: slice,
        }
    }

    pub fn try_push(&mut self, elem: T) -> Result<(), Error> {
        if self.num_elements == self.buffer.len() {
            Err(Error::FullBuffer)
        } else {
            self.num_elements += 1;
            self.buffer[self.tail] = elem;
            self.tail = (self.tail + 1) % self.buffer.len();
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.num_elements == 0 {
            None
        } else {
            self.num_elements -= 1;
            let elem = self.buffer[self.head];
            self.head = (self.head + 1) % self.buffer.len();
            Some(elem)
        }
    }

    pub fn is_full(&self) -> bool {
        self.num_elements == self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.num_elements == 0
    }

    pub fn len(&self) -> usize {
        self.num_elements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_push_returns_expected_result_filling_buffer() {
        let mut slice = [0_u8; 5];
        let mut cirq = Cirq::from_slice(&mut slice);

        for i in 0..5 {
            assert_eq!(cirq.try_push(0), Ok(()));
            assert_eq!(cirq.len(), i + 1);
        }

        assert_eq!(cirq.is_full(), true);
        assert_eq!(cirq.try_push(0), Err(Error::FullBuffer));
    }

    #[test]
    fn pop_returns_none_on_empty_cirq() {
        let mut slice = [0_u8; 5];
        let mut cirq = Cirq::from_slice(&mut slice);

        assert_eq!(cirq.is_empty(), true);
        assert_eq!(cirq.pop(), None);
    }

    #[test]
    fn pop_returns_expected_results_for_draining_buffer() {
        let mut slice = [0_u8; 5];
        let mut cirq = Cirq::from_slice(&mut slice);

        for i in 0_u8..5 {
            cirq.try_push(i).unwrap();
            assert_eq!(cirq.len(), (i + 1) as usize);
        }

        for i in 0_u8..5 {
            assert_eq!(cirq.pop(), Some(i));
            assert_eq!(cirq.len(), (4 - i) as usize);
        }

        assert_eq!(cirq.pop(), None);
    }
}
