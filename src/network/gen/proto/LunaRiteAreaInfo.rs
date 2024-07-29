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

//! Generated file from `LunaRiteAreaInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LunaRiteAreaInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LunaRiteAreaInfo {
    // message fields
    // @@protoc_insertion_point(field:LunaRiteAreaInfo.NLADEGOOIDA)
    pub NLADEGOOIDA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LunaRiteAreaInfo.hint_status)
    pub hint_status: ::protobuf::EnumOrUnknown<super::LunaRiteHintStatusType::LunaRiteHintStatusType>,
    // @@protoc_insertion_point(field:LunaRiteAreaInfo.PAJINADKPNM)
    pub PAJINADKPNM: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LunaRiteAreaInfo.area_id)
    pub area_id: u32,
    // @@protoc_insertion_point(field:LunaRiteAreaInfo.challenge_index)
    pub challenge_index: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LunaRiteAreaInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LunaRiteAreaInfo {
    fn default() -> &'a LunaRiteAreaInfo {
        <LunaRiteAreaInfo as ::protobuf::Message>::default_instance()
    }
}

impl LunaRiteAreaInfo {
    pub fn new() -> LunaRiteAreaInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NLADEGOOIDA",
            |m: &LunaRiteAreaInfo| { &m.NLADEGOOIDA },
            |m: &mut LunaRiteAreaInfo| { &mut m.NLADEGOOIDA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hint_status",
            |m: &LunaRiteAreaInfo| { &m.hint_status },
            |m: &mut LunaRiteAreaInfo| { &mut m.hint_status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PAJINADKPNM",
            |m: &LunaRiteAreaInfo| { &m.PAJINADKPNM },
            |m: &mut LunaRiteAreaInfo| { &mut m.PAJINADKPNM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "area_id",
            |m: &LunaRiteAreaInfo| { &m.area_id },
            |m: &mut LunaRiteAreaInfo| { &mut m.area_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "challenge_index",
            |m: &LunaRiteAreaInfo| { &m.challenge_index },
            |m: &mut LunaRiteAreaInfo| { &mut m.challenge_index },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LunaRiteAreaInfo>(
            "LunaRiteAreaInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LunaRiteAreaInfo {
    const NAME: &'static str = "LunaRiteAreaInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.NLADEGOOIDA)?;
                },
                16 => {
                    self.NLADEGOOIDA.push(is.read_uint32()?);
                },
                56 => {
                    self.hint_status = is.read_enum_or_unknown()?;
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.PAJINADKPNM)?;
                },
                64 => {
                    self.PAJINADKPNM.push(is.read_uint32()?);
                },
                72 => {
                    self.area_id = is.read_uint32()?;
                },
                80 => {
                    self.challenge_index = is.read_uint32()?;
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
        for value in &self.NLADEGOOIDA {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        if self.hint_status != ::protobuf::EnumOrUnknown::new(super::LunaRiteHintStatusType::LunaRiteHintStatusType::LUNA_RITE_HINT_STATUS_DEFAULT) {
            my_size += ::protobuf::rt::int32_size(7, self.hint_status.value());
        }
        for value in &self.PAJINADKPNM {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        if self.area_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.area_id);
        }
        if self.challenge_index != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.challenge_index);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.NLADEGOOIDA {
            os.write_uint32(2, *v)?;
        };
        if self.hint_status != ::protobuf::EnumOrUnknown::new(super::LunaRiteHintStatusType::LunaRiteHintStatusType::LUNA_RITE_HINT_STATUS_DEFAULT) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.hint_status))?;
        }
        for v in &self.PAJINADKPNM {
            os.write_uint32(8, *v)?;
        };
        if self.area_id != 0 {
            os.write_uint32(9, self.area_id)?;
        }
        if self.challenge_index != 0 {
            os.write_uint32(10, self.challenge_index)?;
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

    fn new() -> LunaRiteAreaInfo {
        LunaRiteAreaInfo::new()
    }

    fn clear(&mut self) {
        self.NLADEGOOIDA.clear();
        self.hint_status = ::protobuf::EnumOrUnknown::new(super::LunaRiteHintStatusType::LunaRiteHintStatusType::LUNA_RITE_HINT_STATUS_DEFAULT);
        self.PAJINADKPNM.clear();
        self.area_id = 0;
        self.challenge_index = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LunaRiteAreaInfo {
        static instance: LunaRiteAreaInfo = LunaRiteAreaInfo {
            NLADEGOOIDA: ::std::vec::Vec::new(),
            hint_status: ::protobuf::EnumOrUnknown::from_i32(0),
            PAJINADKPNM: ::std::vec::Vec::new(),
            area_id: 0,
            challenge_index: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LunaRiteAreaInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LunaRiteAreaInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LunaRiteAreaInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LunaRiteAreaInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16LunaRiteAreaInfo.proto\x1a\x1cLunaRiteHintStatusType.proto\"\xd2\
    \x01\n\x10LunaRiteAreaInfo\x12\x20\n\x0bNLADEGOOIDA\x18\x02\x20\x03(\rR\
    \x0bNLADEGOOIDA\x128\n\x0bhint_status\x18\x07\x20\x01(\x0e2\x17.LunaRite\
    HintStatusTypeR\nhintStatus\x12\x20\n\x0bPAJINADKPNM\x18\x08\x20\x03(\rR\
    \x0bPAJINADKPNM\x12\x17\n\x07area_id\x18\t\x20\x01(\rR\x06areaId\x12'\n\
    \x0fchallenge_index\x18\n\x20\x01(\rR\x0echallengeIndexB\x1b\n\x19emu.gr\
    asscutter.net.protob\x06proto3\
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
            deps.push(super::LunaRiteHintStatusType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LunaRiteAreaInfo::generated_message_descriptor_data());
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