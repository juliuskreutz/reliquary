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

//! Generated file from `MHNHDNOPKML.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:MHNHDNOPKML)
pub enum MHNHDNOPKML {
    // @@protoc_insertion_point(enum_value:MHNHDNOPKML.BATTLE_END_NONE)
    BATTLE_END_NONE = 0,
    // @@protoc_insertion_point(enum_value:MHNHDNOPKML.BATTLE_END_WIN)
    BATTLE_END_WIN = 1,
    // @@protoc_insertion_point(enum_value:MHNHDNOPKML.BATTLE_END_LOSE)
    BATTLE_END_LOSE = 2,
    // @@protoc_insertion_point(enum_value:MHNHDNOPKML.BATTLE_END_QUIT)
    BATTLE_END_QUIT = 3,
}

impl ::protobuf::Enum for MHNHDNOPKML {
    const NAME: &'static str = "MHNHDNOPKML";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MHNHDNOPKML> {
        match value {
            0 => ::std::option::Option::Some(MHNHDNOPKML::BATTLE_END_NONE),
            1 => ::std::option::Option::Some(MHNHDNOPKML::BATTLE_END_WIN),
            2 => ::std::option::Option::Some(MHNHDNOPKML::BATTLE_END_LOSE),
            3 => ::std::option::Option::Some(MHNHDNOPKML::BATTLE_END_QUIT),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<MHNHDNOPKML> {
        match str {
            "BATTLE_END_NONE" => ::std::option::Option::Some(MHNHDNOPKML::BATTLE_END_NONE),
            "BATTLE_END_WIN" => ::std::option::Option::Some(MHNHDNOPKML::BATTLE_END_WIN),
            "BATTLE_END_LOSE" => ::std::option::Option::Some(MHNHDNOPKML::BATTLE_END_LOSE),
            "BATTLE_END_QUIT" => ::std::option::Option::Some(MHNHDNOPKML::BATTLE_END_QUIT),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [MHNHDNOPKML] = &[
        MHNHDNOPKML::BATTLE_END_NONE,
        MHNHDNOPKML::BATTLE_END_WIN,
        MHNHDNOPKML::BATTLE_END_LOSE,
        MHNHDNOPKML::BATTLE_END_QUIT,
    ];
}

impl ::protobuf::EnumFull for MHNHDNOPKML {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("MHNHDNOPKML").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for MHNHDNOPKML {
    fn default() -> Self {
        MHNHDNOPKML::BATTLE_END_NONE
    }
}

impl MHNHDNOPKML {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<MHNHDNOPKML>("MHNHDNOPKML")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MHNHDNOPKML.proto*`\n\x0bMHNHDNOPKML\x12\x13\n\x0fBATTLE_END_NONE\
    \x10\0\x12\x12\n\x0eBATTLE_END_WIN\x10\x01\x12\x13\n\x0fBATTLE_END_LOSE\
    \x10\x02\x12\x13\n\x0fBATTLE_END_QUIT\x10\x03b\x06proto3\
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
            enums.push(MHNHDNOPKML::generated_enum_descriptor_data());
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