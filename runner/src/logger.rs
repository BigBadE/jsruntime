use std::ptr;

pub struct Logger {
    pub buffer: Box<[u8; 2048]>,
    pub index: usize,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            buffer: Box::new([0; 2048]),
            index: 0,
        }
    }

    pub fn log(&mut self, mut message: String) {
        if message.len() > 2048 {
            message = message[message.len()-2048..].to_string()
        }

        if self.index + message.len() > 2048 {
            let removing = self.index + message.len() - 2048;
            self.index -= removing;
            Logger::copy_to_start(&mut self.buffer, self.index + message.len());
        }

        let length = message.len();
        Logger::copy(&mut self.buffer, message, self.index);
        self.index += length;
    }

    fn copy(vec: &mut [u8; 2048], mut target: String, index: usize) {
        let dest = vec.as_mut_ptr();

        unsafe {
            let src = target.as_bytes_mut().as_mut_ptr();

            ptr::copy_nonoverlapping(src, dest.offset(index as isize),
                                     target.len())
        }
    }

    fn copy_to_start(vec: &mut [u8; 2048], length: usize) {
        let src = vec.as_mut_ptr();

        unsafe {
            ptr::copy(src.clone().offset((2048-length) as isize), src, length)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy() {
        let mut original = [0 as u8; 2048];
        let target = String::from_utf8(Vec::from([1 as u8; 2048])).unwrap();
        Logger::copy(&mut original, target, 40);

        for i in 0..40 {
            assert_eq!(0, original[i]);
        }
        for i in 41..2048 {
            assert_eq!(1, original[i]);
        }
    }

    #[test]
    fn test_copy_to_start() {
        let mut original = [0 as u8; 2048];

        let mut val = 0;
        for i in 48..2048 {
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