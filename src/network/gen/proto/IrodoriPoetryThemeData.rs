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

//! Generated file from `IrodoriPoetryThemeData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:IrodoriPoetryThemeData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct IrodoriPoetryThemeData {
    // message fields
    // @@protoc_insertion_point(field:IrodoriPoetryThemeData.CCPLLHMEOKJ)
    pub CCPLLHMEOKJ: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IrodoriPoetryThemeData.BAEKFBCJFJK)
    pub BAEKFBCJFJK: u32,
    // @@protoc_insertion_point(field:IrodoriPoetryThemeData.BLMAPGDEBCJ)
    pub BLMAPGDEBCJ: u32,
    // @@protoc_insertion_point(field:IrodoriPoetryThemeData.AJONPJMNANN)
    pub AJONPJMNANN: u32,
    // @@protoc_insertion_point(field:IrodoriPoetryThemeData.LNHHEKIDIAG)
    pub LNHHEKIDIAG: u32,
    // @@protoc_insertion_point(field:IrodoriPoetryThemeData.BNGOJINJALA)
    pub BNGOJINJALA: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:IrodoriPoetryThemeData.progress)
    pub progress: u32,
    // special fields
    // @@protoc_insertion_point(special_field:IrodoriPoetryThemeData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IrodoriPoetryThemeData {
    fn default() -> &'a IrodoriPoetryThemeData {
        <IrodoriPoetryThemeData as ::protobuf::Message>::default_instance()
    }
}

impl IrodoriPoetryThemeData {
    pub fn new() -> IrodoriPoetryThemeData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CCPLLHMEOKJ",
            |m: &IrodoriPoetryThemeData| { &m.CCPLLHMEOKJ },
            |m: &mut IrodoriPoetryThemeData| { &mut m.CCPLLHMEOKJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BAEKFBCJFJK",
            |m: &IrodoriPoetryThemeData| { &m.BAEKFBCJFJK },
            |m: &mut IrodoriPoetryThemeData| { &mut m.BAEKFBCJFJK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BLMAPGDEBCJ",
            |m: &IrodoriPoetryThemeData| { &m.BLMAPGDEBCJ },
            |m: &mut IrodoriPoetryThemeData| { &mut m.BLMAPGDEBCJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AJONPJMNANN",
            |m: &IrodoriPoetryThemeData| { &m.AJONPJMNANN },
            |m: &mut IrodoriPoetryThemeData| { &mut m.AJONPJMNANN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LNHHEKIDIAG",
            |m: &IrodoriPoetryThemeData| { &m.LNHHEKIDIAG },
            |m: &mut IrodoriPoetryThemeData| { &mut m.LNHHEKIDIAG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BNGOJINJALA",
            |m: &IrodoriPoetryThemeData| { &m.BNGOJINJALA },
            |m: &mut IrodoriPoetryThemeData| { &mut m.BNGOJINJALA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "progress",
            |m: &IrodoriPoetryThemeData| { &m.progress },
            |m: &mut IrodoriPoetryThemeData| { &mut m.progress },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IrodoriPoetryThemeData>(
            "IrodoriPoetryThemeData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IrodoriPoetryThemeData {
    const NAME: &'static str = "IrodoriPoetryThemeData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.CCPLLHMEOKJ)?;
                },
                88 => {
                    self.CCPLLHMEOKJ.push(is.read_uint32()?);
                },
                24 => {
                    self.BAEKFBCJFJK = is.read_uint32()?;
                },
                96 => {
                    self.BLMAPGDEBCJ = is.read_uint32()?;
                },
                80 => {
                    self.AJONPJMNANN = is.read_uint32()?;
                },
                48 => {
                    self.LNHHEKIDIAG = is.read_uint32()?;
                },
                114 => {
                    is.read_repeated_packed_uint32_into(&mut self.BNGOJINJALA)?;
                },
                112 => {
                    self.BNGOJINJALA.push(is.read_uint32()?);
                },
                40 => {
                    self.progress = is.read_uint32()?;
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
        for value in &self.CCPLLHMEOKJ {
            my_size += ::protobuf::rt::uint32_size(11, *value);
        };
        if self.BAEKFBCJFJK != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.BAEKFBCJFJK);
        }
        if self.BLMAPGDEBCJ != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.BLMAPGDEBCJ);
        }
        if self.AJONPJMNANN != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.AJONPJMNANN);
        }
        if self.LNHHEKIDIAG != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.LNHHEKIDIAG);
        }
        for value in &self.BNGOJINJALA {
            my_size += ::protobuf::rt::uint32_size(14, *value);
        };
        if self.progress != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.progress);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.CCPLLHMEOKJ {
            os.write_uint32(11, *v)?;
        };
        if self.BAEKFBCJFJK != 0 {
            os.write_uint32(3, self.BAEKFBCJFJK)?;
        }
        if self.BLMAPGDEBCJ != 0 {
            os.write_uint32(12, self.BLMAPGDEBCJ)?;
        }
        if self.AJONPJMNANN != 0 {
            os.write_uint32(10, self.AJONPJMNANN)?;
        }
        if self.LNHHEKIDIAG != 0 {
            os.write_uint32(6, self.LNHHEKIDIAG)?;
        }
        for v in &self.BNGOJINJALA {
            os.write_uint32(14, *v)?;
        };
        if self.progress != 0 {
            os.write_uint32(5, self.progress)?;
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

    fn new() -> IrodoriPoetryThemeData {
        IrodoriPoetryThemeData::new()
    }

    fn clear(&mut self) {
        self.CCPLLHMEOKJ.clear();
        self.BAEKFBCJFJK = 0;
        self.BLMAPGDEBCJ = 0;
        self.AJONPJMNANN = 0;
        self.LNHHEKIDIAG = 0;
        self.BNGOJINJALA.clear();
        self.progress = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IrodoriPoetryThemeData {
        static instance: IrodoriPoetryThemeData = IrodoriPoetryThemeData {
            CCPLLHMEOKJ: ::std::vec::Vec::new(),
            BAEKFBCJFJK: 0,
            BLMAPGDEBCJ: 0,
            AJONPJMNANN: 0,
            LNHHEKIDIAG: 0,
            BNGOJINJALA: ::std::vec::Vec::new(),
            progress: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IrodoriPoetryThemeData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IrodoriPoetryThemeData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IrodoriPoetryThemeData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IrodoriPoetryThemeData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cIrodoriPoetryThemeData.proto\"\x80\x02\n\x16IrodoriPoetryThemeData\
    \x12\x20\n\x0bCCPLLHMEOKJ\x18\x0b\x20\x03(\rR\x0bCCPLLHMEOKJ\x12\x20\n\
    \x0bBAEKFBCJFJK\x18\x03\x20\x01(\rR\x0bBAEKFBCJFJK\x12\x20\n\x0bBLMAPGDE\
    BCJ\x18\x0c\x20\x01(\rR\x0bBLMAPGDEBCJ\x12\x20\n\x0bAJONPJMNANN\x18\n\
    \x20\x01(\rR\x0bAJONPJMNANN\x12\x20\n\x0bLNHHEKIDIAG\x18\x06\x20\x01(\rR\
    \x0bLNHHEKIDIAG\x12\x20\n\x0bBNGOJINJALA\x18\x0e\x20\x03(\rR\x0bBNGOJINJ\
    ALA\x12\x1a\n\x08progress\x18\x05\x20\x01(\rR\x08progressB\x1b\n\x19emu.\
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
            messages.push(IrodoriPoetryThemeData::generated_message_descriptor_data());
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