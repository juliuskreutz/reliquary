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

//! Generated file from `MapMarkPoint.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MapMarkPoint)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MapMarkPoint {
    // message fields
    // @@protoc_insertion_point(field:MapMarkPoint.scene_id)
    pub scene_id: u32,
    // @@protoc_insertion_point(field:MapMarkPoint.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:MapMarkPoint.pos)
    pub pos: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:MapMarkPoint.point_type)
    pub point_type: ::protobuf::EnumOrUnknown<super::MapMarkPointType::MapMarkPointType>,
    // @@protoc_insertion_point(field:MapMarkPoint.monster_id)
    pub monster_id: u32,
    // @@protoc_insertion_point(field:MapMarkPoint.from_type)
    pub from_type: ::protobuf::EnumOrUnknown<super::MapMarkFromType::MapMarkFromType>,
    // @@protoc_insertion_point(field:MapMarkPoint.quest_id)
    pub quest_id: u32,
    // @@protoc_insertion_point(field:MapMarkPoint.AAOGCHADHPL)
    pub AAOGCHADHPL: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MapMarkPoint.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MapMarkPoint {
    fn default() -> &'a MapMarkPoint {
        <MapMarkPoint as ::protobuf::Message>::default_instance()
    }
}

impl MapMarkPoint {
    pub fn new() -> MapMarkPoint {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scene_id",
            |m: &MapMarkPoint| { &m.scene_id },
            |m: &mut MapMarkPoint| { &mut m.scene_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &MapMarkPoint| { &m.name },
            |m: &mut MapMarkPoint| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "pos",
            |m: &MapMarkPoint| { &m.pos },
            |m: &mut MapMarkPoint| { &mut m.pos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "point_type",
            |m: &MapMarkPoint| { &m.point_type },
            |m: &mut MapMarkPoint| { &mut m.point_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "monster_id",
            |m: &MapMarkPoint| { &m.monster_id },
            |m: &mut MapMarkPoint| { &mut m.monster_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "from_type",
            |m: &MapMarkPoint| { &m.from_type },
            |m: &mut MapMarkPoint| { &mut m.from_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "quest_id",
            |m: &MapMarkPoint| { &m.quest_id },
            |m: &mut MapMarkPoint| { &mut m.quest_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AAOGCHADHPL",
            |m: &MapMarkPoint| { &m.AAOGCHADHPL },
            |m: &mut MapMarkPoint| { &mut m.AAOGCHADHPL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MapMarkPoint>(
            "MapMarkPoint",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MapMarkPoint {
    const NAME: &'static str = "MapMarkPoint";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.scene_id = is.read_uint32()?;
                },
                18 => {
                    self.name = is.read_string()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pos)?;
                },
                32 => {
                    self.point_type = is.read_enum_or_unknown()?;
                },
                40 => {
                    self.monster_id = is.read_uint32()?;
                },
                48 => {
                    self.from_type = is.read_enum_or_unknown()?;
                },
                56 => {
                    self.quest_id = is.read_uint32()?;
                },
                64 => {
                    self.AAOGCHADHPL = is.read_uint32()?;
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
        if self.scene_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.scene_id);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if let Some(v) = self.pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.point_type != ::protobuf::EnumOrUnknown::new(super::MapMarkPointType::MapMarkPointType::MAP_MARK_POINT_TYPE_NPC) {
            my_size += ::protobuf::rt::int32_size(4, self.point_type.value());
        }
        if self.monster_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.monster_id);
        }
        if self.from_type != ::protobuf::EnumOrUnknown::new(super::MapMarkFromType::MapMarkFromType::MAP_MARK_FROM_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(6, self.from_type.value());
        }
        if self.quest_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.quest_id);
        }
        if self.AAOGCHADHPL != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.AAOGCHADHPL);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.scene_id != 0 {
            os.write_uint32(1, self.scene_id)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if let Some(v) = self.pos.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if self.point_type != ::protobuf::EnumOrUnknown::new(super::MapMarkPointType::MapMarkPointType::MAP_MARK_POINT_TYPE_NPC) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.point_type))?;
        }
        if self.monster_id != 0 {
            os.write_uint32(5, self.monster_id)?;
        }
        if self.from_type != ::protobuf::EnumOrUnknown::new(super::MapMarkFromType::MapMarkFromType::MAP_MARK_FROM_TYPE_NONE) {
            os.write_enum(6, ::protobuf::EnumOrUnknown::value(&self.from_type))?;
        }
        if self.quest_id != 0 {
            os.write_uint32(7, self.quest_id)?;
        }
        if self.AAOGCHADHPL != 0 {
            os.write_uint32(8, self.AAOGCHADHPL)?;
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

    fn new() -> MapMarkPoint {
        MapMarkPoint::new()
    }

    fn clear(&mut self) {
        self.scene_id = 0;
        self.name.clear();
        self.pos.clear();
        self.point_type = ::protobuf::EnumOrUnknown::new(super::MapMarkPointType::MapMarkPointType::MAP_MARK_POINT_TYPE_NPC);
        self.monster_id = 0;
        self.from_type = ::protobuf::EnumOrUnknown::new(super::MapMarkFromType::MapMarkFromType::MAP_MARK_FROM_TYPE_NONE);
        self.quest_id = 0;
        self.AAOGCHADHPL = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MapMarkPoint {
        static instance: MapMarkPoint = MapMarkPoint {
            scene_id: 0,
            name: ::std::string::String::new(),
            pos: ::protobuf::MessageField::none(),
            point_type: ::protobuf::EnumOrUnknown::from_i32(0),
            monster_id: 0,
            from_type: ::protobuf::EnumOrUnknown::from_i32(0),
            quest_id: 0,
            AAOGCHADHPL: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MapMarkPoint {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MapMarkPoint").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MapMarkPoint {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MapMarkPoint {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12MapMarkPoint.proto\x1a\x0cVector.proto\x1a\x16MapMarkPointType.pro\
    to\x1a\x15MapMarkFromType.proto\"\x95\x02\n\x0cMapMarkPoint\x12\x19\n\
    \x08scene_id\x18\x01\x20\x01(\rR\x07sceneId\x12\x12\n\x04name\x18\x02\
    \x20\x01(\tR\x04name\x12\x19\n\x03pos\x18\x03\x20\x01(\x0b2\x07.VectorR\
    \x03pos\x120\n\npoint_type\x18\x04\x20\x01(\x0e2\x11.MapMarkPointTypeR\t\
    pointType\x12\x1d\n\nmonster_id\x18\x05\x20\x01(\rR\tmonsterId\x12-\n\tf\
    rom_type\x18\x06\x20\x01(\x0e2\x10.MapMarkFromTypeR\x08fromType\x12\x19\
    \n\x08quest_id\x18\x07\x20\x01(\rR\x07questId\x12\x20\n\x0bAAOGCHADHPL\
    \x18\x08\x20\x01(\rR\x0bAAOGCHADHPLB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::Vector::file_descriptor().clone());
            deps.push(super::MapMarkPointType::file_descriptor().clone());
            deps.push(super::MapMarkFromType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MapMarkPoint::generated_message_descriptor_data());
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
