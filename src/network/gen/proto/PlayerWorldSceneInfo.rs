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

//! Generated file from `PlayerWorldSceneInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlayerWorldSceneInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerWorldSceneInfo {
    // message fields
    // @@protoc_insertion_point(field:PlayerWorldSceneInfo.map_layer_info)
    pub map_layer_info: ::protobuf::MessageField<super::MapLayerInfo::MapLayerInfo>,
    // @@protoc_insertion_point(field:PlayerWorldSceneInfo.scene_id)
    pub scene_id: u32,
    // @@protoc_insertion_point(field:PlayerWorldSceneInfo.is_locked)
    pub is_locked: bool,
    // @@protoc_insertion_point(field:PlayerWorldSceneInfo.scene_tag_id_list)
    pub scene_tag_id_list: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerWorldSceneInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerWorldSceneInfo {
    fn default() -> &'a PlayerWorldSceneInfo {
        <PlayerWorldSceneInfo as ::protobuf::Message>::default_instance()
    }
}

impl PlayerWorldSceneInfo {
    pub fn new() -> PlayerWorldSceneInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MapLayerInfo::MapLayerInfo>(
            "map_layer_info",
            |m: &PlayerWorldSceneInfo| { &m.map_layer_info },
            |m: &mut PlayerWorldSceneInfo| { &mut m.map_layer_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scene_id",
            |m: &PlayerWorldSceneInfo| { &m.scene_id },
            |m: &mut PlayerWorldSceneInfo| { &mut m.scene_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_locked",
            |m: &PlayerWorldSceneInfo| { &m.is_locked },
            |m: &mut PlayerWorldSceneInfo| { &mut m.is_locked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "scene_tag_id_list",
            |m: &PlayerWorldSceneInfo| { &m.scene_tag_id_list },
            |m: &mut PlayerWorldSceneInfo| { &mut m.scene_tag_id_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerWorldSceneInfo>(
            "PlayerWorldSceneInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerWorldSceneInfo {
    const NAME: &'static str = "PlayerWorldSceneInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.map_layer_info)?;
                },
                96 => {
                    self.scene_id = is.read_uint32()?;
                },
                24 => {
                    self.is_locked = is.read_bool()?;
                },
                50 => {
                    is.read_repeated_packed_uint32_into(&mut self.scene_tag_id_list)?;
                },
                48 => {
                    self.scene_tag_id_list.push(is.read_uint32()?);
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
        if let Some(v) = self.map_layer_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.scene_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.scene_id);
        }
        if self.is_locked != false {
            my_size += 1 + 1;
        }
        for value in &self.scene_tag_id_list {
            my_size += ::protobuf::rt::uint32_size(6, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.map_layer_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if self.scene_id != 0 {
            os.write_uint32(12, self.scene_id)?;
        }
        if self.is_locked != false {
            os.write_bool(3, self.is_locked)?;
        }
        for v in &self.scene_tag_id_list {
            os.write_uint32(6, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PlayerWorldSceneInfo {
        PlayerWorldSceneInfo::new()
    }

    fn clear(&mut self) {
        self.map_layer_info.clear();
        self.scene_id = 0;
        self.is_locked = false;
        self.scene_tag_id_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerWorldSceneInfo {
        static instance: PlayerWorldSceneInfo = PlayerWorldSceneInfo {
            map_layer_info: ::protobuf::MessageField::none(),
            scene_id: 0,
            is_locked: false,
            scene_tag_id_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerWorldSceneInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerWorldSceneInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerWorldSceneInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerWorldSceneInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1aPlayerWorldSceneInfo.proto\x1a\x12MapLayerInfo.proto\"\xae\x01\n\
    \x14PlayerWorldSceneInfo\x123\n\x0emap_layer_info\x18\n\x20\x01(\x0b2\r.\
    MapLayerInfoR\x0cmapLayerInfo\x12\x19\n\x08scene_id\x18\x0c\x20\x01(\rR\
    \x07sceneId\x12\x1b\n\tis_locked\x18\x03\x20\x01(\x08R\x08isLocked\x12)\
    \n\x11scene_tag_id_list\x18\x06\x20\x03(\rR\x0esceneTagIdListB\x1b\n\x19\
    emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::MapLayerInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlayerWorldSceneInfo::generated_message_descriptor_data());
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