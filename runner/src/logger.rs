use std::ptr;

pub const SIZE: usize = 0xF00;

pub struct Logger {
    pub buffer: Box<[u8; SIZE]>,
    pub index: usize,
    pub updated: bool,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            buffer: Box::new([0; SIZE]),
            index: 0,
            updated: false,
        }
    }

    pub fn log(&mut self, message: &str) {
        let mut length = message.len();
        if length > SIZE {
            length = SIZE;
        }

        if self.index + length > SIZE {
            let removing = self.index + length - SIZE;
            self.index -= removing;
            Logger::copy_to_start(&mut self.buffer, length);
        }

        Logger::copy(&mut self.buffer, message, self.index);
        self.index += length;
        self.updated = true;
    }

    fn copy(vec: &mut [u8; SIZE], target: &str, index: usize) {
        let dest = vec.as_mut_ptr();

        unsafe {
            let src = target.as_bytes().as_ptr();

            ptr::copy_nonoverlapping(src, dest.offset(index as isize),
                                     target.len())
        }
    }

    fn copy_to_start(vec: &mut [u8; SIZE], length: usize) {
        let src = vec.as_mut_ptr();

        unsafe {
            ptr::copy(src.clone().offset(length as isize), src, SIZE - length)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy() {
        let mut original = [0 as u8; SIZE];
        let target = String::from_utf8(Vec::from([1 as u8; SIZE])).unwrap();
        Logger::copy(&mut original, &target, 40);

        for i in 0..40 {
            assert_eq!(0, original[i]);
        }
        for i in 41..SIZE {
            assert_eq!(1, original[i]);
        }
    }

    #[test]
    fn test_copy_to_start() {
        let mut original = [0 as u8; SIZE];

        let mut val = 0;
        for i in SIZE - 2000..SIZE {
            original[i] = val;
            if val == 255 {
                val = 0;
            } else {
                val += 1;
            }
        }

        Logger::copy_to_start(&mut original, 2000);

        for i in 0..2000 {
            assert_eq!(original[i], (i % 256) as u8);
        }
    }
}