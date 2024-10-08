// This file is generated by rust-protobuf 3.6.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `ChessRogueStartScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:ChessRogueStartScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ChessRogueStartScRsp {
    // message fields
    // @@protoc_insertion_point(field:ChessRogueStartScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:ChessRogueStartScRsp.current_info)
    pub current_info: ::protobuf::MessageField<super::ChessRogueCurrentInfo::ChessRogueCurrentInfo>,
    // @@protoc_insertion_point(field:ChessRogueStartScRsp.lineup)
    pub lineup: ::protobuf::MessageField<super::LineupInfo::LineupInfo>,
    // @@protoc_insertion_point(field:ChessRogueStartScRsp.cell_info)
    pub cell_info: ::protobuf::MessageField<super::ChessRogueCellInfo::ChessRogueCellInfo>,
    // @@protoc_insertion_point(field:ChessRogueStartScRsp.scene)
    pub scene: ::protobuf::MessageField<super::SceneInfo::SceneInfo>,
    // @@protoc_insertion_point(field:ChessRogueStartScRsp.chess_rogue_info)
    pub chess_rogue_info: ::protobuf::MessageField<super::ChessRogueInfo::ChessRogueInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:ChessRogueStartScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ChessRogueStartScRsp {
    fn default() -> &'a ChessRogueStartScRsp {
        <ChessRogueStartScRsp as ::protobuf::Message>::default_instance()
    }
}

impl ChessRogueStartScRsp {
    pub fn new() -> ChessRogueStartScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &ChessRogueStartScRsp| { &m.retcode },
            |m: &mut ChessRogueStartScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueCurrentInfo::ChessRogueCurrentInfo>(
            "current_info",
            |m: &ChessRogueStartScRsp| { &m.current_info },
            |m: &mut ChessRogueStartScRsp| { &mut m.current_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LineupInfo::LineupInfo>(
            "lineup",
            |m: &ChessRogueStartScRsp| { &m.lineup },
            |m: &mut ChessRogueStartScRsp| { &mut m.lineup },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueCellInfo::ChessRogueCellInfo>(
            "cell_info",
            |m: &ChessRogueStartScRsp| { &m.cell_info },
            |m: &mut ChessRogueStartScRsp| { &mut m.cell_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::SceneInfo::SceneInfo>(
            "scene",
            |m: &ChessRogueStartScRsp| { &m.scene },
            |m: &mut ChessRogueStartScRsp| { &mut m.scene },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ChessRogueInfo::ChessRogueInfo>(
            "chess_rogue_info",
            |m: &ChessRogueStartScRsp| { &m.chess_rogue_info },
            |m: &mut ChessRogueStartScRsp| { &mut m.chess_rogue_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ChessRogueStartScRsp>(
            "ChessRogueStartScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ChessRogueStartScRsp {
    const NAME: &'static str = "ChessRogueStartScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.retcode = is.read_uint32()?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.current_info)?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.lineup)?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.cell_info)?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.scene)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.chess_rogue_info)?;
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.retcode);
        }
        if let Some(v) = self.current_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.lineup.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.cell_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.scene.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.chess_rogue_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_uint32(2, self.retcode)?;
        }
        if let Some(v) = self.current_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.lineup.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if let Some(v) = self.cell_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.scene.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.chess_rogue_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
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

    fn new() -> ChessRogueStartScRsp {
        ChessRogueStartScRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.current_info.clear();
        self.lineup.clear();
        self.cell_info.clear();
        self.scene.clear();
        self.chess_rogue_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ChessRogueStartScRsp {
        static instance: ChessRogueStartScRsp = ChessRogueStartScRsp {
            retcode: 0,
            current_info: ::protobuf::MessageField::none(),
            lineup: ::protobuf::MessageField::none(),
            cell_info: ::protobuf::MessageField::none(),
            scene: ::protobuf::MessageField::none(),
            chess_rogue_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ChessRogueStartScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ChessRogueStartScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ChessRogueStartScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChessRogueStartScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aChessRogueStartScRsp.proto\x1a\x1bChessRogueCurrentInfo.proto\x1a\
    \x10LineupInfo.proto\x1a\x18ChessRogueCellInfo.proto\x1a\x0fSceneInfo.pr\
    oto\x1a\x14ChessRogueInfo.proto\"\x9f\x02\n\x14ChessRogueStartScRsp\x12\
    \x18\n\x07retcode\x18\x02\x20\x01(\rR\x07retcode\x129\n\x0ccurrent_info\
    \x18\x01\x20\x01(\x0b2\x16.ChessRogueCurrentInfoR\x0bcurrentInfo\x12#\n\
    \x06lineup\x18\x08\x20\x01(\x0b2\x0b.LineupInfoR\x06lineup\x120\n\tcell_\
    info\x18\r\x20\x01(\x0b2\x13.ChessRogueCellInfoR\x08cellInfo\x12\x20\n\
    \x05scene\x18\x0f\x20\x01(\x0b2\n.SceneInfoR\x05scene\x129\n\x10chess_ro\
    gue_info\x18\x0e\x20\x01(\x0b2\x0f.ChessRogueInfoR\x0echessRogueInfoB\
    \x15\n\x13emu.lunarcore.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(5);
            deps.push(super::ChessRogueCurrentInfo::file_descriptor().clone());
            deps.push(super::LineupInfo::file_descriptor().clone());
            deps.push(super::ChessRogueCellInfo::file_descriptor().clone());
            deps.push(super::SceneInfo::file_descriptor().clone());
            deps.push(super::ChessRogueInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ChessRogueStartScRsp::generated_message_descriptor_data());
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
