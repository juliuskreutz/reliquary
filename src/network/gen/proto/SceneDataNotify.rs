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

//! Generated file from `SceneDataNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SceneDataNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneDataNotify {
    // message fields
    // @@protoc_insertion_point(field:SceneDataNotify.level_config_name_list)
    pub level_config_name_list: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:SceneDataNotify.scene_tag_id_list)
    pub scene_tag_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SceneDataNotify.map_layer_info)
    pub map_layer_info: ::protobuf::MessageField<super::MapLayerInfo::MapLayerInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:SceneDataNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneDataNotify {
    fn default() -> &'a SceneDataNotify {
        <SceneDataNotify as ::protobuf::Message>::default_instance()
    }
}

impl SceneDataNotify {
    pub fn new() -> SceneDataNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "level_config_name_list",
            |m: &SceneDataNotify| { &m.level_config_name_list },
            |m: &mut SceneDataNotify| { &mut m.level_config_name_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "scene_tag_id_list",
            |m: &SceneDataNotify| { &m.scene_tag_id_list },
            |m: &mut SceneDataNotify| { &mut m.scene_tag_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MapLayerInfo::MapLayerInfo>(
            "map_layer_info",
            |m: &SceneDataNotify| { &m.map_layer_info },
            |m: &mut SceneDataNotify| { &mut m.map_layer_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneDataNotify>(
            "SceneDataNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneDataNotify {
    const NAME: &'static str = "SceneDataNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    self.level_config_name_list.push(is.read_string()?);
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.scene_tag_id_list)?;
                },
                40 => {
                    self.scene_tag_id_list.push(is.read_uint32()?);
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.map_layer_info)?;
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
        for value in &self.level_config_name_list {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.scene_tag_id_list {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        if let Some(v) = self.map_layer_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.level_config_name_list {
            os.write_string(2, &v)?;
        };
        for v in &self.scene_tag_id_list {
            os.write_uint32(5, *v)?;
        };
        if let Some(v) = self.map_layer_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> SceneDataNotify {
        SceneDataNotify::new()
    }

    fn clear(&mut self) {
        self.level_config_name_list.clear();
        self.scene_tag_id_list.clear();
        self.map_layer_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneDataNotify {
        static instance: SceneDataNotify = SceneDataNotify {
            level_config_name_list: ::std::vec::Vec::new(),
            scene_tag_id_list: ::std::vec::Vec::new(),
            map_layer_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneDataNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneDataNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneDataNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneDataNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15SceneDataNotify.proto\x1a\x12MapLayerInfo.proto\"\xa6\x01\n\x0fSce\
    neDataNotify\x123\n\x16level_config_name_list\x18\x02\x20\x03(\tR\x13lev\
    elConfigNameList\x12)\n\x11scene_tag_id_list\x18\x05\x20\x03(\rR\x0escen\
    eTagIdList\x123\n\x0emap_layer_info\x18\x0b\x20\x01(\x0b2\r.MapLayerInfo\
    R\x0cmapLayerInfoB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(SceneDataNotify::generated_message_descriptor_data());
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
