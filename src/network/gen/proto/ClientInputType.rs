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

//! Generated file from `ClientInputType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:ClientInputType)
pub enum ClientInputType {
    // @@protoc_insertion_point(enum_value:ClientInputType.CLIENT_INPUT_NONE)
    CLIENT_INPUT_NONE = 0,
    // @@protoc_insertion_point(enum_value:ClientInputType.CLIENT_INPUT_KEYBORD_MOUSE)
    CLIENT_INPUT_KEYBORD_MOUSE = 1,
    // @@protoc_insertion_point(enum_value:ClientInputType.CLIENT_INPUT_GAMEPAD)
    CLIENT_INPUT_GAMEPAD = 2,
    // @@protoc_insertion_point(enum_value:ClientInputType.CLIENT_INPUT_TOUCH_PANEL)
    CLIENT_INPUT_TOUCH_PANEL = 3,
}

impl ::protobuf::Enum for ClientInputType {
    const NAME: &'static str = "ClientInputType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ClientInputType> {
        match value {
            0 => ::std::option::Option::Some(ClientInputType::CLIENT_INPUT_NONE),
            1 => ::std::option::Option::Some(ClientInputType::CLIENT_INPUT_KEYBORD_MOUSE),
            2 => ::std::option::Option::Some(ClientInputType::CLIENT_INPUT_GAMEPAD),
            3 => ::std::option::Option::Some(ClientInputType::CLIENT_INPUT_TOUCH_PANEL),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<ClientInputType> {
        match str {
            "CLIENT_INPUT_NONE" => ::std::option::Option::Some(ClientInputType::CLIENT_INPUT_NONE),
            "CLIENT_INPUT_KEYBORD_MOUSE" => ::std::option::Option::Some(ClientInputType::CLIENT_INPUT_KEYBORD_MOUSE),
            "CLIENT_INPUT_GAMEPAD" => ::std::option::Option::Some(ClientInputType::CLIENT_INPUT_GAMEPAD),
            "CLIENT_INPUT_TOUCH_PANEL" => ::std::option::Option::Some(ClientInputType::CLIENT_INPUT_TOUCH_PANEL),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [ClientInputType] = &[
        ClientInputType::CLIENT_INPUT_NONE,
        ClientInputType::CLIENT_INPUT_KEYBORD_MOUSE,
        ClientInputType::CLIENT_INPUT_GAMEPAD,
        ClientInputType::CLIENT_INPUT_TOUCH_PANEL,
    ];
}

impl ::protobuf::EnumFull for ClientInputType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("ClientInputType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for ClientInputType {
    fn default() -> Self {
        ClientInputType::CLIENT_INPUT_NONE
    }
}

impl ClientInputType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<ClientInputType>("ClientInputType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15ClientInputType.proto*\x80\x01\n\x0fClientInputType\x12\x15\n\x11C\
    LIENT_INPUT_NONE\x10\0\x12\x1e\n\x1aCLIENT_INPUT_KEYBORD_MOUSE\x10\x01\
    \x12\x18\n\x14CLIENT_INPUT_GAMEPAD\x10\x02\x12\x1c\n\x18CLIENT_INPUT_TOU\
    CH_PANEL\x10\x03B\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            enums.push(ClientInputType::generated_enum_descriptor_data());
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
