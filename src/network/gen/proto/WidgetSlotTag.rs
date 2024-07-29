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

//! Generated file from `WidgetSlotTag.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:WidgetSlotTag)
pub enum WidgetSlotTag {
    // @@protoc_insertion_point(enum_value:WidgetSlotTag.WIDGET_SLOT_TAG_QUICK_USE)
    WIDGET_SLOT_TAG_QUICK_USE = 0,
    // @@protoc_insertion_point(enum_value:WidgetSlotTag.WIDGET_SLOT_TAG_ATTACH_AVATAR)
    WIDGET_SLOT_TAG_ATTACH_AVATAR = 1,
}

impl ::protobuf::Enum for WidgetSlotTag {
    const NAME: &'static str = "WidgetSlotTag";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<WidgetSlotTag> {
        match value {
            0 => ::std::option::Option::Some(WidgetSlotTag::WIDGET_SLOT_TAG_QUICK_USE),
            1 => ::std::option::Option::Some(WidgetSlotTag::WIDGET_SLOT_TAG_ATTACH_AVATAR),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<WidgetSlotTag> {
        match str {
            "WIDGET_SLOT_TAG_QUICK_USE" => ::std::option::Option::Some(WidgetSlotTag::WIDGET_SLOT_TAG_QUICK_USE),
            "WIDGET_SLOT_TAG_ATTACH_AVATAR" => ::std::option::Option::Some(WidgetSlotTag::WIDGET_SLOT_TAG_ATTACH_AVATAR),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [WidgetSlotTag] = &[
        WidgetSlotTag::WIDGET_SLOT_TAG_QUICK_USE,
        WidgetSlotTag::WIDGET_SLOT_TAG_ATTACH_AVATAR,
    ];
}

impl ::protobuf::EnumFull for WidgetSlotTag {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("WidgetSlotTag").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for WidgetSlotTag {
    fn default() -> Self {
        WidgetSlotTag::WIDGET_SLOT_TAG_QUICK_USE
    }
}

impl WidgetSlotTag {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<WidgetSlotTag>("WidgetSlotTag")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13WidgetSlotTag.proto*Q\n\rWidgetSlotTag\x12\x1d\n\x19WIDGET_SLOT_TA\
    G_QUICK_USE\x10\0\x12!\n\x1dWIDGET_SLOT_TAG_ATTACH_AVATAR\x10\x01B\x1b\n\
    \x19emu.grasscutter.net.protob\x06proto3\
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
            enums.push(WidgetSlotTag::generated_enum_descriptor_data());
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
