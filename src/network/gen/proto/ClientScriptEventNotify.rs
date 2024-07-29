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

//! Generated file from `ClientScriptEventNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ClientScriptEventNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ClientScriptEventNotify {
    // message fields
    // @@protoc_insertion_point(field:ClientScriptEventNotify.source_entity_id)
    pub source_entity_id: u32,
    // @@protoc_insertion_point(field:ClientScriptEventNotify.target_entity_id)
    pub target_entity_id: u32,
    // @@protoc_insertion_point(field:ClientScriptEventNotify.event_type)
    pub event_type: u32,
    // @@protoc_insertion_point(field:ClientScriptEventNotify.param_list)
    pub param_list: ::std::vec::Vec<i32>,
    // special fields
    // @@protoc_insertion_point(special_field:ClientScriptEventNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ClientScriptEventNotify {
    fn default() -> &'a ClientScriptEventNotify {
        <ClientScriptEventNotify as ::protobuf::Message>::default_instance()
    }
}

impl ClientScriptEventNotify {
    pub fn new() -> ClientScriptEventNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "source_entity_id",
            |m: &ClientScriptEventNotify| { &m.source_entity_id },
            |m: &mut ClientScriptEventNotify| { &mut m.source_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_entity_id",
            |m: &ClientScriptEventNotify| { &m.target_entity_id },
            |m: &mut ClientScriptEventNotify| { &mut m.target_entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "event_type",
            |m: &ClientScriptEventNotify| { &m.event_type },
            |m: &mut ClientScriptEventNotify| { &mut m.event_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "param_list",
            |m: &ClientScriptEventNotify| { &m.param_list },
            |m: &mut ClientScriptEventNotify| { &mut m.param_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ClientScriptEventNotify>(
            "ClientScriptEventNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ClientScriptEventNotify {
    const NAME: &'static str = "ClientScriptEventNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.source_entity_id = is.read_uint32()?;
                },
                8 => {
                    self.target_entity_id = is.read_uint32()?;
                },
                32 => {
                    self.event_type = is.read_uint32()?;
                },
                18 => {
                    is.read_repeated_packed_int32_into(&mut self.param_list)?;
                },
                16 => {
                    self.param_list.push(is.read_int32()?);
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
        if self.source_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.source_entity_id);
        }
        if self.target_entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.target_entity_id);
        }
        if self.event_type != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.event_type);
        }
        for value in &self.param_list {
            my_size += ::protobuf::rt::int32_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.source_entity_id != 0 {
            os.write_uint32(14, self.source_entity_id)?;
        }
        if self.target_entity_id != 0 {
            os.write_uint32(1, self.target_entity_id)?;
        }
        if self.event_type != 0 {
            os.write_uint32(4, self.event_type)?;
        }
        for v in &self.param_list {
            os.write_int32(2, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ClientScriptEventNotify {
        ClientScriptEventNotify::new()
    }

    fn clear(&mut self) {
        self.source_entity_id = 0;
        self.target_entity_id = 0;
        self.event_type = 0;
        self.param_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ClientScriptEventNotify {
        static instance: ClientScriptEventNotify = ClientScriptEventNotify {
            source_entity_id: 0,
            target_entity_id: 0,
            event_type: 0,
            param_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ClientScriptEventNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ClientScriptEventNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ClientScriptEventNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClientScriptEventNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dClientScriptEventNotify.proto\"\xab\x01\n\x17ClientScriptEventNoti\
    fy\x12(\n\x10source_entity_id\x18\x0e\x20\x01(\rR\x0esourceEntityId\x12(\
    \n\x10target_entity_id\x18\x01\x20\x01(\rR\x0etargetEntityId\x12\x1d\n\n\
    event_type\x18\x04\x20\x01(\rR\teventType\x12\x1d\n\nparam_list\x18\x02\
    \x20\x03(\x05R\tparamListB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(ClientScriptEventNotify::generated_message_descriptor_data());
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