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

//! Generated file from `MistTrialLevelData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MistTrialLevelData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MistTrialLevelData {
    // message fields
    // @@protoc_insertion_point(field:MistTrialLevelData.level_id)
    pub level_id: u32,
    // @@protoc_insertion_point(field:MistTrialLevelData.first_pass_time)
    pub first_pass_time: u32,
    // @@protoc_insertion_point(field:MistTrialLevelData.is_open)
    pub is_open: bool,
    // @@protoc_insertion_point(field:MistTrialLevelData.best_hit_avatar)
    pub best_hit_avatar: ::protobuf::MessageField<super::MistTrialBestAvatar::MistTrialBestAvatar>,
    // @@protoc_insertion_point(field:MistTrialLevelData.best_avatar_list)
    pub best_avatar_list: ::std::vec::Vec<super::MistTrialBestAvatar::MistTrialBestAvatar>,
    // @@protoc_insertion_point(field:MistTrialLevelData.open_time)
    pub open_time: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MistTrialLevelData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MistTrialLevelData {
    fn default() -> &'a MistTrialLevelData {
        <MistTrialLevelData as ::protobuf::Message>::default_instance()
    }
}

impl MistTrialLevelData {
    pub fn new() -> MistTrialLevelData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level_id",
            |m: &MistTrialLevelData| { &m.level_id },
            |m: &mut MistTrialLevelData| { &mut m.level_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "first_pass_time",
            |m: &MistTrialLevelData| { &m.first_pass_time },
            |m: &mut MistTrialLevelData| { &mut m.first_pass_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_open",
            |m: &MistTrialLevelData| { &m.is_open },
            |m: &mut MistTrialLevelData| { &mut m.is_open },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MistTrialBestAvatar::MistTrialBestAvatar>(
            "best_hit_avatar",
            |m: &MistTrialLevelData| { &m.best_hit_avatar },
            |m: &mut MistTrialLevelData| { &mut m.best_hit_avatar },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "best_avatar_list",
            |m: &MistTrialLevelData| { &m.best_avatar_list },
            |m: &mut MistTrialLevelData| { &mut m.best_avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "open_time",
            |m: &MistTrialLevelData| { &m.open_time },
            |m: &mut MistTrialLevelData| { &mut m.open_time },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MistTrialLevelData>(
            "MistTrialLevelData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MistTrialLevelData {
    const NAME: &'static str = "MistTrialLevelData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.level_id = is.read_uint32()?;
                },
                88 => {
                    self.first_pass_time = is.read_uint32()?;
                },
                112 => {
                    self.is_open = is.read_bool()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.best_hit_avatar)?;
                },
                82 => {
                    self.best_avatar_list.push(is.read_message()?);
                },
                56 => {
                    self.open_time = is.read_uint32()?;
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
        if self.level_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.level_id);
        }
        if self.first_pass_time != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.first_pass_time);
        }
        if self.is_open != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.best_hit_avatar.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.best_avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.open_time != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.open_time);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.level_id != 0 {
            os.write_uint32(13, self.level_id)?;
        }
        if self.first_pass_time != 0 {
            os.write_uint32(11, self.first_pass_time)?;
        }
        if self.is_open != false {
            os.write_bool(14, self.is_open)?;
        }
        if let Some(v) = self.best_hit_avatar.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        for v in &self.best_avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if self.open_time != 0 {
            os.write_uint32(7, self.open_time)?;
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

    fn new() -> MistTrialLevelData {
        MistTrialLevelData::new()
    }

    fn clear(&mut self) {
        self.level_id = 0;
        self.first_pass_time = 0;
        self.is_open = false;
        self.best_hit_avatar.clear();
        self.best_avatar_list.clear();
        self.open_time = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MistTrialLevelData {
        static instance: MistTrialLevelData = MistTrialLevelData {
            level_id: 0,
            first_pass_time: 0,
            is_open: false,
            best_hit_avatar: ::protobuf::MessageField::none(),
            best_avatar_list: ::std::vec::Vec::new(),
            open_time: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MistTrialLevelData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MistTrialLevelData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MistTrialLevelData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MistTrialLevelData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18MistTrialLevelData.proto\x1a\x19MistTrialBestAvatar.proto\"\x8b\
    \x02\n\x12MistTrialLevelData\x12\x19\n\x08level_id\x18\r\x20\x01(\rR\x07\
    levelId\x12&\n\x0ffirst_pass_time\x18\x0b\x20\x01(\rR\rfirstPassTime\x12\
    \x17\n\x07is_open\x18\x0e\x20\x01(\x08R\x06isOpen\x12<\n\x0fbest_hit_ava\
    tar\x18\t\x20\x01(\x0b2\x14.MistTrialBestAvatarR\rbestHitAvatar\x12>\n\
    \x10best_avatar_list\x18\n\x20\x03(\x0b2\x14.MistTrialBestAvatarR\x0ebes\
    tAvatarList\x12\x1b\n\topen_time\x18\x07\x20\x01(\rR\x08openTimeB\x1b\n\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::MistTrialBestAvatar::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MistTrialLevelData::generated_message_descriptor_data());
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
