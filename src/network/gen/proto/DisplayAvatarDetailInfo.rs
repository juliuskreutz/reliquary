// This file is generated by rust-protobuf 3.6.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `DisplayAvatarDetailInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_6_0;

// @@protoc_insertion_point(message:DisplayAvatarDetailInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DisplayAvatarDetailInfo {
    // message fields
    // @@protoc_insertion_point(field:DisplayAvatarDetailInfo.pos)
    pub pos: u32,
    // @@protoc_insertion_point(field:DisplayAvatarDetailInfo.relic_list)
    pub relic_list: ::std::vec::Vec<super::DisplayRelicInfo::DisplayRelicInfo>,
    // @@protoc_insertion_point(field:DisplayAvatarDetailInfo.rank)
    pub rank: u32,
    // @@protoc_insertion_point(field:DisplayAvatarDetailInfo.level)
    pub level: u32,
    // @@protoc_insertion_point(field:DisplayAvatarDetailInfo.exp)
    pub exp: u32,
    // @@protoc_insertion_point(field:DisplayAvatarDetailInfo.promotion)
    pub promotion: u32,
    // @@protoc_insertion_point(field:DisplayAvatarDetailInfo.skilltree_list)
    pub skilltree_list: ::std::vec::Vec<super::AvatarSkillTree::AvatarSkillTree>,
    // @@protoc_insertion_point(field:DisplayAvatarDetailInfo.equipment)
    pub equipment: ::protobuf::MessageField<super::DisplayEquipmentInfo::DisplayEquipmentInfo>,
    // @@protoc_insertion_point(field:DisplayAvatarDetailInfo.avatar_id)
    pub avatar_id: u32,
    // @@protoc_insertion_point(field:DisplayAvatarDetailInfo.dressed_skin_id)
    pub dressed_skin_id: u32,
    // @@protoc_insertion_point(field:DisplayAvatarDetailInfo.all_path_unlocked_special_point_id_list)
    pub all_path_unlocked_special_point_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:DisplayAvatarDetailInfo.all_path_special_skilltree_list)
    pub all_path_special_skilltree_list: ::std::vec::Vec<super::AvatarSkillTree::AvatarSkillTree>,
    // special fields
    // @@protoc_insertion_point(special_field:DisplayAvatarDetailInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DisplayAvatarDetailInfo {
    fn default() -> &'a DisplayAvatarDetailInfo {
        <DisplayAvatarDetailInfo as ::protobuf::Message>::default_instance()
    }
}

