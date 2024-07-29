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

//! Generated file from `QuestDestroyEntityReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:QuestDestroyEntityReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct QuestDestroyEntityReq {
    // message fields
    // @@protoc_insertion_point(field:QuestDestroyEntityReq.quest_id)
    pub quest_id: u32,
    // @@protoc_insertion_point(field:QuestDestroyEntityReq.entity_id)
    pub entity_id: u32,
    // @@protoc_insertion_point(field:QuestDestroyEntityReq.scene_id)
    pub scene_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:QuestDestroyEntityReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a QuestDestroyEntityReq {
    fn default() -> &'a QuestDestroyEntityReq {
        <QuestDestroyEntityReq as ::protobuf::Message>::default_instance()
    }
}

impl QuestDestroyEntityReq {
    pub fn new() -> QuestDestroyEntityReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "quest_id",
            |m: &QuestDestroyEntityReq| { &m.quest_id },
            |m: &mut QuestDestroyEntityReq| { &mut m.quest_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entity_id",
            |m: &QuestDestroyEntityReq| { &m.entity_id },
            |m: &mut QuestDestroyEntityReq| { &mut m.entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scene_id",
            |m: &QuestDestroyEntityReq| { &m.scene_id },
            |m: &mut QuestDestroyEntityReq| { &mut m.scene_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<QuestDestroyEntityReq>(
            "QuestDestroyEntityReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for QuestDestroyEntityReq {
    const NAME: &'static str = "QuestDestroyEntityReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.quest_id = is.read_uint32()?;
                },
                32 => {
                    self.entity_id = is.read_uint32()?;
                },
                40 => {
                    self.scene_id = is.read_uint32()?;
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
        if self.quest_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.quest_id);
        }
        if self.entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.entity_id);
        }
        if self.scene_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.scene_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.quest_id != 0 {
            os.write_uint32(3, self.quest_id)?;
        }
        if self.entity_id != 0 {
            os.write_uint32(4, self.entity_id)?;
        }
        if self.scene_id != 0 {
            os.write_uint32(5, self.scene_id)?;
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

    fn new() -> QuestDestroyEntityReq {
        QuestDestroyEntityReq::new()
    }

    fn clear(&mut self) {
        self.quest_id = 0;
        self.entity_id = 0;
        self.scene_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static QuestDestroyEntityReq {
        static instance: QuestDestroyEntityReq = QuestDestroyEntityReq {
            quest_id: 0,
            entity_id: 0,
            scene_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for QuestDestroyEntityReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("QuestDestroyEntityReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for QuestDestroyEntityReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QuestDestroyEntityReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bQuestDestroyEntityReq.proto\"j\n\x15QuestDestroyEntityReq\x12\x19\
    \n\x08quest_id\x18\x03\x20\x01(\rR\x07questId\x12\x1b\n\tentity_id\x18\
    \x04\x20\x01(\rR\x08entityId\x12\x19\n\x08scene_id\x18\x05\x20\x01(\rR\
    \x07sceneIdB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(QuestDestroyEntityReq::generated_message_descriptor_data());
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