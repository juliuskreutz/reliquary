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

//! Generated file from `MOCBKBGIFPJ.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MOCBKBGIFPJ)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MOCBKBGIFPJ {
    // message fields
    // @@protoc_insertion_point(field:MOCBKBGIFPJ.level_id)
    pub level_id: u32,
    // @@protoc_insertion_point(field:MOCBKBGIFPJ.is_open)
    pub is_open: bool,
    // @@protoc_insertion_point(field:MOCBKBGIFPJ.score)
    pub score: u32,
    // @@protoc_insertion_point(field:MOCBKBGIFPJ.is_finish)
    pub is_finish: bool,
    // special fields
    // @@protoc_insertion_point(special_field:MOCBKBGIFPJ.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MOCBKBGIFPJ {
    fn default() -> &'a MOCBKBGIFPJ {
        <MOCBKBGIFPJ as ::protobuf::Message>::default_instance()
    }
}

impl MOCBKBGIFPJ {
    pub fn new() -> MOCBKBGIFPJ {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level_id",
            |m: &MOCBKBGIFPJ| { &m.level_id },
            |m: &mut MOCBKBGIFPJ| { &mut m.level_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_open",
            |m: &MOCBKBGIFPJ| { &m.is_open },
            |m: &mut MOCBKBGIFPJ| { &mut m.is_open },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "score",
            |m: &MOCBKBGIFPJ| { &m.score },
            |m: &mut MOCBKBGIFPJ| { &mut m.score },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_finish",
            |m: &MOCBKBGIFPJ| { &m.is_finish },
            |m: &mut MOCBKBGIFPJ| { &mut m.is_finish },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MOCBKBGIFPJ>(
            "MOCBKBGIFPJ",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MOCBKBGIFPJ {
    const NAME: &'static str = "MOCBKBGIFPJ";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.level_id = is.read_uint32()?;
                },
                72 => {
                    self.is_open = is.read_bool()?;
                },
                104 => {
                    self.score = is.read_uint32()?;
                },
                16 => {
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
        if self.level_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.level_id);
        }
        if self.is_open != false {
            my_size += 1 + 1;
        }
        if self.score != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.score);
        }
        if self.is_finish != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level_id != 0 {
            os.write_uint32(4, self.level_id)?;
        }
        if self.is_open != false {
            os.write_bool(9, self.is_open)?;
        }
        if self.score != 0 {
            os.write_uint32(13, self.score)?;
        }
        if self.is_finish != false {
            os.write_bool(2, self.is_finish)?;
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

    fn new() -> MOCBKBGIFPJ {
        MOCBKBGIFPJ::new()
    }

    fn clear(&mut self) {
        self.level_id = 0;
        self.is_open = false;
        self.score = 0;
        self.is_finish = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MOCBKBGIFPJ {
        static instance: MOCBKBGIFPJ = MOCBKBGIFPJ {
            level_id: 0,
            is_open: false,
            score: 0,
            is_finish: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MOCBKBGIFPJ {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MOCBKBGIFPJ").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MOCBKBGIFPJ {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MOCBKBGIFPJ {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MOCBKBGIFPJ.proto\"t\n\x0bMOCBKBGIFPJ\x12\x19\n\x08level_id\x18\
    \x04\x20\x01(\rR\x07levelId\x12\x17\n\x07is_open\x18\t\x20\x01(\x08R\x06\
    isOpen\x12\x14\n\x05score\x18\r\x20\x01(\rR\x05score\x12\x1b\n\tis_finis\
    h\x18\x02\x20\x01(\x08R\x08isFinishB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MOCBKBGIFPJ::generated_message_descriptor_data());
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
