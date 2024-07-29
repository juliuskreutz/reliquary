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

//! Generated file from `ActivityWatcherInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ActivityWatcherInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ActivityWatcherInfo {
    // message fields
    // @@protoc_insertion_point(field:ActivityWatcherInfo.total_progress)
    pub total_progress: u32,
    // @@protoc_insertion_point(field:ActivityWatcherInfo.watcher_id)
    pub watcher_id: u32,
    // @@protoc_insertion_point(field:ActivityWatcherInfo.is_taken_reward)
    pub is_taken_reward: bool,
    // @@protoc_insertion_point(field:ActivityWatcherInfo.cur_progress)
    pub cur_progress: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ActivityWatcherInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ActivityWatcherInfo {
    fn default() -> &'a ActivityWatcherInfo {
        <ActivityWatcherInfo as ::protobuf::Message>::default_instance()
    }
}

impl ActivityWatcherInfo {
    pub fn new() -> ActivityWatcherInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "total_progress",
            |m: &ActivityWatcherInfo| { &m.total_progress },
            |m: &mut ActivityWatcherInfo| { &mut m.total_progress },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "watcher_id",
            |m: &ActivityWatcherInfo| { &m.watcher_id },
            |m: &mut ActivityWatcherInfo| { &mut m.watcher_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_taken_reward",
            |m: &ActivityWatcherInfo| { &m.is_taken_reward },
            |m: &mut ActivityWatcherInfo| { &mut m.is_taken_reward },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_progress",
            |m: &ActivityWatcherInfo| { &m.cur_progress },
            |m: &mut ActivityWatcherInfo| { &mut m.cur_progress },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ActivityWatcherInfo>(
            "ActivityWatcherInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ActivityWatcherInfo {
    const NAME: &'static str = "ActivityWatcherInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.total_progress = is.read_uint32()?;
                },
                104 => {
                    self.watcher_id = is.read_uint32()?;
                },
                64 => {
                    self.is_taken_reward = is.read_bool()?;
                },
                120 => {
                    self.cur_progress = is.read_uint32()?;
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
        if self.total_progress != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.total_progress);
        }
        if self.watcher_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.watcher_id);
        }
        if self.is_taken_reward != false {
            my_size += 1 + 1;
        }
        if self.cur_progress != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.cur_progress);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.total_progress != 0 {
            os.write_uint32(14, self.total_progress)?;
        }
        if self.watcher_id != 0 {
            os.write_uint32(13, self.watcher_id)?;
        }
        if self.is_taken_reward != false {
            os.write_bool(8, self.is_taken_reward)?;
        }
        if self.cur_progress != 0 {
            os.write_uint32(15, self.cur_progress)?;
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

    fn new() -> ActivityWatcherInfo {
        ActivityWatcherInfo::new()
    }

    fn clear(&mut self) {
        self.total_progress = 0;
        self.watcher_id = 0;
        self.is_taken_reward = false;
        self.cur_progress = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ActivityWatcherInfo {
        static instance: ActivityWatcherInfo = ActivityWatcherInfo {
            total_progress: 0,
            watcher_id: 0,
            is_taken_reward: false,
            cur_progress: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ActivityWatcherInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ActivityWatcherInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ActivityWatcherInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActivityWatcherInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19ActivityWatcherInfo.proto\"\xa6\x01\n\x13ActivityWatcherInfo\x12%\
    \n\x0etotal_progress\x18\x0e\x20\x01(\rR\rtotalProgress\x12\x1d\n\nwatch\
    er_id\x18\r\x20\x01(\rR\twatcherId\x12&\n\x0fis_taken_reward\x18\x08\x20\
    \x01(\x08R\risTakenReward\x12!\n\x0ccur_progress\x18\x0f\x20\x01(\rR\x0b\
    curProgressB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(ActivityWatcherInfo::generated_message_descriptor_data());
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