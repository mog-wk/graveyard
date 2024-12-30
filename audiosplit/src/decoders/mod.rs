// #[cfg(target_feature = "vorbis")]
pub mod vorbis;

#[cfg(target_feature = "flac")]
pub mod flac;

#[cfg(target_feature = "mp3")]
pub mod mp3;

pub trait Decoder<'a> {}

// impl for ogg first
#[derive(Debug)]
pub struct CodecFormat<'a> {
    id: &'a [u8], // OggS
    version: u8,
    header_type: u8,
    granule_position: &'a [u8],
    bitstream_serial_number: &'a [u8],
    page_sequence_number: &'a [u8],
    checksum: &'a [u8],
    page_segment: u8,
    segment_table: &'a [u8],
}

impl<'a> Decoder<'a> for CodecFormat<'a> {}

// TODO: impl specific length in bitstream
pub fn format_codec<'a>(bs: &'a [u8]) -> impl Decoder<'a> {
    CodecFormat {
        id: &bs[0..4],
        version: bs[4],
        header_type: bs[5],
        granule_position: &bs[6..14],
        bitstream_serial_number: &bs[14..18],
        page_sequence_number: &bs[18..22],
        checksum: &bs[22..26],
        page_segment: bs[26],
        segment_table: &bs[27..],
    }
}
// 0 	0x01 	Continuationhe first packet on this page is a continuation of the previous packet in the logical bitstream.
// 1 	0x02 	BOS 	Beginning Of Stream. This page is the first page in the logical bitstream.
// The BOS flag must be set on the first page of every logical bitstream, and must not be set on any other page.
// 2 	0x04 	EOS 	End Of Stream. This page is the last page in the logical bitstream.
// The EOS flag must be set on the final page of every logical bitstream, and must not be set on any other page.

pub enum HeaderType {
    Continuation,
    BOS,
    EOS,
}

impl From<u8> for HeaderType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Continuation,
            1 => Self::BOS,
            2 => Self::EOS,
            _ => panic!("unable to convert to HeaderType"),
        }
    }
}
