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

//! Generated file from `SalesmanStatusType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:SalesmanStatusType)
pub enum SalesmanStatusType {
    // @@protoc_insertion_point(enum_value:SalesmanStatusType.SALESMAN_STATUS_NONE)
    SALESMAN_STATUS_NONE = 0,
    // @@protoc_insertion_point(enum_value:SalesmanStatusType.SALESMAN_STATUS_UNSTARTED)
    SALESMAN_STATUS_UNSTARTED = 1,
    // @@protoc_insertion_point(enum_value:SalesmanStatusType.SALESMAN_STATUS_STARTED)
    SALESMAN_STATUS_STARTED = 2,
    // @@protoc_insertion_point(enum_value:SalesmanStatusType.SALESMAN_STATUS_DELIVERED)
    SALESMAN_STATUS_DELIVERED = 3,
}

impl ::protobuf::Enum for SalesmanStatusType {
    const NAME: &'static str = "SalesmanStatusType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<SalesmanStatusType> {
        match value {
            0 => ::std::option::Option::Some(SalesmanStatusType::SALESMAN_STATUS_NONE),
            1 => ::std::option::Option::Some(SalesmanStatusType::SALESMAN_STATUS_UNSTARTED),
            2 => ::std::option::Option::Some(SalesmanStatusType::SALESMAN_STATUS_STARTED),
            3 => ::std::option::Option::Some(SalesmanStatusType::SALESMAN_STATUS_DELIVERED),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<SalesmanStatusType> {
        match str {
            "SALESMAN_STATUS_NONE" => ::std::option::Option::Some(SalesmanStatusType::SALESMAN_STATUS_NONE),
            "SALESMAN_STATUS_UNSTARTED" => ::std::option::Option::Some(SalesmanStatusType::SALESMAN_STATUS_UNSTARTED),
            "SALESMAN_STATUS_STARTED" => ::std::option::Option::Some(SalesmanStatusType::SALESMAN_STATUS_STARTED),
            "SALESMAN_STATUS_DELIVERED" => ::std::option::Option::Some(SalesmanStatusType::SALESMAN_STATUS_DELIVERED),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [SalesmanStatusType] = &[
        SalesmanStatusType::SALESMAN_STATUS_NONE,
        SalesmanStatusType::SALESMAN_STATUS_UNSTARTED,
        SalesmanStatusType::SALESMAN_STATUS_STARTED,
        SalesmanStatusType::SALESMAN_STATUS_DELIVERED,
    ];
}

impl ::protobuf::EnumFull for SalesmanStatusType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("SalesmanStatusType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for SalesmanStatusType {
    fn default() -> Self {
        SalesmanStatusType::SALESMAN_STATUS_NONE
    }
}

impl SalesmanStatusType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<SalesmanStatusType>("SalesmanStatusType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18SalesmanStatusType.proto*\x89\x01\n\x12SalesmanStatusType\x12\x18\
    \n\x14SALESMAN_STATUS_NONE\x10\0\x12\x1d\n\x19SALESMAN_STATUS_UNSTARTED\
    \x10\x01\x12\x1b\n\x17SALESMAN_STATUS_STARTED\x10\x02\x12\x1d\n\x19SALES\
    MAN_STATUS_DELIVERED\x10\x03B\x1b\n\x19emu.grasscutter.net.protob\x06pro\
    to3\
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
            enums.push(SalesmanStatusType::generated_enum_descriptor_data());
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
