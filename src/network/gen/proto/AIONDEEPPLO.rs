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

//! Generated file from `AIONDEEPPLO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AIONDEEPPLO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AIONDEEPPLO {
    // message fields
    // @@protoc_insertion_point(field:AIONDEEPPLO.PLNPPJMHGFE)
    pub PLNPPJMHGFE: ::std::vec::Vec<super::CCMHPCONEME::CCMHPCONEME>,
    // @@protoc_insertion_point(field:AIONDEEPPLO.DOKHJDCPPIB)
    pub DOKHJDCPPIB: ::std::vec::Vec<super::CCMHPCONEME::CCMHPCONEME>,
    // @@protoc_insertion_point(field:AIONDEEPPLO.KFCIJHFJDCA)
    pub KFCIJHFJDCA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AIONDEEPPLO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AIONDEEPPLO {
    fn default() -> &'a AIONDEEPPLO {
        <AIONDEEPPLO as ::protobuf::Message>::default_instance()
    }
}

impl AIONDEEPPLO {
    pub fn new() -> AIONDEEPPLO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PLNPPJMHGFE",
            |m: &AIONDEEPPLO| { &m.PLNPPJMHGFE },
            |m: &mut AIONDEEPPLO| { &mut m.PLNPPJMHGFE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DOKHJDCPPIB",
            |m: &AIONDEEPPLO| { &m.DOKHJDCPPIB },
            |m: &mut AIONDEEPPLO| { &mut m.DOKHJDCPPIB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KFCIJHFJDCA",
            |m: &AIONDEEPPLO| { &m.KFCIJHFJDCA },
            |m: &mut AIONDEEPPLO| { &mut m.KFCIJHFJDCA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AIONDEEPPLO>(
            "AIONDEEPPLO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AIONDEEPPLO {
    const NAME: &'static str = "AIONDEEPPLO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.PLNPPJMHGFE.push(is.read_message()?);
                },
                58 => {
                    self.DOKHJDCPPIB.push(is.read_message()?);
                },
                64 => {
                    self.KFCIJHFJDCA = is.read_uint32()?;
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
        for value in &self.PLNPPJMHGFE {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.DOKHJDCPPIB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.KFCIJHFJDCA != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.KFCIJHFJDCA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.PLNPPJMHGFE {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        };
        for v in &self.DOKHJDCPPIB {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        if self.KFCIJHFJDCA != 0 {
            os.write_uint32(8, self.KFCIJHFJDCA)?;
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

    fn new() -> AIONDEEPPLO {
        AIONDEEPPLO::new()
    }

    fn clear(&mut self) {
        self.PLNPPJMHGFE.clear();
        self.DOKHJDCPPIB.clear();
        self.KFCIJHFJDCA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AIONDEEPPLO {
        static instance: AIONDEEPPLO = AIONDEEPPLO {
            PLNPPJMHGFE: ::std::vec::Vec::new(),
            DOKHJDCPPIB: ::std::vec::Vec::new(),
            KFCIJHFJDCA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AIONDEEPPLO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AIONDEEPPLO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AIONDEEPPLO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AIONDEEPPLO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AIONDEEPPLO.proto\x1a\x11CCMHPCONEME.proto\"\x8f\x01\n\x0bAIONDEEP\
    PLO\x12.\n\x0bPLNPPJMHGFE\x18\x01\x20\x03(\x0b2\x0c.CCMHPCONEMER\x0bPLNP\
    PJMHGFE\x12.\n\x0bDOKHJDCPPIB\x18\x07\x20\x03(\x0b2\x0c.CCMHPCONEMER\x0b\
    DOKHJDCPPIB\x12\x20\n\x0bKFCIJHFJDCA\x18\x08\x20\x01(\rR\x0bKFCIJHFJDCAb\
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
            deps.push(super::CCMHPCONEME::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AIONDEEPPLO::generated_message_descriptor_data());
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