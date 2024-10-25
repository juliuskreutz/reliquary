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

//! Generated file from `CmdFightType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:CmdFightType)
pub enum CmdFightType {
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightTypeNone)
    CmdFightTypeNone = 0,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightSessionStopScNotify)
    CmdFightSessionStopScNotify = 30033,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightKickOutScNotify)
    CmdFightKickOutScNotify = 30042,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightEnterCsReq)
    CmdFightEnterCsReq = 30098,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightEnterScRsp)
    CmdFightEnterScRsp = 30071,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightLeaveScNotify)
    CmdFightLeaveScNotify = 30083,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightHeartBeatCsReq)
    CmdFightHeartBeatCsReq = 30079,
    // @@protoc_insertion_point(enum_value:CmdFightType.CmdFightHeartBeatScRsp)
    CmdFightHeartBeatScRsp = 30077,
}

impl ::protobuf::Enum for CmdFightType {
    const NAME: &'static str = "CmdFightType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdFightType> {
        match value {
            0 => ::std::option::Option::Some(CmdFightType::CmdFightTypeNone),
            30033 => ::std::option::Option::Some(CmdFightType::CmdFightSessionStopScNotify),
            30042 => ::std::option::Option::Some(CmdFightType::CmdFightKickOutScNotify),
            30098 => ::std::option::Option::Some(CmdFightType::CmdFightEnterCsReq),
            30071 => ::std::option::Option::Some(CmdFightType::CmdFightEnterScRsp),
            30083 => ::std::option::Option::Some(CmdFightType::CmdFightLeaveScNotify),
            30079 => ::std::option::Option::Some(CmdFightType::CmdFightHeartBeatCsReq),
            30077 => ::std::option::Option::Some(CmdFightType::CmdFightHeartBeatScRsp),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<CmdFightType> {
        match str {
            "CmdFightTypeNone" => ::std::option::Option::Some(CmdFightType::CmdFightTypeNone),
            "CmdFightSessionStopScNotify" => ::std::option::Option::Some(CmdFightType::CmdFightSessionStopScNotify),
            "CmdFightKickOutScNotify" => ::std::option::Option::Some(CmdFightType::CmdFightKickOutScNotify),
            "CmdFightEnterCsReq" => ::std::option::Option::Some(CmdFightType::CmdFightEnterCsReq),
            "CmdFightEnterScRsp" => ::std::option::Option::Some(CmdFightType::CmdFightEnterScRsp),
            "CmdFightLeaveScNotify" => ::std::option::Option::Some(CmdFightType::CmdFightLeaveScNotify),
            "CmdFightHeartBeatCsReq" => ::std::option::Option::Some(CmdFightType::CmdFightHeartBeatCsReq),
            "CmdFightHeartBeatScRsp" => ::std::option::Option::Some(CmdFightType::CmdFightHeartBeatScRsp),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [CmdFightType] = &[
        CmdFightType::CmdFightTypeNone,
        CmdFightType::CmdFightSessionStopScNotify,
        CmdFightType::CmdFightKickOutScNotify,
        CmdFightType::CmdFightEnterCsReq,
        CmdFightType::CmdFightEnterScRsp,
        CmdFightType::CmdFightLeaveScNotify,
        CmdFightType::CmdFightHeartBeatCsReq,
        CmdFightType::CmdFightHeartBeatScRsp,
    ];
}

impl ::protobuf::EnumFull for CmdFightType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("CmdFightType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            CmdFightType::CmdFightTypeNone => 0,
            CmdFightType::CmdFightSessionStopScNotify => 1,
            CmdFightType::CmdFightKickOutScNotify => 2,
            CmdFightType::CmdFightEnterCsReq => 3,
            CmdFightType::CmdFightEnterScRsp => 4,
            CmdFightType::CmdFightLeaveScNotify => 5,
            CmdFightType::CmdFightHeartBeatCsReq => 6,
            CmdFightType::CmdFightHeartBeatScRsp => 7,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for CmdFightType {
    fn default() -> Self {
        CmdFightType::CmdFightTypeNone
    }
}

impl CmdFightType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<CmdFightType>("CmdFightType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12CmdFightType.proto*\xf3\x01\n\x0cCmdFightType\x12\x14\n\x10CmdFigh\
    tTypeNone\x10\0\x12!\n\x1bCmdFightSessionStopScNotify\x10\xd1\xea\x01\
    \x12\x1d\n\x17CmdFightKickOutScNotify\x10\xda\xea\x01\x12\x18\n\x12CmdFi\
    ghtEnterCsReq\x10\x92\xeb\x01\x12\x18\n\x12CmdFightEnterScRsp\x10\xf7\
    \xea\x01\x12\x1b\n\x15CmdFightLeaveScNotify\x10\x83\xeb\x01\x12\x1c\n\
    \x16CmdFightHeartBeatCsReq\x10\xff\xea\x01\x12\x1c\n\x16CmdFightHeartBea\
    tScRsp\x10\xfd\xea\x01b\x06proto3\
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
            enums.push(CmdFightType::generated_enum_descriptor_data());
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