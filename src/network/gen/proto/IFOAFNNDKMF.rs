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

//! Generated file from `IFOAFNNDKMF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IFOAFNNDKMF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IFOAFNNDKMF {
    // message fields
    // @@protoc_insertion_point(field:IFOAFNNDKMF.OOEHLCOLPED)
    pub OOEHLCOLPED: ::protobuf::MessageField<super::BILHHPCGJPP::BILHHPCGJPP>,
    // @@protoc_insertion_point(field:IFOAFNNDKMF.PCMOBMKHKKB)
    pub PCMOBMKHKKB: ::std::vec::Vec<super::PPKNNAOJJMK::PPKNNAOJJMK>,
    // special fields
    // @@protoc_insertion_point(special_field:IFOAFNNDKMF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IFOAFNNDKMF {
    fn default() -> &'a IFOAFNNDKMF {
        <IFOAFNNDKMF as ::protobuf::Message>::default_instance()
    }
}

impl IFOAFNNDKMF {
    pub fn new() -> IFOAFNNDKMF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BILHHPCGJPP::BILHHPCGJPP>(
            "OOEHLCOLPED",
            |m: &IFOAFNNDKMF| { &m.OOEHLCOLPED },
            |m: &mut IFOAFNNDKMF| { &mut m.OOEHLCOLPED },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PCMOBMKHKKB",
            |m: &IFOAFNNDKMF| { &m.PCMOBMKHKKB },
            |m: &mut IFOAFNNDKMF| { &mut m.PCMOBMKHKKB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IFOAFNNDKMF>(
            "IFOAFNNDKMF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IFOAFNNDKMF {
    const NAME: &'static str = "IFOAFNNDKMF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OOEHLCOLPED)?;
                },
                18 => {
                    self.PCMOBMKHKKB.push(is.read_message()?);
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
        if let Some(v) = self.OOEHLCOLPED.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.PCMOBMKHKKB {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.OOEHLCOLPED.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        for v in &self.PCMOBMKHKKB {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> IFOAFNNDKMF {
        IFOAFNNDKMF::new()
    }

    fn clear(&mut self) {
        self.OOEHLCOLPED.clear();
        self.PCMOBMKHKKB.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IFOAFNNDKMF {
        static instance: IFOAFNNDKMF = IFOAFNNDKMF {
            OOEHLCOLPED: ::protobuf::MessageField::none(),
            PCMOBMKHKKB: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IFOAFNNDKMF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IFOAFNNDKMF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IFOAFNNDKMF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IFOAFNNDKMF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11IFOAFNNDKMF.proto\x1a\x11BILHHPCGJPP.proto\x1a\x11PPKNNAOJJMK.prot\
    o\"m\n\x0bIFOAFNNDKMF\x12.\n\x0bOOEHLCOLPED\x18\x01\x20\x01(\x0b2\x0c.BI\
    LHHPCGJPPR\x0bOOEHLCOLPED\x12.\n\x0bPCMOBMKHKKB\x18\x02\x20\x03(\x0b2\
    \x0c.PPKNNAOJJMKR\x0bPCMOBMKHKKBb\x06proto3\
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
            deps.push(super::BILHHPCGJPP::file_descriptor().clone());
            deps.push(super::PPKNNAOJJMK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IFOAFNNDKMF::generated_message_descriptor_data());
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