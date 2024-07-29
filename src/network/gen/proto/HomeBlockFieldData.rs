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

//! Generated file from `HomeBlockFieldData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HomeBlockFieldData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HomeBlockFieldData {
    // message fields
    // @@protoc_insertion_point(field:HomeBlockFieldData.guid)
    pub guid: u32,
    // @@protoc_insertion_point(field:HomeBlockFieldData.sub_field_list)
    pub sub_field_list: ::std::vec::Vec<super::HomeBlockSubFieldData::HomeBlockSubFieldData>,
    // @@protoc_insertion_point(field:HomeBlockFieldData.furniture_id)
    pub furniture_id: u32,
    // @@protoc_insertion_point(field:HomeBlockFieldData.rot)
    pub rot: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:HomeBlockFieldData.pos)
    pub pos: ::protobuf::MessageField<super::Vector::Vector>,
    // special fields
    // @@protoc_insertion_point(special_field:HomeBlockFieldData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HomeBlockFieldData {
    fn default() -> &'a HomeBlockFieldData {
        <HomeBlockFieldData as ::protobuf::Message>::default_instance()
    }
}

impl HomeBlockFieldData {
    pub fn new() -> HomeBlockFieldData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "guid",
            |m: &HomeBlockFieldData| { &m.guid },
            |m: &mut HomeBlockFieldData| { &mut m.guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "sub_field_list",
            |m: &HomeBlockFieldData| { &m.sub_field_list },
            |m: &mut HomeBlockFieldData| { &mut m.sub_field_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "furniture_id",
            |m: &HomeBlockFieldData| { &m.furniture_id },
            |m: &mut HomeBlockFieldData| { &mut m.furniture_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "rot",
            |m: &HomeBlockFieldData| { &m.rot },
            |m: &mut HomeBlockFieldData| { &mut m.rot },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "pos",
            |m: &HomeBlockFieldData| { &m.pos },
            |m: &mut HomeBlockFieldData| { &mut m.pos },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HomeBlockFieldData>(
            "HomeBlockFieldData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HomeBlockFieldData {
    const NAME: &'static str = "HomeBlockFieldData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.guid = is.read_uint32()?;
                },
                58 => {
                    self.sub_field_list.push(is.read_message()?);
                },
                72 => {
                    self.furniture_id = is.read_uint32()?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rot)?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pos)?;
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
        if self.guid != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.guid);
        }
        for value in &self.sub_field_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.furniture_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.furniture_id);
        }
        if let Some(v) = self.rot.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.guid != 0 {
            os.write_uint32(4, self.guid)?;
        }
        for v in &self.sub_field_list {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        if self.furniture_id != 0 {
            os.write_uint32(9, self.furniture_id)?;
        }
        if let Some(v) = self.rot.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if let Some(v) = self.pos.as_ref() {
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

    fn new() -> HomeBlockFieldData {
        HomeBlockFieldData::new()
    }

    fn clear(&mut self) {
        self.guid = 0;
        self.sub_field_list.clear();
        self.furniture_id = 0;
        self.rot.clear();
        self.pos.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HomeBlockFieldData {
        static instance: HomeBlockFieldData = HomeBlockFieldData {
            guid: 0,
            sub_field_list: ::std::vec::Vec::new(),
            furniture_id: 0,
            rot: ::protobuf::MessageField::none(),
            pos: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HomeBlockFieldData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HomeBlockFieldData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HomeBlockFieldData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HomeBlockFieldData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18HomeBlockFieldData.proto\x1a\x0cVector.proto\x1a\x1bHomeBlockSubFi\
    eldData.proto\"\xbf\x01\n\x12HomeBlockFieldData\x12\x12\n\x04guid\x18\
    \x04\x20\x01(\rR\x04guid\x12<\n\x0esub_field_list\x18\x07\x20\x03(\x0b2\
    \x16.HomeBlockSubFieldDataR\x0csubFieldList\x12!\n\x0cfurniture_id\x18\t\
    \x20\x01(\rR\x0bfurnitureId\x12\x19\n\x03rot\x18\x0b\x20\x01(\x0b2\x07.V\
    ectorR\x03rot\x12\x19\n\x03pos\x18\x0e\x20\x01(\x0b2\x07.VectorR\x03posB\
    \x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::Vector::file_descriptor().clone());
            deps.push(super::HomeBlockSubFieldData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(HomeBlockFieldData::generated_message_descriptor_data());
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