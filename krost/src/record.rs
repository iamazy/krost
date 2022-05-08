use crate::types::{VarInt, VarLong};
use crate::{KrostError, KrostType};
use std::io::{Cursor, Read, Write};

#[derive(Debug, PartialEq, Eq)]
pub struct RecordHeader {
    pub key: String,
    pub value: Vec<u8>,
}

impl KrostType for RecordHeader {
    fn decode<D: Read>(buf: &mut D, version: i16) -> Result<Self, KrostError> {
        // key
        let len = VarInt::decode(buf, version)?;
        let len = usize::try_from(len.0).map_err(|e| KrostError::Malformed(Box::new(e)))?;
        let mut key = Vec::with_capacity(len);
        let _ = buf.read_exact(&mut key);
        let key = String::from_utf8(key).map_err(|e| KrostError::Malformed(Box::new(e)))?;

        // value
        let len = VarInt::decode(buf, version)?;
        let len = usize::try_from(len.0).map_err(|e| KrostError::Malformed(Box::new(e)))?;
        let mut value = Vec::with_capacity(len);
        let _ = buf.read_exact(&mut value);

        Ok(Self { key, value })
    }

    fn encode<E: Write>(&self, buf: &mut E, version: i16) -> Result<usize, KrostError> {
        let l = i32::try_from(self.key.len()).map_err(|e| KrostError::Malformed(Box::new(e)))?;
        let mut len = VarInt(l).encode(buf, version)?;
        buf.write_all(self.key.as_bytes())?;
        len += self.key.len();

        // value
        let l = i32::try_from(self.value.len()).map_err(|e| KrostError::Malformed(Box::new(e)))?;
        len += VarInt(l).encode(buf, version)?;
        buf.write_all(&self.value)?;
        len += self.value.len();

        Ok(len)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Record {
    pub timestamp_delta: i64,
    pub offset_delta: i32,
    pub key: Option<Vec<u8>>,
    pub value: Option<Vec<u8>>,
    pub headers: Vec<RecordHeader>,
}

impl KrostType for Record {
    fn decode<D: Read>(buf: &mut D, version: i16) -> Result<Self, KrostError> {
        // length
        let len = VarInt::decode(buf, version)?;
        let len = u64::try_from(len.0).map_err(|e| KrostError::Malformed(Box::new(e)))?;
        let reader = &mut buf.take(len);

        // attributes
        i8::decode(reader, version)?;

        // timestampDelta
        let timestamp_delta = VarLong::decode(reader, version)?.0;

        // offsetDelta
        let offset_delta = VarInt::decode(reader, version)?.0;

        // key
        let len = VarInt::decode(reader, version)?.0;
        let key = if len == -1 {
            None
        } else {
            let len = usize::try_from(len).map_err(|e| KrostError::Malformed(Box::new(e)))?;
            let mut key = Vec::with_capacity(len);
            let _ = reader.read_exact(&mut key);
            Some(key)
        };

        // value
        let len = VarInt::decode(reader, version)?.0;
        let value = if len == -1 {
            None
        } else {
            let len = usize::try_from(len).map_err(|e| KrostError::Malformed(Box::new(e)))?;
            let mut value = Vec::with_capacity(len);
            let _ = reader.read_exact(&mut value);
            Some(value)
        };

        // headers
        // Note: This is NOT a normal array but uses a Varint instead.
        let n_headers = VarInt::decode(reader, version)?;
        let n_headers =
            usize::try_from(n_headers.0).map_err(|e| KrostError::Malformed(Box::new(e)))?;
        let mut headers = Vec::with_capacity(n_headers);
        for _ in 0..n_headers {
            headers.push(RecordHeader::decode(reader, version)?);
        }

        // check if there is any trailing data because this is likely a bug
        if reader.limit() != 0 {
            return Err(KrostError::Malformed(
                format!("Found {} trailing bytes after Record", reader.limit()).into(),
            ));
        }

        Ok(Self {
            timestamp_delta,
            offset_delta,
            key,
            value,
            headers,
        })
    }

    fn encode<E: Write>(&self, buf: &mut E, version: i16) -> Result<usize, KrostError> {
        let mut data = vec![];

        // attributes
        0i8.encode(&mut data, version)?;

        // timestampDelta
        VarLong(self.timestamp_delta).encode(&mut data, version)?;

        // offsetDelta
        VarInt(self.offset_delta).encode(&mut data, version)?;

        // key
        match &self.key {
            Some(key) => {
                let l = i32::try_from(key.len()).map_err(|e| KrostError::Malformed(Box::new(e)))?;
                VarInt(l).encode(&mut data, version)?;
                data.write_all(key)?;
            }
            None => {
                VarInt(-1).encode(&mut data, version)?;
            }
        }

        // value
        match &self.value {
            Some(value) => {
                let l =
                    i32::try_from(value.len()).map_err(|e| KrostError::Malformed(Box::new(e)))?;
                VarInt(l).encode(&mut data, version)?;
                data.write_all(value)?;
            }
            None => {
                VarInt(-1).encode(&mut data, version)?;
            }
        }

        // headers
        // Note: This is NOT a normal array but uses a VarInt instead.
        let l =
            i32::try_from(self.headers.len()).map_err(|e| KrostError::Malformed(Box::new(e)))?;
        VarInt(l).encode(&mut data, version)?;
        for header in &self.headers {
            header.encode(&mut data, version)?;
        }

        // now write accumulated data
        let l = i32::try_from(data.len()).map_err(|e| KrostError::Malformed(Box::new(e)))?;
        VarInt(l).encode(buf, version)?;
        buf.write_all(&data)?;

        Ok(4 + data.len())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlBatchRecord {
    Abort,
    Commit,
}

impl KrostType for ControlBatchRecord {
    fn decode<D: Read>(buf: &mut D, version: i16) -> Result<Self, KrostError> {
        // version
        let version = i16::decode(buf, version)?;
        if version != 0 {
            return Err(KrostError::Malformed(
                format!("Unknown control batch record version: {}", version).into(),
            ));
        }

        // type
        let t = i16::decode(buf, version)?;
        match t {
            0 => Ok(Self::Abort),
            1 => Ok(Self::Commit),
            _ => Err(KrostError::Malformed(
                format!("Unknown control batch record type: {}", t).into(),
            )),
        }
    }

    fn encode<E: Write>(&self, buf: &mut E, version: i16) -> Result<usize, KrostError> {
        // version
        let mut len = 0i16.encode(buf, version)?;

        // type
        let t: i16 = match self {
            Self::Abort => 0,
            Self::Commit => 1,
        };
        len += t.encode(buf, version)?;

        Ok(len)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ControlBatchOrRecords {
    ControlBatch(ControlBatchRecord),
    Records(Vec<Record>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordBatchCompression {
    NoCompression,
    Gzip,
    Snappy,
    Lz4,
    Zstd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordBatchTimestampType {
    CreateTime,
    LogAppendTime,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RecordBatch {
    pub base_offset: i64,
    pub partition_leader_epoch: i32,
    pub last_offset_delta: i32,
    pub first_timestamp: i64,
    pub max_timestamp: i64,
    pub producer_id: i64,
    pub producer_epoch: i16,
    pub base_sequence: i32,
    pub records: ControlBatchOrRecords,
    pub compression: RecordBatchCompression,
    pub is_transactional: bool,
    pub timestamp_type: RecordBatchTimestampType,
}

impl RecordBatch {
    fn decode_records<D>(
        buf: &mut D,
        is_control: bool,
        n_records: usize,
        version: i16,
    ) -> Result<ControlBatchOrRecords, KrostError>
    where
        D: Read,
    {
        if is_control {
            if n_records != 1 {
                return Err(KrostError::Malformed(
                    format!("Expected 1 control record but got {}", n_records).into(),
                ));
            }

            let record = ControlBatchRecord::decode(buf, version)?;
            Ok(ControlBatchOrRecords::ControlBatch(record))
        } else {
            let mut records = Vec::with_capacity(n_records);
            for _ in 0..n_records {
                records.push(Record::decode(buf, version)?);
            }
            Ok(ControlBatchOrRecords::Records(records))
        }
    }

    fn encode_records<E>(
        buf: &mut E,
        records: &ControlBatchOrRecords,
        version: i16,
    ) -> Result<usize, KrostError>
    where
        E: Write,
    {
        match records {
            ControlBatchOrRecords::ControlBatch(control_batch) => {
                control_batch.encode(buf, version)
            }
            ControlBatchOrRecords::Records(records) => {
                let mut len = 0;
                for record in records {
                    len += record.encode(buf, version)?;
                }
                Ok(len)
            }
        }
    }
}

impl KrostType for RecordBatch {
    fn decode<D: Read>(buf: &mut D, version: i16) -> Result<Self, KrostError> {
        let base_offset = i64::decode(buf, version)?;

        // batchLength
        //
        // Contains all fields AFTER the length field (so excluding `baseOffset` and `batchLength`). To determine the
        // size of the CRC-checked part we must substract all sized from this field to and including the CRC field.
        let len = i32::decode(buf, version)?;
        let len = usize::try_from(len).map_err(|e| KrostError::Malformed(Box::new(e)))?;
        let len = len
            .checked_sub(
                4 // partitionLeaderEpoch
                    + 1 // magic
                    + 4, // crc
            )
            .ok_or_else(|| {
                KrostError::Malformed(format!("Record batch len too small: {}", len).into())
            })?;

        // partitionLeaderEpoch
        let partition_leader_epoch = i32::decode(buf, version)?;

        // magic
        let magic = i8::decode(buf, version)?;
        if magic != 2 {
            return Err(KrostError::Malformed(
                format!("Invalid magic number in record batch: {}", magic).into(),
            ));
        }

        // crc
        let crc = i32::decode(buf, version)?;
        let crc = u32::from_be_bytes(crc.to_be_bytes());

        // data
        let mut data = Vec::with_capacity(len);
        buf.read_exact(&mut data)?;

        let actual_crc = crc32c::crc32c(&data);
        if crc != actual_crc {
            return Err(KrostError::Malformed(
                format!("CRC error, got 0x{:x}, expected 0x{:x}", actual_crc, crc).into(),
            ));
        }

        let data = Cursor::new(data);
        // ------
        // let body = RecordBatchBody::decode(&mut data)?;

        // attributes
        let attributes = i16::decode(buf, version)?;
        let compression = match attributes & 0x7 {
            0 => RecordBatchCompression::NoCompression,
            1 => RecordBatchCompression::Gzip,
            2 => RecordBatchCompression::Snappy,
            3 => RecordBatchCompression::Lz4,
            4 => RecordBatchCompression::Zstd,
            other => {
                return Err(KrostError::Malformed(
                    format!("Invalid compression type: {}", other).into(),
                ));
            }
        };
        let timestamp_type = if ((attributes >> 3) & 0x1) == 0 {
            RecordBatchTimestampType::CreateTime
        } else {
            RecordBatchTimestampType::LogAppendTime
        };
        let is_transactional = ((attributes >> 4) & 0x1) == 1;
        let is_control = ((attributes >> 5) & 0x1) == 1;

        // lastOffsetDelta
        let last_offset_delta = i32::decode(buf, version)?;

        // firstTimestamp
        let first_timestamp = i64::decode(buf, version)?;

        // maxTimestamp
        let max_timestamp = i64::decode(buf, version)?;

        // producerId
        let producer_id = i64::decode(buf, version)?;

        // producerEpoch
        let producer_epoch = i16::decode(buf, version)?;

        // baseSequence
        let base_sequence = i32::decode(buf, version)?;

        // records
        let n_records = match i32::decode(buf, version)? {
            -1 => 0,
            n => usize::try_from(n)?,
        };
        let records = match compression {
            RecordBatchCompression::NoCompression => {
                Self::decode_records(buf, is_control, n_records, version)?
            }
            #[cfg(feature = "compression-gzip")]
            RecordBatchCompression::Gzip => {
                use flate2::read::GzDecoder;

                let mut decoder = GzDecoder::new(buf);
                let records = Self::decode_records(&mut decoder, is_control, n_records, version)?;

                ensure_eof(&mut decoder, "Data left in gzip block")?;

                records
            }
            #[cfg(feature = "compression-lz4")]
            RecordBatchCompression::Lz4 => {
                use lz4::Decoder;

                let mut decoder = Decoder::new(buf)?;
                let records = Self::decode_records(&mut decoder, is_control, n_records, version)?;

                // the lz4 decoder requires us to consume the whole inner stream until we reach EOF
                ensure_eof(&mut decoder, "Data left in LZ4 block")?;

                let (_reader, res) = decoder.finish();
                res?;

                records
            }
            #[cfg(feature = "compression-snappy")]
            RecordBatchCompression::Snappy => {
                // Construct the input for the raw decoder.
                let mut input = vec![];
                buf.read_to_end(&mut input)?;

                const JAVA_MAGIC: &[u8] = &[0x82, b'S', b'N', b'A', b'P', b'P', b'Y', 0];

                // There are "normal" compression libs, and there is Java
                // See https://github.com/edenhill/librdkafka/blob/2b76b65212e5efda213961d5f84e565038036270/src/rdkafka_msgset_reader.c#L307-L318
                let output = if input.starts_with(JAVA_MAGIC) {
                    let mut cursor = Cursor::new(&input[JAVA_MAGIC.len()..]);

                    let mut buf_version = [0u8; 4];
                    cursor.read_exact(&mut buf_version)?;
                    if buf_version != [0, 0, 0, 1] {
                        return Err(KrostError::Malformed(
                            format!("Detected Java-specific Snappy compression, but got unknown version: {buf_version:?}").into(),
                        ));
                    }

                    let mut buf_compatible = [0u8; 4];
                    cursor.read_exact(&mut buf_compatible)?;
                    if buf_compatible != [0, 0, 0, 1] {
                        return Err(KrostError::Malformed(
                            format!("Detected Java-specific Snappy compression, but got unknown compat flags: {buf_compatible:?}").into(),
                        ));
                    }

                    let mut output = vec![];
                    while cursor.position() < cursor.get_ref().len() as u64 {
                        let mut buf_chunk_length = [0u8; 4];
                        cursor.read_exact(&mut buf_chunk_length)?;
                        let chunk_length = u32::from_be_bytes(buf_chunk_length) as usize;

                        let mut chunk_data = vec![0u8; chunk_length];
                        cursor.read_exact(&mut chunk_data)?;

                        let mut buf = carefully_decompress_snappy(&chunk_data)?;
                        output.append(&mut buf);
                    }

                    output
                } else {
                    carefully_decompress_snappy(&input)?
                };

                // Read uncompressed records.
                let mut decoder = Cursor::new(output);
                let records = Self::decode_records(&mut decoder, is_control, n_records, version)?;

                // Check that there's no data left within the uncompressed block.
                ensure_eof(&mut decoder, "Data left in Snappy block")?;

                records
            }
            #[cfg(feature = "compression-zstd")]
            RecordBatchCompression::Zstd => {
                use zstd::Decoder;

                let mut decoder = Decoder::new(buf)?;
                let records = Self::decode_records(&mut decoder, is_control, n_records, version)?;

                ensure_eof(&mut decoder, "Data left in zstd block")?;

                records
            }
            #[allow(unreachable_patterns)]
            _ => {
                return Err(KrostError::Malformed(
                    format!("Unimplemented compression: {:?}", compression).into(),
                ));
            }
        };
        // ------
        // check if there is any trailing data because this is likely a bug
        let bytes_read = data.position();
        let bytes_total = u64::try_from(data.into_inner().len()).map_err(KrostError::Overflow)?;
        let bytes_left = bytes_total - bytes_read;
        if bytes_left != 0 {
            return Err(KrostError::Malformed(
                format!("Found {} trailing bytes after RecordBatch", bytes_left).into(),
            ));
        }

        Ok(Self {
            base_offset,
            partition_leader_epoch,
            last_offset_delta,
            first_timestamp,
            max_timestamp,
            producer_id,
            producer_epoch,
            base_sequence,
            compression,
            timestamp_type,
            is_transactional,
            records,
        })
    }

    fn encode<E: Write>(&self, buf: &mut E, version: i16) -> Result<usize, KrostError> {
        // collect everything that should be part of the CRC calculation
        let mut data = vec![];

        // attributes
        let mut attributes: i16 = match self.compression {
            RecordBatchCompression::NoCompression => 0,
            RecordBatchCompression::Gzip => 1,
            RecordBatchCompression::Snappy => 2,
            RecordBatchCompression::Lz4 => 3,
            RecordBatchCompression::Zstd => 4,
        };
        match self.timestamp_type {
            RecordBatchTimestampType::CreateTime => (),
            RecordBatchTimestampType::LogAppendTime => {
                attributes |= 1 << 3;
            }
        }
        if self.is_transactional {
            attributes |= 1 << 4;
        }
        if matches!(self.records, ControlBatchOrRecords::ControlBatch(_)) {
            attributes |= 1 << 5;
        }
        attributes.encode(&mut data, version)?;

        // lastOffsetDelta
        self.last_offset_delta.encode(&mut data, version)?;

        // firstTimestamp
        self.first_timestamp.encode(&mut data, version)?;

        // maxTimestamp
        self.max_timestamp.encode(&mut data, version)?;

        // producerId
        self.producer_id.encode(&mut data, version)?;

        // producerEpoch
        self.producer_epoch.encode(&mut data, version)?;

        // baseSequence
        self.base_sequence.encode(&mut data, version)?;

        // records
        let n_records = match &self.records {
            ControlBatchOrRecords::ControlBatch(_) => 1,
            ControlBatchOrRecords::Records(records) => records.len(),
        };
        i32::try_from(n_records)?.encode(&mut data, version)?;
        match self.compression {
            RecordBatchCompression::NoCompression => {
                Self::encode_records(&mut data, &self.records, version)?;
            }
            #[cfg(feature = "compression-gzip")]
            RecordBatchCompression::Gzip => {
                use flate2::{write::GzEncoder, Compression};

                let mut encoder = GzEncoder::new(&mut data, Compression::default());
                Self::encode_records(&mut encoder, &self.records, version)?;
                encoder.finish()?;
            }
            #[cfg(feature = "compression-lz4")]
            RecordBatchCompression::Lz4 => {
                use lz4::{liblz4::BlockMode, EncoderBuilder};

                let mut encoder = EncoderBuilder::new()
                    .block_mode(
                        // the only one supported by Kafka
                        BlockMode::Independent,
                    )
                    .build(&mut data)?;
                Self::encode_records(&mut encoder, &self.records, version)?;
                let (_writer, res) = encoder.finish();
                res?;
            }
            #[cfg(feature = "compression-snappy")]
            RecordBatchCompression::Snappy => {
                use snap::raw::{max_compress_len, Encoder};

                let mut input = vec![];
                Self::encode_records(&mut input, &self.records, version)?;

                let mut encoder = Encoder::new();
                let mut output = vec![0; max_compress_len(input.len())];
                let len = encoder
                    .compress(&input, &mut output)
                    .map_err(|e| KrostError::Malformed(Box::new(e)))?;

                data.write_all(&output[..len])?;
            }
            #[cfg(feature = "compression-zstd")]
            RecordBatchCompression::Zstd => {
                use zstd::Encoder;

                let mut encoder = Encoder::new(&mut data, 0)?;
                Self::encode_records(&mut encoder, &self.records, version)?;
                encoder.finish()?;
            }
            #[allow(unreachable_patterns)]
            _ => {
                return Err(KrostError::Malformed(
                    format!("Unimplemented compression: {:?}", self.compression).into(),
                ));
            }
        }

        // ==========================================================================================
        // ==========================================================================================

        // baseOffset
        let mut len = self.base_offset.encode(buf, version)?;

        // batchLength
        //
        // Contains all fields AFTER the length field (so excluding `baseOffset` and `batchLength`, but including
        // `partitionLeaderEpoch`, `magic`, and `crc).
        //
        // See
        // https://github.com/kafka-rust/kafka-rust/blob/657202832806cda77d0a1801d618dc6c382b4d79/src/protocol/produce.rs#L224-L226
        let l = i32::try_from(
            data.len()
                + 4 // partitionLeaderEpoch
                + 1 // magic
                + 4, // crc
        )
        .map_err(|e| KrostError::Malformed(Box::new(e)))?;
        len += l.encode(buf, version)?;

        // partitionLeaderEpoch
        len += self.partition_leader_epoch.encode(buf, version)?;

        // magic
        len += 2i8.encode(buf, version)?;

        // crc
        // See
        // https://github.com/kafka-rust/kafka-rust/blob/a551b6231a7adc9b715552b635a69ac2856ec8a1/src/protocol/mod.rs#L161-L163
        // WARNING: the range in the code linked above is correct but the polynomial is wrong!
        let crc = crc32c::crc32c(&data);
        let crc = i32::from_be_bytes(crc.to_be_bytes());
        len += crc.encode(buf, version)?;

        // the actual CRC-checked data
        buf.write_all(&data)?;

        Ok(len + data.len())
    }
}

/// Ensure that given reader is at EOF.
fn ensure_eof<R>(buf: &mut R, msg: &str) -> Result<(), KrostError>
where
    R: Read,
{
    let mut buffer = [0u8; 1];
    match buf.read(&mut buffer) {
        Ok(0) => Ok(()),
        Ok(_) => Err(KrostError::Malformed(msg.to_string().into())),
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(()),
        Err(e) => Err(KrostError::IO(e)),
    }
}

#[cfg(feature = "compression-snappy")]
fn carefully_decompress_snappy(input: &[u8]) -> Result<Vec<u8>, KrostError> {
    use snap::raw::{decompress_len, Decoder};

    // The snappy compression used here is unframed aka "raw". So we first need to figure out the
    // uncompressed length. See
    //
    // - https://github.com/edenhill/librdkafka/blob/2b76b65212e5efda213961d5f84e565038036270/src/rdkafka_msgset_reader.c#L345-L348
    // - https://github.com/edenhill/librdkafka/blob/747f77c98fbddf7dc6508f76398e0fc9ee91450f/src/snappy.c#L779
    let uncompressed_size =
        decompress_len(input).map_err(|e| KrostError::Malformed(Box::new(e)))?;

    // Decode snappy payload.
    // The uncompressed length is unchecked and can be up to 2^32-1 bytes. To avoid a DDoS vector we try to
    // limit it to a small size and if that fails we double that size;
    let mut max_uncompressed_size = 1024 * 1024 * 10;

    loop {
        let try_uncompressed_size = uncompressed_size.min(max_uncompressed_size);

        let mut decoder = Decoder::new();
        let mut output = vec![0; try_uncompressed_size];
        let actual_uncompressed_size = match decoder.decompress(input, &mut output) {
            Ok(size) => size,
            Err(snap::Error::BufferTooSmall { .. })
                if max_uncompressed_size < uncompressed_size =>
            {
                // try larger buffer
                max_uncompressed_size *= 2;
                continue;
            }
            Err(e) => {
                return Err(KrostError::Malformed(Box::new(e)));
            }
        };
        if actual_uncompressed_size != uncompressed_size {
            return Err(KrostError::Malformed(
                "broken snappy data".to_string().into(),
            ));
        }

        break Ok(output);
    }
}
