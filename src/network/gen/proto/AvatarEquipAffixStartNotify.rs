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

//! Generated file from `AvatarEquipAffixStartNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AvatarEquipAffixStartNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AvatarEquipAffixStartNotify {
    // message fields
    // @@protoc_insertion_point(field:AvatarEquipAffixStartNotify.equip_affix_info)
    pub equip_affix_info: ::protobuf::MessageField<super::AvatarEquipAffixInfo::AvatarEquipAffixInfo>,
    // @@protoc_insertion_point(field:AvatarEquipAffixStartNotify.avatar_guid)
    pub avatar_guid: u64,
    // special fields
    // @@protoc_insertion_point(special_field:AvatarEquipAffixStartNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AvatarEquipAffixStartNotify {
    fn default() -> &'a AvatarEquipAffixStartNotify {
        <AvatarEquipAffixStartNotify as ::protobuf::Message>::default_instance()
    }
}

impl AvatarEquipAffixStartNotify {
    pub fn new() -> AvatarEquipAffixStartNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AvatarEquipAffixInfo::AvatarEquipAffixInfo>(
            "equip_affix_info",
            |m: &AvatarEquipAffixStartNotify| { &m.equip_affix_info },
            |m: &mut AvatarEquipAffixStartNotify| { &mut m.equip_affix_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_guid",
            |m: &AvatarEquipAffixStartNotify| { &m.avatar_guid },
            |m: &mut AvatarEquipAffixStartNotify| { &mut m.avatar_guid },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AvatarEquipAffixStartNotify>(
            "AvatarEquipAffixStartNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AvatarEquipAffixStartNotify {
    const NAME: &'static str = "AvatarEquipAffixStartNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.equip_affix_info)?;
                },
                96 => {
                    self.avatar_guid = is.read_uint64()?;
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
        if let Some(v) = self.equip_affix_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.avatar_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(12, self.avatar_guid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.equip_affix_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        }
        if self.avatar_guid != 0 {
            os.write_uint64(12, self.avatar_guid)?;
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

    fn new() -> AvatarEquipAffixStartNotify {
        AvatarEquipAffixStartNotify::new()
    }

    fn clear(&mut self) {
        self.equip_affix_info.clear();
        self.avatar_guid = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AvatarEquipAffixStartNotify {
        static instance: AvatarEquipAffixStartNotify = AvatarEquipAffixStartNotify {
            equip_affix_info: ::protobuf::MessageField::none(),
            avatar_guid: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AvatarEquipAffixStartNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AvatarEquipAffixStartNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AvatarEquipAffixStartNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AvatarEquipAffixStartNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!AvatarEquipAffixStartNotify.proto\x1a\x1aAvatarEquipAffixInfo.proto\"\
    \x7f\n\x1bAvatarEquipAffixStartNotify\x12?\n\x10equip_affix_info\x18\x05\
    \x20\x01(\x0b2\x15.AvatarEquipAffixInfoR\x0eequipAffixInfo\x12\x1f\n\x0b\
    avatar_guid\x18\x0c\x20\x01(\x04R\navatarGuidB\x1b\n\x19emu.grasscutter.\
    net.protob\x06proto3\
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
            deps.push(super::AvatarEquipAffixInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AvatarEquipAffixStartNotify::generated_message_descriptor_data());
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