use core::fmt;
use core::slice;

pub struct LogString<'a> {
    raw_slice: &'a mut [u8],
    s: &'a str,
    len: usize,
    msg_len: &'a mut Option<xous::MemorySize>,
}

impl<'a> LogString<'a> {
    pub fn from_message(message: &'a mut xous::MemoryMessage) -> LogString<'a> {
        println!("Message address is at {:08x}", message.buf.addr.get());
        let raw_slice = unsafe { slice::from_raw_parts_mut(message.buf.as_ptr() as *mut u8, message.buf.len()) };
        let starting_length = message.valid.map(|x| x.get()).unwrap_or(0);
        LogString {
            s: unsafe {
                core::str::from_utf8_unchecked(slice::from_raw_parts(
                    message.buf.as_ptr() as *mut u8,
                    starting_length,
                ))
            },
            len: starting_length,
            raw_slice,
            msg_len: &mut message.valid,
        }
    }
}

impl<'a> fmt::Display for LogString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl<'a> fmt::Write for LogString<'a> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for c in s.bytes() {
            self.raw_slice[self.len] = c;
            self.len += 1;
        }
        self.s = unsafe {
            core::str::from_utf8_unchecked(slice::from_raw_parts(self.raw_slice.as_ptr(), self.len))
        };
        *self.msg_len = Some(xous::MemorySize::new(self.len).unwrap());
        Ok(())
    }
}
