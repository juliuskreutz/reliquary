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

//! Generated file from `ChannelerSlabLoopDungeonResultInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChannelerSlabLoopDungeonResultInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChannelerSlabLoopDungeonResultInfo {
    // message fields
    // @@protoc_insertion_point(field:ChannelerSlabLoopDungeonResultInfo.challenge_max_score)
    pub challenge_max_score: u32,
    // @@protoc_insertion_point(field:ChannelerSlabLoopDungeonResultInfo.dungeon_index)
    pub dungeon_index: u32,
    // @@protoc_insertion_point(field:ChannelerSlabLoopDungeonResultInfo.is_success)
    pub is_success: bool,
    // @@protoc_insertion_point(field:ChannelerSlabLoopDungeonResultInfo.challenge_score)
    pub challenge_score: u32,
    // @@protoc_insertion_point(field:ChannelerSlabLoopDungeonResultInfo.is_in_time_limit)
    pub is_in_time_limit: bool,
    // special fields
    // @@protoc_insertion_point(special_field:ChannelerSlabLoopDungeonResultInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChannelerSlabLoopDungeonResultInfo {
    fn default() -> &'a ChannelerSlabLoopDungeonResultInfo {
        <ChannelerSlabLoopDungeonResultInfo as ::protobuf::Message>::default_instance()
    }
}

impl ChannelerSlabLoopDungeonResultInfo {
    pub fn new() -> ChannelerSlabLoopDungeonResultInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "challenge_max_score",
            |m: &ChannelerSlabLoopDungeonResultInfo| { &m.challenge_max_score },
            |m: &mut ChannelerSlabLoopDungeonResultInfo| { &mut m.challenge_max_score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dungeon_index",
            |m: &ChannelerSlabLoopDungeonResultInfo| { &m.dungeon_index },
            |m: &mut ChannelerSlabLoopDungeonResultInfo| { &mut m.dungeon_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_success",
            |m: &ChannelerSlabLoopDungeonResultInfo| { &m.is_success },
            |m: &mut ChannelerSlabLoopDungeonResultInfo| { &mut m.is_success },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "challenge_score",
            |m: &ChannelerSlabLoopDungeonResultInfo| { &m.challenge_score },
            |m: &mut ChannelerSlabLoopDungeonResultInfo| { &mut m.challenge_score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_in_time_limit",
            |m: &ChannelerSlabLoopDungeonResultInfo| { &m.is_in_time_limit },
            |m: &mut ChannelerSlabLoopDungeonResultInfo| { &mut m.is_in_time_limit },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChannelerSlabLoopDungeonResultInfo>(
            "ChannelerSlabLoopDungeonResultInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChannelerSlabLoopDungeonResultInfo {
    const NAME: &'static str = "ChannelerSlabLoopDungeonResultInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.challenge_max_score = is.read_uint32()?;
                },
                16 => {
                    self.dungeon_index = is.read_uint32()?;
                },
                24 => {
                    self.is_success = is.read_bool()?;
                },
                48 => {
                    self.challenge_score = is.read_uint32()?;
                },
                104 => {
                    self.is_in_time_limit = is.read_bool()?;
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
        if self.challenge_max_score != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.challenge_max_score);
        }
        if self.dungeon_index != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.dungeon_index);
        }
        if self.is_success != false {
            my_size += 1 + 1;
        }
        if self.challenge_score != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.challenge_score);
        }
        if self.is_in_time_limit != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.challenge_max_score != 0 {
            os.write_uint32(1, self.challenge_max_score)?;
        }
        if self.dungeon_index != 0 {
            os.write_uint32(2, self.dungeon_index)?;
        }
        if self.is_success != false {
            os.write_bool(3, self.is_success)?;
        }
        if self.challenge_score != 0 {
            os.write_uint32(6, self.challenge_score)?;
        }
        if self.is_in_time_limit != false {
            os.write_bool(13, self.is_in_time_limit)?;
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

    fn new() -> ChannelerSlabLoopDungeonResultInfo {
        ChannelerSlabLoopDungeonResultInfo::new()
    }

    fn clear(&mut self) {
        self.challenge_max_score = 0;
        self.dungeon_index = 0;
        self.is_success = false;
        self.challenge_score = 0;
        self.is_in_time_limit = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChannelerSlabLoopDungeonResultInfo {
        static instance: ChannelerSlabLoopDungeonResultInfo = ChannelerSlabLoopDungeonResultInfo {
            challenge_max_score: 0,
            dungeon_index: 0,
            is_success: false,
            challenge_score: 0,
            is_in_time_limit: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChannelerSlabLoopDungeonResultInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChannelerSlabLoopDungeonResultInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChannelerSlabLoopDungeonResultInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChannelerSlabLoopDungeonResultInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(ChannelerSlabLoopDungeonResultInfo.proto\"\xea\x01\n\"ChannelerSlabLo\
    opDungeonResultInfo\x12.\n\x13challenge_max_score\x18\x01\x20\x01(\rR\
    \x11challengeMaxScore\x12#\n\rdungeon_index\x18\x02\x20\x01(\rR\x0cdunge\
    onIndex\x12\x1d\n\nis_success\x18\x03\x20\x01(\x08R\tisSuccess\x12'\n\
    \x0fchallenge_score\x18\x06\x20\x01(\rR\x0echallengeScore\x12'\n\x10is_i\
    n_time_limit\x18\r\x20\x01(\x08R\risInTimeLimitB\x1b\n\x19emu.grasscutte\
    r.net.protob\x06proto3\
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
            messages.push(ChannelerSlabLoopDungeonResultInfo::generated_message_descriptor_data());
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