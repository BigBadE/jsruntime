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
            Logger::copy_to_start(&mut self.buffer, removing);
        }

        self.index += message.len();
        Logger::copy(&mut self.buffer, message);
    }

    fn copy(vec: &mut [u8; 2048], mut target: String) {
        let dest = vec.as_mut_ptr();

        unsafe {
            let src = target.as_bytes_mut().as_mut_ptr();

            ptr::copy_nonoverlapping(src, dest.offset((vec.len() - target.len()) as isize),
                                     target.len())
        }
    }

    fn copy_to_start(vec: &mut [u8; 2048], length: usize) {
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
        let mut test_case: [u8; 2048] = [0; 2048];
        Logger::copy(&mut test_case, original);
        for i in 3..4 {
            assert_eq!(test_case[i+2046], i as u8);
        }
    }

    #[test]
    fn test_copy_to_start() {
        let mut test_case: [u8; 2048] = [0; 2048];

        for i in 0..2 {
            test_case[2048-i] = i as u8;
        }

        Logger::copy_to_start(&mut test_case, 3);
        for i in 0..2 {
            assert_eq!(test_case[i], i as u8);
        }
    }
}