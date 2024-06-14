// @generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: org/podcastindex/podping/podping_medium.capnp


#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PodpingMedium {
  Mixed = 0,
  Podcast = 1,
  PodcastL = 2,
  Music = 3,
  MusicL = 4,
  Video = 5,
  VideoL = 6,
  Film = 7,
  FilmL = 8,
  Audiobook = 9,
  AudiobookL = 10,
  Newsletter = 11,
  NewsletterL = 12,
  Blog = 13,
  BlogL = 14,
}

impl ::capnp::introspect::Introspect for PodpingMedium {
  fn introspect() -> ::capnp::introspect::Type { ::capnp::introspect::TypeVariant::Enum(::capnp::introspect::RawEnumSchema { encoded_node: &podping_medium::ENCODED_NODE, annotation_types: podping_medium::get_annotation_types }).into() }
}
impl <'a> ::core::convert::From<PodpingMedium> for ::capnp::dynamic_value::Reader<'a> {
  fn from(e: PodpingMedium) -> Self { ::capnp::dynamic_value::Enum::new(e.into(), ::capnp::introspect::RawEnumSchema { encoded_node: &podping_medium::ENCODED_NODE, annotation_types: podping_medium::get_annotation_types }.into()).into() }
}
impl ::core::convert::TryFrom<u16> for PodpingMedium {
  type Error = ::capnp::NotInSchema;
  fn try_from(value: u16) -> ::core::result::Result<Self, <PodpingMedium as ::core::convert::TryFrom<u16>>::Error> {
    match value {
      0 => ::core::result::Result::Ok(Self::Mixed),
      1 => ::core::result::Result::Ok(Self::Podcast),
      2 => ::core::result::Result::Ok(Self::PodcastL),
      3 => ::core::result::Result::Ok(Self::Music),
      4 => ::core::result::Result::Ok(Self::MusicL),
      5 => ::core::result::Result::Ok(Self::Video),
      6 => ::core::result::Result::Ok(Self::VideoL),
      7 => ::core::result::Result::Ok(Self::Film),
      8 => ::core::result::Result::Ok(Self::FilmL),
      9 => ::core::result::Result::Ok(Self::Audiobook),
      10 => ::core::result::Result::Ok(Self::AudiobookL),
      11 => ::core::result::Result::Ok(Self::Newsletter),
      12 => ::core::result::Result::Ok(Self::NewsletterL),
      13 => ::core::result::Result::Ok(Self::Blog),
      14 => ::core::result::Result::Ok(Self::BlogL),
      n => ::core::result::Result::Err(::capnp::NotInSchema(n)),
    }
  }
}
impl From<PodpingMedium> for u16 {
  #[inline]
  fn from(x: PodpingMedium) -> u16 { x as u16 }
}
impl ::capnp::traits::HasTypeId for PodpingMedium {
  const TYPE_ID: u64 = 0xaae8_1675_a576_a280u64;
}
mod podping_medium {
pub static ENCODED_NODE: [::capnp::Word; 87] = [
  ::capnp::word(0, 0, 0, 0, 5, 0, 6, 0),
  ::capnp::word(128, 162, 118, 165, 117, 22, 232, 170),
  ::capnp::word(46, 0, 0, 0, 2, 0, 0, 0),
  ::capnp::word(254, 38, 182, 200, 31, 143, 218, 237),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(21, 0, 0, 0, 226, 1, 0, 0),
  ::capnp::word(49, 0, 0, 0, 7, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(45, 0, 0, 0, 111, 1, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(111, 114, 103, 47, 112, 111, 100, 99),
  ::capnp::word(97, 115, 116, 105, 110, 100, 101, 120),
  ::capnp::word(47, 112, 111, 100, 112, 105, 110, 103),
  ::capnp::word(47, 112, 111, 100, 112, 105, 110, 103),
  ::capnp::word(95, 109, 101, 100, 105, 117, 109, 46),
  ::capnp::word(99, 97, 112, 110, 112, 58, 80, 111),
  ::capnp::word(100, 112, 105, 110, 103, 77, 101, 100),
  ::capnp::word(105, 117, 109, 0, 0, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 1, 0, 1, 0),
  ::capnp::word(60, 0, 0, 0, 1, 0, 2, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(173, 0, 0, 0, 50, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(1, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(165, 0, 0, 0, 66, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(2, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(157, 0, 0, 0, 74, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(3, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(153, 0, 0, 0, 50, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(4, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(145, 0, 0, 0, 58, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(5, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(137, 0, 0, 0, 50, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(6, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(129, 0, 0, 0, 58, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(7, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(121, 0, 0, 0, 42, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(8, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(113, 0, 0, 0, 50, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(9, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(105, 0, 0, 0, 82, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(10, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(101, 0, 0, 0, 90, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(11, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(97, 0, 0, 0, 90, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(12, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(93, 0, 0, 0, 98, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(13, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(89, 0, 0, 0, 42, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(14, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(81, 0, 0, 0, 50, 0, 0, 0),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(109, 105, 120, 101, 100, 0, 0, 0),
  ::capnp::word(112, 111, 100, 99, 97, 115, 116, 0),
  ::capnp::word(112, 111, 100, 99, 97, 115, 116, 76),
  ::capnp::word(0, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(109, 117, 115, 105, 99, 0, 0, 0),
  ::capnp::word(109, 117, 115, 105, 99, 76, 0, 0),
  ::capnp::word(118, 105, 100, 101, 111, 0, 0, 0),
  ::capnp::word(118, 105, 100, 101, 111, 76, 0, 0),
  ::capnp::word(102, 105, 108, 109, 0, 0, 0, 0),
  ::capnp::word(102, 105, 108, 109, 76, 0, 0, 0),
  ::capnp::word(97, 117, 100, 105, 111, 98, 111, 111),
  ::capnp::word(107, 0, 0, 0, 0, 0, 0, 0),
  ::capnp::word(97, 117, 100, 105, 111, 98, 111, 111),
  ::capnp::word(107, 76, 0, 0, 0, 0, 0, 0),
  ::capnp::word(110, 101, 119, 115, 108, 101, 116, 116),
  ::capnp::word(101, 114, 0, 0, 0, 0, 0, 0),
  ::capnp::word(110, 101, 119, 115, 108, 101, 116, 116),
  ::capnp::word(101, 114, 76, 0, 0, 0, 0, 0),
  ::capnp::word(98, 108, 111, 103, 0, 0, 0, 0),
  ::capnp::word(98, 108, 111, 103, 76, 0, 0, 0),
];
pub fn get_annotation_types(child_index: Option<u16>, index: u32) -> ::capnp::introspect::Type {
  panic!("invalid annotation indices ({:?}, {}) ", child_index, index)
}
}
