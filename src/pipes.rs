use interprocess::local_socket::LocalSocketListener;
use std::{error::Error, io::Read};

pub struct NozomiPipes {
    pipe: LocalSocketListener,
}

impl NozomiPipes {
    pub fn new(pipe_name: &str) -> Result<Self, Box<dyn Error>> {
        let pipe = LocalSocketListener::bind(pipe_name)?;
        Ok(NozomiPipes { pipe })
    }

    pub fn read_data(&mut self, buffer: &mut String) -> Result<usize, std::io::Error> {
        let mut wa = self.pipe.accept()?;

        wa.read_to_string(buffer)
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
