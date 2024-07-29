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

//! Generated file from `ReportSubtype.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:ReportSubtype)
pub enum ReportSubtype {
    // @@protoc_insertion_point(enum_value:ReportSubtype.REPORT_REASON_SUBTYPE_NONE)
    REPORT_REASON_SUBTYPE_NONE = 0,
    // @@protoc_insertion_point(enum_value:ReportSubtype.REPORT_REASON_SUBTYPE_INVALID_NICKNAME)
    REPORT_REASON_SUBTYPE_INVALID_NICKNAME = 1,
    // @@protoc_insertion_point(enum_value:ReportSubtype.REPORT_REASON_SUBTYPE_INVALID_SIGNATURE)
    REPORT_REASON_SUBTYPE_INVALID_SIGNATURE = 2,
    // @@protoc_insertion_point(enum_value:ReportSubtype.REPORT_REASON_SUBTYPE_INVALID_ARRANGEMENT)
    REPORT_REASON_SUBTYPE_INVALID_ARRANGEMENT = 3,
    // @@protoc_insertion_point(enum_value:ReportSubtype.REPORT_REASON_SUBTYPE_INVALID_CHAT)
    REPORT_REASON_SUBTYPE_INVALID_CHAT = 4,
    // @@protoc_insertion_point(enum_value:ReportSubtype.REPORT_REASON_SUBTYPE_INVALID_AVATAR_NAME)
    REPORT_REASON_SUBTYPE_INVALID_AVATAR_NAME = 5,
    // @@protoc_insertion_point(enum_value:ReportSubtype.REPORT_REASON_SUBTYPE_INVALID_OTHER)
    REPORT_REASON_SUBTYPE_INVALID_OTHER = 6,
}

impl ::protobuf::Enum for ReportSubtype {
    const NAME: &'static str = "ReportSubtype";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReportSubtype> {
        match value {
            0 => ::std::option::Option::Some(ReportSubtype::REPORT_REASON_SUBTYPE_NONE),
            1 => ::std::option::Option::Some(ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_NICKNAME),
            2 => ::std::option::Option::Some(ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_SIGNATURE),
            3 => ::std::option::Option::Some(ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_ARRANGEMENT),
            4 => ::std::option::Option::Some(ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_CHAT),
            5 => ::std::option::Option::Some(ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_AVATAR_NAME),
            6 => ::std::option::Option::Some(ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_OTHER),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<ReportSubtype> {
        match str {
            "REPORT_REASON_SUBTYPE_NONE" => ::std::option::Option::Some(ReportSubtype::REPORT_REASON_SUBTYPE_NONE),
            "REPORT_REASON_SUBTYPE_INVALID_NICKNAME" => ::std::option::Option::Some(ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_NICKNAME),
            "REPORT_REASON_SUBTYPE_INVALID_SIGNATURE" => ::std::option::Option::Some(ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_SIGNATURE),
            "REPORT_REASON_SUBTYPE_INVALID_ARRANGEMENT" => ::std::option::Option::Some(ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_ARRANGEMENT),
            "REPORT_REASON_SUBTYPE_INVALID_CHAT" => ::std::option::Option::Some(ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_CHAT),
            "REPORT_REASON_SUBTYPE_INVALID_AVATAR_NAME" => ::std::option::Option::Some(ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_AVATAR_NAME),
            "REPORT_REASON_SUBTYPE_INVALID_OTHER" => ::std::option::Option::Some(ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_OTHER),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [ReportSubtype] = &[
        ReportSubtype::REPORT_REASON_SUBTYPE_NONE,
        ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_NICKNAME,
        ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_SIGNATURE,
        ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_ARRANGEMENT,
        ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_CHAT,
        ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_AVATAR_NAME,
        ReportSubtype::REPORT_REASON_SUBTYPE_INVALID_OTHER,
    ];
}

impl ::protobuf::EnumFull for ReportSubtype {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("ReportSubtype").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for ReportSubtype {
    fn default() -> Self {
        ReportSubtype::REPORT_REASON_SUBTYPE_NONE
    }
}

impl ReportSubtype {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<ReportSubtype>("ReportSubtype")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13ReportSubtype.proto*\xb7\x02\n\rReportSubtype\x12\x1e\n\x1aREPORT_\
    REASON_SUBTYPE_NONE\x10\0\x12*\n&REPORT_REASON_SUBTYPE_INVALID_NICKNAME\
    \x10\x01\x12+\n'REPORT_REASON_SUBTYPE_INVALID_SIGNATURE\x10\x02\x12-\n)R\
    EPORT_REASON_SUBTYPE_INVALID_ARRANGEMENT\x10\x03\x12&\n\"REPORT_REASON_S\
    UBTYPE_INVALID_CHAT\x10\x04\x12-\n)REPORT_REASON_SUBTYPE_INVALID_AVATAR_\
    NAME\x10\x05\x12'\n#REPORT_REASON_SUBTYPE_INVALID_OTHER\x10\x06B\x1b\n\
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
            enums.push(ReportSubtype::generated_enum_descriptor_data());
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
