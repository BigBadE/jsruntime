use std::ptr;

pub struct Logger {
    pub buffer: Box<[u8]>,
    pub index: usize,
}

impl Logger {
    pub fn new() -> Logger {
        Logger {
            buffer: Vec::with_capacity(2048).into_boxed_slice(),
            index: 0,
        }
    }

    pub fn log(&mut self, message: String) {
        let removing = self.index + message.len() - 2048;

        if removing > 0 {
            self.index -= removing;
            Logger::copy_to_start(&mut self.buffer, message.len() - removing);
        }

        Logger::copy(&mut self.buffer, message)
    }

    fn copy(vec: &mut [u8], mut target: String) {
        let dest = vec.as_mut_ptr();

        unsafe {
            let src = target.as_bytes_mut().as_mut_ptr();

            ptr::copy_nonoverlapping(src, dest.offset((vec.len() - target.len()) as isize),
                                     target.len())
        }
    }

    fn copy_to_start(vec: &mut [u8], length: usize) {
        let src = vec.as_mut_ptr();

        unsafe {
            ptr::copy(src.clone().offset((vec.len()-length) as isize), src, length)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy() {
        let original = String::from_utf8(vec!(2, 3)).unwrap();
        let mut test_case: [u8; 4] = [0, 1, 0, 0];
        Logger::copy(&mut test_case, original);
        for i in 0..4 {
            assert_eq!(test_case[i], i as u8);
        }
    }

    #[test]
    fn test_copy_to_start() {
        let mut test_case: [u8; 4] = [0, 0, 1, 2];
        Logger::copy_to_start(&mut test_case, 3);
        print!("[{}, {}, {}, {}]", test_case[0], test_case[1], test_case[2], test_case[3]);
        for i in 0..2 {
            assert_eq!(test_case[i], i as u8);
        }
    }
}