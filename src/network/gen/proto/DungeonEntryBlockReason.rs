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

//! Generated file from `DungeonEntryBlockReason.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:DungeonEntryBlockReason)
pub enum DungeonEntryBlockReason {
    // @@protoc_insertion_point(enum_value:DungeonEntryBlockReason.DUNGEON_ENTRY_REASON_NONE)
    DUNGEON_ENTRY_REASON_NONE = 0,
    // @@protoc_insertion_point(enum_value:DungeonEntryBlockReason.DUNGEON_ENTRY_REASON_LEVEL)
    DUNGEON_ENTRY_REASON_LEVEL = 1,
    // @@protoc_insertion_point(enum_value:DungeonEntryBlockReason.DUNGEON_ENTRY_REASON_QUEST)
    DUNGEON_ENTRY_REASON_QUEST = 2,
    // @@protoc_insertion_point(enum_value:DungeonEntryBlockReason.DUNGEON_ENTRY_REASON_MULIPLE)
    DUNGEON_ENTRY_REASON_MULIPLE = 3,
}

impl ::protobuf::Enum for DungeonEntryBlockReason {
    const NAME: &'static str = "DungeonEntryBlockReason";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DungeonEntryBlockReason> {
        match value {
            0 => ::std::option::Option::Some(DungeonEntryBlockReason::DUNGEON_ENTRY_REASON_NONE),
            1 => ::std::option::Option::Some(DungeonEntryBlockReason::DUNGEON_ENTRY_REASON_LEVEL),
            2 => ::std::option::Option::Some(DungeonEntryBlockReason::DUNGEON_ENTRY_REASON_QUEST),
            3 => ::std::option::Option::Some(DungeonEntryBlockReason::DUNGEON_ENTRY_REASON_MULIPLE),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<DungeonEntryBlockReason> {
        match str {
            "DUNGEON_ENTRY_REASON_NONE" => ::std::option::Option::Some(DungeonEntryBlockReason::DUNGEON_ENTRY_REASON_NONE),
            "DUNGEON_ENTRY_REASON_LEVEL" => ::std::option::Option::Some(DungeonEntryBlockReason::DUNGEON_ENTRY_REASON_LEVEL),
            "DUNGEON_ENTRY_REASON_QUEST" => ::std::option::Option::Some(DungeonEntryBlockReason::DUNGEON_ENTRY_REASON_QUEST),
            "DUNGEON_ENTRY_REASON_MULIPLE" => ::std::option::Option::Some(DungeonEntryBlockReason::DUNGEON_ENTRY_REASON_MULIPLE),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [DungeonEntryBlockReason] = &[
        DungeonEntryBlockReason::DUNGEON_ENTRY_REASON_NONE,
        DungeonEntryBlockReason::DUNGEON_ENTRY_REASON_LEVEL,
        DungeonEntryBlockReason::DUNGEON_ENTRY_REASON_QUEST,
        DungeonEntryBlockReason::DUNGEON_ENTRY_REASON_MULIPLE,
    ];
}

impl ::protobuf::EnumFull for DungeonEntryBlockReason {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("DungeonEntryBlockReason").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for DungeonEntryBlockReason {
    fn default() -> Self {
        DungeonEntryBlockReason::DUNGEON_ENTRY_REASON_NONE
    }
}

impl DungeonEntryBlockReason {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<DungeonEntryBlockReason>("DungeonEntryBlockReason")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dDungeonEntryBlockReason.proto*\x9a\x01\n\x17DungeonEntryBlockReaso\
    n\x12\x1d\n\x19DUNGEON_ENTRY_REASON_NONE\x10\0\x12\x1e\n\x1aDUNGEON_ENTR\
    Y_REASON_LEVEL\x10\x01\x12\x1e\n\x1aDUNGEON_ENTRY_REASON_QUEST\x10\x02\
    \x12\x20\n\x1cDUNGEON_ENTRY_REASON_MULIPLE\x10\x03B\x1b\n\x19emu.grasscu\
    tter.net.protob\x06proto3\
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
            enums.push(DungeonEntryBlockReason::generated_enum_descriptor_data());
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
