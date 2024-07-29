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

//! Generated file from `FireworksLaunchParam.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FireworksLaunchParam)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FireworksLaunchParam {
    // message fields
    // @@protoc_insertion_point(field:FireworksLaunchParam.type)
    pub type_: ::protobuf::EnumOrUnknown<super::FireworksLaunchParamType::FireworksLaunchParamType>,
    // @@protoc_insertion_point(field:FireworksLaunchParam.value)
    pub value: i32,
    // special fields
    // @@protoc_insertion_point(special_field:FireworksLaunchParam.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FireworksLaunchParam {
    fn default() -> &'a FireworksLaunchParam {
        <FireworksLaunchParam as ::protobuf::Message>::default_instance()
    }
}

impl FireworksLaunchParam {
    pub fn new() -> FireworksLaunchParam {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &FireworksLaunchParam| { &m.type_ },
            |m: &mut FireworksLaunchParam| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "value",
            |m: &FireworksLaunchParam| { &m.value },
            |m: &mut FireworksLaunchParam| { &mut m.value },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FireworksLaunchParam>(
            "FireworksLaunchParam",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FireworksLaunchParam {
    const NAME: &'static str = "FireworksLaunchParam";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.type_ = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.value = is.read_int32()?;
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
        if self.type_ != ::protobuf::EnumOrUnknown::new(super::FireworksLaunchParamType::FireworksLaunchParamType::FIREWORKS_LAUNCH_PARAM_NONE) {
            my_size += ::protobuf::rt::int32_size(14, self.type_.value());
        }
        if self.value != 0 {
            my_size += ::protobuf::rt::int32_size(2, self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.type_ != ::protobuf::EnumOrUnknown::new(super::FireworksLaunchParamType::FireworksLaunchParamType::FIREWORKS_LAUNCH_PARAM_NONE) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.type_))?;
        }
        if self.value != 0 {
            os.write_int32(2, self.value)?;
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

    fn new() -> FireworksLaunchParam {
        FireworksLaunchParam::new()
    }

    fn clear(&mut self) {
        self.type_ = ::protobuf::EnumOrUnknown::new(super::FireworksLaunchParamType::FireworksLaunchParamType::FIREWORKS_LAUNCH_PARAM_NONE);
        self.value = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FireworksLaunchParam {
        static instance: FireworksLaunchParam = FireworksLaunchParam {
            type_: ::protobuf::EnumOrUnknown::from_i32(0),
            value: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FireworksLaunchParam {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FireworksLaunchParam").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FireworksLaunchParam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FireworksLaunchParam {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aFireworksLaunchParam.proto\x1a\x1eFireworksLaunchParamType.proto\"\
    [\n\x14FireworksLaunchParam\x12-\n\x04type\x18\x0e\x20\x01(\x0e2\x19.Fir\
    eworksLaunchParamTypeR\x04type\x12\x14\n\x05value\x18\x02\x20\x01(\x05R\
    \x05valueB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::FireworksLaunchParamType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FireworksLaunchParam::generated_message_descriptor_data());
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