impl DisplayAvatarDetailInfo {
    pub fn new() -> DisplayAvatarDetailInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(12);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "pos",
            |m: &DisplayAvatarDetailInfo| { &m.pos },
            |m: &mut DisplayAvatarDetailInfo| { &mut m.pos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "relic_list",
            |m: &DisplayAvatarDetailInfo| { &m.relic_list },
            |m: &mut DisplayAvatarDetailInfo| { &mut m.relic_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "rank",
            |m: &DisplayAvatarDetailInfo| { &m.rank },
            |m: &mut DisplayAvatarDetailInfo| { &mut m.rank },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &DisplayAvatarDetailInfo| { &m.level },
            |m: &mut DisplayAvatarDetailInfo| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &DisplayAvatarDetailInfo| { &m.exp },
            |m: &mut DisplayAvatarDetailInfo| { &mut m.exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "promotion",
            |m: &DisplayAvatarDetailInfo| { &m.promotion },
            |m: &mut DisplayAvatarDetailInfo| { &mut m.promotion },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "skilltree_list",
            |m: &DisplayAvatarDetailInfo| { &m.skilltree_list },
            |m: &mut DisplayAvatarDetailInfo| { &mut m.skilltree_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DisplayEquipmentInfo::DisplayEquipmentInfo>(
            "equipment",
            |m: &DisplayAvatarDetailInfo| { &m.equipment },
            |m: &mut DisplayAvatarDetailInfo| { &mut m.equipment },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &DisplayAvatarDetailInfo| { &m.avatar_id },
            |m: &mut DisplayAvatarDetailInfo| { &mut m.avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dressed_skin_id",
            |m: &DisplayAvatarDetailInfo| { &m.dressed_skin_id },
            |m: &mut DisplayAvatarDetailInfo| { &mut m.dressed_skin_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "all_path_unlocked_special_point_id_list",
            |m: &DisplayAvatarDetailInfo| { &m.all_path_unlocked_special_point_id_list },
            |m: &mut DisplayAvatarDetailInfo| { &mut m.all_path_unlocked_special_point_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "all_path_special_skilltree_list",
            |m: &DisplayAvatarDetailInfo| { &m.all_path_special_skilltree_list },
            |m: &mut DisplayAvatarDetailInfo| { &mut m.all_path_special_skilltree_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DisplayAvatarDetailInfo>(
            "DisplayAvatarDetailInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DisplayAvatarDetailInfo {
    const NAME: &'static str = "DisplayAvatarDetailInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.pos = is.read_uint32()?;
                },
                106 => {
                    self.relic_list.push(is.read_message()?);
                },
                80 => {
                    self.rank = is.read_uint32()?;
                },
                64 => {
                    self.level = is.read_uint32()?;
                },
                96 => {
                    self.exp = is.read_uint32()?;
                },
                48 => {
                    self.promotion = is.read_uint32()?;
                },
                58 => {
                    self.skilltree_list.push(is.read_message()?);
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.equipment)?;
                },
                8 => {
                    self.avatar_id = is.read_uint32()?;
                },
                112 => {
                    self.dressed_skin_id = is.read_uint32()?;
                },
                7994 => {
                    is.read_repeated_packed_uint32_into(&mut self.all_path_unlocked_special_point_id_list)?;
                },
                7992 => {
                    self.all_path_unlocked_special_point_id_list.push(is.read_uint32()?);
                },
                79994 => {
                    self.all_path_special_skilltree_list.push(is.read_message()?);
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
        if self.pos != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.pos);
        }
        for value in &self.relic_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.rank != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.rank);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.level);
        }
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.exp);
        }
        if self.promotion != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.promotion);
        }
        for value in &self.skilltree_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.equipment.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.avatar_id);
        }
        if self.dressed_skin_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.dressed_skin_id);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(999, &self.all_path_unlocked_special_point_id_list);
        for value in &self.all_path_special_skilltree_list {
            let len = value.compute_size();
            my_size += 3 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.pos != 0 {
            os.write_uint32(5, self.pos)?;
        }
        for v in &self.relic_list {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.rank != 0 {
            os.write_uint32(10, self.rank)?;
        }
        if self.level != 0 {
            os.write_uint32(8, self.level)?;
        }
        if self.exp != 0 {
            os.write_uint32(12, self.exp)?;
        }
        if self.promotion != 0 {
            os.write_uint32(6, self.promotion)?;
        }
        for v in &self.skilltree_list {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        if let Some(v) = self.equipment.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if self.avatar_id != 0 {
            os.write_uint32(1, self.avatar_id)?;
        }
        if self.dressed_skin_id != 0 {
            os.write_uint32(14, self.dressed_skin_id)?;
        }
        os.write_repeated_packed_uint32(999, &self.all_path_unlocked_special_point_id_list)?;
        for v in &self.all_path_special_skilltree_list {
            ::protobuf::rt::write_message_field_with_cached_size(9999, v, os)?;
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

    fn new() -> DisplayAvatarDetailInfo {
        DisplayAvatarDetailInfo::new()
    }

    fn clear(&mut self) {
        self.pos = 0;
        self.relic_list.clear();
        self.rank = 0;
        self.level = 0;
        self.exp = 0;
        self.promotion = 0;
        self.skilltree_list.clear();
        self.equipment.clear();
        self.avatar_id = 0;
        self.dressed_skin_id = 0;
        self.all_path_unlocked_special_point_id_list.clear();
        self.all_path_special_skilltree_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DisplayAvatarDetailInfo {
        static instance: DisplayAvatarDetailInfo = DisplayAvatarDetailInfo {
            pos: 0,
            relic_list: ::std::vec::Vec::new(),
            rank: 0,
            level: 0,
            exp: 0,
            promotion: 0,
            skilltree_list: ::std::vec::Vec::new(),
            equipment: ::protobuf::MessageField::none(),
            avatar_id: 0,
            dressed_skin_id: 0,
            all_path_unlocked_special_point_id_list: ::std::vec::Vec::new(),
            all_path_special_skilltree_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DisplayAvatarDetailInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DisplayAvatarDetailInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DisplayAvatarDetailInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DisplayAvatarDetailInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dDisplayAvatarDetailInfo.proto\x1a\x1aDisplayEquipmentInfo.proto\
    \x1a\x15AvatarSkillTree.proto\x1a\x16DisplayRelicInfo.proto\"\x98\x04\n\
    \x17DisplayAvatarDetailInfo\x12\x10\n\x03pos\x18\x05\x20\x01(\rR\x03pos\
    \x120\n\nrelic_list\x18\r\x20\x03(\x0b2\x11.DisplayRelicInfoR\trelicList\
    \x12\x12\n\x04rank\x18\n\x20\x01(\rR\x04rank\x12\x14\n\x05level\x18\x08\
    \x20\x01(\rR\x05level\x12\x10\n\x03exp\x18\x0c\x20\x01(\rR\x03exp\x12\
    \x1c\n\tpromotion\x18\x06\x20\x01(\rR\tpromotion\x127\n\x0eskilltree_lis\
    t\x18\x07\x20\x03(\x0b2\x10.AvatarSkillTreeR\rskilltreeList\x123\n\tequi\
    pment\x18\x0f\x20\x01(\x0b2\x15.DisplayEquipmentInfoR\tequipment\x12\x1b\
    \n\tavatar_id\x18\x01\x20\x01(\rR\x08avatarId\x12&\n\x0fdressed_skin_id\
    \x18\x0e\x20\x01(\rR\rdressedSkinId\x12S\n'all_path_unlocked_special_poi\
    nt_id_list\x18\xe7\x07\x20\x03(\rR!allPathUnlockedSpecialPointIdList\x12\
    W\n\x1fall_path_special_skilltree_list\x18\x8fN\x20\x03(\x0b2\x10.Avatar\
    SkillTreeR\x1ballPathSpecialSkilltreeListB\x15\n\x13emu.lunarcore.protob\
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
            deps.push(super::DisplayEquipmentInfo::file_descriptor().clone());
            deps.push(super::AvatarSkillTree::file_descriptor().clone());
            deps.push(super::DisplayRelicInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DisplayAvatarDetailInfo::generated_message_descriptor_data());
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
