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

//! Generated file from `TakeChallengeRewardScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TakeChallengeRewardScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TakeChallengeRewardScRsp {
    // message fields
    // @@protoc_insertion_point(field:TakeChallengeRewardScRsp.taken_reward_list)
    pub taken_reward_list: ::std::vec::Vec<super::TakenChallengeRewardInfo::TakenChallengeRewardInfo>,
    // @@protoc_insertion_point(field:TakeChallengeRewardScRsp.group_id)
    pub group_id: u32,
    // @@protoc_insertion_point(field:TakeChallengeRewardScRsp.retcode)
    pub retcode: u32,
    // special fields
    // @@protoc_insertion_point(special_field:TakeChallengeRewardScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TakeChallengeRewardScRsp {
    fn default() -> &'a TakeChallengeRewardScRsp {
        <TakeChallengeRewardScRsp as ::protobuf::Message>::default_instance()
    }
}

impl TakeChallengeRewardScRsp {
    pub fn new() -> TakeChallengeRewardScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "taken_reward_list",
            |m: &TakeChallengeRewardScRsp| { &m.taken_reward_list },
            |m: &mut TakeChallengeRewardScRsp| { &mut m.taken_reward_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "group_id",
            |m: &TakeChallengeRewardScRsp| { &m.group_id },
            |m: &mut TakeChallengeRewardScRsp| { &mut m.group_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &TakeChallengeRewardScRsp| { &m.retcode },
            |m: &mut TakeChallengeRewardScRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TakeChallengeRewardScRsp>(
            "TakeChallengeRewardScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TakeChallengeRewardScRsp {
    const NAME: &'static str = "TakeChallengeRewardScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    self.taken_reward_list.push(is.read_message()?);
                },
                40 => {
                    self.group_id = is.read_uint32()?;
                },
                120 => {
                    self.retcode = is.read_uint32()?;
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
        for value in &self.taken_reward_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.group_id);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.taken_reward_list {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.group_id != 0 {
            os.write_uint32(5, self.group_id)?;
        }
        if self.retcode != 0 {
            os.write_uint32(15, self.retcode)?;
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

    fn new() -> TakeChallengeRewardScRsp {
        TakeChallengeRewardScRsp::new()
    }

    fn clear(&mut self) {
        self.taken_reward_list.clear();
        self.group_id = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TakeChallengeRewardScRsp {
        static instance: TakeChallengeRewardScRsp = TakeChallengeRewardScRsp {
            taken_reward_list: ::std::vec::Vec::new(),
            group_id: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TakeChallengeRewardScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TakeChallengeRewardScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TakeChallengeRewardScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TakeChallengeRewardScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eTakeChallengeRewardScRsp.proto\x1a\x1eTakenChallengeRewardInfo.pro\
    to\"\x96\x01\n\x18TakeChallengeRewardScRsp\x12E\n\x11taken_reward_list\
    \x18\r\x20\x03(\x0b2\x19.TakenChallengeRewardInfoR\x0ftakenRewardList\
    \x12\x19\n\x08group_id\x18\x05\x20\x01(\rR\x07groupId\x12\x18\n\x07retco\
    de\x18\x0f\x20\x01(\rR\x07retcodeB\x15\n\x13emu.lunarcore.protob\x06prot\
    o3\
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
            deps.push(super::TakenChallengeRewardInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TakeChallengeRewardScRsp::generated_message_descriptor_data());
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