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

//! Generated file from `ChatChannelShieldNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ChatChannelShieldNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChatChannelShieldNotify {
    // message fields
    // @@protoc_insertion_point(field:ChatChannelShieldNotify.channel_id)
    pub channel_id: u32,
    // @@protoc_insertion_point(field:ChatChannelShieldNotify.is_shield)
    pub is_shield: bool,
    // special fields
    // @@protoc_insertion_point(special_field:ChatChannelShieldNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChatChannelShieldNotify {
    fn default() -> &'a ChatChannelShieldNotify {
        <ChatChannelShieldNotify as ::protobuf::Message>::default_instance()
    }
}

impl ChatChannelShieldNotify {
    pub fn new() -> ChatChannelShieldNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "channel_id",
            |m: &ChatChannelShieldNotify| { &m.channel_id },
            |m: &mut ChatChannelShieldNotify| { &mut m.channel_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_shield",
            |m: &ChatChannelShieldNotify| { &m.is_shield },
            |m: &mut ChatChannelShieldNotify| { &mut m.is_shield },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChatChannelShieldNotify>(
            "ChatChannelShieldNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChatChannelShieldNotify {
    const NAME: &'static str = "ChatChannelShieldNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.channel_id = is.read_uint32()?;
                },
                64 => {
                    self.is_shield = is.read_bool()?;
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
        if self.channel_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.channel_id);
        }
        if self.is_shield != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.channel_id != 0 {
            os.write_uint32(6, self.channel_id)?;
        }
        if self.is_shield != false {
            os.write_bool(8, self.is_shield)?;
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

    fn new() -> ChatChannelShieldNotify {
        ChatChannelShieldNotify::new()
    }

    fn clear(&mut self) {
        self.channel_id = 0;
        self.is_shield = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChatChannelShieldNotify {
        static instance: ChatChannelShieldNotify = ChatChannelShieldNotify {
            channel_id: 0,
            is_shield: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChatChannelShieldNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChatChannelShieldNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChatChannelShieldNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChatChannelShieldNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dChatChannelShieldNotify.proto\"U\n\x17ChatChannelShieldNotify\x12\
    \x1d\n\nchannel_id\x18\x06\x20\x01(\rR\tchannelId\x12\x1b\n\tis_shield\
    \x18\x08\x20\x01(\x08R\x08isShieldB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChatChannelShieldNotify::generated_message_descriptor_data());
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