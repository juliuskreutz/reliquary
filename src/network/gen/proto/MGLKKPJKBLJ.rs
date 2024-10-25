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

//! Generated file from `MGLKKPJKBLJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MGLKKPJKBLJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MGLKKPJKBLJ {
    // message fields
    // @@protoc_insertion_point(field:MGLKKPJKBLJ.GGEPGHPENFG)
    pub GGEPGHPENFG: u32,
    // @@protoc_insertion_point(field:MGLKKPJKBLJ.FALLAINIGJD)
    pub FALLAINIGJD: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MGLKKPJKBLJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MGLKKPJKBLJ {
    fn default() -> &'a MGLKKPJKBLJ {
        <MGLKKPJKBLJ as ::protobuf::Message>::default_instance()
    }
}

impl MGLKKPJKBLJ {
    pub fn new() -> MGLKKPJKBLJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GGEPGHPENFG",
            |m: &MGLKKPJKBLJ| { &m.GGEPGHPENFG },
            |m: &mut MGLKKPJKBLJ| { &mut m.GGEPGHPENFG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FALLAINIGJD",
            |m: &MGLKKPJKBLJ| { &m.FALLAINIGJD },
            |m: &mut MGLKKPJKBLJ| { &mut m.FALLAINIGJD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MGLKKPJKBLJ>(
            "MGLKKPJKBLJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MGLKKPJKBLJ {
    const NAME: &'static str = "MGLKKPJKBLJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.GGEPGHPENFG = is.read_uint32()?;
                },
                8 => {
                    self.FALLAINIGJD = is.read_uint32()?;
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
        if self.GGEPGHPENFG != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.GGEPGHPENFG);
        }
        if self.FALLAINIGJD != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.FALLAINIGJD);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GGEPGHPENFG != 0 {
            os.write_uint32(6, self.GGEPGHPENFG)?;
        }
        if self.FALLAINIGJD != 0 {
            os.write_uint32(1, self.FALLAINIGJD)?;
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

    fn new() -> MGLKKPJKBLJ {
        MGLKKPJKBLJ::new()
    }

    fn clear(&mut self) {
        self.GGEPGHPENFG = 0;
        self.FALLAINIGJD = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MGLKKPJKBLJ {
        static instance: MGLKKPJKBLJ = MGLKKPJKBLJ {
            GGEPGHPENFG: 0,
            FALLAINIGJD: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MGLKKPJKBLJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MGLKKPJKBLJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MGLKKPJKBLJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MGLKKPJKBLJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MGLKKPJKBLJ.proto\"Q\n\x0bMGLKKPJKBLJ\x12\x20\n\x0bGGEPGHPENFG\x18\
    \x06\x20\x01(\rR\x0bGGEPGHPENFG\x12\x20\n\x0bFALLAINIGJD\x18\x01\x20\x01\
    (\rR\x0bFALLAINIGJDb\x06proto3\
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
            messages.push(MGLKKPJKBLJ::generated_message_descriptor_data());
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