mod beu32_str_codec;
mod byte_slice_ref;
pub mod facet;
mod field_id_word_count_codec;
mod fst_set_codec;
mod obkv_codec;
mod roaring_bitmap;
mod roaring_bitmap_length;
mod script_language_codec;
mod str_beu32_codec;
mod str_ref;
mod str_str_u8_codec;

pub use byte_slice_ref::ByteSliceRefCodec;
pub use str_ref::StrRefCodec;

pub use self::beu32_str_codec::BEU32StrCodec;
pub use self::field_id_word_count_codec::FieldIdWordCountCodec;
pub use self::fst_set_codec::FstSetCodec;
pub use self::obkv_codec::ObkvCodec;
pub use self::roaring_bitmap::{BoRoaringBitmapCodec, CboRoaringBitmapCodec, RoaringBitmapCodec};
pub use self::roaring_bitmap_length::{
    BoRoaringBitmapLenCodec, CboRoaringBitmapLenCodec, RoaringBitmapLenCodec,
};
pub use self::script_language_codec::ScriptLanguageCodec;
pub use self::str_beu32_codec::{StrBEU16Codec, StrBEU32Codec};
pub use self::str_str_u8_codec::{U8StrStrCodec, UncheckedU8StrStrCodec};

pub trait BytesDecodeOwned {
    type DItem;

    fn bytes_decode_owned(bytes: &[u8]) -> Option<Self::DItem>;
}
