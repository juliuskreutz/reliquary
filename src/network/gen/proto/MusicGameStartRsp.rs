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

//! Generated file from `MusicGameStartRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MusicGameStartRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MusicGameStartRsp {
    // message fields
    // @@protoc_insertion_point(field:MusicGameStartRsp.music_basic_id)
    pub music_basic_id: u32,
    // @@protoc_insertion_point(field:MusicGameStartRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:MusicGameStartRsp.ugc_guid)
    pub ugc_guid: u64,
    // special fields
    // @@protoc_insertion_point(special_field:MusicGameStartRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MusicGameStartRsp {
    fn default() -> &'a MusicGameStartRsp {
        <MusicGameStartRsp as ::protobuf::Message>::default_instance()
    }
}

impl MusicGameStartRsp {
    pub fn new() -> MusicGameStartRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "music_basic_id",
            |m: &MusicGameStartRsp| { &m.music_basic_id },
            |m: &mut MusicGameStartRsp| { &mut m.music_basic_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &MusicGameStartRsp| { &m.retcode },
            |m: &mut MusicGameStartRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ugc_guid",
            |m: &MusicGameStartRsp| { &m.ugc_guid },
            |m: &mut MusicGameStartRsp| { &mut m.ugc_guid },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MusicGameStartRsp>(
            "MusicGameStartRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MusicGameStartRsp {
    const NAME: &'static str = "MusicGameStartRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.music_basic_id = is.read_uint32()?;
                },
                16 => {
                    self.retcode = is.read_int32()?;
                },
                8 => {
                    self.ugc_guid = is.read_uint64()?;
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
        if self.music_basic_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.music_basic_id);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(2, self.retcode);
        }
        if self.ugc_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.ugc_guid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.music_basic_id != 0 {
            os.write_uint32(5, self.music_basic_id)?;
        }
        if self.retcode != 0 {
            os.write_int32(2, self.retcode)?;
        }
        if self.ugc_guid != 0 {
            os.write_uint64(1, self.ugc_guid)?;
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

    fn new() -> MusicGameStartRsp {
        MusicGameStartRsp::new()
    }

    fn clear(&mut self) {
        self.music_basic_id = 0;
        self.retcode = 0;
        self.ugc_guid = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MusicGameStartRsp {
        static instance: MusicGameStartRsp = MusicGameStartRsp {
            music_basic_id: 0,
            retcode: 0,
            ugc_guid: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MusicGameStartRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MusicGameStartRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MusicGameStartRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MusicGameStartRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17MusicGameStartRsp.proto\"n\n\x11MusicGameStartRsp\x12$\n\x0emusic_\
    basic_id\x18\x05\x20\x01(\rR\x0cmusicBasicId\x12\x18\n\x07retcode\x18\
    \x02\x20\x01(\x05R\x07retcode\x12\x19\n\x08ugc_guid\x18\x01\x20\x01(\x04\
    R\x07ugcGuidB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(MusicGameStartRsp::generated_message_descriptor_data());
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