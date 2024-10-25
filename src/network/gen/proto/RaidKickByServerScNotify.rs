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

//! Generated file from `RaidKickByServerScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RaidKickByServerScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RaidKickByServerScNotify {
    // message fields
    // @@protoc_insertion_point(field:RaidKickByServerScNotify.OGIOAKBPMAE)
    pub OGIOAKBPMAE: u32,
    // @@protoc_insertion_point(field:RaidKickByServerScNotify.MMMOHAJFMID)
    pub MMMOHAJFMID: u32,
    // @@protoc_insertion_point(field:RaidKickByServerScNotify.EMDECAJPAPM)
    pub EMDECAJPAPM: ::protobuf::MessageField<super::FHGPCKGFGAO::FHGPCKGFGAO>,
    // @@protoc_insertion_point(field:RaidKickByServerScNotify.DGDDHBLKMLI)
    pub DGDDHBLKMLI: ::protobuf::EnumOrUnknown<super::EGHDABBBNJO::EGHDABBBNJO>,
    // @@protoc_insertion_point(field:RaidKickByServerScNotify.COCFMLGGMKE)
    pub COCFMLGGMKE: ::protobuf::MessageField<super::FJPJJEIJLLP::FJPJJEIJLLP>,
    // special fields
    // @@protoc_insertion_point(special_field:RaidKickByServerScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RaidKickByServerScNotify {
    fn default() -> &'a RaidKickByServerScNotify {
        <RaidKickByServerScNotify as ::protobuf::Message>::default_instance()
    }
}

impl RaidKickByServerScNotify {
    pub fn new() -> RaidKickByServerScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OGIOAKBPMAE",
            |m: &RaidKickByServerScNotify| { &m.OGIOAKBPMAE },
            |m: &mut RaidKickByServerScNotify| { &mut m.OGIOAKBPMAE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MMMOHAJFMID",
            |m: &RaidKickByServerScNotify| { &m.MMMOHAJFMID },
            |m: &mut RaidKickByServerScNotify| { &mut m.MMMOHAJFMID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FHGPCKGFGAO::FHGPCKGFGAO>(
            "EMDECAJPAPM",
            |m: &RaidKickByServerScNotify| { &m.EMDECAJPAPM },
            |m: &mut RaidKickByServerScNotify| { &mut m.EMDECAJPAPM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DGDDHBLKMLI",
            |m: &RaidKickByServerScNotify| { &m.DGDDHBLKMLI },
            |m: &mut RaidKickByServerScNotify| { &mut m.DGDDHBLKMLI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FJPJJEIJLLP::FJPJJEIJLLP>(
            "COCFMLGGMKE",
            |m: &RaidKickByServerScNotify| { &m.COCFMLGGMKE },
            |m: &mut RaidKickByServerScNotify| { &mut m.COCFMLGGMKE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RaidKickByServerScNotify>(
            "RaidKickByServerScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RaidKickByServerScNotify {
    const NAME: &'static str = "RaidKickByServerScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.OGIOAKBPMAE = is.read_uint32()?;
                },
                96 => {
                    self.MMMOHAJFMID = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EMDECAJPAPM)?;
                },
                112 => {
                    self.DGDDHBLKMLI = is.read_enum_or_unknown()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.COCFMLGGMKE)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.OGIOAKBPMAE != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.OGIOAKBPMAE);
        }
        if self.MMMOHAJFMID != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.MMMOHAJFMID);
        }
        if let Some(v) = self.EMDECAJPAPM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.DGDDHBLKMLI != ::protobuf::EnumOrUnknown::new(super::EGHDABBBNJO::EGHDABBBNJO::RAID_KICK_REASON_NONE) {
            my_size += ::protobuf::rt::int32_size(14, self.DGDDHBLKMLI.value());
        }
        if let Some(v) = self.COCFMLGGMKE.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.OGIOAKBPMAE != 0 {
            os.write_uint32(11, self.OGIOAKBPMAE)?;
        }
        if self.MMMOHAJFMID != 0 {
            os.write_uint32(12, self.MMMOHAJFMID)?;
        }
        if let Some(v) = self.EMDECAJPAPM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.DGDDHBLKMLI != ::protobuf::EnumOrUnknown::new(super::EGHDABBBNJO::EGHDABBBNJO::RAID_KICK_REASON_NONE) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.DGDDHBLKMLI))?;
        }
        if let Some(v) = self.COCFMLGGMKE.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> RaidKickByServerScNotify {
        RaidKickByServerScNotify::new()
    }

    fn clear(&mut self) {
        self.OGIOAKBPMAE = 0;
        self.MMMOHAJFMID = 0;
        self.EMDECAJPAPM.clear();
        self.DGDDHBLKMLI = ::protobuf::EnumOrUnknown::new(super::EGHDABBBNJO::EGHDABBBNJO::RAID_KICK_REASON_NONE);
        self.COCFMLGGMKE.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RaidKickByServerScNotify {
        static instance: RaidKickByServerScNotify = RaidKickByServerScNotify {
            OGIOAKBPMAE: 0,
            MMMOHAJFMID: 0,
            EMDECAJPAPM: ::protobuf::MessageField::none(),
            DGDDHBLKMLI: ::protobuf::EnumOrUnknown::from_i32(0),
            COCFMLGGMKE: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RaidKickByServerScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RaidKickByServerScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RaidKickByServerScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaidKickByServerScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eRaidKickByServerScNotify.proto\x1a\x11EGHDABBBNJO.proto\x1a\x11FHG\
    PCKGFGAO.proto\x1a\x11FJPJJEIJLLP.proto\"\xee\x01\n\x18RaidKickByServerS\
    cNotify\x12\x20\n\x0bOGIOAKBPMAE\x18\x0b\x20\x01(\rR\x0bOGIOAKBPMAE\x12\
    \x20\n\x0bMMMOHAJFMID\x18\x0c\x20\x01(\rR\x0bMMMOHAJFMID\x12.\n\x0bEMDEC\
    AJPAPM\x18\x07\x20\x01(\x0b2\x0c.FHGPCKGFGAOR\x0bEMDECAJPAPM\x12.\n\x0bD\
    GDDHBLKMLI\x18\x0e\x20\x01(\x0e2\x0c.EGHDABBBNJOR\x0bDGDDHBLKMLI\x12.\n\
    \x0bCOCFMLGGMKE\x18\t\x20\x01(\x0b2\x0c.FJPJJEIJLLPR\x0bCOCFMLGGMKEb\x06\
    proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::EGHDABBBNJO::file_descriptor().clone());
            deps.push(super::FHGPCKGFGAO::file_descriptor().clone());
            deps.push(super::FJPJJEIJLLP::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RaidKickByServerScNotify::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
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