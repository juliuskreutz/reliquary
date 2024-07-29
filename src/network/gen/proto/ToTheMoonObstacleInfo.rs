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

//! Generated file from `ToTheMoonObstacleInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ToTheMoonObstacleInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ToTheMoonObstacleInfo {
    // message fields
    // @@protoc_insertion_point(field:ToTheMoonObstacleInfo.center)
    pub center: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:ToTheMoonObstacleInfo.handle_id)
    pub handle_id: i32,
    // @@protoc_insertion_point(field:ToTheMoonObstacleInfo.half_extents)
    pub half_extents: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:ToTheMoonObstacleInfo.type)
    pub type_: ::protobuf::EnumOrUnknown<to_the_moon_obstacle_info::ShapeType>,
    // @@protoc_insertion_point(field:ToTheMoonObstacleInfo.rotation)
    pub rotation: ::protobuf::MessageField<super::MathQuaternion::MathQuaternion>,
    // special fields
    // @@protoc_insertion_point(special_field:ToTheMoonObstacleInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ToTheMoonObstacleInfo {
    fn default() -> &'a ToTheMoonObstacleInfo {
        <ToTheMoonObstacleInfo as ::protobuf::Message>::default_instance()
    }
}

impl ToTheMoonObstacleInfo {
    pub fn new() -> ToTheMoonObstacleInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "center",
            |m: &ToTheMoonObstacleInfo| { &m.center },
            |m: &mut ToTheMoonObstacleInfo| { &mut m.center },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "handle_id",
            |m: &ToTheMoonObstacleInfo| { &m.handle_id },
            |m: &mut ToTheMoonObstacleInfo| { &mut m.handle_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "half_extents",
            |m: &ToTheMoonObstacleInfo| { &m.half_extents },
            |m: &mut ToTheMoonObstacleInfo| { &mut m.half_extents },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &ToTheMoonObstacleInfo| { &m.type_ },
            |m: &mut ToTheMoonObstacleInfo| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MathQuaternion::MathQuaternion>(
            "rotation",
            |m: &ToTheMoonObstacleInfo| { &m.rotation },
            |m: &mut ToTheMoonObstacleInfo| { &mut m.rotation },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ToTheMoonObstacleInfo>(
            "ToTheMoonObstacleInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ToTheMoonObstacleInfo {
    const NAME: &'static str = "ToTheMoonObstacleInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.center)?;
                },
                24 => {
                    self.handle_id = is.read_int32()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.half_extents)?;
                },
                80 => {
                    self.type_ = is.read_enum_or_unknown()?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rotation)?;
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
        if let Some(v) = self.center.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.handle_id != 0 {
            my_size += ::protobuf::rt::int32_size(3, self.handle_id);
        }
        if let Some(v) = self.half_extents.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.type_ != ::protobuf::EnumOrUnknown::new(to_the_moon_obstacle_info::ShapeType::OBSTACLE_SHAPE_CAPSULE) {
            my_size += ::protobuf::rt::int32_size(10, self.type_.value());
        }
        if let Some(v) = self.rotation.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.center.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.handle_id != 0 {
            os.write_int32(3, self.handle_id)?;
        }
        if let Some(v) = self.half_extents.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.type_ != ::protobuf::EnumOrUnknown::new(to_the_moon_obstacle_info::ShapeType::OBSTACLE_SHAPE_CAPSULE) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.type_))?;
        }
        if let Some(v) = self.rotation.as_ref() {
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

    fn new() -> ToTheMoonObstacleInfo {
        ToTheMoonObstacleInfo::new()
    }

    fn clear(&mut self) {
        self.center.clear();
        self.handle_id = 0;
        self.half_extents.clear();
        self.type_ = ::protobuf::EnumOrUnknown::new(to_the_moon_obstacle_info::ShapeType::OBSTACLE_SHAPE_CAPSULE);
        self.rotation.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ToTheMoonObstacleInfo {
        static instance: ToTheMoonObstacleInfo = ToTheMoonObstacleInfo {
            center: ::protobuf::MessageField::none(),
            handle_id: 0,
            half_extents: ::protobuf::MessageField::none(),
            type_: ::protobuf::EnumOrUnknown::from_i32(0),
            rotation: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ToTheMoonObstacleInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ToTheMoonObstacleInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ToTheMoonObstacleInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ToTheMoonObstacleInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ToTheMoonObstacleInfo`
pub mod to_the_moon_obstacle_info {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:ToTheMoonObstacleInfo.ShapeType)
    pub enum ShapeType {
        // @@protoc_insertion_point(enum_value:ToTheMoonObstacleInfo.ShapeType.OBSTACLE_SHAPE_CAPSULE)
        OBSTACLE_SHAPE_CAPSULE = 0,
        // @@protoc_insertion_point(enum_value:ToTheMoonObstacleInfo.ShapeType.OBSTACLE_SHAPE_BOX)
        OBSTACLE_SHAPE_BOX = 1,
    }

    impl ::protobuf::Enum for ShapeType {
        const NAME: &'static str = "ShapeType";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<ShapeType> {
            match value {
                0 => ::std::option::Option::Some(ShapeType::OBSTACLE_SHAPE_CAPSULE),
                1 => ::std::option::Option::Some(ShapeType::OBSTACLE_SHAPE_BOX),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<ShapeType> {
            match str {
                "OBSTACLE_SHAPE_CAPSULE" => ::std::option::Option::Some(ShapeType::OBSTACLE_SHAPE_CAPSULE),
                "OBSTACLE_SHAPE_BOX" => ::std::option::Option::Some(ShapeType::OBSTACLE_SHAPE_BOX),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [ShapeType] = &[
            ShapeType::OBSTACLE_SHAPE_CAPSULE,
            ShapeType::OBSTACLE_SHAPE_BOX,
        ];
    }

    impl ::protobuf::EnumFull for ShapeType {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("ToTheMoonObstacleInfo.ShapeType").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for ShapeType {
        fn default() -> Self {
            ShapeType::OBSTACLE_SHAPE_CAPSULE
        }
    }

    impl ShapeType {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<ShapeType>("ToTheMoonObstacleInfo.ShapeType")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bToTheMoonObstacleInfo.proto\x1a\x14MathQuaternion.proto\x1a\x0cVec\
    tor.proto\"\xa5\x02\n\x15ToTheMoonObstacleInfo\x12\x1f\n\x06center\x18\
    \x02\x20\x01(\x0b2\x07.VectorR\x06center\x12\x1b\n\thandle_id\x18\x03\
    \x20\x01(\x05R\x08handleId\x12*\n\x0chalf_extents\x18\x04\x20\x01(\x0b2\
    \x07.VectorR\x0bhalfExtents\x124\n\x04type\x18\n\x20\x01(\x0e2\x20.ToThe\
    MoonObstacleInfo.ShapeTypeR\x04type\x12+\n\x08rotation\x18\x0b\x20\x01(\
    \x0b2\x0f.MathQuaternionR\x08rotation\"?\n\tShapeType\x12\x1a\n\x16OBSTA\
    CLE_SHAPE_CAPSULE\x10\0\x12\x16\n\x12OBSTACLE_SHAPE_BOX\x10\x01B\x1b\n\
    \x19emu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::MathQuaternion::file_descriptor().clone());
            deps.push(super::Vector::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ToTheMoonObstacleInfo::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(to_the_moon_obstacle_info::ShapeType::generated_enum_descriptor_data());
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