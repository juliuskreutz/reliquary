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

//! Generated file from `SummerTimeDetailInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SummerTimeDetailInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SummerTimeDetailInfo {
    // message fields
    // @@protoc_insertion_point(field:SummerTimeDetailInfo.sprint_boat_info)
    pub sprint_boat_info: ::protobuf::MessageField<super::SummerTimeSprintBoatInfo::SummerTimeSprintBoatInfo>,
    // @@protoc_insertion_point(field:SummerTimeDetailInfo.content_close_time)
    pub content_close_time: u32,
    // @@protoc_insertion_point(field:SummerTimeDetailInfo.stage_map)
    pub stage_map: ::std::collections::HashMap<u32, super::SummerTimeStageInfo::SummerTimeStageInfo>,
    // @@protoc_insertion_point(field:SummerTimeDetailInfo.is_content_closed)
    pub is_content_closed: bool,
    // special fields
    // @@protoc_insertion_point(special_field:SummerTimeDetailInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SummerTimeDetailInfo {
    fn default() -> &'a SummerTimeDetailInfo {
        <SummerTimeDetailInfo as ::protobuf::Message>::default_instance()
    }
}

impl SummerTimeDetailInfo {
    pub fn new() -> SummerTimeDetailInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::SummerTimeSprintBoatInfo::SummerTimeSprintBoatInfo>(
            "sprint_boat_info",
            |m: &SummerTimeDetailInfo| { &m.sprint_boat_info },
            |m: &mut SummerTimeDetailInfo| { &mut m.sprint_boat_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "content_close_time",
            |m: &SummerTimeDetailInfo| { &m.content_close_time },
            |m: &mut SummerTimeDetailInfo| { &mut m.content_close_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "stage_map",
            |m: &SummerTimeDetailInfo| { &m.stage_map },
            |m: &mut SummerTimeDetailInfo| { &mut m.stage_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_content_closed",
            |m: &SummerTimeDetailInfo| { &m.is_content_closed },
            |m: &mut SummerTimeDetailInfo| { &mut m.is_content_closed },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SummerTimeDetailInfo>(
            "SummerTimeDetailInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SummerTimeDetailInfo {
    const NAME: &'static str = "SummerTimeDetailInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.sprint_boat_info)?;
                },
                48 => {
                    self.content_close_time = is.read_uint32()?;
                },
                106 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.stage_map.insert(key, value);
                },
                96 => {
                    self.is_content_closed = is.read_bool()?;
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
        if let Some(v) = self.sprint_boat_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.content_close_time != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.content_close_time);
        }
        for (k, v) in &self.stage_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.is_content_closed != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.sprint_boat_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.content_close_time != 0 {
            os.write_uint32(6, self.content_close_time)?;
        }
        for (k, v) in &self.stage_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(106)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.is_content_closed != false {
            os.write_bool(12, self.is_content_closed)?;
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

    fn new() -> SummerTimeDetailInfo {
        SummerTimeDetailInfo::new()
    }

    fn clear(&mut self) {
        self.sprint_boat_info.clear();
        self.content_close_time = 0;
        self.stage_map.clear();
        self.is_content_closed = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SummerTimeDetailInfo {
        static instance: ::protobuf::rt::Lazy<SummerTimeDetailInfo> = ::protobuf::rt::Lazy::new();
        instance.get(SummerTimeDetailInfo::new)
    }
}

impl ::protobuf::MessageFull for SummerTimeDetailInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SummerTimeDetailInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SummerTimeDetailInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SummerTimeDetailInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aSummerTimeDetailInfo.proto\x1a\x1eSummerTimeSprintBoatInfo.proto\
    \x1a\x19SummerTimeStageInfo.proto\"\xca\x02\n\x14SummerTimeDetailInfo\
    \x12C\n\x10sprint_boat_info\x18\x01\x20\x01(\x0b2\x19.SummerTimeSprintBo\
    atInfoR\x0esprintBoatInfo\x12,\n\x12content_close_time\x18\x06\x20\x01(\
    \rR\x10contentCloseTime\x12@\n\tstage_map\x18\r\x20\x03(\x0b2#.SummerTim\
    eDetailInfo.StageMapEntryR\x08stageMap\x12*\n\x11is_content_closed\x18\
    \x0c\x20\x01(\x08R\x0fisContentClosed\x1aQ\n\rStageMapEntry\x12\x10\n\
    \x03key\x18\x01\x20\x01(\rR\x03key\x12*\n\x05value\x18\x02\x20\x01(\x0b2\
    \x14.SummerTimeStageInfoR\x05value:\x028\x01B\x1b\n\x19emu.grasscutter.n\
    et.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::SummerTimeSprintBoatInfo::file_descriptor().clone());
            deps.push(super::SummerTimeStageInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SummerTimeDetailInfo::generated_message_descriptor_data());
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
