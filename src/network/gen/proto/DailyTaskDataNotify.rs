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

//! Generated file from `DailyTaskDataNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DailyTaskDataNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DailyTaskDataNotify {
    // message fields
    // @@protoc_insertion_point(field:DailyTaskDataNotify.is_taken_score_reward)
    pub is_taken_score_reward: bool,
    // @@protoc_insertion_point(field:DailyTaskDataNotify.score_reward_id)
    pub score_reward_id: u32,
    // @@protoc_insertion_point(field:DailyTaskDataNotify.finished_num)
    pub finished_num: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DailyTaskDataNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DailyTaskDataNotify {
    fn default() -> &'a DailyTaskDataNotify {
        <DailyTaskDataNotify as ::protobuf::Message>::default_instance()
    }
}

impl DailyTaskDataNotify {
    pub fn new() -> DailyTaskDataNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_taken_score_reward",
            |m: &DailyTaskDataNotify| { &m.is_taken_score_reward },
            |m: &mut DailyTaskDataNotify| { &mut m.is_taken_score_reward },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score_reward_id",
            |m: &DailyTaskDataNotify| { &m.score_reward_id },
            |m: &mut DailyTaskDataNotify| { &mut m.score_reward_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "finished_num",
            |m: &DailyTaskDataNotify| { &m.finished_num },
            |m: &mut DailyTaskDataNotify| { &mut m.finished_num },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DailyTaskDataNotify>(
            "DailyTaskDataNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DailyTaskDataNotify {
    const NAME: &'static str = "DailyTaskDataNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.is_taken_score_reward = is.read_bool()?;
                },
                24 => {
                    self.score_reward_id = is.read_uint32()?;
                },
                104 => {
                    self.finished_num = is.read_uint32()?;
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
        if self.is_taken_score_reward != false {
            my_size += 1 + 1;
        }
        if self.score_reward_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.score_reward_id);
        }
        if self.finished_num != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.finished_num);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_taken_score_reward != false {
            os.write_bool(11, self.is_taken_score_reward)?;
        }
        if self.score_reward_id != 0 {
            os.write_uint32(3, self.score_reward_id)?;
        }
        if self.finished_num != 0 {
            os.write_uint32(13, self.finished_num)?;
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

    fn new() -> DailyTaskDataNotify {
        DailyTaskDataNotify::new()
    }

    fn clear(&mut self) {
        self.is_taken_score_reward = false;
        self.score_reward_id = 0;
        self.finished_num = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DailyTaskDataNotify {
        static instance: DailyTaskDataNotify = DailyTaskDataNotify {
            is_taken_score_reward: false,
            score_reward_id: 0,
            finished_num: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DailyTaskDataNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DailyTaskDataNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DailyTaskDataNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DailyTaskDataNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19DailyTaskDataNotify.proto\"\x93\x01\n\x13DailyTaskDataNotify\x121\
    \n\x15is_taken_score_reward\x18\x0b\x20\x01(\x08R\x12isTakenScoreReward\
    \x12&\n\x0fscore_reward_id\x18\x03\x20\x01(\rR\rscoreRewardId\x12!\n\x0c\
    finished_num\x18\r\x20\x01(\rR\x0bfinishedNumB\x1b\n\x19emu.grasscutter.\
    net.protob\x06proto3\
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
            messages.push(DailyTaskDataNotify::generated_message_descriptor_data());
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