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

//! Generated file from `GetSceneAreaRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetSceneAreaRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetSceneAreaRsp {
    // message fields
    // @@protoc_insertion_point(field:GetSceneAreaRsp.city_info_list)
    pub city_info_list: ::std::vec::Vec<super::CityInfo::CityInfo>,
    // @@protoc_insertion_point(field:GetSceneAreaRsp.area_id_list)
    pub area_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetSceneAreaRsp.scene_id)
    pub scene_id: u32,
    // @@protoc_insertion_point(field:GetSceneAreaRsp.retcode)
    pub retcode: i32,
    // special fields
    // @@protoc_insertion_point(special_field:GetSceneAreaRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetSceneAreaRsp {
    fn default() -> &'a GetSceneAreaRsp {
        <GetSceneAreaRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetSceneAreaRsp {
    pub fn new() -> GetSceneAreaRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "city_info_list",
            |m: &GetSceneAreaRsp| { &m.city_info_list },
            |m: &mut GetSceneAreaRsp| { &mut m.city_info_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "area_id_list",
            |m: &GetSceneAreaRsp| { &m.area_id_list },
            |m: &mut GetSceneAreaRsp| { &mut m.area_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scene_id",
            |m: &GetSceneAreaRsp| { &m.scene_id },
            |m: &mut GetSceneAreaRsp| { &mut m.scene_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetSceneAreaRsp| { &m.retcode },
            |m: &mut GetSceneAreaRsp| { &mut m.retcode },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetSceneAreaRsp>(
            "GetSceneAreaRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetSceneAreaRsp {
    const NAME: &'static str = "GetSceneAreaRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                122 => {
                    self.city_info_list.push(is.read_message()?);
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.area_id_list)?;
                },
                24 => {
                    self.area_id_list.push(is.read_uint32()?);
                },
                40 => {
                    self.scene_id = is.read_uint32()?;
                },
                48 => {
                    self.retcode = is.read_int32()?;
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
        for value in &self.city_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.area_id_list {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        if self.scene_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.scene_id);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.retcode);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.city_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        for v in &self.area_id_list {
            os.write_uint32(3, *v)?;
        };
        if self.scene_id != 0 {
            os.write_uint32(5, self.scene_id)?;
        }
        if self.retcode != 0 {
            os.write_int32(6, self.retcode)?;
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

    fn new() -> GetSceneAreaRsp {
        GetSceneAreaRsp::new()
    }

    fn clear(&mut self) {
        self.city_info_list.clear();
        self.area_id_list.clear();
        self.scene_id = 0;
        self.retcode = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetSceneAreaRsp {
        static instance: GetSceneAreaRsp = GetSceneAreaRsp {
            city_info_list: ::std::vec::Vec::new(),
            area_id_list: ::std::vec::Vec::new(),
            scene_id: 0,
            retcode: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetSceneAreaRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetSceneAreaRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetSceneAreaRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetSceneAreaRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15GetSceneAreaRsp.proto\x1a\x0eCityInfo.proto\"\x99\x01\n\x0fGetScen\
    eAreaRsp\x12/\n\x0ecity_info_list\x18\x0f\x20\x03(\x0b2\t.CityInfoR\x0cc\
    ityInfoList\x12\x20\n\x0carea_id_list\x18\x03\x20\x03(\rR\nareaIdList\
    \x12\x19\n\x08scene_id\x18\x05\x20\x01(\rR\x07sceneId\x12\x18\n\x07retco\
    de\x18\x06\x20\x01(\x05R\x07retcodeB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::CityInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetSceneAreaRsp::generated_message_descriptor_data());
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