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

//! Generated file from `QuestRenameAvatarRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:QuestRenameAvatarRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct QuestRenameAvatarRsp {
    // message fields
    // @@protoc_insertion_point(field:QuestRenameAvatarRsp.rename_id)
    pub rename_id: u32,
    // @@protoc_insertion_point(field:QuestRenameAvatarRsp.avatar_name)
    pub avatar_name: ::std::string::String,
    // @@protoc_insertion_point(field:QuestRenameAvatarRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:QuestRenameAvatarRsp.is_check)
    pub is_check: bool,
    // special fields
    // @@protoc_insertion_point(special_field:QuestRenameAvatarRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a QuestRenameAvatarRsp {
    fn default() -> &'a QuestRenameAvatarRsp {
        <QuestRenameAvatarRsp as ::protobuf::Message>::default_instance()
    }
}

impl QuestRenameAvatarRsp {
    pub fn new() -> QuestRenameAvatarRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rename_id",
            |m: &QuestRenameAvatarRsp| { &m.rename_id },
            |m: &mut QuestRenameAvatarRsp| { &mut m.rename_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_name",
            |m: &QuestRenameAvatarRsp| { &m.avatar_name },
            |m: &mut QuestRenameAvatarRsp| { &mut m.avatar_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &QuestRenameAvatarRsp| { &m.retcode },
            |m: &mut QuestRenameAvatarRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_check",
            |m: &QuestRenameAvatarRsp| { &m.is_check },
            |m: &mut QuestRenameAvatarRsp| { &mut m.is_check },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<QuestRenameAvatarRsp>(
            "QuestRenameAvatarRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for QuestRenameAvatarRsp {
    const NAME: &'static str = "QuestRenameAvatarRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.rename_id = is.read_uint32()?;
                },
                90 => {
                    self.avatar_name = is.read_string()?;
                },
                104 => {
                    self.retcode = is.read_int32()?;
                },
                112 => {
                    self.is_check = is.read_bool()?;
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
        if self.rename_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.rename_id);
        }
        if !self.avatar_name.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.avatar_name);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(13, self.retcode);
        }
        if self.is_check != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.rename_id != 0 {
            os.write_uint32(9, self.rename_id)?;
        }
        if !self.avatar_name.is_empty() {
            os.write_string(11, &self.avatar_name)?;
        }
        if self.retcode != 0 {
            os.write_int32(13, self.retcode)?;
        }
        if self.is_check != false {
            os.write_bool(14, self.is_check)?;
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

    fn new() -> QuestRenameAvatarRsp {
        QuestRenameAvatarRsp::new()
    }

    fn clear(&mut self) {
        self.rename_id = 0;
        self.avatar_name.clear();
        self.retcode = 0;
        self.is_check = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static QuestRenameAvatarRsp {
        static instance: QuestRenameAvatarRsp = QuestRenameAvatarRsp {
            rename_id: 0,
            avatar_name: ::std::string::String::new(),
            retcode: 0,
            is_check: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for QuestRenameAvatarRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("QuestRenameAvatarRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for QuestRenameAvatarRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for QuestRenameAvatarRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aQuestRenameAvatarRsp.proto\"\x89\x01\n\x14QuestRenameAvatarRsp\x12\
    \x1b\n\trename_id\x18\t\x20\x01(\rR\x08renameId\x12\x1f\n\x0bavatar_name\
    \x18\x0b\x20\x01(\tR\navatarName\x12\x18\n\x07retcode\x18\r\x20\x01(\x05\
    R\x07retcode\x12\x19\n\x08is_check\x18\x0e\x20\x01(\x08R\x07isCheckB\x1b\
    \n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(QuestRenameAvatarRsp::generated_message_descriptor_data());
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
