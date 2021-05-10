use std::io::{Read, Result, Write};

pub struct ReadStats<READER: Read> {
    bytes_read: usize,
    reads: usize,
    reader: READER,
}

impl<READER: Read> ReadStats<READER> {
    pub fn new(reader: READER) -> ReadStats<READER> {
        Self {
            reader,
            bytes_read: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &READER {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<READER: Read> Read for ReadStats<READER> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self.reader.read(buf)?;
        self.bytes_read += bytes_read;
        self.reads += 1;
        Ok(bytes_read)
    }
}

pub struct WriteStats<WRITER: Write> {
    bytes_written: usize,
    writes: usize,
    writer: WRITER,
}

impl<WRITER: Write> WriteStats<WRITER> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(writer: WRITER) -> WriteStats<WRITER> {
        Self {
            writer,
            bytes_written: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &WRITER {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<WRITER: Write> Write for WriteStats<WRITER> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self.writer.write(buf)?;
        self.bytes_written += bytes_written;
        self.writes += 1;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
