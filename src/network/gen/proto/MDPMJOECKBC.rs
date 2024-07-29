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

//! Generated file from `MDPMJOECKBC.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MDPMJOECKBC)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MDPMJOECKBC {
    // message fields
    // @@protoc_insertion_point(field:MDPMJOECKBC.is_open)
    pub is_open: bool,
    // @@protoc_insertion_point(field:MDPMJOECKBC.level_id)
    pub level_id: u32,
    // @@protoc_insertion_point(field:MDPMJOECKBC.FMHAPOLMKGI)
    pub FMHAPOLMKGI: u32,
    // @@protoc_insertion_point(field:MDPMJOECKBC.is_finish)
    pub is_finish: bool,
    // special fields
    // @@protoc_insertion_point(special_field:MDPMJOECKBC.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MDPMJOECKBC {
    fn default() -> &'a MDPMJOECKBC {
        <MDPMJOECKBC as ::protobuf::Message>::default_instance()
    }
}

impl MDPMJOECKBC {
    pub fn new() -> MDPMJOECKBC {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_open",
            |m: &MDPMJOECKBC| { &m.is_open },
            |m: &mut MDPMJOECKBC| { &mut m.is_open },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level_id",
            |m: &MDPMJOECKBC| { &m.level_id },
            |m: &mut MDPMJOECKBC| { &mut m.level_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FMHAPOLMKGI",
            |m: &MDPMJOECKBC| { &m.FMHAPOLMKGI },
            |m: &mut MDPMJOECKBC| { &mut m.FMHAPOLMKGI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_finish",
            |m: &MDPMJOECKBC| { &m.is_finish },
            |m: &mut MDPMJOECKBC| { &mut m.is_finish },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MDPMJOECKBC>(
            "MDPMJOECKBC",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MDPMJOECKBC {
    const NAME: &'static str = "MDPMJOECKBC";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.is_open = is.read_bool()?;
                },
                48 => {
                    self.level_id = is.read_uint32()?;
                },
                64 => {
                    self.FMHAPOLMKGI = is.read_uint32()?;
                },
                8 => {
                    self.is_finish = is.read_bool()?;
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
        if self.is_open != false {
            my_size += 1 + 1;
        }
        if self.level_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.level_id);
        }
        if self.FMHAPOLMKGI != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.FMHAPOLMKGI);
        }
        if self.is_finish != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_open != false {
            os.write_bool(7, self.is_open)?;
        }
        if self.level_id != 0 {
            os.write_uint32(6, self.level_id)?;
        }
        if self.FMHAPOLMKGI != 0 {
            os.write_uint32(8, self.FMHAPOLMKGI)?;
        }
        if self.is_finish != false {
            os.write_bool(1, self.is_finish)?;
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

    fn new() -> MDPMJOECKBC {
        MDPMJOECKBC::new()
    }

    fn clear(&mut self) {
        self.is_open = false;
        self.level_id = 0;
        self.FMHAPOLMKGI = 0;
        self.is_finish = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MDPMJOECKBC {
        static instance: MDPMJOECKBC = MDPMJOECKBC {
            is_open: false,
            level_id: 0,
            FMHAPOLMKGI: 0,
            is_finish: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MDPMJOECKBC {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MDPMJOECKBC").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MDPMJOECKBC {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MDPMJOECKBC {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MDPMJOECKBC.proto\"\x80\x01\n\x0bMDPMJOECKBC\x12\x17\n\x07is_open\
    \x18\x07\x20\x01(\x08R\x06isOpen\x12\x19\n\x08level_id\x18\x06\x20\x01(\
    \rR\x07levelId\x12\x20\n\x0bFMHAPOLMKGI\x18\x08\x20\x01(\rR\x0bFMHAPOLMK\
    GI\x12\x1b\n\tis_finish\x18\x01\x20\x01(\x08R\x08isFinishB\x1b\n\x19emu.\
    grasscutter.net.protob\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MDPMJOECKBC::generated_message_descriptor_data());
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