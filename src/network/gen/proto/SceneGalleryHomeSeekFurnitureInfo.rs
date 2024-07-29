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

//! Generated file from `SceneGalleryHomeSeekFurnitureInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SceneGalleryHomeSeekFurnitureInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneGalleryHomeSeekFurnitureInfo {
    // message fields
    // @@protoc_insertion_point(field:SceneGalleryHomeSeekFurnitureInfo.MPKMCJKPIAI)
    pub MPKMCJKPIAI: u32,
    // @@protoc_insertion_point(field:SceneGalleryHomeSeekFurnitureInfo.player_score_map)
    pub player_score_map: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:SceneGalleryHomeSeekFurnitureInfo.JMKGIKHIGKF)
    pub JMKGIKHIGKF: u32,
    // @@protoc_insertion_point(field:SceneGalleryHomeSeekFurnitureInfo.FOMNOCFGOCP)
    pub FOMNOCFGOCP: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SceneGalleryHomeSeekFurnitureInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneGalleryHomeSeekFurnitureInfo {
    fn default() -> &'a SceneGalleryHomeSeekFurnitureInfo {
        <SceneGalleryHomeSeekFurnitureInfo as ::protobuf::Message>::default_instance()
    }
}

impl SceneGalleryHomeSeekFurnitureInfo {
    pub fn new() -> SceneGalleryHomeSeekFurnitureInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MPKMCJKPIAI",
            |m: &SceneGalleryHomeSeekFurnitureInfo| { &m.MPKMCJKPIAI },
            |m: &mut SceneGalleryHomeSeekFurnitureInfo| { &mut m.MPKMCJKPIAI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "player_score_map",
            |m: &SceneGalleryHomeSeekFurnitureInfo| { &m.player_score_map },
            |m: &mut SceneGalleryHomeSeekFurnitureInfo| { &mut m.player_score_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JMKGIKHIGKF",
            |m: &SceneGalleryHomeSeekFurnitureInfo| { &m.JMKGIKHIGKF },
            |m: &mut SceneGalleryHomeSeekFurnitureInfo| { &mut m.JMKGIKHIGKF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FOMNOCFGOCP",
            |m: &SceneGalleryHomeSeekFurnitureInfo| { &m.FOMNOCFGOCP },
            |m: &mut SceneGalleryHomeSeekFurnitureInfo| { &mut m.FOMNOCFGOCP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneGalleryHomeSeekFurnitureInfo>(
            "SceneGalleryHomeSeekFurnitureInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneGalleryHomeSeekFurnitureInfo {
    const NAME: &'static str = "SceneGalleryHomeSeekFurnitureInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.MPKMCJKPIAI = is.read_uint32()?;
                },
                34 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.player_score_map.insert(key, value);
                },
                120 => {
                    self.JMKGIKHIGKF = is.read_uint32()?;
                },
                48 => {
                    self.FOMNOCFGOCP = is.read_uint32()?;
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
        if self.MPKMCJKPIAI != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.MPKMCJKPIAI);
        }
        for (k, v) in &self.player_score_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.JMKGIKHIGKF != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.JMKGIKHIGKF);
        }
        if self.FOMNOCFGOCP != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.FOMNOCFGOCP);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.MPKMCJKPIAI != 0 {
            os.write_uint32(2, self.MPKMCJKPIAI)?;
        }
        for (k, v) in &self.player_score_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(34)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if self.JMKGIKHIGKF != 0 {
            os.write_uint32(15, self.JMKGIKHIGKF)?;
        }
        if self.FOMNOCFGOCP != 0 {
            os.write_uint32(6, self.FOMNOCFGOCP)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SceneGalleryHomeSeekFurnitureInfo {
        SceneGalleryHomeSeekFurnitureInfo::new()
    }

    fn clear(&mut self) {
        self.MPKMCJKPIAI = 0;
        self.player_score_map.clear();
        self.JMKGIKHIGKF = 0;
        self.FOMNOCFGOCP = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneGalleryHomeSeekFurnitureInfo {
        static instance: ::protobuf::rt::Lazy<SceneGalleryHomeSeekFurnitureInfo> = ::protobuf::rt::Lazy::new();
        instance.get(SceneGalleryHomeSeekFurnitureInfo::new)
    }
}

impl ::protobuf::MessageFull for SceneGalleryHomeSeekFurnitureInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneGalleryHomeSeekFurnitureInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneGalleryHomeSeekFurnitureInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneGalleryHomeSeekFurnitureInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'SceneGalleryHomeSeekFurnitureInfo.proto\"\xae\x02\n!SceneGalleryHomeS\
    eekFurnitureInfo\x12\x20\n\x0bMPKMCJKPIAI\x18\x02\x20\x01(\rR\x0bMPKMCJK\
    PIAI\x12`\n\x10player_score_map\x18\x04\x20\x03(\x0b26.SceneGalleryHomeS\
    eekFurnitureInfo.PlayerScoreMapEntryR\x0eplayerScoreMap\x12\x20\n\x0bJMK\
    GIKHIGKF\x18\x0f\x20\x01(\rR\x0bJMKGIKHIGKF\x12\x20\n\x0bFOMNOCFGOCP\x18\
    \x06\x20\x01(\rR\x0bFOMNOCFGOCP\x1aA\n\x13PlayerScoreMapEntry\x12\x10\n\
    \x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\r\
    R\x05value:\x028\x01B\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneGalleryHomeSeekFurnitureInfo::generated_message_descriptor_data());
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