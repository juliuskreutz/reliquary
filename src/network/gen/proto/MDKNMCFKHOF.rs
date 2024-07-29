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

//! Generated file from `MDKNMCFKHOF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MDKNMCFKHOF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MDKNMCFKHOF {
    // message fields
    // @@protoc_insertion_point(field:MDKNMCFKHOF.CLFLBPJPBJB)
    pub CLFLBPJPBJB: u32,
    // @@protoc_insertion_point(field:MDKNMCFKHOF.affix_list)
    pub affix_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:MDKNMCFKHOF.max_score)
    pub max_score: u32,
    // @@protoc_insertion_point(field:MDKNMCFKHOF.level_id)
    pub level_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MDKNMCFKHOF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MDKNMCFKHOF {
    fn default() -> &'a MDKNMCFKHOF {
        <MDKNMCFKHOF as ::protobuf::Message>::default_instance()
    }
}

impl MDKNMCFKHOF {
    pub fn new() -> MDKNMCFKHOF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CLFLBPJPBJB",
            |m: &MDKNMCFKHOF| { &m.CLFLBPJPBJB },
            |m: &mut MDKNMCFKHOF| { &mut m.CLFLBPJPBJB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "affix_list",
            |m: &MDKNMCFKHOF| { &m.affix_list },
            |m: &mut MDKNMCFKHOF| { &mut m.affix_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_score",
            |m: &MDKNMCFKHOF| { &m.max_score },
            |m: &mut MDKNMCFKHOF| { &mut m.max_score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level_id",
            |m: &MDKNMCFKHOF| { &m.level_id },
            |m: &mut MDKNMCFKHOF| { &mut m.level_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MDKNMCFKHOF>(
            "MDKNMCFKHOF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MDKNMCFKHOF {
    const NAME: &'static str = "MDKNMCFKHOF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.CLFLBPJPBJB = is.read_uint32()?;
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.affix_list)?;
                },
                104 => {
                    self.affix_list.push(is.read_uint32()?);
                },
                16 => {
                    self.max_score = is.read_uint32()?;
                },
                112 => {
                    self.level_id = is.read_uint32()?;
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
        if self.CLFLBPJPBJB != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.CLFLBPJPBJB);
        }
        for value in &self.affix_list {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        if self.max_score != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.max_score);
        }
        if self.level_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.level_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.CLFLBPJPBJB != 0 {
            os.write_uint32(15, self.CLFLBPJPBJB)?;
        }
        for v in &self.affix_list {
            os.write_uint32(13, *v)?;
        };
        if self.max_score != 0 {
            os.write_uint32(2, self.max_score)?;
        }
        if self.level_id != 0 {
            os.write_uint32(14, self.level_id)?;
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

    fn new() -> MDKNMCFKHOF {
        MDKNMCFKHOF::new()
    }

    fn clear(&mut self) {
        self.CLFLBPJPBJB = 0;
        self.affix_list.clear();
        self.max_score = 0;
        self.level_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MDKNMCFKHOF {
        static instance: MDKNMCFKHOF = MDKNMCFKHOF {
            CLFLBPJPBJB: 0,
            affix_list: ::std::vec::Vec::new(),
            max_score: 0,
            level_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MDKNMCFKHOF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MDKNMCFKHOF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MDKNMCFKHOF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MDKNMCFKHOF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MDKNMCFKHOF.proto\"\x86\x01\n\x0bMDKNMCFKHOF\x12\x20\n\x0bCLFLBPJP\
    BJB\x18\x0f\x20\x01(\rR\x0bCLFLBPJPBJB\x12\x1d\n\naffix_list\x18\r\x20\
    \x03(\rR\taffixList\x12\x1b\n\tmax_score\x18\x02\x20\x01(\rR\x08maxScore\
    \x12\x19\n\x08level_id\x18\x0e\x20\x01(\rR\x07levelIdB\x1b\n\x19emu.gras\
    scutter.net.protob\x06proto3\
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
            messages.push(MDKNMCFKHOF::generated_message_descriptor_data());
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
