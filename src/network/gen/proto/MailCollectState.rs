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

//! Generated file from `MailCollectState.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:MailCollectState)
pub enum MailCollectState {
    // @@protoc_insertion_point(enum_value:MailCollectState.MAIL_COLLECT_STATE_COLLECTIBLE_UNKNOWN)
    MAIL_COLLECT_STATE_COLLECTIBLE_UNKNOWN = 0,
    // @@protoc_insertion_point(enum_value:MailCollectState.MAIL_COLLECT_STATE_NOT_COLLECTIBLE)
    MAIL_COLLECT_STATE_NOT_COLLECTIBLE = 1,
    // @@protoc_insertion_point(enum_value:MailCollectState.MAIL_COLLECT_STATE_COLLECTIBLE_UNCOLLECTED)
    MAIL_COLLECT_STATE_COLLECTIBLE_UNCOLLECTED = 2,
    // @@protoc_insertion_point(enum_value:MailCollectState.MAIL_COLLECT_STATE_COLLECTIBLE_COLLECTED)
    MAIL_COLLECT_STATE_COLLECTIBLE_COLLECTED = 3,
}

impl ::protobuf::Enum for MailCollectState {
    const NAME: &'static str = "MailCollectState";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MailCollectState> {
        match value {
            0 => ::std::option::Option::Some(MailCollectState::MAIL_COLLECT_STATE_COLLECTIBLE_UNKNOWN),
            1 => ::std::option::Option::Some(MailCollectState::MAIL_COLLECT_STATE_NOT_COLLECTIBLE),
            2 => ::std::option::Option::Some(MailCollectState::MAIL_COLLECT_STATE_COLLECTIBLE_UNCOLLECTED),
            3 => ::std::option::Option::Some(MailCollectState::MAIL_COLLECT_STATE_COLLECTIBLE_COLLECTED),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<MailCollectState> {
        match str {
            "MAIL_COLLECT_STATE_COLLECTIBLE_UNKNOWN" => ::std::option::Option::Some(MailCollectState::MAIL_COLLECT_STATE_COLLECTIBLE_UNKNOWN),
            "MAIL_COLLECT_STATE_NOT_COLLECTIBLE" => ::std::option::Option::Some(MailCollectState::MAIL_COLLECT_STATE_NOT_COLLECTIBLE),
            "MAIL_COLLECT_STATE_COLLECTIBLE_UNCOLLECTED" => ::std::option::Option::Some(MailCollectState::MAIL_COLLECT_STATE_COLLECTIBLE_UNCOLLECTED),
            "MAIL_COLLECT_STATE_COLLECTIBLE_COLLECTED" => ::std::option::Option::Some(MailCollectState::MAIL_COLLECT_STATE_COLLECTIBLE_COLLECTED),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [MailCollectState] = &[
        MailCollectState::MAIL_COLLECT_STATE_COLLECTIBLE_UNKNOWN,
        MailCollectState::MAIL_COLLECT_STATE_NOT_COLLECTIBLE,
        MailCollectState::MAIL_COLLECT_STATE_COLLECTIBLE_UNCOLLECTED,
        MailCollectState::MAIL_COLLECT_STATE_COLLECTIBLE_COLLECTED,
    ];
}

impl ::protobuf::EnumFull for MailCollectState {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("MailCollectState").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for MailCollectState {
    fn default() -> Self {
        MailCollectState::MAIL_COLLECT_STATE_COLLECTIBLE_UNKNOWN
    }
}

impl MailCollectState {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<MailCollectState>("MailCollectState")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16MailCollectState.proto*\xc4\x01\n\x10MailCollectState\x12*\n&MAIL_\
    COLLECT_STATE_COLLECTIBLE_UNKNOWN\x10\0\x12&\n\"MAIL_COLLECT_STATE_NOT_C\
    OLLECTIBLE\x10\x01\x12.\n*MAIL_COLLECT_STATE_COLLECTIBLE_UNCOLLECTED\x10\
    \x02\x12,\n(MAIL_COLLECT_STATE_COLLECTIBLE_COLLECTED\x10\x03B\x1b\n\x19e\
    mu.grasscutter.net.protob\x06proto3\
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
            enums.push(MailCollectState::generated_enum_descriptor_data());
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
