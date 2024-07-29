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

//! Generated file from `SeaLampV3DetailInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SeaLampV3DetailInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SeaLampV3DetailInfo {
    // message fields
    // @@protoc_insertion_point(field:SeaLampV3DetailInfo.camp_info)
    pub camp_info: ::protobuf::MessageField<super::SeaLampV3CampInfo::SeaLampV3CampInfo>,
    // @@protoc_insertion_point(field:SeaLampV3DetailInfo.shadow_info)
    pub shadow_info: ::protobuf::MessageField<super::SeaLampV3ShadowInfo::SeaLampV3ShadowInfo>,
    // @@protoc_insertion_point(field:SeaLampV3DetailInfo.race_info)
    pub race_info: ::protobuf::MessageField<super::SeaLampV3RaceInfo::SeaLampV3RaceInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:SeaLampV3DetailInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SeaLampV3DetailInfo {
    fn default() -> &'a SeaLampV3DetailInfo {
        <SeaLampV3DetailInfo as ::protobuf::Message>::default_instance()
    }
}

impl SeaLampV3DetailInfo {
    pub fn new() -> SeaLampV3DetailInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::SeaLampV3CampInfo::SeaLampV3CampInfo>(
            "camp_info",
            |m: &SeaLampV3DetailInfo| { &m.camp_info },
            |m: &mut SeaLampV3DetailInfo| { &mut m.camp_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::SeaLampV3ShadowInfo::SeaLampV3ShadowInfo>(
            "shadow_info",
            |m: &SeaLampV3DetailInfo| { &m.shadow_info },
            |m: &mut SeaLampV3DetailInfo| { &mut m.shadow_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::SeaLampV3RaceInfo::SeaLampV3RaceInfo>(
            "race_info",
            |m: &SeaLampV3DetailInfo| { &m.race_info },
            |m: &mut SeaLampV3DetailInfo| { &mut m.race_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SeaLampV3DetailInfo>(
            "SeaLampV3DetailInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SeaLampV3DetailInfo {
    const NAME: &'static str = "SeaLampV3DetailInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.camp_info)?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.shadow_info)?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.race_info)?;
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
        if let Some(v) = self.camp_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.shadow_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.race_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.camp_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.shadow_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if let Some(v) = self.race_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> SeaLampV3DetailInfo {
        SeaLampV3DetailInfo::new()
    }

    fn clear(&mut self) {
        self.camp_info.clear();
        self.shadow_info.clear();
        self.race_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SeaLampV3DetailInfo {
        static instance: SeaLampV3DetailInfo = SeaLampV3DetailInfo {
            camp_info: ::protobuf::MessageField::none(),
            shadow_info: ::protobuf::MessageField::none(),
            race_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SeaLampV3DetailInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SeaLampV3DetailInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SeaLampV3DetailInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SeaLampV3DetailInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19SeaLampV3DetailInfo.proto\x1a\x17SeaLampV3CampInfo.proto\x1a\x19Se\
    aLampV3ShadowInfo.proto\x1a\x17SeaLampV3RaceInfo.proto\"\xae\x01\n\x13Se\
    aLampV3DetailInfo\x12/\n\tcamp_info\x18\x0b\x20\x01(\x0b2\x12.SeaLampV3C\
    ampInfoR\x08campInfo\x125\n\x0bshadow_info\x18\x07\x20\x01(\x0b2\x14.Sea\
    LampV3ShadowInfoR\nshadowInfo\x12/\n\trace_info\x18\x01\x20\x01(\x0b2\
    \x12.SeaLampV3RaceInfoR\x08raceInfoB\x1b\n\x19emu.grasscutter.net.protob\
    \x06proto3\
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
            deps.push(super::SeaLampV3CampInfo::file_descriptor().clone());
            deps.push(super::SeaLampV3ShadowInfo::file_descriptor().clone());
            deps.push(super::SeaLampV3RaceInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SeaLampV3DetailInfo::generated_message_descriptor_data());
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