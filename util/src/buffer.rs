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

    pub fn push(&mut self, string: &str) {
        let mut pumped = String::new();
        let mut rest_string = string;
        let mut split_at_opt = rest_string.find('\n');
        while split_at_opt.is_some() {
            let split_at = split_at_opt.unwrap() + '\n'.len_utf8();
            let splitted = rest_string.split_at(split_at);
            pumped.push_str(splitted.0);
            if !splitted.1.is_empty() {
                for _ in 0 .. self.indents {
                    pumped.push_str(self.indent_string);
                }
            }
            rest_string = splitted.1;
            split_at_opt = rest_string.find('\n');
        }
        pumped.push_str(rest_string);
        self.buffer.push_str(pumped.as_str());
    }

    pub fn flush(self) -> String { self.buffer }
}

impl Add<&'_ str> for Buffer {
    type Output = Buffer;

    fn add(mut self, rhs: &'_ str) -> Self::Output {
        self.push(rhs);
        self
    }
}

impl Add<Buffer> for Buffer {
    type Output = Buffer;

    fn add(mut self, rhs: Buffer) -> Self::Output {
        self.push(rhs.flush().as_str());
        self
    }
}

impl AddAssign<&str> for Buffer {
    fn add_assign(&mut self, rhs: &str) { self.push(rhs); }
}

#[cfg(test)]
mod tests {
    use crate::buffer::Buffer;

    #[test]
    fn test_empty() {
        let buf = Buffer::default();
        let expected = String::new();
        assert_eq!(buf.flush(), expected);
    }

    #[test]
    fn test_add() {
        let buf = Buffer::default();
        let new_buf = buf + "test";
        let expected = String::from("test");
        assert_eq!(new_buf.flush(), expected);
    }

    #[test]
    fn test_add_assign() {
        let mut buf = Buffer::default();
        buf += "test";
        let expected = String::from("test");
        assert_eq!(buf.flush(), expected);
    }

    #[test]
    fn test_indent() {
        let mut buf = Buffer::default();
        buf.indent();
        buf.new_line();
        buf += "test";
        let expected = String::from("\n\ttest");
        assert_eq!(buf.flush(), expected);
    }

    #[test]
    fn test_unindent() {
        let mut buf = Buffer::default();
        buf += "0";
        buf.indent();
        buf.new_line();
        buf += "test";
        buf.unindent();
        buf.new_line();
        buf += "0";
        let expected = String::from("0\n\ttest\n0");
        assert_eq!(buf.flush(), expected);
    }

    #[test]
    fn test_indent_indent() {
        let mut buf = Buffer::default();
        let mut buf2 = Buffer::default();
        buf2 += "1";
        buf2.indent();
        buf2.new_line();
        buf2 += "test";
        buf2.unindent();
        buf2.new_line();
        buf2 += "1";
        buf += "0";
        buf.indent();
        buf.new_line();
        buf += buf2.flush().as_str();
        buf.unindent();
        buf.new_line();
        buf += "0";
        let expected = String::from("0\n\t1\n\t\ttest\n\t1\n0");
        assert_eq!(buf.flush(), expected);
    }
}
