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

//! Generated file from `GetUgcRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetUgcRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetUgcRsp {
    // message fields
    // @@protoc_insertion_point(field:GetUgcRsp.ugc_record_usage)
    pub ugc_record_usage: ::protobuf::EnumOrUnknown<super::RecordUsage::RecordUsage>,
    // @@protoc_insertion_point(field:GetUgcRsp.ugc_guid)
    pub ugc_guid: u64,
    // @@protoc_insertion_point(field:GetUgcRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:GetUgcRsp.ugc_type)
    pub ugc_type: ::protobuf::EnumOrUnknown<super::UgcType::UgcType>,
    // @@protoc_insertion_point(field:GetUgcRsp.music_record)
    pub music_record: ::protobuf::MessageField<super::UgcMusicRecord::UgcMusicRecord>,
    // @@protoc_insertion_point(field:GetUgcRsp.music_brief_info)
    pub music_brief_info: ::protobuf::MessageField<super::UgcMusicBriefInfo::UgcMusicBriefInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:GetUgcRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetUgcRsp {
    fn default() -> &'a GetUgcRsp {
        <GetUgcRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetUgcRsp {
    pub fn new() -> GetUgcRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ugc_record_usage",
            |m: &GetUgcRsp| { &m.ugc_record_usage },
            |m: &mut GetUgcRsp| { &mut m.ugc_record_usage },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ugc_guid",
            |m: &GetUgcRsp| { &m.ugc_guid },
            |m: &mut GetUgcRsp| { &mut m.ugc_guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetUgcRsp| { &m.retcode },
            |m: &mut GetUgcRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ugc_type",
            |m: &GetUgcRsp| { &m.ugc_type },
            |m: &mut GetUgcRsp| { &mut m.ugc_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::UgcMusicRecord::UgcMusicRecord>(
            "music_record",
            |m: &GetUgcRsp| { &m.music_record },
            |m: &mut GetUgcRsp| { &mut m.music_record },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::UgcMusicBriefInfo::UgcMusicBriefInfo>(
            "music_brief_info",
            |m: &GetUgcRsp| { &m.music_brief_info },
            |m: &mut GetUgcRsp| { &mut m.music_brief_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetUgcRsp>(
            "GetUgcRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetUgcRsp {
    const NAME: &'static str = "GetUgcRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.ugc_record_usage = is.read_enum_or_unknown()?;
                },
                48 => {
                    self.ugc_guid = is.read_uint64()?;
                },
                72 => {
                    self.retcode = is.read_int32()?;
                },
                112 => {
                    self.ugc_type = is.read_enum_or_unknown()?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.music_record)?;
                },
                6330 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.music_brief_info)?;
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
        if self.ugc_record_usage != ::protobuf::EnumOrUnknown::new(super::RecordUsage::RecordUsage::UGC_RECORD_USAGE_NONE) {
            my_size += ::protobuf::rt::int32_size(13, self.ugc_record_usage.value());
        }
        if self.ugc_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(6, self.ugc_guid);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(9, self.retcode);
        }
        if self.ugc_type != ::protobuf::EnumOrUnknown::new(super::UgcType::UgcType::UGC_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(14, self.ugc_type.value());
        }
        if let Some(v) = self.music_record.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.music_brief_info.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ugc_record_usage != ::protobuf::EnumOrUnknown::new(super::RecordUsage::RecordUsage::UGC_RECORD_USAGE_NONE) {
            os.write_enum(13, ::protobuf::EnumOrUnknown::value(&self.ugc_record_usage))?;
        }
        if self.ugc_guid != 0 {
            os.write_uint64(6, self.ugc_guid)?;
        }
        if self.retcode != 0 {
            os.write_int32(9, self.retcode)?;
        }
        if self.ugc_type != ::protobuf::EnumOrUnknown::new(super::UgcType::UgcType::UGC_TYPE_NONE) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.ugc_type))?;
        }
        if let Some(v) = self.music_record.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.music_brief_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(791, v, os)?;
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

    fn new() -> GetUgcRsp {
        GetUgcRsp::new()
    }

    fn clear(&mut self) {
        self.ugc_record_usage = ::protobuf::EnumOrUnknown::new(super::RecordUsage::RecordUsage::UGC_RECORD_USAGE_NONE);
        self.ugc_guid = 0;
        self.retcode = 0;
        self.ugc_type = ::protobuf::EnumOrUnknown::new(super::UgcType::UgcType::UGC_TYPE_NONE);
        self.music_record.clear();
        self.music_brief_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetUgcRsp {
        static instance: GetUgcRsp = GetUgcRsp {
            ugc_record_usage: ::protobuf::EnumOrUnknown::from_i32(0),
            ugc_guid: 0,
            retcode: 0,
            ugc_type: ::protobuf::EnumOrUnknown::from_i32(0),
            music_record: ::protobuf::MessageField::none(),
            music_brief_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetUgcRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetUgcRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetUgcRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetUgcRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fGetUgcRsp.proto\x1a\x11RecordUsage.proto\x1a\rUgcType.proto\x1a\
    \x14UgcMusicRecord.proto\x1a\x17UgcMusicBriefInfo.proto\"\xc0\x02\n\tGet\
    UgcRsp\x126\n\x10ugc_record_usage\x18\r\x20\x01(\x0e2\x0c.RecordUsageR\
    \x0eugcRecordUsage\x12\x19\n\x08ugc_guid\x18\x06\x20\x01(\x04R\x07ugcGui\
    d\x12\x18\n\x07retcode\x18\t\x20\x01(\x05R\x07retcode\x12#\n\x08ugc_type\
    \x18\x0e\x20\x01(\x0e2\x08.UgcTypeR\x07ugcType\x127\n\x0cmusic_record\
    \x18\n\x20\x01(\x0b2\x0f.UgcMusicRecordH\0R\x0bmusicRecord\x88\x01\x01\
    \x12B\n\x10music_brief_info\x18\x97\x06\x20\x01(\x0b2\x12.UgcMusicBriefI\
    nfoH\x01R\x0emusicBriefInfo\x88\x01\x01B\x0f\n\r_music_recordB\x13\n\x11\
    _music_brief_infoB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::RecordUsage::file_descriptor().clone());
            deps.push(super::UgcType::file_descriptor().clone());
            deps.push(super::UgcMusicRecord::file_descriptor().clone());
            deps.push(super::UgcMusicBriefInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetUgcRsp::generated_message_descriptor_data());
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
