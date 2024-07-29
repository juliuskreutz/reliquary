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

//! Generated file from `AvatarExpeditionState.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:AvatarExpeditionState)
pub enum AvatarExpeditionState {
    // @@protoc_insertion_point(enum_value:AvatarExpeditionState.AVATAR_EXPEDITION_NONE)
    AVATAR_EXPEDITION_NONE = 0,
    // @@protoc_insertion_point(enum_value:AvatarExpeditionState.AVATAR_EXPEDITION_DOING)
    AVATAR_EXPEDITION_DOING = 1,
    // @@protoc_insertion_point(enum_value:AvatarExpeditionState.AVATAR_EXPEDITION_FINISH_WAIT_REWARD)
    AVATAR_EXPEDITION_FINISH_WAIT_REWARD = 2,
    // @@protoc_insertion_point(enum_value:AvatarExpeditionState.AVATAR_EXPEDITION_CALLBACK_WAIT_REWARD)
    AVATAR_EXPEDITION_CALLBACK_WAIT_REWARD = 3,
    // @@protoc_insertion_point(enum_value:AvatarExpeditionState.AVATAR_EXPEDITION_LOCKED)
    AVATAR_EXPEDITION_LOCKED = 4,
}

impl ::protobuf::Enum for AvatarExpeditionState {
    const NAME: &'static str = "AvatarExpeditionState";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AvatarExpeditionState> {
        match value {
            0 => ::std::option::Option::Some(AvatarExpeditionState::AVATAR_EXPEDITION_NONE),
            1 => ::std::option::Option::Some(AvatarExpeditionState::AVATAR_EXPEDITION_DOING),
            2 => ::std::option::Option::Some(AvatarExpeditionState::AVATAR_EXPEDITION_FINISH_WAIT_REWARD),
            3 => ::std::option::Option::Some(AvatarExpeditionState::AVATAR_EXPEDITION_CALLBACK_WAIT_REWARD),
            4 => ::std::option::Option::Some(AvatarExpeditionState::AVATAR_EXPEDITION_LOCKED),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<AvatarExpeditionState> {
        match str {
            "AVATAR_EXPEDITION_NONE" => ::std::option::Option::Some(AvatarExpeditionState::AVATAR_EXPEDITION_NONE),
            "AVATAR_EXPEDITION_DOING" => ::std::option::Option::Some(AvatarExpeditionState::AVATAR_EXPEDITION_DOING),
            "AVATAR_EXPEDITION_FINISH_WAIT_REWARD" => ::std::option::Option::Some(AvatarExpeditionState::AVATAR_EXPEDITION_FINISH_WAIT_REWARD),
            "AVATAR_EXPEDITION_CALLBACK_WAIT_REWARD" => ::std::option::Option::Some(AvatarExpeditionState::AVATAR_EXPEDITION_CALLBACK_WAIT_REWARD),
            "AVATAR_EXPEDITION_LOCKED" => ::std::option::Option::Some(AvatarExpeditionState::AVATAR_EXPEDITION_LOCKED),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [AvatarExpeditionState] = &[
        AvatarExpeditionState::AVATAR_EXPEDITION_NONE,
        AvatarExpeditionState::AVATAR_EXPEDITION_DOING,
        AvatarExpeditionState::AVATAR_EXPEDITION_FINISH_WAIT_REWARD,
        AvatarExpeditionState::AVATAR_EXPEDITION_CALLBACK_WAIT_REWARD,
        AvatarExpeditionState::AVATAR_EXPEDITION_LOCKED,
    ];
}

impl ::protobuf::EnumFull for AvatarExpeditionState {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("AvatarExpeditionState").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for AvatarExpeditionState {
    fn default() -> Self {
        AvatarExpeditionState::AVATAR_EXPEDITION_NONE
    }
}

impl AvatarExpeditionState {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<AvatarExpeditionState>("AvatarExpeditionState")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bAvatarExpeditionState.proto*\xc4\x01\n\x15AvatarExpeditionState\
    \x12\x1a\n\x16AVATAR_EXPEDITION_NONE\x10\0\x12\x1b\n\x17AVATAR_EXPEDITIO\
    N_DOING\x10\x01\x12(\n$AVATAR_EXPEDITION_FINISH_WAIT_REWARD\x10\x02\x12*\
    \n&AVATAR_EXPEDITION_CALLBACK_WAIT_REWARD\x10\x03\x12\x1c\n\x18AVATAR_EX\
    PEDITION_LOCKED\x10\x04B\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            enums.push(AvatarExpeditionState::generated_enum_descriptor_data());
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
