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

//! Generated file from `EffigyActivityDetailInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EffigyActivityDetailInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EffigyActivityDetailInfo {
    // message fields
    // @@protoc_insertion_point(field:EffigyActivityDetailInfo.taken_reward_index_list)
    pub taken_reward_index_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:EffigyActivityDetailInfo.last_difficulty_id)
    pub last_difficulty_id: u32,
    // @@protoc_insertion_point(field:EffigyActivityDetailInfo.daily_info_list)
    pub daily_info_list: ::std::vec::Vec<super::EffigyDailyInfo::EffigyDailyInfo>,
    // @@protoc_insertion_point(field:EffigyActivityDetailInfo.cur_score)
    pub cur_score: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EffigyActivityDetailInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EffigyActivityDetailInfo {
    fn default() -> &'a EffigyActivityDetailInfo {
        <EffigyActivityDetailInfo as ::protobuf::Message>::default_instance()
    }
}

impl EffigyActivityDetailInfo {
    pub fn new() -> EffigyActivityDetailInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "taken_reward_index_list",
            |m: &EffigyActivityDetailInfo| { &m.taken_reward_index_list },
            |m: &mut EffigyActivityDetailInfo| { &mut m.taken_reward_index_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "last_difficulty_id",
            |m: &EffigyActivityDetailInfo| { &m.last_difficulty_id },
            |m: &mut EffigyActivityDetailInfo| { &mut m.last_difficulty_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "daily_info_list",
            |m: &EffigyActivityDetailInfo| { &m.daily_info_list },
            |m: &mut EffigyActivityDetailInfo| { &mut m.daily_info_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_score",
            |m: &EffigyActivityDetailInfo| { &m.cur_score },
            |m: &mut EffigyActivityDetailInfo| { &mut m.cur_score },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EffigyActivityDetailInfo>(
            "EffigyActivityDetailInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EffigyActivityDetailInfo {
    const NAME: &'static str = "EffigyActivityDetailInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.taken_reward_index_list)?;
                },
                80 => {
                    self.taken_reward_index_list.push(is.read_uint32()?);
                },
                48 => {
                    self.last_difficulty_id = is.read_uint32()?;
                },
                90 => {
                    self.daily_info_list.push(is.read_message()?);
                },
                112 => {
                    self.cur_score = is.read_uint32()?;
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
        for value in &self.taken_reward_index_list {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        if self.last_difficulty_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.last_difficulty_id);
        }
        for value in &self.daily_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.cur_score != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.cur_score);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.taken_reward_index_list {
            os.write_uint32(10, *v)?;
        };
        if self.last_difficulty_id != 0 {
            os.write_uint32(6, self.last_difficulty_id)?;
        }
        for v in &self.daily_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if self.cur_score != 0 {
            os.write_uint32(14, self.cur_score)?;
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

    fn new() -> EffigyActivityDetailInfo {
        EffigyActivityDetailInfo::new()
    }

    fn clear(&mut self) {
        self.taken_reward_index_list.clear();
        self.last_difficulty_id = 0;
        self.daily_info_list.clear();
        self.cur_score = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EffigyActivityDetailInfo {
        static instance: EffigyActivityDetailInfo = EffigyActivityDetailInfo {
            taken_reward_index_list: ::std::vec::Vec::new(),
            last_difficulty_id: 0,
            daily_info_list: ::std::vec::Vec::new(),
            cur_score: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EffigyActivityDetailInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EffigyActivityDetailInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EffigyActivityDetailInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EffigyActivityDetailInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eEffigyActivityDetailInfo.proto\x1a\x15EffigyDailyInfo.proto\"\xd6\
    \x01\n\x18EffigyActivityDetailInfo\x125\n\x17taken_reward_index_list\x18\
    \n\x20\x03(\rR\x14takenRewardIndexList\x12,\n\x12last_difficulty_id\x18\
    \x06\x20\x01(\rR\x10lastDifficultyId\x128\n\x0fdaily_info_list\x18\x0b\
    \x20\x03(\x0b2\x10.EffigyDailyInfoR\rdailyInfoList\x12\x1b\n\tcur_score\
    \x18\x0e\x20\x01(\rR\x08curScoreB\x1b\n\x19emu.grasscutter.net.protob\
    \x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::EffigyDailyInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EffigyActivityDetailInfo::generated_message_descriptor_data());
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
