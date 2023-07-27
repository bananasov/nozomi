use interprocess::os::windows::named_pipe::ByteReaderPipeStream;
use std::error::Error;
use std::fmt;

// #[allow(unused)]
#[derive(Debug)]
pub struct NozomiPipes {
    pipe: ByteReaderPipeStream,
}

impl NozomiPipes {
    pub fn new(pipe_name: &str) -> Result<Self, Box<dyn Error>> {
        let pipe = ByteReaderPipeStream::connect(pipe_name)?;
        Ok(NozomiPipes { pipe })
    }

    pub fn read_data(&mut self, buffer: &mut [u8]) -> Result<usize, Box<dyn Error>> {
        self.pipe.read(buffer)
    }

    // If needed, you can also add a method for writing data to the named pipe.
    // pub fn write_data(&mut self, data: &[u8]) -> Result<usize, Box<dyn Error>> {
    //     self.pipe.write(data)
    // }
}

impl Drop for NozomiPipes {
    fn drop(&mut self) {
        // Close the named pipe when NozomiPipes struct is dropped
        let _ = self.pipe.disconnect();
    }
}

impl fmt::Display for NozomiPipes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NozomiPipes {{ /* UwU fields */}}")
    }
}
