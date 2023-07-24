use interprocess::os::windows::named_pipe::ByteReaderPipeStream;

#[allow(unused)]
pub struct NozomiPipes {
    pipe: ByteReaderPipeStream,
}

impl NozomiPipes {
    pub fn new(pipe_name: &str) -> Result<NozomiPipes, Box<dyn std::error::Error>> {
        let pipe = ByteReaderPipeStream::connect(pipe_name)?;

        Ok(NozomiPipes { pipe })
    }
}
