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

//! Generated file from `FireWorkType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:FireWorkType)
pub enum FireWorkType {
    // @@protoc_insertion_point(enum_value:FireWorkType.FireWorkType_ODJKANKMPPJ)
    FireWorkType_ODJKANKMPPJ = 0,
    // @@protoc_insertion_point(enum_value:FireWorkType.FireWorkType_EFGLHEIODFN)
    FireWorkType_EFGLHEIODFN = 1,
    // @@protoc_insertion_point(enum_value:FireWorkType.FireWorkType_JPBBBCFGHAK)
    FireWorkType_JPBBBCFGHAK = 2,
    // @@protoc_insertion_point(enum_value:FireWorkType.FireWorkType_IDCMGHBHBFH)
    FireWorkType_IDCMGHBHBFH = 3,
    // @@protoc_insertion_point(enum_value:FireWorkType.FireWorkType_ODDBNNDFMBO)
    FireWorkType_ODDBNNDFMBO = 4,
    // @@protoc_insertion_point(enum_value:FireWorkType.FireWorkType_AGIDMOGJOBD)
    FireWorkType_AGIDMOGJOBD = 5,
}

impl ::protobuf::Enum for FireWorkType {
    const NAME: &'static str = "FireWorkType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FireWorkType> {
        match value {
            0 => ::std::option::Option::Some(FireWorkType::FireWorkType_ODJKANKMPPJ),
            1 => ::std::option::Option::Some(FireWorkType::FireWorkType_EFGLHEIODFN),
            2 => ::std::option::Option::Some(FireWorkType::FireWorkType_JPBBBCFGHAK),
            3 => ::std::option::Option::Some(FireWorkType::FireWorkType_IDCMGHBHBFH),
            4 => ::std::option::Option::Some(FireWorkType::FireWorkType_ODDBNNDFMBO),
            5 => ::std::option::Option::Some(FireWorkType::FireWorkType_AGIDMOGJOBD),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<FireWorkType> {
        match str {
            "FireWorkType_ODJKANKMPPJ" => ::std::option::Option::Some(FireWorkType::FireWorkType_ODJKANKMPPJ),
            "FireWorkType_EFGLHEIODFN" => ::std::option::Option::Some(FireWorkType::FireWorkType_EFGLHEIODFN),
            "FireWorkType_JPBBBCFGHAK" => ::std::option::Option::Some(FireWorkType::FireWorkType_JPBBBCFGHAK),
            "FireWorkType_IDCMGHBHBFH" => ::std::option::Option::Some(FireWorkType::FireWorkType_IDCMGHBHBFH),
            "FireWorkType_ODDBNNDFMBO" => ::std::option::Option::Some(FireWorkType::FireWorkType_ODDBNNDFMBO),
            "FireWorkType_AGIDMOGJOBD" => ::std::option::Option::Some(FireWorkType::FireWorkType_AGIDMOGJOBD),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [FireWorkType] = &[
        FireWorkType::FireWorkType_ODJKANKMPPJ,
        FireWorkType::FireWorkType_EFGLHEIODFN,
        FireWorkType::FireWorkType_JPBBBCFGHAK,
        FireWorkType::FireWorkType_IDCMGHBHBFH,
        FireWorkType::FireWorkType_ODDBNNDFMBO,
        FireWorkType::FireWorkType_AGIDMOGJOBD,
    ];
}

impl ::protobuf::EnumFull for FireWorkType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("FireWorkType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for FireWorkType {
    fn default() -> Self {
        FireWorkType::FireWorkType_ODJKANKMPPJ
    }
}

impl FireWorkType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<FireWorkType>("FireWorkType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12FireWorkType.proto*\xc2\x01\n\x0cFireWorkType\x12\x1c\n\x18FireWor\
    kType_ODJKANKMPPJ\x10\0\x12\x1c\n\x18FireWorkType_EFGLHEIODFN\x10\x01\
    \x12\x1c\n\x18FireWorkType_JPBBBCFGHAK\x10\x02\x12\x1c\n\x18FireWorkType\
    _IDCMGHBHBFH\x10\x03\x12\x1c\n\x18FireWorkType_ODDBNNDFMBO\x10\x04\x12\
    \x1c\n\x18FireWorkType_AGIDMOGJOBD\x10\x05B\x1b\n\x19emu.grasscutter.net\
    .protob\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(FireWorkType::generated_enum_descriptor_data());
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