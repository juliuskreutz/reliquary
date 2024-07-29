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

//! Generated file from `ResinCostType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:ResinCostType)
pub enum ResinCostType {
    // @@protoc_insertion_point(enum_value:ResinCostType.RESIN_COST_TYPE_NONE)
    RESIN_COST_TYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:ResinCostType.RESIN_COST_TYPE_NORMAL)
    RESIN_COST_TYPE_NORMAL = 1,
    // @@protoc_insertion_point(enum_value:ResinCostType.RESIN_COST_TYPE_CONDENSE)
    RESIN_COST_TYPE_CONDENSE = 2,
    // @@protoc_insertion_point(enum_value:ResinCostType.RESIN_COST_TYPE_REUNION_PRIVILEGE)
    RESIN_COST_TYPE_REUNION_PRIVILEGE = 3,
    // @@protoc_insertion_point(enum_value:ResinCostType.RESIN_COST_TYPE_OP_ACTIVITY)
    RESIN_COST_TYPE_OP_ACTIVITY = 4,
    // @@protoc_insertion_point(enum_value:ResinCostType.RESIN_COST_TYPE_MATERIAL)
    RESIN_COST_TYPE_MATERIAL = 5,
}

impl ::protobuf::Enum for ResinCostType {
    const NAME: &'static str = "ResinCostType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ResinCostType> {
        match value {
            0 => ::std::option::Option::Some(ResinCostType::RESIN_COST_TYPE_NONE),
            1 => ::std::option::Option::Some(ResinCostType::RESIN_COST_TYPE_NORMAL),
            2 => ::std::option::Option::Some(ResinCostType::RESIN_COST_TYPE_CONDENSE),
            3 => ::std::option::Option::Some(ResinCostType::RESIN_COST_TYPE_REUNION_PRIVILEGE),
            4 => ::std::option::Option::Some(ResinCostType::RESIN_COST_TYPE_OP_ACTIVITY),
            5 => ::std::option::Option::Some(ResinCostType::RESIN_COST_TYPE_MATERIAL),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<ResinCostType> {
        match str {
            "RESIN_COST_TYPE_NONE" => ::std::option::Option::Some(ResinCostType::RESIN_COST_TYPE_NONE),
            "RESIN_COST_TYPE_NORMAL" => ::std::option::Option::Some(ResinCostType::RESIN_COST_TYPE_NORMAL),
            "RESIN_COST_TYPE_CONDENSE" => ::std::option::Option::Some(ResinCostType::RESIN_COST_TYPE_CONDENSE),
            "RESIN_COST_TYPE_REUNION_PRIVILEGE" => ::std::option::Option::Some(ResinCostType::RESIN_COST_TYPE_REUNION_PRIVILEGE),
            "RESIN_COST_TYPE_OP_ACTIVITY" => ::std::option::Option::Some(ResinCostType::RESIN_COST_TYPE_OP_ACTIVITY),
            "RESIN_COST_TYPE_MATERIAL" => ::std::option::Option::Some(ResinCostType::RESIN_COST_TYPE_MATERIAL),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [ResinCostType] = &[
        ResinCostType::RESIN_COST_TYPE_NONE,
        ResinCostType::RESIN_COST_TYPE_NORMAL,
        ResinCostType::RESIN_COST_TYPE_CONDENSE,
        ResinCostType::RESIN_COST_TYPE_REUNION_PRIVILEGE,
        ResinCostType::RESIN_COST_TYPE_OP_ACTIVITY,
        ResinCostType::RESIN_COST_TYPE_MATERIAL,
    ];
}

impl ::protobuf::EnumFull for ResinCostType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("ResinCostType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for ResinCostType {
    fn default() -> Self {
        ResinCostType::RESIN_COST_TYPE_NONE
    }
}

impl ResinCostType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<ResinCostType>("ResinCostType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13ResinCostType.proto*\xc9\x01\n\rResinCostType\x12\x18\n\x14RESIN_C\
    OST_TYPE_NONE\x10\0\x12\x1a\n\x16RESIN_COST_TYPE_NORMAL\x10\x01\x12\x1c\
    \n\x18RESIN_COST_TYPE_CONDENSE\x10\x02\x12%\n!RESIN_COST_TYPE_REUNION_PR\
    IVILEGE\x10\x03\x12\x1f\n\x1bRESIN_COST_TYPE_OP_ACTIVITY\x10\x04\x12\x1c\
    \n\x18RESIN_COST_TYPE_MATERIAL\x10\x05B\x1b\n\x19emu.grasscutter.net.pro\
    tob\x06proto3\
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
            enums.push(ResinCostType::generated_enum_descriptor_data());
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
