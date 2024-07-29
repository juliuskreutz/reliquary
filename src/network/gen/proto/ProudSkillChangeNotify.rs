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

//! Generated file from `ProudSkillChangeNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ProudSkillChangeNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ProudSkillChangeNotify {
    // message fields
    // @@protoc_insertion_point(field:ProudSkillChangeNotify.skill_depot_id)
    pub skill_depot_id: u32,
    // @@protoc_insertion_point(field:ProudSkillChangeNotify.avatar_guid)
    pub avatar_guid: u64,
    // @@protoc_insertion_point(field:ProudSkillChangeNotify.entity_id)
    pub entity_id: u32,
    // @@protoc_insertion_point(field:ProudSkillChangeNotify.proud_skill_list)
    pub proud_skill_list: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:ProudSkillChangeNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ProudSkillChangeNotify {
    fn default() -> &'a ProudSkillChangeNotify {
        <ProudSkillChangeNotify as ::protobuf::Message>::default_instance()
    }
}

impl ProudSkillChangeNotify {
    pub fn new() -> ProudSkillChangeNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "skill_depot_id",
            |m: &ProudSkillChangeNotify| { &m.skill_depot_id },
            |m: &mut ProudSkillChangeNotify| { &mut m.skill_depot_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_guid",
            |m: &ProudSkillChangeNotify| { &m.avatar_guid },
            |m: &mut ProudSkillChangeNotify| { &mut m.avatar_guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entity_id",
            |m: &ProudSkillChangeNotify| { &m.entity_id },
            |m: &mut ProudSkillChangeNotify| { &mut m.entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "proud_skill_list",
            |m: &ProudSkillChangeNotify| { &m.proud_skill_list },
            |m: &mut ProudSkillChangeNotify| { &mut m.proud_skill_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ProudSkillChangeNotify>(
            "ProudSkillChangeNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ProudSkillChangeNotify {
    const NAME: &'static str = "ProudSkillChangeNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.skill_depot_id = is.read_uint32()?;
                },
                80 => {
                    self.avatar_guid = is.read_uint64()?;
                },
                96 => {
                    self.entity_id = is.read_uint32()?;
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.proud_skill_list)?;
                },
                120 => {
                    self.proud_skill_list.push(is.read_uint32()?);
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
        if self.skill_depot_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.skill_depot_id);
        }
        if self.avatar_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(10, self.avatar_guid);
        }
        if self.entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.entity_id);
        }
        for value in &self.proud_skill_list {
            my_size += ::protobuf::rt::uint32_size(15, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.skill_depot_id != 0 {
            os.write_uint32(7, self.skill_depot_id)?;
        }
        if self.avatar_guid != 0 {
            os.write_uint64(10, self.avatar_guid)?;
        }
        if self.entity_id != 0 {
            os.write_uint32(12, self.entity_id)?;
        }
        for v in &self.proud_skill_list {
            os.write_uint32(15, *v)?;
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

    fn new() -> ProudSkillChangeNotify {
        ProudSkillChangeNotify::new()
    }

    fn clear(&mut self) {
        self.skill_depot_id = 0;
        self.avatar_guid = 0;
        self.entity_id = 0;
        self.proud_skill_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ProudSkillChangeNotify {
        static instance: ProudSkillChangeNotify = ProudSkillChangeNotify {
            skill_depot_id: 0,
            avatar_guid: 0,
            entity_id: 0,
            proud_skill_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ProudSkillChangeNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ProudSkillChangeNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ProudSkillChangeNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProudSkillChangeNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cProudSkillChangeNotify.proto\"\xa6\x01\n\x16ProudSkillChangeNotify\
    \x12$\n\x0eskill_depot_id\x18\x07\x20\x01(\rR\x0cskillDepotId\x12\x1f\n\
    \x0bavatar_guid\x18\n\x20\x01(\x04R\navatarGuid\x12\x1b\n\tentity_id\x18\
    \x0c\x20\x01(\rR\x08entityId\x12(\n\x10proud_skill_list\x18\x0f\x20\x03(\
    \rR\x0eproudSkillListB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(ProudSkillChangeNotify::generated_message_descriptor_data());
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
