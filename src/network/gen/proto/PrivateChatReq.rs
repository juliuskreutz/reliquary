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

//! Generated file from `PrivateChatReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PrivateChatReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PrivateChatReq {
    // message fields
    // @@protoc_insertion_point(field:PrivateChatReq.target_uid)
    pub target_uid: u32,
    // message oneof groups
    pub content: ::std::option::Option<private_chat_req::Content>,
    // special fields
    // @@protoc_insertion_point(special_field:PrivateChatReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PrivateChatReq {
    fn default() -> &'a PrivateChatReq {
        <PrivateChatReq as ::protobuf::Message>::default_instance()
    }
}

impl PrivateChatReq {
    pub fn new() -> PrivateChatReq {
        ::std::default::Default::default()
    }

    // string text = 7;

    pub fn text(&self) -> &str {
        match self.content {
            ::std::option::Option::Some(private_chat_req::Content::Text(ref v)) => v,
            _ => "",
        }
    }

    pub fn clear_text(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_text(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(private_chat_req::Content::Text(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: ::std::string::String) {
        self.content = ::std::option::Option::Some(private_chat_req::Content::Text(v))
    }

    // Mutable pointer to the field.
    pub fn mut_text(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(private_chat_req::Content::Text(_)) = self.content {
        } else {
            self.content = ::std::option::Option::Some(private_chat_req::Content::Text(::std::string::String::new()));
        }
        match self.content {
            ::std::option::Option::Some(private_chat_req::Content::Text(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_text(&mut self) -> ::std::string::String {
        if self.has_text() {
            match self.content.take() {
                ::std::option::Option::Some(private_chat_req::Content::Text(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // uint32 icon = 4;

    pub fn icon(&self) -> u32 {
        match self.content {
            ::std::option::Option::Some(private_chat_req::Content::Icon(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_icon(&mut self) {
        self.content = ::std::option::Option::None;
    }

    pub fn has_icon(&self) -> bool {
        match self.content {
            ::std::option::Option::Some(private_chat_req::Content::Icon(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_icon(&mut self, v: u32) {
        self.content = ::std::option::Option::Some(private_chat_req::Content::Icon(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_uid",
            |m: &PrivateChatReq| { &m.target_uid },
            |m: &mut PrivateChatReq| { &mut m.target_uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_deref_has_get_set_simpler_accessor::<_, _>(
            "text",
            PrivateChatReq::has_text,
            PrivateChatReq::text,
            PrivateChatReq::set_text,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "icon",
            PrivateChatReq::has_icon,
            PrivateChatReq::icon,
            PrivateChatReq::set_icon,
        ));
        oneofs.push(private_chat_req::Content::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PrivateChatReq>(
            "PrivateChatReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PrivateChatReq {
    const NAME: &'static str = "PrivateChatReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.target_uid = is.read_uint32()?;
                },
                58 => {
                    self.content = ::std::option::Option::Some(private_chat_req::Content::Text(is.read_string()?));
                },
                32 => {
                    self.content = ::std::option::Option::Some(private_chat_req::Content::Icon(is.read_uint32()?));
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
        if self.target_uid != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.target_uid);
        }
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &private_chat_req::Content::Text(ref v) => {
                    my_size += ::protobuf::rt::string_size(7, &v);
                },
                &private_chat_req::Content::Icon(v) => {
                    my_size += ::protobuf::rt::uint32_size(4, v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.target_uid != 0 {
            os.write_uint32(15, self.target_uid)?;
        }
        if let ::std::option::Option::Some(ref v) = self.content {
            match v {
                &private_chat_req::Content::Text(ref v) => {
                    os.write_string(7, v)?;
                },
                &private_chat_req::Content::Icon(v) => {
                    os.write_uint32(4, v)?;
                },
            };
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

    fn new() -> PrivateChatReq {
        PrivateChatReq::new()
    }

    fn clear(&mut self) {
        self.target_uid = 0;
        self.content = ::std::option::Option::None;
        self.content = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PrivateChatReq {
        static instance: PrivateChatReq = PrivateChatReq {
            target_uid: 0,
            content: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PrivateChatReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PrivateChatReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PrivateChatReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PrivateChatReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `PrivateChatReq`
pub mod private_chat_req {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:PrivateChatReq.content)
    pub enum Content {
        // @@protoc_insertion_point(oneof_field:PrivateChatReq.text)
        Text(::std::string::String),
        // @@protoc_insertion_point(oneof_field:PrivateChatReq.icon)
        Icon(u32),
    }

    impl ::protobuf::Oneof for Content {
    }

    impl ::protobuf::OneofFull for Content {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::PrivateChatReq as ::protobuf::MessageFull>::descriptor().oneof_by_name("content").unwrap()).clone()
        }
    }

    impl Content {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Content>("content")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14PrivateChatReq.proto\"f\n\x0ePrivateChatReq\x12\x1d\n\ntarget_uid\
    \x18\x0f\x20\x01(\rR\ttargetUid\x12\x14\n\x04text\x18\x07\x20\x01(\tH\0R\
    \x04text\x12\x14\n\x04icon\x18\x04\x20\x01(\rH\0R\x04iconB\t\n\x07conten\
    tB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(PrivateChatReq::generated_message_descriptor_data());
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