// This file is generated by rust-protobuf 3.6.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `HeroBasicType.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:HeroBasicType)
pub enum HeroBasicType {
    // @@protoc_insertion_point(enum_value:HeroBasicType.None)
    None = 0,
    // @@protoc_insertion_point(enum_value:HeroBasicType.BoyWarrior)
    BoyWarrior = 8001,
    // @@protoc_insertion_point(enum_value:HeroBasicType.GirlWarrior)
    GirlWarrior = 8002,
    // @@protoc_insertion_point(enum_value:HeroBasicType.BoyKnight)
    BoyKnight = 8003,
    // @@protoc_insertion_point(enum_value:HeroBasicType.GirlKnight)
    GirlKnight = 8004,
    // @@protoc_insertion_point(enum_value:HeroBasicType.BoyRogue)
    BoyRogue = 8005,
    // @@protoc_insertion_point(enum_value:HeroBasicType.GirlRogue)
    GirlRogue = 8006,
    // @@protoc_insertion_point(enum_value:HeroBasicType.BoyMage)
    BoyMage = 8007,
    // @@protoc_insertion_point(enum_value:HeroBasicType.GirlMage)
    GirlMage = 8008,
    // @@protoc_insertion_point(enum_value:HeroBasicType.BoyShaman)
    BoyShaman = 8009,
    // @@protoc_insertion_point(enum_value:HeroBasicType.GirlShaman)
    GirlShaman = 8010,
    // @@protoc_insertion_point(enum_value:HeroBasicType.BoyWarlock)
    BoyWarlock = 8011,
    // @@protoc_insertion_point(enum_value:HeroBasicType.GirlWarlock)
    GirlWarlock = 8012,
    // @@protoc_insertion_point(enum_value:HeroBasicType.BoyPriest)
    BoyPriest = 8013,
    // @@protoc_insertion_point(enum_value:HeroBasicType.GirlPriest)
    GirlPriest = 8014,
}

impl ::protobuf::Enum for HeroBasicType {
    const NAME: &'static str = "HeroBasicType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HeroBasicType> {
        match value {
            0 => ::std::option::Option::Some(HeroBasicType::None),
            8001 => ::std::option::Option::Some(HeroBasicType::BoyWarrior),
            8002 => ::std::option::Option::Some(HeroBasicType::GirlWarrior),
            8003 => ::std::option::Option::Some(HeroBasicType::BoyKnight),
            8004 => ::std::option::Option::Some(HeroBasicType::GirlKnight),
            8005 => ::std::option::Option::Some(HeroBasicType::BoyRogue),
            8006 => ::std::option::Option::Some(HeroBasicType::GirlRogue),
            8007 => ::std::option::Option::Some(HeroBasicType::BoyMage),
            8008 => ::std::option::Option::Some(HeroBasicType::GirlMage),
            8009 => ::std::option::Option::Some(HeroBasicType::BoyShaman),
            8010 => ::std::option::Option::Some(HeroBasicType::GirlShaman),
            8011 => ::std::option::Option::Some(HeroBasicType::BoyWarlock),
            8012 => ::std::option::Option::Some(HeroBasicType::GirlWarlock),
            8013 => ::std::option::Option::Some(HeroBasicType::BoyPriest),
            8014 => ::std::option::Option::Some(HeroBasicType::GirlPriest),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<HeroBasicType> {
        match str {
            "None" => ::std::option::Option::Some(HeroBasicType::None),
            "BoyWarrior" => ::std::option::Option::Some(HeroBasicType::BoyWarrior),
            "GirlWarrior" => ::std::option::Option::Some(HeroBasicType::GirlWarrior),
            "BoyKnight" => ::std::option::Option::Some(HeroBasicType::BoyKnight),
            "GirlKnight" => ::std::option::Option::Some(HeroBasicType::GirlKnight),
            "BoyRogue" => ::std::option::Option::Some(HeroBasicType::BoyRogue),
            "GirlRogue" => ::std::option::Option::Some(HeroBasicType::GirlRogue),
            "BoyMage" => ::std::option::Option::Some(HeroBasicType::BoyMage),
            "GirlMage" => ::std::option::Option::Some(HeroBasicType::GirlMage),
            "BoyShaman" => ::std::option::Option::Some(HeroBasicType::BoyShaman),
            "GirlShaman" => ::std::option::Option::Some(HeroBasicType::GirlShaman),
            "BoyWarlock" => ::std::option::Option::Some(HeroBasicType::BoyWarlock),
            "GirlWarlock" => ::std::option::Option::Some(HeroBasicType::GirlWarlock),
            "BoyPriest" => ::std::option::Option::Some(HeroBasicType::BoyPriest),
            "GirlPriest" => ::std::option::Option::Some(HeroBasicType::GirlPriest),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [HeroBasicType] = &[
        HeroBasicType::None,
        HeroBasicType::BoyWarrior,
        HeroBasicType::GirlWarrior,
        HeroBasicType::BoyKnight,
        HeroBasicType::GirlKnight,
        HeroBasicType::BoyRogue,
        HeroBasicType::GirlRogue,
        HeroBasicType::BoyMage,
        HeroBasicType::GirlMage,
        HeroBasicType::BoyShaman,
        HeroBasicType::GirlShaman,
        HeroBasicType::BoyWarlock,
        HeroBasicType::GirlWarlock,
        HeroBasicType::BoyPriest,
        HeroBasicType::GirlPriest,
    ];
}

impl ::protobuf::EnumFull for HeroBasicType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("HeroBasicType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = match self {
            HeroBasicType::None => 0,
            HeroBasicType::BoyWarrior => 1,
            HeroBasicType::GirlWarrior => 2,
            HeroBasicType::BoyKnight => 3,
            HeroBasicType::GirlKnight => 4,
            HeroBasicType::BoyRogue => 5,
            HeroBasicType::GirlRogue => 6,
            HeroBasicType::BoyMage => 7,
            HeroBasicType::GirlMage => 8,
            HeroBasicType::BoyShaman => 9,
            HeroBasicType::GirlShaman => 10,
            HeroBasicType::BoyWarlock => 11,
            HeroBasicType::GirlWarlock => 12,
            HeroBasicType::BoyPriest => 13,
            HeroBasicType::GirlPriest => 14,
        };
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for HeroBasicType {
    fn default() -> Self {
        HeroBasicType::None
    }
}

impl HeroBasicType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<HeroBasicType>("HeroBasicType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13HeroBasicType.proto*\xfe\x01\n\rHeroBasicType\x12\x08\n\x04None\
    \x10\0\x12\x0f\n\nBoyWarrior\x10\xc1>\x12\x10\n\x0bGirlWarrior\x10\xc2>\
    \x12\x0e\n\tBoyKnight\x10\xc3>\x12\x0f\n\nGirlKnight\x10\xc4>\x12\r\n\
    \x08BoyRogue\x10\xc5>\x12\x0e\n\tGirlRogue\x10\xc6>\x12\x0c\n\x07BoyMage\
    \x10\xc7>\x12\r\n\x08GirlMage\x10\xc8>\x12\x0e\n\tBoyShaman\x10\xc9>\x12\
    \x0f\n\nGirlShaman\x10\xca>\x12\x0f\n\nBoyWarlock\x10\xcb>\x12\x10\n\x0b\
    GirlWarlock\x10\xcc>\x12\x0e\n\tBoyPriest\x10\xcd>\x12\x0f\n\nGirlPriest\
    \x10\xce>B\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            enums.push(HeroBasicType::generated_enum_descriptor_data());
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
