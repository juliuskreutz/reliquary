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

//! Generated file from `ForgeQueueManipulateRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ForgeQueueManipulateRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ForgeQueueManipulateRsp {
    // message fields
    // @@protoc_insertion_point(field:ForgeQueueManipulateRsp.output_item_list)
    pub output_item_list: ::std::vec::Vec<super::ItemParam::ItemParam>,
    // @@protoc_insertion_point(field:ForgeQueueManipulateRsp.extra_output_item_list)
    pub extra_output_item_list: ::std::vec::Vec<super::ItemParam::ItemParam>,
    // @@protoc_insertion_point(field:ForgeQueueManipulateRsp.manipulate_type)
    pub manipulate_type: ::protobuf::EnumOrUnknown<super::ForgeQueueManipulateType::ForgeQueueManipulateType>,
    // @@protoc_insertion_point(field:ForgeQueueManipulateRsp.return_item_list)
    pub return_item_list: ::std::vec::Vec<super::ItemParam::ItemParam>,
    // @@protoc_insertion_point(field:ForgeQueueManipulateRsp.retcode)
    pub retcode: i32,
    // special fields
    // @@protoc_insertion_point(special_field:ForgeQueueManipulateRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ForgeQueueManipulateRsp {
    fn default() -> &'a ForgeQueueManipulateRsp {
        <ForgeQueueManipulateRsp as ::protobuf::Message>::default_instance()
    }
}

impl ForgeQueueManipulateRsp {
    pub fn new() -> ForgeQueueManipulateRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "output_item_list",
            |m: &ForgeQueueManipulateRsp| { &m.output_item_list },
            |m: &mut ForgeQueueManipulateRsp| { &mut m.output_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "extra_output_item_list",
            |m: &ForgeQueueManipulateRsp| { &m.extra_output_item_list },
            |m: &mut ForgeQueueManipulateRsp| { &mut m.extra_output_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "manipulate_type",
            |m: &ForgeQueueManipulateRsp| { &m.manipulate_type },
            |m: &mut ForgeQueueManipulateRsp| { &mut m.manipulate_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "return_item_list",
            |m: &ForgeQueueManipulateRsp| { &m.return_item_list },
            |m: &mut ForgeQueueManipulateRsp| { &mut m.return_item_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ForgeQueueManipulateRsp| { &m.retcode },
            |m: &mut ForgeQueueManipulateRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ForgeQueueManipulateRsp>(
            "ForgeQueueManipulateRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ForgeQueueManipulateRsp {
    const NAME: &'static str = "ForgeQueueManipulateRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    self.output_item_list.push(is.read_message()?);
                },
                106 => {
                    self.extra_output_item_list.push(is.read_message()?);
                },
                40 => {
                    self.manipulate_type = is.read_enum_or_unknown()?;
                },
                34 => {
                    self.return_item_list.push(is.read_message()?);
                },
                48 => {
                    self.retcode = is.read_int32()?;
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
        for value in &self.output_item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.extra_output_item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.manipulate_type != ::protobuf::EnumOrUnknown::new(super::ForgeQueueManipulateType::ForgeQueueManipulateType::FORGE_QUEUE_MANIPULATE_TYPE_RECEIVE_OUTPUT) {
            my_size += ::protobuf::rt::int32_size(5, self.manipulate_type.value());
        }
        for value in &self.return_item_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.output_item_list {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        for v in &self.extra_output_item_list {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.manipulate_type != ::protobuf::EnumOrUnknown::new(super::ForgeQueueManipulateType::ForgeQueueManipulateType::FORGE_QUEUE_MANIPULATE_TYPE_RECEIVE_OUTPUT) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.manipulate_type))?;
        }
        for v in &self.return_item_list {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.retcode != 0 {
            os.write_int32(6, self.retcode)?;
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

    fn new() -> ForgeQueueManipulateRsp {
        ForgeQueueManipulateRsp::new()
    }

    fn clear(&mut self) {
        self.output_item_list.clear();
        self.extra_output_item_list.clear();
        self.manipulate_type = ::protobuf::EnumOrUnknown::new(super::ForgeQueueManipulateType::ForgeQueueManipulateType::FORGE_QUEUE_MANIPULATE_TYPE_RECEIVE_OUTPUT);
        self.return_item_list.clear();
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ForgeQueueManipulateRsp {
        static instance: ForgeQueueManipulateRsp = ForgeQueueManipulateRsp {
            output_item_list: ::std::vec::Vec::new(),
            extra_output_item_list: ::std::vec::Vec::new(),
            manipulate_type: ::protobuf::EnumOrUnknown::from_i32(0),
            return_item_list: ::std::vec::Vec::new(),
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ForgeQueueManipulateRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ForgeQueueManipulateRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ForgeQueueManipulateRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ForgeQueueManipulateRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dForgeQueueManipulateRsp.proto\x1a\x0fItemParam.proto\x1a\x1eForgeQ\
    ueueManipulateType.proto\"\xa4\x02\n\x17ForgeQueueManipulateRsp\x124\n\
    \x10output_item_list\x18\x0c\x20\x03(\x0b2\n.ItemParamR\x0eoutputItemLis\
    t\x12?\n\x16extra_output_item_list\x18\r\x20\x03(\x0b2\n.ItemParamR\x13e\
    xtraOutputItemList\x12B\n\x0fmanipulate_type\x18\x05\x20\x01(\x0e2\x19.F\
    orgeQueueManipulateTypeR\x0emanipulateType\x124\n\x10return_item_list\
    \x18\x04\x20\x03(\x0b2\n.ItemParamR\x0ereturnItemList\x12\x18\n\x07retco\
    de\x18\x06\x20\x01(\x05R\x07retcodeB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::ItemParam::file_descriptor().clone());
            deps.push(super::ForgeQueueManipulateType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ForgeQueueManipulateRsp::generated_message_descriptor_data());
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
