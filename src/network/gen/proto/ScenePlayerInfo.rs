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

//! Generated file from `ScenePlayerInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ScenePlayerInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ScenePlayerInfo {
    // message fields
    // @@protoc_insertion_point(field:ScenePlayerInfo.is_connected)
    pub is_connected: bool,
    // @@protoc_insertion_point(field:ScenePlayerInfo.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:ScenePlayerInfo.scene_id)
    pub scene_id: u32,
    // @@protoc_insertion_point(field:ScenePlayerInfo.uid)
    pub uid: u32,
    // @@protoc_insertion_point(field:ScenePlayerInfo.online_player_info)
    pub online_player_info: ::protobuf::MessageField<super::OnlinePlayerInfo::OnlinePlayerInfo>,
    // @@protoc_insertion_point(field:ScenePlayerInfo.peer_id)
    pub peer_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ScenePlayerInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ScenePlayerInfo {
    fn default() -> &'a ScenePlayerInfo {
        <ScenePlayerInfo as ::protobuf::Message>::default_instance()
    }
}

impl ScenePlayerInfo {
    pub fn new() -> ScenePlayerInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_connected",
            |m: &ScenePlayerInfo| { &m.is_connected },
            |m: &mut ScenePlayerInfo| { &mut m.is_connected },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &ScenePlayerInfo| { &m.name },
            |m: &mut ScenePlayerInfo| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scene_id",
            |m: &ScenePlayerInfo| { &m.scene_id },
            |m: &mut ScenePlayerInfo| { &mut m.scene_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "uid",
            |m: &ScenePlayerInfo| { &m.uid },
            |m: &mut ScenePlayerInfo| { &mut m.uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OnlinePlayerInfo::OnlinePlayerInfo>(
            "online_player_info",
            |m: &ScenePlayerInfo| { &m.online_player_info },
            |m: &mut ScenePlayerInfo| { &mut m.online_player_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "peer_id",
            |m: &ScenePlayerInfo| { &m.peer_id },
            |m: &mut ScenePlayerInfo| { &mut m.peer_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ScenePlayerInfo>(
            "ScenePlayerInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ScenePlayerInfo {
    const NAME: &'static str = "ScenePlayerInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.is_connected = is.read_bool()?;
                },
                90 => {
                    self.name = is.read_string()?;
                },
                120 => {
                    self.scene_id = is.read_uint32()?;
                },
                72 => {
                    self.uid = is.read_uint32()?;
                },
                50 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.online_player_info)?;
                },
                24 => {
                    self.peer_id = is.read_uint32()?;
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
        if self.is_connected != false {
            my_size += 1 + 1;
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.name);
        }
        if self.scene_id != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.scene_id);
        }
        if self.uid != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.uid);
        }
        if let Some(v) = self.online_player_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.peer_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.peer_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_connected != false {
            os.write_bool(7, self.is_connected)?;
        }
        if !self.name.is_empty() {
            os.write_string(11, &self.name)?;
        }
        if self.scene_id != 0 {
            os.write_uint32(15, self.scene_id)?;
        }
        if self.uid != 0 {
            os.write_uint32(9, self.uid)?;
        }
        if let Some(v) = self.online_player_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        }
        if self.peer_id != 0 {
            os.write_uint32(3, self.peer_id)?;
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

    fn new() -> ScenePlayerInfo {
        ScenePlayerInfo::new()
    }

    fn clear(&mut self) {
        self.is_connected = false;
        self.name.clear();
        self.scene_id = 0;
        self.uid = 0;
        self.online_player_info.clear();
        self.peer_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ScenePlayerInfo {
        static instance: ScenePlayerInfo = ScenePlayerInfo {
            is_connected: false,
            name: ::std::string::String::new(),
            scene_id: 0,
            uid: 0,
            online_player_info: ::protobuf::MessageField::none(),
            peer_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ScenePlayerInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ScenePlayerInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ScenePlayerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScenePlayerInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15ScenePlayerInfo.proto\x1a\x16OnlinePlayerInfo.proto\"\xcf\x01\n\
    \x0fScenePlayerInfo\x12!\n\x0cis_connected\x18\x07\x20\x01(\x08R\x0bisCo\
    nnected\x12\x12\n\x04name\x18\x0b\x20\x01(\tR\x04name\x12\x19\n\x08scene\
    _id\x18\x0f\x20\x01(\rR\x07sceneId\x12\x10\n\x03uid\x18\t\x20\x01(\rR\
    \x03uid\x12?\n\x12online_player_info\x18\x06\x20\x01(\x0b2\x11.OnlinePla\
    yerInfoR\x10onlinePlayerInfo\x12\x17\n\x07peer_id\x18\x03\x20\x01(\rR\
    \x06peerIdB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::OnlinePlayerInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ScenePlayerInfo::generated_message_descriptor_data());
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
