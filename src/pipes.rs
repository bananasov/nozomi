use interprocess::os::windows::named_pipe::ByteReaderPipeStream;
use std::{error::Error, io::Read};

pub struct NozomiPipes {
    pipe: ByteReaderPipeStream,
}

impl NozomiPipes {
    pub fn new(pipe_name: &str) -> Result<Self, Box<dyn Error>> {
        let pipe = ByteReaderPipeStream::connect(pipe_name)?;
        Ok(NozomiPipes { pipe })
    }

    pub fn read_data(&mut self, buffer: &mut [u8]) -> Result<usize, std::io::Error> {
        self.pipe.read(buffer)
    }

    // If needed add a method for writing data to the named pipe.
    // pub fn write_data(&mut self, data: &[u8]) -> Result<usize, Box<dyn Error>> {
    //     self.pipe.write(data)
    // }
}

impl Drop for NozomiPipes {
    fn drop(&mut self) {
        // No need for explicit disconnection as it is handled automatically when 'pipe' goes out of scope.
    }
}
