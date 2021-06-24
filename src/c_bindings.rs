use super::buffer::{Buffer, Mmap};

/** Allocates a new empty Buffer. */
#[no_mangle]
pub extern fn mijit_new() -> Box<Mmap> {
    Box::new(Mmap::new())
}

/** Frees a Buffer. */
#[no_mangle]
pub extern fn mijit_drop(_buffer: Box<Mmap>) {}

#[no_mangle]
pub extern fn five(/*buffer: &Mmap*/) -> i64 {
    5 //buffer.memory[0] as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn five() {
        let f = super::five();
        assert_eq!(f, 5);
    }
}
