// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `MusicGameActivityDetailInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MusicGameActivityDetailInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MusicGameActivityDetailInfo {
    // message fields
    // @@protoc_insertion_point(field:MusicGameActivityDetailInfo.LDBACGEPLLE)
    pub LDBACGEPLLE: ::std::vec::Vec<super::UgcMusicBriefInfo::UgcMusicBriefInfo>,
    // @@protoc_insertion_point(field:MusicGameActivityDetailInfo.music_game_record_map)
    pub music_game_record_map: ::std::collections::HashMap<u32, super::MusicGameRecord::MusicGameRecord>,
    // @@protoc_insertion_point(field:MusicGameActivityDetailInfo.OHEAGJIPKGF)
    pub OHEAGJIPKGF: ::std::vec::Vec<super::UgcMusicBriefInfo::UgcMusicBriefInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:MusicGameActivityDetailInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MusicGameActivityDetailInfo {
    fn default() -> &'a MusicGameActivityDetailInfo {
        <MusicGameActivityDetailInfo as ::protobuf::Message>::default_instance()
    }
}

impl MusicGameActivityDetailInfo {
    pub fn new() -> MusicGameActivityDetailInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LDBACGEPLLE",
            |m: &MusicGameActivityDetailInfo| { &m.LDBACGEPLLE },
            |m: &mut MusicGameActivityDetailInfo| { &mut m.LDBACGEPLLE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "music_game_record_map",
            |m: &MusicGameActivityDetailInfo| { &m.music_game_record_map },
            |m: &mut MusicGameActivityDetailInfo| { &mut m.music_game_record_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "OHEAGJIPKGF",
            |m: &MusicGameActivityDetailInfo| { &m.OHEAGJIPKGF },
            |m: &mut MusicGameActivityDetailInfo| { &mut m.OHEAGJIPKGF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MusicGameActivityDetailInfo>(
            "MusicGameActivityDetailInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MusicGameActivityDetailInfo {
    const NAME: &'static str = "MusicGameActivityDetailInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                74 => {
                    self.LDBACGEPLLE.push(is.read_message()?);
                },
                34 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.music_game_record_map.insert(key, value);
                },
                114 => {
                    self.OHEAGJIPKGF.push(is.read_message()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        for value in &self.LDBACGEPLLE {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for (k, v) in &self.music_game_record_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.OHEAGJIPKGF {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.LDBACGEPLLE {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        for (k, v) in &self.music_game_record_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(34)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        for v in &self.OHEAGJIPKGF {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MusicGameActivityDetailInfo {
        MusicGameActivityDetailInfo::new()
    }

    fn clear(&mut self) {
        self.LDBACGEPLLE.clear();
        self.music_game_record_map.clear();
        self.OHEAGJIPKGF.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MusicGameActivityDetailInfo {
        static instance: ::protobuf::rt::Lazy<MusicGameActivityDetailInfo> = ::protobuf::rt::Lazy::new();
        instance.get(MusicGameActivityDetailInfo::new)
    }
}

impl ::protobuf::MessageFull for MusicGameActivityDetailInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MusicGameActivityDetailInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MusicGameActivityDetailInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MusicGameActivityDetailInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!MusicGameActivityDetailInfo.proto\x1a\x17UgcMusicBriefInfo.proto\x1a\
    \x15MusicGameRecord.proto\"\xcb\x02\n\x1bMusicGameActivityDetailInfo\x12\
    4\n\x0bLDBACGEPLLE\x18\t\x20\x03(\x0b2\x12.UgcMusicBriefInfoR\x0bLDBACGE\
    PLLE\x12g\n\x15music_game_record_map\x18\x04\x20\x03(\x0b24.MusicGameAct\
    ivityDetailInfo.MusicGameRecordMapEntryR\x12musicGameRecordMap\x124\n\
    \x0bOHEAGJIPKGF\x18\x0e\x20\x03(\x0b2\x12.UgcMusicBriefInfoR\x0bOHEAGJIP\
    KGF\x1aW\n\x17MusicGameRecordMapEntry\x12\x10\n\x03key\x18\x01\x20\x01(\
    \rR\x03key\x12&\n\x05value\x18\x02\x20\x01(\x0b2\x10.MusicGameRecordR\
    \x05value:\x028\x01B\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::UgcMusicBriefInfo::file_descriptor().clone());
            deps.push(super::MusicGameRecord::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MusicGameActivityDetailInfo::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
