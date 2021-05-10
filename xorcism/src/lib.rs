use std::{borrow::Borrow, io};

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<Key: AsRef<[u8]>> {
    key: Key,
    cursor: usize,
}

impl<'a, Key: AsRef<[u8]> + 'a> Xorcism<Key> {
    pub fn new(key: Key) -> Xorcism<Key> {
        Self { key, cursor: 0 }
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for datum in data.iter_mut() {
            *datum ^= self.next_datum_key();
        }
    }

    pub fn munge<'b, Data: IntoIterator<Item = impl Borrow<u8>> + 'b>(
        &'b mut self,
        data: Data,
    ) -> impl Iterator<Item = u8> + 'b {
        data.into_iter()
            .map(|v| *v.borrow())
            .map(move |data| data ^ self.next_datum_key())
    }

    pub fn reader(self, reader: &'a mut dyn io::Read) -> impl io::Read + 'a {
        XorcismRead {
            decoder: self,
            reader,
        }
    }

    pub fn writer<'b>(self, writer: &'a mut dyn io::Write) -> impl io::Write + 'b
    where
        'a: 'b,
    {
        XorcismWrite {
            encoder: self,
            writer,
        }
    }

    fn next_datum_key(&mut self) -> u8 {
        let key = self.key.as_ref()[self.cursor];
        self.cursor = (self.cursor + 1) % self.key.as_ref().len();
        key
    }
}

struct XorcismRead<'a, Key: AsRef<[u8]>> {
    decoder: Xorcism<Key>,
    reader: &'a mut dyn io::Read,
}

impl<'a, Key: AsRef<[u8]>> io::Read for XorcismRead<'a, Key> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let result = self.reader.read(buf);
        self.decoder.munge_in_place(buf);
        result
    }
}

struct XorcismWrite<'a, Key: AsRef<[u8]>> {
    encoder: Xorcism<Key>,
    writer: &'a mut dyn io::Write,
}

impl<'a, Key: AsRef<[u8]>> io::Write for XorcismWrite<'a, Key> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let buf = self.encoder.munge(buf).collect::<Vec<_>>();
        self.writer.write(&buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}
