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

//! Generated file from `InferenceWordInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:InferenceWordInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct InferenceWordInfo {
    // message fields
    // @@protoc_insertion_point(field:InferenceWordInfo.word_id)
    pub word_id: u32,
    // @@protoc_insertion_point(field:InferenceWordInfo.AAOCFCGKKHC)
    pub AAOCFCGKKHC: bool,
    // @@protoc_insertion_point(field:InferenceWordInfo.DMBPILCCFNK)
    pub DMBPILCCFNK: bool,
    // @@protoc_insertion_point(field:InferenceWordInfo.unlock_by_word_id)
    pub unlock_by_word_id: u32,
    // @@protoc_insertion_point(field:InferenceWordInfo.HJLIADLKJOP)
    pub HJLIADLKJOP: bool,
    // special fields
    // @@protoc_insertion_point(special_field:InferenceWordInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a InferenceWordInfo {
    fn default() -> &'a InferenceWordInfo {
        <InferenceWordInfo as ::protobuf::Message>::default_instance()
    }
}

impl InferenceWordInfo {
    pub fn new() -> InferenceWordInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "word_id",
            |m: &InferenceWordInfo| { &m.word_id },
            |m: &mut InferenceWordInfo| { &mut m.word_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AAOCFCGKKHC",
            |m: &InferenceWordInfo| { &m.AAOCFCGKKHC },
            |m: &mut InferenceWordInfo| { &mut m.AAOCFCGKKHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DMBPILCCFNK",
            |m: &InferenceWordInfo| { &m.DMBPILCCFNK },
            |m: &mut InferenceWordInfo| { &mut m.DMBPILCCFNK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unlock_by_word_id",
            |m: &InferenceWordInfo| { &m.unlock_by_word_id },
            |m: &mut InferenceWordInfo| { &mut m.unlock_by_word_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HJLIADLKJOP",
            |m: &InferenceWordInfo| { &m.HJLIADLKJOP },
            |m: &mut InferenceWordInfo| { &mut m.HJLIADLKJOP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<InferenceWordInfo>(
            "InferenceWordInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for InferenceWordInfo {
    const NAME: &'static str = "InferenceWordInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.word_id = is.read_uint32()?;
                },
                56 => {
                    self.AAOCFCGKKHC = is.read_bool()?;
                },
                80 => {
                    self.DMBPILCCFNK = is.read_bool()?;
                },
                104 => {
                    self.unlock_by_word_id = is.read_uint32()?;
                },
                112 => {
                    self.HJLIADLKJOP = is.read_bool()?;
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
        if self.word_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.word_id);
        }
        if self.AAOCFCGKKHC != false {
            my_size += 1 + 1;
        }
        if self.DMBPILCCFNK != false {
            my_size += 1 + 1;
        }
        if self.unlock_by_word_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.unlock_by_word_id);
        }
        if self.HJLIADLKJOP != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.word_id != 0 {
            os.write_uint32(3, self.word_id)?;
        }
        if self.AAOCFCGKKHC != false {
            os.write_bool(7, self.AAOCFCGKKHC)?;
        }
        if self.DMBPILCCFNK != false {
            os.write_bool(10, self.DMBPILCCFNK)?;
        }
        if self.unlock_by_word_id != 0 {
            os.write_uint32(13, self.unlock_by_word_id)?;
        }
        if self.HJLIADLKJOP != false {
            os.write_bool(14, self.HJLIADLKJOP)?;
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

    fn new() -> InferenceWordInfo {
        InferenceWordInfo::new()
    }

    fn clear(&mut self) {
        self.word_id = 0;
        self.AAOCFCGKKHC = false;
        self.DMBPILCCFNK = false;
        self.unlock_by_word_id = 0;
        self.HJLIADLKJOP = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static InferenceWordInfo {
        static instance: InferenceWordInfo = InferenceWordInfo {
            word_id: 0,
            AAOCFCGKKHC: false,
            DMBPILCCFNK: false,
            unlock_by_word_id: 0,
            HJLIADLKJOP: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for InferenceWordInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("InferenceWordInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for InferenceWordInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InferenceWordInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17InferenceWordInfo.proto\"\xbd\x01\n\x11InferenceWordInfo\x12\x17\n\
    \x07word_id\x18\x03\x20\x01(\rR\x06wordId\x12\x20\n\x0bAAOCFCGKKHC\x18\
    \x07\x20\x01(\x08R\x0bAAOCFCGKKHC\x12\x20\n\x0bDMBPILCCFNK\x18\n\x20\x01\
    (\x08R\x0bDMBPILCCFNK\x12)\n\x11unlock_by_word_id\x18\r\x20\x01(\rR\x0eu\
    nlockByWordId\x12\x20\n\x0bHJLIADLKJOP\x18\x0e\x20\x01(\x08R\x0bHJLIADLK\
    JOPB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(InferenceWordInfo::generated_message_descriptor_data());
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
