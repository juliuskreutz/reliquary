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

//! Generated file from `ChallengeInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChallengeInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChallengeInfo {
    // message fields
    // @@protoc_insertion_point(field:ChallengeInfo.challenge_id)
    pub challenge_id: u32,
    // @@protoc_insertion_point(field:ChallengeInfo.story_info)
    pub story_info: ::protobuf::MessageField<super::ChallengeStoryInfo::ChallengeStoryInfo>,
    // @@protoc_insertion_point(field:ChallengeInfo.score)
    pub score: u32,
    // @@protoc_insertion_point(field:ChallengeInfo.round_count)
    pub round_count: u32,
    // @@protoc_insertion_point(field:ChallengeInfo.score_two)
    pub score_two: u32,
    // @@protoc_insertion_point(field:ChallengeInfo.extra_lineup_type)
    pub extra_lineup_type: ::protobuf::EnumOrUnknown<super::ExtraLineupType::ExtraLineupType>,
    // @@protoc_insertion_point(field:ChallengeInfo.status)
    pub status: ::protobuf::EnumOrUnknown<super::ChallengeStatus::ChallengeStatus>,
    // special fields
    // @@protoc_insertion_point(special_field:ChallengeInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChallengeInfo {
    fn default() -> &'a ChallengeInfo {
        <ChallengeInfo as ::protobuf::Message>::default_instance()
    }
}

impl ChallengeInfo {
    pub fn new() -> ChallengeInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "challenge_id",
            |m: &ChallengeInfo| { &m.challenge_id },
            |m: &mut ChallengeInfo| { &mut m.challenge_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChallengeStoryInfo::ChallengeStoryInfo>(
            "story_info",
            |m: &ChallengeInfo| { &m.story_info },
            |m: &mut ChallengeInfo| { &mut m.story_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score",
            |m: &ChallengeInfo| { &m.score },
            |m: &mut ChallengeInfo| { &mut m.score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "round_count",
            |m: &ChallengeInfo| { &m.round_count },
            |m: &mut ChallengeInfo| { &mut m.round_count },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score_two",
            |m: &ChallengeInfo| { &m.score_two },
            |m: &mut ChallengeInfo| { &mut m.score_two },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "extra_lineup_type",
            |m: &ChallengeInfo| { &m.extra_lineup_type },
            |m: &mut ChallengeInfo| { &mut m.extra_lineup_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &ChallengeInfo| { &m.status },
            |m: &mut ChallengeInfo| { &mut m.status },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChallengeInfo>(
            "ChallengeInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChallengeInfo {
    const NAME: &'static str = "ChallengeInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.challenge_id = is.read_uint32()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.story_info)?;
                },
                24 => {
                    self.score = is.read_uint32()?;
                },
                80 => {
                    self.round_count = is.read_uint32()?;
                },
                72 => {
                    self.score_two = is.read_uint32()?;
                },
                112 => {
                    self.extra_lineup_type = is.read_enum_or_unknown()?;
                },
                104 => {
                    self.status = is.read_enum_or_unknown()?;
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
        if self.challenge_id != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.challenge_id);
        }
        if let Some(v) = self.story_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.score != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.score);
        }
        if self.round_count != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.round_count);
        }
        if self.score_two != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.score_two);
        }
        if self.extra_lineup_type != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            my_size += ::protobuf::rt::int32_size(14, self.extra_lineup_type.value());
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::ChallengeStatus::ChallengeStatus::CHALLENGE_UNKNOWN) {
            my_size += ::protobuf::rt::int32_size(13, self.status.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.challenge_id != 0 {
            os.write_uint32(15, self.challenge_id)?;
        }
        if let Some(v) = self.story_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.score != 0 {
            os.write_uint32(3, self.score)?;
        }
        if self.round_count != 0 {
            os.write_uint32(10, self.round_count)?;
        }
        if self.score_two != 0 {
            os.write_uint32(9, self.score_two)?;
        }
        if self.extra_lineup_type != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.extra_lineup_type))?;
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::ChallengeStatus::ChallengeStatus::CHALLENGE_UNKNOWN) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.status))?;
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

    fn new() -> ChallengeInfo {
        ChallengeInfo::new()
    }

    fn clear(&mut self) {
        self.challenge_id = 0;
        self.story_info.clear();
        self.score = 0;
        self.round_count = 0;
        self.score_two = 0;
        self.extra_lineup_type = ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE);
        self.status = ::protobuf::EnumOrUnknown::new(super::ChallengeStatus::ChallengeStatus::CHALLENGE_UNKNOWN);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChallengeInfo {
        static instance: ChallengeInfo = ChallengeInfo {
            challenge_id: 0,
            story_info: ::protobuf::MessageField::none(),
            score: 0,
            round_count: 0,
            score_two: 0,
            extra_lineup_type: ::protobuf::EnumOrUnknown::from_i32(0),
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChallengeInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChallengeInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChallengeInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChallengeInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13ChallengeInfo.proto\x1a\x15ExtraLineupType.proto\x1a\x18ChallengeS\
    toryInfo.proto\x1a\x15ChallengeStatus.proto\"\xa2\x02\n\rChallengeInfo\
    \x12!\n\x0cchallenge_id\x18\x0f\x20\x01(\rR\x0bchallengeId\x122\n\nstory\
    _info\x18\x04\x20\x01(\x0b2\x13.ChallengeStoryInfoR\tstoryInfo\x12\x14\n\
    \x05score\x18\x03\x20\x01(\rR\x05score\x12\x1f\n\x0bround_count\x18\n\
    \x20\x01(\rR\nroundCount\x12\x1b\n\tscore_two\x18\t\x20\x01(\rR\x08score\
    Two\x12<\n\x11extra_lineup_type\x18\x0e\x20\x01(\x0e2\x10.ExtraLineupTyp\
    eR\x0fextraLineupType\x12(\n\x06status\x18\r\x20\x01(\x0e2\x10.Challenge\
    StatusR\x06statusB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::ExtraLineupType::file_descriptor().clone());
            deps.push(super::ChallengeStoryInfo::file_descriptor().clone());
            deps.push(super::ChallengeStatus::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChallengeInfo::generated_message_descriptor_data());
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
