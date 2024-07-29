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

//! Generated file from `BrickBreakerLevelInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BrickBreakerLevelInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BrickBreakerLevelInfo {
    // message fields
    // @@protoc_insertion_point(field:BrickBreakerLevelInfo.level_id)
    pub level_id: u32,
    // @@protoc_insertion_point(field:BrickBreakerLevelInfo.chosen_avatar_list)
    pub chosen_avatar_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BrickBreakerLevelInfo.chosen_skill_list)
    pub chosen_skill_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:BrickBreakerLevelInfo.max_score)
    pub max_score: u32,
    // @@protoc_insertion_point(field:BrickBreakerLevelInfo.is_finish)
    pub is_finish: bool,
    // @@protoc_insertion_point(field:BrickBreakerLevelInfo.JNFKFCEMOFM)
    pub JNFKFCEMOFM: bool,
    // special fields
    // @@protoc_insertion_point(special_field:BrickBreakerLevelInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BrickBreakerLevelInfo {
    fn default() -> &'a BrickBreakerLevelInfo {
        <BrickBreakerLevelInfo as ::protobuf::Message>::default_instance()
    }
}

impl BrickBreakerLevelInfo {
    pub fn new() -> BrickBreakerLevelInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level_id",
            |m: &BrickBreakerLevelInfo| { &m.level_id },
            |m: &mut BrickBreakerLevelInfo| { &mut m.level_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "chosen_avatar_list",
            |m: &BrickBreakerLevelInfo| { &m.chosen_avatar_list },
            |m: &mut BrickBreakerLevelInfo| { &mut m.chosen_avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "chosen_skill_list",
            |m: &BrickBreakerLevelInfo| { &m.chosen_skill_list },
            |m: &mut BrickBreakerLevelInfo| { &mut m.chosen_skill_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_score",
            |m: &BrickBreakerLevelInfo| { &m.max_score },
            |m: &mut BrickBreakerLevelInfo| { &mut m.max_score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_finish",
            |m: &BrickBreakerLevelInfo| { &m.is_finish },
            |m: &mut BrickBreakerLevelInfo| { &mut m.is_finish },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JNFKFCEMOFM",
            |m: &BrickBreakerLevelInfo| { &m.JNFKFCEMOFM },
            |m: &mut BrickBreakerLevelInfo| { &mut m.JNFKFCEMOFM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BrickBreakerLevelInfo>(
            "BrickBreakerLevelInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BrickBreakerLevelInfo {
    const NAME: &'static str = "BrickBreakerLevelInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.level_id = is.read_uint32()?;
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.chosen_avatar_list)?;
                },
                8 => {
                    self.chosen_avatar_list.push(is.read_uint32()?);
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.chosen_skill_list)?;
                },
                104 => {
                    self.chosen_skill_list.push(is.read_uint32()?);
                },
                96 => {
                    self.max_score = is.read_uint32()?;
                },
                88 => {
                    self.is_finish = is.read_bool()?;
                },
                64 => {
                    self.JNFKFCEMOFM = is.read_bool()?;
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
        if self.level_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.level_id);
        }
        for value in &self.chosen_avatar_list {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        for value in &self.chosen_skill_list {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        if self.max_score != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.max_score);
        }
        if self.is_finish != false {
            my_size += 1 + 1;
        }
        if self.JNFKFCEMOFM != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level_id != 0 {
            os.write_uint32(3, self.level_id)?;
        }
        for v in &self.chosen_avatar_list {
            os.write_uint32(1, *v)?;
        };
        for v in &self.chosen_skill_list {
            os.write_uint32(13, *v)?;
        };
        if self.max_score != 0 {
            os.write_uint32(12, self.max_score)?;
        }
        if self.is_finish != false {
            os.write_bool(11, self.is_finish)?;
        }
        if self.JNFKFCEMOFM != false {
            os.write_bool(8, self.JNFKFCEMOFM)?;
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

    fn new() -> BrickBreakerLevelInfo {
        BrickBreakerLevelInfo::new()
    }

    fn clear(&mut self) {
        self.level_id = 0;
        self.chosen_avatar_list.clear();
        self.chosen_skill_list.clear();
        self.max_score = 0;
        self.is_finish = false;
        self.JNFKFCEMOFM = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BrickBreakerLevelInfo {
        static instance: BrickBreakerLevelInfo = BrickBreakerLevelInfo {
            level_id: 0,
            chosen_avatar_list: ::std::vec::Vec::new(),
            chosen_skill_list: ::std::vec::Vec::new(),
            max_score: 0,
            is_finish: false,
            JNFKFCEMOFM: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BrickBreakerLevelInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BrickBreakerLevelInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BrickBreakerLevelInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BrickBreakerLevelInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bBrickBreakerLevelInfo.proto\"\xe8\x01\n\x15BrickBreakerLevelInfo\
    \x12\x19\n\x08level_id\x18\x03\x20\x01(\rR\x07levelId\x12,\n\x12chosen_a\
    vatar_list\x18\x01\x20\x03(\rR\x10chosenAvatarList\x12*\n\x11chosen_skil\
    l_list\x18\r\x20\x03(\rR\x0fchosenSkillList\x12\x1b\n\tmax_score\x18\x0c\
    \x20\x01(\rR\x08maxScore\x12\x1b\n\tis_finish\x18\x0b\x20\x01(\x08R\x08i\
    sFinish\x12\x20\n\x0bJNFKFCEMOFM\x18\x08\x20\x01(\x08R\x0bJNFKFCEMOFMB\
    \x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(BrickBreakerLevelInfo::generated_message_descriptor_data());
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
