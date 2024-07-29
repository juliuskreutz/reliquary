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

//! Generated file from `TrialAvatarInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:TrialAvatarInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct TrialAvatarInfo {
    // message fields
    // @@protoc_insertion_point(field:TrialAvatarInfo.trial_avatar_id)
    pub trial_avatar_id: u32,
    // @@protoc_insertion_point(field:TrialAvatarInfo.trial_equip_list)
    pub trial_equip_list: ::std::vec::Vec<super::Item::Item>,
    // @@protoc_insertion_point(field:TrialAvatarInfo.grant_record)
    pub grant_record: ::protobuf::MessageField<super::TrialAvatarGrantRecord::TrialAvatarGrantRecord>,
    // special fields
    // @@protoc_insertion_point(special_field:TrialAvatarInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a TrialAvatarInfo {
    fn default() -> &'a TrialAvatarInfo {
        <TrialAvatarInfo as ::protobuf::Message>::default_instance()
    }
}

impl TrialAvatarInfo {
    pub fn new() -> TrialAvatarInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "trial_avatar_id",
            |m: &TrialAvatarInfo| { &m.trial_avatar_id },
            |m: &mut TrialAvatarInfo| { &mut m.trial_avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "trial_equip_list",
            |m: &TrialAvatarInfo| { &m.trial_equip_list },
            |m: &mut TrialAvatarInfo| { &mut m.trial_equip_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::TrialAvatarGrantRecord::TrialAvatarGrantRecord>(
            "grant_record",
            |m: &TrialAvatarInfo| { &m.grant_record },
            |m: &mut TrialAvatarInfo| { &mut m.grant_record },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<TrialAvatarInfo>(
            "TrialAvatarInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for TrialAvatarInfo {
    const NAME: &'static str = "TrialAvatarInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.trial_avatar_id = is.read_uint32()?;
                },
                18 => {
                    self.trial_equip_list.push(is.read_message()?);
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.grant_record)?;
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
        if self.trial_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.trial_avatar_id);
        }
        for value in &self.trial_equip_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.grant_record.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.trial_avatar_id != 0 {
            os.write_uint32(1, self.trial_avatar_id)?;
        }
        for v in &self.trial_equip_list {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if let Some(v) = self.grant_record.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

    fn new() -> TrialAvatarInfo {
        TrialAvatarInfo::new()
    }

    fn clear(&mut self) {
        self.trial_avatar_id = 0;
        self.trial_equip_list.clear();
        self.grant_record.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static TrialAvatarInfo {
        static instance: TrialAvatarInfo = TrialAvatarInfo {
            trial_avatar_id: 0,
            trial_equip_list: ::std::vec::Vec::new(),
            grant_record: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for TrialAvatarInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("TrialAvatarInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for TrialAvatarInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TrialAvatarInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15TrialAvatarInfo.proto\x1a\nItem.proto\x1a\x1cTrialAvatarGrantRecor\
    d.proto\"\xa6\x01\n\x0fTrialAvatarInfo\x12&\n\x0ftrial_avatar_id\x18\x01\
    \x20\x01(\rR\rtrialAvatarId\x12/\n\x10trial_equip_list\x18\x02\x20\x03(\
    \x0b2\x05.ItemR\x0etrialEquipList\x12:\n\x0cgrant_record\x18\x03\x20\x01\
    (\x0b2\x17.TrialAvatarGrantRecordR\x0bgrantRecordB\x1b\n\x19emu.grasscut\
    ter.net.protob\x06proto3\
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
            deps.push(super::Item::file_descriptor().clone());
            deps.push(super::TrialAvatarGrantRecord::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(TrialAvatarInfo::generated_message_descriptor_data());
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