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

//! Generated file from `CrystalLinkLevelInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CrystalLinkLevelInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CrystalLinkLevelInfo {
    // message fields
    // @@protoc_insertion_point(field:CrystalLinkLevelInfo.level_id)
    pub level_id: u32,
    // @@protoc_insertion_point(field:CrystalLinkLevelInfo.best_score)
    pub best_score: u32,
    // @@protoc_insertion_point(field:CrystalLinkLevelInfo.team_info_list)
    pub team_info_list: ::std::vec::Vec<super::CrystalLinkTeamInfo::CrystalLinkTeamInfo>,
    // @@protoc_insertion_point(field:CrystalLinkLevelInfo.is_open)
    pub is_open: bool,
    // special fields
    // @@protoc_insertion_point(special_field:CrystalLinkLevelInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CrystalLinkLevelInfo {
    fn default() -> &'a CrystalLinkLevelInfo {
        <CrystalLinkLevelInfo as ::protobuf::Message>::default_instance()
    }
}

impl CrystalLinkLevelInfo {
    pub fn new() -> CrystalLinkLevelInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level_id",
            |m: &CrystalLinkLevelInfo| { &m.level_id },
            |m: &mut CrystalLinkLevelInfo| { &mut m.level_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "best_score",
            |m: &CrystalLinkLevelInfo| { &m.best_score },
            |m: &mut CrystalLinkLevelInfo| { &mut m.best_score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "team_info_list",
            |m: &CrystalLinkLevelInfo| { &m.team_info_list },
            |m: &mut CrystalLinkLevelInfo| { &mut m.team_info_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_open",
            |m: &CrystalLinkLevelInfo| { &m.is_open },
            |m: &mut CrystalLinkLevelInfo| { &mut m.is_open },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CrystalLinkLevelInfo>(
            "CrystalLinkLevelInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CrystalLinkLevelInfo {
    const NAME: &'static str = "CrystalLinkLevelInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.level_id = is.read_uint32()?;
                },
                24 => {
                    self.best_score = is.read_uint32()?;
                },
                98 => {
                    self.team_info_list.push(is.read_message()?);
                },
                112 => {
                    self.is_open = is.read_bool()?;
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
        if self.level_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.level_id);
        }
        if self.best_score != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.best_score);
        }
        for value in &self.team_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.is_open != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level_id != 0 {
            os.write_uint32(2, self.level_id)?;
        }
        if self.best_score != 0 {
            os.write_uint32(3, self.best_score)?;
        }
        for v in &self.team_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        };
        if self.is_open != false {
            os.write_bool(14, self.is_open)?;
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

    fn new() -> CrystalLinkLevelInfo {
        CrystalLinkLevelInfo::new()
    }

    fn clear(&mut self) {
        self.level_id = 0;
        self.best_score = 0;
        self.team_info_list.clear();
        self.is_open = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CrystalLinkLevelInfo {
        static instance: CrystalLinkLevelInfo = CrystalLinkLevelInfo {
            level_id: 0,
            best_score: 0,
            team_info_list: ::std::vec::Vec::new(),
            is_open: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CrystalLinkLevelInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CrystalLinkLevelInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CrystalLinkLevelInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrystalLinkLevelInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aCrystalLinkLevelInfo.proto\x1a\x19CrystalLinkTeamInfo.proto\"\xa5\
    \x01\n\x14CrystalLinkLevelInfo\x12\x19\n\x08level_id\x18\x02\x20\x01(\rR\
    \x07levelId\x12\x1d\n\nbest_score\x18\x03\x20\x01(\rR\tbestScore\x12:\n\
    \x0eteam_info_list\x18\x0c\x20\x03(\x0b2\x14.CrystalLinkTeamInfoR\x0ctea\
    mInfoList\x12\x17\n\x07is_open\x18\x0e\x20\x01(\x08R\x06isOpenB\x1b\n\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::CrystalLinkTeamInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CrystalLinkLevelInfo::generated_message_descriptor_data());
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