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

//! Generated file from `StopServerInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:StopServerInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct StopServerInfo {
    // message fields
    // @@protoc_insertion_point(field:StopServerInfo.stop_begin_time)
    pub stop_begin_time: u32,
    // @@protoc_insertion_point(field:StopServerInfo.stop_end_time)
    pub stop_end_time: u32,
    // @@protoc_insertion_point(field:StopServerInfo.url)
    pub url: ::std::string::String,
    // @@protoc_insertion_point(field:StopServerInfo.content_msg)
    pub content_msg: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:StopServerInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StopServerInfo {
    fn default() -> &'a StopServerInfo {
        <StopServerInfo as ::protobuf::Message>::default_instance()
    }
}

impl StopServerInfo {
    pub fn new() -> StopServerInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stop_begin_time",
            |m: &StopServerInfo| { &m.stop_begin_time },
            |m: &mut StopServerInfo| { &mut m.stop_begin_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stop_end_time",
            |m: &StopServerInfo| { &m.stop_end_time },
            |m: &mut StopServerInfo| { &mut m.stop_end_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "url",
            |m: &StopServerInfo| { &m.url },
            |m: &mut StopServerInfo| { &mut m.url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "content_msg",
            |m: &StopServerInfo| { &m.content_msg },
            |m: &mut StopServerInfo| { &mut m.content_msg },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StopServerInfo>(
            "StopServerInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StopServerInfo {
    const NAME: &'static str = "StopServerInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.stop_begin_time = is.read_uint32()?;
                },
                16 => {
                    self.stop_end_time = is.read_uint32()?;
                },
                26 => {
                    self.url = is.read_string()?;
                },
                34 => {
                    self.content_msg = is.read_string()?;
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
        if self.stop_begin_time != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.stop_begin_time);
        }
        if self.stop_end_time != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.stop_end_time);
        }
        if !self.url.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.url);
        }
        if !self.content_msg.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.content_msg);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.stop_begin_time != 0 {
            os.write_uint32(1, self.stop_begin_time)?;
        }
        if self.stop_end_time != 0 {
            os.write_uint32(2, self.stop_end_time)?;
        }
        if !self.url.is_empty() {
            os.write_string(3, &self.url)?;
        }
        if !self.content_msg.is_empty() {
            os.write_string(4, &self.content_msg)?;
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

    fn new() -> StopServerInfo {
        StopServerInfo::new()
    }

    fn clear(&mut self) {
        self.stop_begin_time = 0;
        self.stop_end_time = 0;
        self.url.clear();
        self.content_msg.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StopServerInfo {
        static instance: StopServerInfo = StopServerInfo {
            stop_begin_time: 0,
            stop_end_time: 0,
            url: ::std::string::String::new(),
            content_msg: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StopServerInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StopServerInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StopServerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StopServerInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14StopServerInfo.proto\"\x8f\x01\n\x0eStopServerInfo\x12&\n\x0fstop_\
    begin_time\x18\x01\x20\x01(\rR\rstopBeginTime\x12\"\n\rstop_end_time\x18\
    \x02\x20\x01(\rR\x0bstopEndTime\x12\x10\n\x03url\x18\x03\x20\x01(\tR\x03\
    url\x12\x1f\n\x0bcontent_msg\x18\x04\x20\x01(\tR\ncontentMsgB\x1b\n\x19e\
    mu.grasscutter.net.protob\x06proto3\
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
            messages.push(StopServerInfo::generated_message_descriptor_data());
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