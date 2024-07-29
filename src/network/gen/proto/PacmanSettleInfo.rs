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

//! Generated file from `PacmanSettleInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PacmanSettleInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PacmanSettleInfo {
    // message fields
    // @@protoc_insertion_point(field:PacmanSettleInfo.final_score)
    pub final_score: u32,
    // @@protoc_insertion_point(field:PacmanSettleInfo.GIMCFEIADKI)
    pub GIMCFEIADKI: u32,
    // @@protoc_insertion_point(field:PacmanSettleInfo.is_new_record)
    pub is_new_record: bool,
    // @@protoc_insertion_point(field:PacmanSettleInfo.NHFGIBDCPGC)
    pub NHFGIBDCPGC: u32,
    // @@protoc_insertion_point(field:PacmanSettleInfo.reason)
    pub reason: ::protobuf::EnumOrUnknown<super::FKMJAPDCONH::FKMJAPDCONH>,
    // special fields
    // @@protoc_insertion_point(special_field:PacmanSettleInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PacmanSettleInfo {
    fn default() -> &'a PacmanSettleInfo {
        <PacmanSettleInfo as ::protobuf::Message>::default_instance()
    }
}

impl PacmanSettleInfo {
    pub fn new() -> PacmanSettleInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "final_score",
            |m: &PacmanSettleInfo| { &m.final_score },
            |m: &mut PacmanSettleInfo| { &mut m.final_score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GIMCFEIADKI",
            |m: &PacmanSettleInfo| { &m.GIMCFEIADKI },
            |m: &mut PacmanSettleInfo| { &mut m.GIMCFEIADKI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_new_record",
            |m: &PacmanSettleInfo| { &m.is_new_record },
            |m: &mut PacmanSettleInfo| { &mut m.is_new_record },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NHFGIBDCPGC",
            |m: &PacmanSettleInfo| { &m.NHFGIBDCPGC },
            |m: &mut PacmanSettleInfo| { &mut m.NHFGIBDCPGC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "reason",
            |m: &PacmanSettleInfo| { &m.reason },
            |m: &mut PacmanSettleInfo| { &mut m.reason },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PacmanSettleInfo>(
            "PacmanSettleInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PacmanSettleInfo {
    const NAME: &'static str = "PacmanSettleInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.final_score = is.read_uint32()?;
                },
                48 => {
                    self.GIMCFEIADKI = is.read_uint32()?;
                },
                24 => {
                    self.is_new_record = is.read_bool()?;
                },
                32 => {
                    self.NHFGIBDCPGC = is.read_uint32()?;
                },
                40 => {
                    self.reason = is.read_enum_or_unknown()?;
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
        if self.final_score != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.final_score);
        }
        if self.GIMCFEIADKI != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.GIMCFEIADKI);
        }
        if self.is_new_record != false {
            my_size += 1 + 1;
        }
        if self.NHFGIBDCPGC != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.NHFGIBDCPGC);
        }
        if self.reason != ::protobuf::EnumOrUnknown::new(super::FKMJAPDCONH::FKMJAPDCONH::FKMJAPDCONH_PacmanDungeonStopNone) {
            my_size += ::protobuf::rt::int32_size(5, self.reason.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.final_score != 0 {
            os.write_uint32(8, self.final_score)?;
        }
        if self.GIMCFEIADKI != 0 {
            os.write_uint32(6, self.GIMCFEIADKI)?;
        }
        if self.is_new_record != false {
            os.write_bool(3, self.is_new_record)?;
        }
        if self.NHFGIBDCPGC != 0 {
            os.write_uint32(4, self.NHFGIBDCPGC)?;
        }
        if self.reason != ::protobuf::EnumOrUnknown::new(super::FKMJAPDCONH::FKMJAPDCONH::FKMJAPDCONH_PacmanDungeonStopNone) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.reason))?;
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

    fn new() -> PacmanSettleInfo {
        PacmanSettleInfo::new()
    }

    fn clear(&mut self) {
        self.final_score = 0;
        self.GIMCFEIADKI = 0;
        self.is_new_record = false;
        self.NHFGIBDCPGC = 0;
        self.reason = ::protobuf::EnumOrUnknown::new(super::FKMJAPDCONH::FKMJAPDCONH::FKMJAPDCONH_PacmanDungeonStopNone);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PacmanSettleInfo {
        static instance: PacmanSettleInfo = PacmanSettleInfo {
            final_score: 0,
            GIMCFEIADKI: 0,
            is_new_record: false,
            NHFGIBDCPGC: 0,
            reason: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PacmanSettleInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PacmanSettleInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PacmanSettleInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PacmanSettleInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16PacmanSettleInfo.proto\x1a\x11FKMJAPDCONH.proto\"\xc1\x01\n\x10Pac\
    manSettleInfo\x12\x1f\n\x0bfinal_score\x18\x08\x20\x01(\rR\nfinalScore\
    \x12\x20\n\x0bGIMCFEIADKI\x18\x06\x20\x01(\rR\x0bGIMCFEIADKI\x12\"\n\ris\
    _new_record\x18\x03\x20\x01(\x08R\x0bisNewRecord\x12\x20\n\x0bNHFGIBDCPG\
    C\x18\x04\x20\x01(\rR\x0bNHFGIBDCPGC\x12$\n\x06reason\x18\x05\x20\x01(\
    \x0e2\x0c.FKMJAPDCONHR\x06reasonB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::FKMJAPDCONH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PacmanSettleInfo::generated_message_descriptor_data());
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