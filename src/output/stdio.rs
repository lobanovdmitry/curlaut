use std::io::{Write, stderr, stdout};

pub struct CurlautStdOutput {
    out: curlautStdOut,
    err: curlautStdErr,
}

impl CurlautStdOutput {
    pub fn new() -> Self {
        Self {
            out: curlautStdOut {},
            err: curlautStdErr { enabled: false },
        }
    }
}

impl crate::output::curlautOutput for CurlautStdOutput {
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

struct curlautStdOut {}

struct curlautStdErr {
    enabled: bool,
}

impl Write for curlautStdOut {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        stdout().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        stdout().flush()
    }
}

impl Write for curlautStdErr {
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
