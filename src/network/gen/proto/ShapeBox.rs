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

//! Generated file from `ShapeBox.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ShapeBox)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ShapeBox {
    // message fields
    // @@protoc_insertion_point(field:ShapeBox.center)
    pub center: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:ShapeBox.axis0)
    pub axis0: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:ShapeBox.axis1)
    pub axis1: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:ShapeBox.axis4)
    pub axis4: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:ShapeBox.extents)
    pub extents: ::protobuf::MessageField<super::Vector::Vector>,
    // special fields
    // @@protoc_insertion_point(special_field:ShapeBox.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ShapeBox {
    fn default() -> &'a ShapeBox {
        <ShapeBox as ::protobuf::Message>::default_instance()
    }
}

impl ShapeBox {
    pub fn new() -> ShapeBox {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "center",
            |m: &ShapeBox| { &m.center },
            |m: &mut ShapeBox| { &mut m.center },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "axis0",
            |m: &ShapeBox| { &m.axis0 },
            |m: &mut ShapeBox| { &mut m.axis0 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "axis1",
            |m: &ShapeBox| { &m.axis1 },
            |m: &mut ShapeBox| { &mut m.axis1 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "axis4",
            |m: &ShapeBox| { &m.axis4 },
            |m: &mut ShapeBox| { &mut m.axis4 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "extents",
            |m: &ShapeBox| { &m.extents },
            |m: &mut ShapeBox| { &mut m.extents },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ShapeBox>(
            "ShapeBox",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ShapeBox {
    const NAME: &'static str = "ShapeBox";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.center)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.axis0)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.axis1)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.axis4)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.extents)?;
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
        if let Some(v) = self.axis0.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.axis1.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.axis4.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.extents.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.center.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.axis0.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.axis1.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.axis4.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.extents.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> ShapeBox {
        ShapeBox::new()
    }

    fn clear(&mut self) {
        self.center.clear();
        self.axis0.clear();
        self.axis1.clear();
        self.axis4.clear();
        self.extents.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ShapeBox {
        static instance: ShapeBox = ShapeBox {
            center: ::protobuf::MessageField::none(),
            axis0: ::protobuf::MessageField::none(),
            axis1: ::protobuf::MessageField::none(),
            axis4: ::protobuf::MessageField::none(),
            extents: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ShapeBox {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ShapeBox").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ShapeBox {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShapeBox {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eShapeBox.proto\x1a\x0cVector.proto\"\xab\x01\n\x08ShapeBox\x12\x1f\
    \n\x06center\x18\x01\x20\x01(\x0b2\x07.VectorR\x06center\x12\x1d\n\x05ax\
    is0\x18\x02\x20\x01(\x0b2\x07.VectorR\x05axis0\x12\x1d\n\x05axis1\x18\
    \x03\x20\x01(\x0b2\x07.VectorR\x05axis1\x12\x1d\n\x05axis4\x18\x04\x20\
    \x01(\x0b2\x07.VectorR\x05axis4\x12!\n\x07extents\x18\x05\x20\x01(\x0b2\
    \x07.VectorR\x07extentsB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::Vector::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ShapeBox::generated_message_descriptor_data());
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