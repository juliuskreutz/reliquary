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

//! Generated file from `DailyTaskInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DailyTaskInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DailyTaskInfo {
    // message fields
    // @@protoc_insertion_point(field:DailyTaskInfo.is_finished)
    pub is_finished: bool,
    // @@protoc_insertion_point(field:DailyTaskInfo.progress)
    pub progress: u32,
    // @@protoc_insertion_point(field:DailyTaskInfo.daily_task_id)
    pub daily_task_id: u32,
    // @@protoc_insertion_point(field:DailyTaskInfo.finish_progress)
    pub finish_progress: u32,
    // @@protoc_insertion_point(field:DailyTaskInfo.reward_id)
    pub reward_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DailyTaskInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DailyTaskInfo {
    fn default() -> &'a DailyTaskInfo {
        <DailyTaskInfo as ::protobuf::Message>::default_instance()
    }
}

impl DailyTaskInfo {
    pub fn new() -> DailyTaskInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_finished",
            |m: &DailyTaskInfo| { &m.is_finished },
            |m: &mut DailyTaskInfo| { &mut m.is_finished },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "progress",
            |m: &DailyTaskInfo| { &m.progress },
            |m: &mut DailyTaskInfo| { &mut m.progress },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "daily_task_id",
            |m: &DailyTaskInfo| { &m.daily_task_id },
            |m: &mut DailyTaskInfo| { &mut m.daily_task_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "finish_progress",
            |m: &DailyTaskInfo| { &m.finish_progress },
            |m: &mut DailyTaskInfo| { &mut m.finish_progress },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "reward_id",
            |m: &DailyTaskInfo| { &m.reward_id },
            |m: &mut DailyTaskInfo| { &mut m.reward_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DailyTaskInfo>(
            "DailyTaskInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DailyTaskInfo {
    const NAME: &'static str = "DailyTaskInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.is_finished = is.read_bool()?;
                },
                64 => {
                    self.progress = is.read_uint32()?;
                },
                96 => {
                    self.daily_task_id = is.read_uint32()?;
                },
                88 => {
                    self.finish_progress = is.read_uint32()?;
                },
                80 => {
                    self.reward_id = is.read_uint32()?;
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
        if self.is_finished != false {
            my_size += 1 + 1;
        }
        if self.progress != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.progress);
        }
        if self.daily_task_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.daily_task_id);
        }
        if self.finish_progress != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.finish_progress);
        }
        if self.reward_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.reward_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_finished != false {
            os.write_bool(13, self.is_finished)?;
        }
        if self.progress != 0 {
            os.write_uint32(8, self.progress)?;
        }
        if self.daily_task_id != 0 {
            os.write_uint32(12, self.daily_task_id)?;
        }
        if self.finish_progress != 0 {
            os.write_uint32(11, self.finish_progress)?;
        }
        if self.reward_id != 0 {
            os.write_uint32(10, self.reward_id)?;
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

    fn new() -> DailyTaskInfo {
        DailyTaskInfo::new()
    }

    fn clear(&mut self) {
        self.is_finished = false;
        self.progress = 0;
        self.daily_task_id = 0;
        self.finish_progress = 0;
        self.reward_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DailyTaskInfo {
        static instance: DailyTaskInfo = DailyTaskInfo {
            is_finished: false,
            progress: 0,
            daily_task_id: 0,
            finish_progress: 0,
            reward_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DailyTaskInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DailyTaskInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DailyTaskInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DailyTaskInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13DailyTaskInfo.proto\"\xb6\x01\n\rDailyTaskInfo\x12\x1f\n\x0bis_fin\
    ished\x18\r\x20\x01(\x08R\nisFinished\x12\x1a\n\x08progress\x18\x08\x20\
    \x01(\rR\x08progress\x12\"\n\rdaily_task_id\x18\x0c\x20\x01(\rR\x0bdaily\
    TaskId\x12'\n\x0ffinish_progress\x18\x0b\x20\x01(\rR\x0efinishProgress\
    \x12\x1b\n\treward_id\x18\n\x20\x01(\rR\x08rewardIdB\x1b\n\x19emu.grassc\
    utter.net.protob\x06proto3\
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
            messages.push(DailyTaskInfo::generated_message_descriptor_data());
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