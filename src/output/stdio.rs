use std::io::{stderr, stdout, Write};

pub struct CurlautStdOutput {
    out: CurlautStdOut,
    err: CurlautStdErr,
}

impl CurlautStdOutput {
    pub fn new() -> Self {
        Self {
            out: CurlautStdOut {},
            err: CurlautStdErr { enabled: false },
        }
    }
}

impl crate::output::CurlautOutput for CurlautStdOutput {
    fn enable_verbose(&mut self) {
        self.err.enabled = true;
    }

    fn common(&mut self) -> &mut impl Write {
        &mut self.out
    }

    fn verbose(&mut self) -> &mut impl Write {
        &mut self.err
    }
}

struct CurlautStdOut {}

struct CurlautStdErr {
    enabled: bool,
}

impl Write for CurlautStdOut {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        stdout().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        stdout().flush()
    }
}

impl Write for CurlautStdErr {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if self.enabled {
            return stderr().write(buf);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        if self.enabled {
            return stderr().flush();
        }
        Ok(())
    }
}
