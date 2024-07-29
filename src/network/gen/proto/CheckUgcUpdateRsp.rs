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

//! Generated file from `CheckUgcUpdateRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CheckUgcUpdateRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CheckUgcUpdateRsp {
    // message fields
    // @@protoc_insertion_point(field:CheckUgcUpdateRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:CheckUgcUpdateRsp.update_ugc_guid_list)
    pub update_ugc_guid_list: ::std::vec::Vec<u64>,
    // @@protoc_insertion_point(field:CheckUgcUpdateRsp.ugc_type)
    pub ugc_type: ::protobuf::EnumOrUnknown<super::UgcType::UgcType>,
    // special fields
    // @@protoc_insertion_point(special_field:CheckUgcUpdateRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CheckUgcUpdateRsp {
    fn default() -> &'a CheckUgcUpdateRsp {
        <CheckUgcUpdateRsp as ::protobuf::Message>::default_instance()
    }
}

impl CheckUgcUpdateRsp {
    pub fn new() -> CheckUgcUpdateRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &CheckUgcUpdateRsp| { &m.retcode },
            |m: &mut CheckUgcUpdateRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "update_ugc_guid_list",
            |m: &CheckUgcUpdateRsp| { &m.update_ugc_guid_list },
            |m: &mut CheckUgcUpdateRsp| { &mut m.update_ugc_guid_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ugc_type",
            |m: &CheckUgcUpdateRsp| { &m.ugc_type },
            |m: &mut CheckUgcUpdateRsp| { &mut m.ugc_type },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CheckUgcUpdateRsp>(
            "CheckUgcUpdateRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CheckUgcUpdateRsp {
    const NAME: &'static str = "CheckUgcUpdateRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.retcode = is.read_int32()?;
                },
                10 => {
                    is.read_repeated_packed_uint64_into(&mut self.update_ugc_guid_list)?;
                },
                8 => {
                    self.update_ugc_guid_list.push(is.read_uint64()?);
                },
                72 => {
                    self.ugc_type = is.read_enum_or_unknown()?;
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.retcode);
        }
        for value in &self.update_ugc_guid_list {
            my_size += ::protobuf::rt::uint64_size(1, *value);
        };
        if self.ugc_type != ::protobuf::EnumOrUnknown::new(super::UgcType::UgcType::UGC_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(9, self.ugc_type.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_int32(6, self.retcode)?;
        }
        for v in &self.update_ugc_guid_list {
            os.write_uint64(1, *v)?;
        };
        if self.ugc_type != ::protobuf::EnumOrUnknown::new(super::UgcType::UgcType::UGC_TYPE_NONE) {
            os.write_enum(9, ::protobuf::EnumOrUnknown::value(&self.ugc_type))?;
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

    fn new() -> CheckUgcUpdateRsp {
        CheckUgcUpdateRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.update_ugc_guid_list.clear();
        self.ugc_type = ::protobuf::EnumOrUnknown::new(super::UgcType::UgcType::UGC_TYPE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CheckUgcUpdateRsp {
        static instance: CheckUgcUpdateRsp = CheckUgcUpdateRsp {
            retcode: 0,
            update_ugc_guid_list: ::std::vec::Vec::new(),
            ugc_type: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CheckUgcUpdateRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CheckUgcUpdateRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CheckUgcUpdateRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CheckUgcUpdateRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17CheckUgcUpdateRsp.proto\x1a\rUgcType.proto\"\x83\x01\n\x11CheckUgc\
    UpdateRsp\x12\x18\n\x07retcode\x18\x06\x20\x01(\x05R\x07retcode\x12/\n\
    \x14update_ugc_guid_list\x18\x01\x20\x03(\x04R\x11updateUgcGuidList\x12#\
    \n\x08ugc_type\x18\t\x20\x01(\x0e2\x08.UgcTypeR\x07ugcTypeB\x1b\n\x19emu\
    .grasscutter.net.protob\x06proto3\
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
            deps.push(super::UgcType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CheckUgcUpdateRsp::generated_message_descriptor_data());
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