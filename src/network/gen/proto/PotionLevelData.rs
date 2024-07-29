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

//! Generated file from `PotionLevelData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PotionLevelData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PotionLevelData {
    // message fields
    // @@protoc_insertion_point(field:PotionLevelData.difficulty_level)
    pub difficulty_level: u32,
    // @@protoc_insertion_point(field:PotionLevelData.score)
    pub score: u32,
    // @@protoc_insertion_point(field:PotionLevelData.mode_id)
    pub mode_id: u32,
    // @@protoc_insertion_point(field:PotionLevelData.level_id)
    pub level_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PotionLevelData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PotionLevelData {
    fn default() -> &'a PotionLevelData {
        <PotionLevelData as ::protobuf::Message>::default_instance()
    }
}

impl PotionLevelData {
    pub fn new() -> PotionLevelData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "difficulty_level",
            |m: &PotionLevelData| { &m.difficulty_level },
            |m: &mut PotionLevelData| { &mut m.difficulty_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score",
            |m: &PotionLevelData| { &m.score },
            |m: &mut PotionLevelData| { &mut m.score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "mode_id",
            |m: &PotionLevelData| { &m.mode_id },
            |m: &mut PotionLevelData| { &mut m.mode_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level_id",
            |m: &PotionLevelData| { &m.level_id },
            |m: &mut PotionLevelData| { &mut m.level_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PotionLevelData>(
            "PotionLevelData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PotionLevelData {
    const NAME: &'static str = "PotionLevelData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.difficulty_level = is.read_uint32()?;
                },
                96 => {
                    self.score = is.read_uint32()?;
                },
                80 => {
                    self.mode_id = is.read_uint32()?;
                },
                120 => {
                    self.level_id = is.read_uint32()?;
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
        if self.difficulty_level != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.difficulty_level);
        }
        if self.score != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.score);
        }
        if self.mode_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.mode_id);
        }
        if self.level_id != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.level_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.difficulty_level != 0 {
            os.write_uint32(13, self.difficulty_level)?;
        }
        if self.score != 0 {
            os.write_uint32(12, self.score)?;
        }
        if self.mode_id != 0 {
            os.write_uint32(10, self.mode_id)?;
        }
        if self.level_id != 0 {
            os.write_uint32(15, self.level_id)?;
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

    fn new() -> PotionLevelData {
        PotionLevelData::new()
    }

    fn clear(&mut self) {
        self.difficulty_level = 0;
        self.score = 0;
        self.mode_id = 0;
        self.level_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PotionLevelData {
        static instance: PotionLevelData = PotionLevelData {
            difficulty_level: 0,
            score: 0,
            mode_id: 0,
            level_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PotionLevelData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PotionLevelData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PotionLevelData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PotionLevelData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15PotionLevelData.proto\"\x86\x01\n\x0fPotionLevelData\x12)\n\x10dif\
    ficulty_level\x18\r\x20\x01(\rR\x0fdifficultyLevel\x12\x14\n\x05score\
    \x18\x0c\x20\x01(\rR\x05score\x12\x17\n\x07mode_id\x18\n\x20\x01(\rR\x06\
    modeId\x12\x19\n\x08level_id\x18\x0f\x20\x01(\rR\x07levelIdB\x1b\n\x19em\
    u.grasscutter.net.protob\x06proto3\
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
            messages.push(PotionLevelData::generated_message_descriptor_data());
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