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

//! Generated file from `EHOJLJMGOGL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EHOJLJMGOGL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EHOJLJMGOGL {
    // message fields
    // @@protoc_insertion_point(field:EHOJLJMGOGL.NAGBMPFECGG)
    pub NAGBMPFECGG: u32,
    // @@protoc_insertion_point(field:EHOJLJMGOGL.HFMDIBOEOKN)
    pub HFMDIBOEOKN: u32,
    // @@protoc_insertion_point(field:EHOJLJMGOGL.HCKOMMJEOEK)
    pub HCKOMMJEOEK: ::std::vec::Vec<super::NEKKDMCDGPK::NEKKDMCDGPK>,
    // @@protoc_insertion_point(field:EHOJLJMGOGL.NLMOBJCCDEL)
    pub NLMOBJCCDEL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EHOJLJMGOGL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EHOJLJMGOGL {
    fn default() -> &'a EHOJLJMGOGL {
        <EHOJLJMGOGL as ::protobuf::Message>::default_instance()
    }
}

impl EHOJLJMGOGL {
    pub fn new() -> EHOJLJMGOGL {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NAGBMPFECGG",
            |m: &EHOJLJMGOGL| { &m.NAGBMPFECGG },
            |m: &mut EHOJLJMGOGL| { &mut m.NAGBMPFECGG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HFMDIBOEOKN",
            |m: &EHOJLJMGOGL| { &m.HFMDIBOEOKN },
            |m: &mut EHOJLJMGOGL| { &mut m.HFMDIBOEOKN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HCKOMMJEOEK",
            |m: &EHOJLJMGOGL| { &m.HCKOMMJEOEK },
            |m: &mut EHOJLJMGOGL| { &mut m.HCKOMMJEOEK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NLMOBJCCDEL",
            |m: &EHOJLJMGOGL| { &m.NLMOBJCCDEL },
            |m: &mut EHOJLJMGOGL| { &mut m.NLMOBJCCDEL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EHOJLJMGOGL>(
            "EHOJLJMGOGL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EHOJLJMGOGL {
    const NAME: &'static str = "EHOJLJMGOGL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.NAGBMPFECGG = is.read_uint32()?;
                },
                16 => {
                    self.HFMDIBOEOKN = is.read_uint32()?;
                },
                26 => {
                    self.HCKOMMJEOEK.push(is.read_message()?);
                },
                32 => {
                    self.NLMOBJCCDEL = is.read_uint32()?;
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
        if self.NAGBMPFECGG != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.NAGBMPFECGG);
        }
        if self.HFMDIBOEOKN != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.HFMDIBOEOKN);
        }
        for value in &self.HCKOMMJEOEK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.NLMOBJCCDEL != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.NLMOBJCCDEL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.NAGBMPFECGG != 0 {
            os.write_uint32(1, self.NAGBMPFECGG)?;
        }
        if self.HFMDIBOEOKN != 0 {
            os.write_uint32(2, self.HFMDIBOEOKN)?;
        }
        for v in &self.HCKOMMJEOEK {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.NLMOBJCCDEL != 0 {
            os.write_uint32(4, self.NLMOBJCCDEL)?;
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

    fn new() -> EHOJLJMGOGL {
        EHOJLJMGOGL::new()
    }

    fn clear(&mut self) {
        self.NAGBMPFECGG = 0;
        self.HFMDIBOEOKN = 0;
        self.HCKOMMJEOEK.clear();
        self.NLMOBJCCDEL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EHOJLJMGOGL {
        static instance: EHOJLJMGOGL = EHOJLJMGOGL {
            NAGBMPFECGG: 0,
            HFMDIBOEOKN: 0,
            HCKOMMJEOEK: ::std::vec::Vec::new(),
            NLMOBJCCDEL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EHOJLJMGOGL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EHOJLJMGOGL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EHOJLJMGOGL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EHOJLJMGOGL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11EHOJLJMGOGL.proto\x1a\x11NEKKDMCDGPK.proto\"\xa3\x01\n\x0bEHOJLJMG\
    OGL\x12\x20\n\x0bNAGBMPFECGG\x18\x01\x20\x01(\rR\x0bNAGBMPFECGG\x12\x20\
    \n\x0bHFMDIBOEOKN\x18\x02\x20\x01(\rR\x0bHFMDIBOEOKN\x12.\n\x0bHCKOMMJEO\
    EK\x18\x03\x20\x03(\x0b2\x0c.NEKKDMCDGPKR\x0bHCKOMMJEOEK\x12\x20\n\x0bNL\
    MOBJCCDEL\x18\x04\x20\x01(\rR\x0bNLMOBJCCDELb\x06proto3\
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
            deps.push(super::NEKKDMCDGPK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EHOJLJMGOGL::generated_message_descriptor_data());
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