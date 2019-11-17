use std::ops::{Add, AddAssign};

pub struct Buffer {
    buffer: String,
    indents: i8,
    indent_string: &'static str,
}

impl Buffer {
    pub fn default() -> Buffer { Buffer { buffer: String::new(), indent_string: "\t", indents: 0 } }

    pub fn indent(&mut self) { self.indents += 1; }

    pub fn unindent(&mut self) { self.indents -= 1; }

    pub fn new_line(&mut self) {
        self.buffer.push_str("\n");
        for _ in 0 .. self.indents {
            self.buffer.push_str(self.indent_string);
        }
    }

    pub fn flush(self) -> String { self.buffer }
}

impl Add<&'_ str> for Buffer {
    type Output = Buffer;

    fn add(mut self, rhs: &'_ str) -> Self::Output {
        self.buffer.push_str(rhs);
        self
    }
}

impl Add<Buffer> for Buffer {
    type Output = Buffer;

    fn add(mut self, rhs: Buffer) -> Self::Output {
        self.buffer.push_str(rhs.flush().as_str());
        self
    }
}

impl AddAssign<&str> for Buffer {
    fn add_assign(&mut self, rhs: &str) { self.buffer.push_str(rhs); }
}
