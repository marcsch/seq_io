macro_rules! impl_fastq_reader {
    ($multiline:expr, $DefaultPositionStore:ty, ($($mod_path:expr),*)) => {

impl_reader!(
    Reader, crate::fastq::RefRecord<S>, crate::fastq::OwnedRecord,
    crate::fastq::RecordSet<S>, crate::fastq::Error,
    $DefaultPositionStore, false, $multiline,
    "fastq", ($($mod_path),*), "\n@id\nACGT\n+\nIIII\n", "\n@id1\nACGT\n+\nIIII\n@id2\nTGCA\n+\nIIII\n",
    ["OwnedRecord {head: b\"id1\".to_vec(), seq: b\"ACGT\".to_vec(), qual: b\"IIII\".to_vec()}",
     "OwnedRecord {head: b\"id2\".to_vec(), seq: b\"TGCA\".to_vec(), qual: b\"IIII\".to_vec()}"]
);

/// FASTQ parser
pub struct Reader<R, P = crate::policy::StdPolicy, S = $DefaultPositionStore>
where
    R: std::io::Read,
    P: crate::policy::BufPolicy,
    S: crate::core::PositionStore,
{
    inner: crate::core::CoreReader<R, P, S>,
}


impl<R> Reader<R>
where
    R: std::io::Read,
{
    #[inline]
    fn _with_capacity(reader: R, capacity: usize) -> Self {
        Reader {
            inner: crate::core::CoreReader::with_capacity(reader, capacity),
        }
    }

    #[inline]
    fn _from_buf_reader(rdr: crate::core::BufReader<R>, byte_offset: usize, line_idx: u64) -> Self {
        Reader {
            inner: crate::core::CoreReader::from_buf_reader(rdr, byte_offset, line_idx),
        }
    }
}

impl<R, P, S> Reader<R, P, S>
where
    R: std::io::Read,
    P: crate::policy::BufPolicy,
    S: crate::core::PositionStore,
{
    #[inline]
    fn _check_is_fasta(&mut self) -> super::Result<Option<bool>> {
        Ok(Some(false))
    }

    #[inline]
    fn _format(&self) -> Option<crate::fastx::SeqFormat> {
        Some(crate::fastx::SeqFormat::FASTQ)
    }

    #[inline]
    fn _set_store<T: crate::core::PositionStore>(self) -> Reader<R, P, T> {
        Reader {
            inner: self.inner.set_store(),
        }
    }

    #[inline]
    fn _set_policy<T: crate::policy::BufPolicy>(self, buf_policy: T) -> Reader<R, T, S> {
        Reader {
            inner: self.inner.set_policy(buf_policy),
        }
    }
}

}
}

// apply the macro
impl_fastq_reader!(false, super::RangeStore, ("fastq"));
