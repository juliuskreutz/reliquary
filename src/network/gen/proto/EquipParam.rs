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

//! Generated file from `EquipParam.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EquipParam)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EquipParam {
    // message fields
    // @@protoc_insertion_point(field:EquipParam.item_id)
    pub item_id: u32,
    // @@protoc_insertion_point(field:EquipParam.item_num)
    pub item_num: u32,
    // @@protoc_insertion_point(field:EquipParam.item_level)
    pub item_level: u32,
    // @@protoc_insertion_point(field:EquipParam.promote_level)
    pub promote_level: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EquipParam.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EquipParam {
    fn default() -> &'a EquipParam {
        <EquipParam as ::protobuf::Message>::default_instance()
    }
}

impl EquipParam {
    pub fn new() -> EquipParam {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "item_id",
            |m: &EquipParam| { &m.item_id },
            |m: &mut EquipParam| { &mut m.item_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "item_num",
            |m: &EquipParam| { &m.item_num },
            |m: &mut EquipParam| { &mut m.item_num },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "item_level",
            |m: &EquipParam| { &m.item_level },
            |m: &mut EquipParam| { &mut m.item_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "promote_level",
            |m: &EquipParam| { &m.promote_level },
            |m: &mut EquipParam| { &mut m.promote_level },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EquipParam>(
            "EquipParam",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EquipParam {
    const NAME: &'static str = "EquipParam";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.item_id = is.read_uint32()?;
                },
                16 => {
                    self.item_num = is.read_uint32()?;
                },
                24 => {
                    self.item_level = is.read_uint32()?;
                },
                32 => {
                    self.promote_level = is.read_uint32()?;
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
        if self.item_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.item_id);
        }
        if self.item_num != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.item_num);
        }
        if self.item_level != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.item_level);
        }
        if self.promote_level != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.promote_level);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.item_id != 0 {
            os.write_uint32(1, self.item_id)?;
        }
        if self.item_num != 0 {
            os.write_uint32(2, self.item_num)?;
        }
        if self.item_level != 0 {
            os.write_uint32(3, self.item_level)?;
        }
        if self.promote_level != 0 {
            os.write_uint32(4, self.promote_level)?;
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

    fn new() -> EquipParam {
        EquipParam::new()
    }

    fn clear(&mut self) {
        self.item_id = 0;
        self.item_num = 0;
        self.item_level = 0;
        self.promote_level = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EquipParam {
        static instance: EquipParam = EquipParam {
            item_id: 0,
            item_num: 0,
            item_level: 0,
            promote_level: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EquipParam {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EquipParam").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EquipParam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EquipParam {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10EquipParam.proto\"\x84\x01\n\nEquipParam\x12\x17\n\x07item_id\x18\
    \x01\x20\x01(\rR\x06itemId\x12\x19\n\x08item_num\x18\x02\x20\x01(\rR\x07\
    itemNum\x12\x1d\n\nitem_level\x18\x03\x20\x01(\rR\titemLevel\x12#\n\rpro\
    mote_level\x18\x04\x20\x01(\rR\x0cpromoteLevelB\x1b\n\x19emu.grasscutter\
    .net.protob\x06proto3\
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
            messages.push(EquipParam::generated_message_descriptor_data());
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