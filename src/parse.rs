use petgraph::data::Build;
use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;

pub struct DotParser<R, B>
where
    R: std::io::Read,
{
    input: BufReader<R>,
    _output: PhantomData<B>,
}

impl<R, B> DotParser<R, B>
where
    R: std::io::Read,
    B: Build + Default,
{
    pub fn new(reader: R) -> Self {
        Self {
            input: BufReader::new(reader),
            _output: PhantomData,
        }
    }

    pub fn parse_into_graph(&self) -> B {
        B::default()
    }
}
