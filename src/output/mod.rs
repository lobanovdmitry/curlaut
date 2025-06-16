use std::io::Write;

pub mod stdio;

pub trait CurlautOutput {
    fn enable_verbose(&mut self);

    fn common(&mut self) -> &mut impl Write;

    fn verbose(&mut self) -> &mut impl Write;
}
