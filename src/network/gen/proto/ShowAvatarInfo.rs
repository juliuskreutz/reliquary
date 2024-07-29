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

//! Generated file from `ShowAvatarInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ShowAvatarInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ShowAvatarInfo {
    // message fields
    // @@protoc_insertion_point(field:ShowAvatarInfo.avatar_id)
    pub avatar_id: u32,
    // @@protoc_insertion_point(field:ShowAvatarInfo.prop_map)
    pub prop_map: ::std::collections::HashMap<u32, super::PropValue::PropValue>,
    // @@protoc_insertion_point(field:ShowAvatarInfo.talent_id_list)
    pub talent_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ShowAvatarInfo.fight_prop_map)
    pub fight_prop_map: ::std::collections::HashMap<u32, f32>,
    // @@protoc_insertion_point(field:ShowAvatarInfo.skill_depot_id)
    pub skill_depot_id: u32,
    // @@protoc_insertion_point(field:ShowAvatarInfo.core_proud_skill_level)
    pub core_proud_skill_level: u32,
    // @@protoc_insertion_point(field:ShowAvatarInfo.inherent_proud_skill_list)
    pub inherent_proud_skill_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:ShowAvatarInfo.skill_level_map)
    pub skill_level_map: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:ShowAvatarInfo.proud_skill_extra_level_map)
    pub proud_skill_extra_level_map: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:ShowAvatarInfo.equip_list)
    pub equip_list: ::std::vec::Vec<super::ShowEquip::ShowEquip>,
    // @@protoc_insertion_point(field:ShowAvatarInfo.fetter_info)
    pub fetter_info: ::protobuf::MessageField<super::AvatarFetterInfo::AvatarFetterInfo>,
    // @@protoc_insertion_point(field:ShowAvatarInfo.costume_id)
    pub costume_id: u32,
    // @@protoc_insertion_point(field:ShowAvatarInfo.excel_info)
    pub excel_info: ::protobuf::MessageField<super::AvatarExcelInfo::AvatarExcelInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:ShowAvatarInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ShowAvatarInfo {
    fn default() -> &'a ShowAvatarInfo {
        <ShowAvatarInfo as ::protobuf::Message>::default_instance()
    }
}

impl ShowAvatarInfo {
    pub fn new() -> ShowAvatarInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "avatar_id",
            |m: &ShowAvatarInfo| { &m.avatar_id },
            |m: &mut ShowAvatarInfo| { &mut m.avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "prop_map",
            |m: &ShowAvatarInfo| { &m.prop_map },
            |m: &mut ShowAvatarInfo| { &mut m.prop_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "talent_id_list",
            |m: &ShowAvatarInfo| { &m.talent_id_list },
            |m: &mut ShowAvatarInfo| { &mut m.talent_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "fight_prop_map",
            |m: &ShowAvatarInfo| { &m.fight_prop_map },
            |m: &mut ShowAvatarInfo| { &mut m.fight_prop_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "skill_depot_id",
            |m: &ShowAvatarInfo| { &m.skill_depot_id },
            |m: &mut ShowAvatarInfo| { &mut m.skill_depot_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "core_proud_skill_level",
            |m: &ShowAvatarInfo| { &m.core_proud_skill_level },
            |m: &mut ShowAvatarInfo| { &mut m.core_proud_skill_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "inherent_proud_skill_list",
            |m: &ShowAvatarInfo| { &m.inherent_proud_skill_list },
            |m: &mut ShowAvatarInfo| { &mut m.inherent_proud_skill_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "skill_level_map",
            |m: &ShowAvatarInfo| { &m.skill_level_map },
            |m: &mut ShowAvatarInfo| { &mut m.skill_level_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "proud_skill_extra_level_map",
            |m: &ShowAvatarInfo| { &m.proud_skill_extra_level_map },
            |m: &mut ShowAvatarInfo| { &mut m.proud_skill_extra_level_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "equip_list",
            |m: &ShowAvatarInfo| { &m.equip_list },
            |m: &mut ShowAvatarInfo| { &mut m.equip_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AvatarFetterInfo::AvatarFetterInfo>(
            "fetter_info",
            |m: &ShowAvatarInfo| { &m.fetter_info },
            |m: &mut ShowAvatarInfo| { &mut m.fetter_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "costume_id",
            |m: &ShowAvatarInfo| { &m.costume_id },
            |m: &mut ShowAvatarInfo| { &mut m.costume_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AvatarExcelInfo::AvatarExcelInfo>(
            "excel_info",
            |m: &ShowAvatarInfo| { &m.excel_info },
            |m: &mut ShowAvatarInfo| { &mut m.excel_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ShowAvatarInfo>(
            "ShowAvatarInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ShowAvatarInfo {
    const NAME: &'static str = "ShowAvatarInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.avatar_id = is.read_uint32()?;
                },
                18 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.prop_map.insert(key, value);
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.talent_id_list)?;
                },
                24 => {
                    self.talent_id_list.push(is.read_uint32()?);
                },
                34 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            21 => value = is.read_float()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.fight_prop_map.insert(key, value);
                },
                40 => {
                    self.skill_depot_id = is.read_uint32()?;
                },
                48 => {
                    self.core_proud_skill_level = is.read_uint32()?;
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.inherent_proud_skill_list)?;
                },
                56 => {
                    self.inherent_proud_skill_list.push(is.read_uint32()?);
                },
                66 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.skill_level_map.insert(key, value);
                },
                74 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.proud_skill_extra_level_map.insert(key, value);
                },
                82 => {
                    self.equip_list.push(is.read_message()?);
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.fetter_info)?;
                },
                96 => {
                    self.costume_id = is.read_uint32()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.excel_info)?;
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
        if self.avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.avatar_id);
        }
        for (k, v) in &self.prop_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.talent_id_list {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        for (k, v) in &self.fight_prop_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += 1 + 4;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.skill_depot_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.skill_depot_id);
        }
        if self.core_proud_skill_level != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.core_proud_skill_level);
        }
        for value in &self.inherent_proud_skill_list {
            my_size += ::protobuf::rt::uint32_size(7, *value);
        };
        for (k, v) in &self.skill_level_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for (k, v) in &self.proud_skill_extra_level_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.equip_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.fetter_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.costume_id != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.costume_id);
        }
        if let Some(v) = self.excel_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.avatar_id != 0 {
            os.write_uint32(1, self.avatar_id)?;
        }
        for (k, v) in &self.prop_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(18)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        for v in &self.talent_id_list {
            os.write_uint32(3, *v)?;
        };
        for (k, v) in &self.fight_prop_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += 1 + 4;
            os.write_raw_varint32(34)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_float(2, *v)?;
        };
        if self.skill_depot_id != 0 {
            os.write_uint32(5, self.skill_depot_id)?;
        }
        if self.core_proud_skill_level != 0 {
            os.write_uint32(6, self.core_proud_skill_level)?;
        }
        for v in &self.inherent_proud_skill_list {
            os.write_uint32(7, *v)?;
        };
        for (k, v) in &self.skill_level_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(66)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        for (k, v) in &self.proud_skill_extra_level_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(74)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        for v in &self.equip_list {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        };
        if let Some(v) = self.fetter_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.costume_id != 0 {
            os.write_uint32(12, self.costume_id)?;
        }
        if let Some(v) = self.excel_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
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

    fn new() -> ShowAvatarInfo {
        ShowAvatarInfo::new()
    }

    fn clear(&mut self) {
        self.avatar_id = 0;
        self.prop_map.clear();
        self.talent_id_list.clear();
        self.fight_prop_map.clear();
        self.skill_depot_id = 0;
        self.core_proud_skill_level = 0;
        self.inherent_proud_skill_list.clear();
        self.skill_level_map.clear();
        self.proud_skill_extra_level_map.clear();
        self.equip_list.clear();
        self.fetter_info.clear();
        self.costume_id = 0;
        self.excel_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ShowAvatarInfo {
        static instance: ::protobuf::rt::Lazy<ShowAvatarInfo> = ::protobuf::rt::Lazy::new();
        instance.get(ShowAvatarInfo::new)
    }
}

impl ::protobuf::MessageFull for ShowAvatarInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ShowAvatarInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ShowAvatarInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ShowAvatarInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14ShowAvatarInfo.proto\x1a\x0fPropValue.proto\x1a\x0fShowEquip.proto\
    \x1a\x16AvatarFetterInfo.proto\x1a\x15AvatarExcelInfo.proto\"\xe9\x07\n\
    \x0eShowAvatarInfo\x12\x1b\n\tavatar_id\x18\x01\x20\x01(\rR\x08avatarId\
    \x127\n\x08prop_map\x18\x02\x20\x03(\x0b2\x1c.ShowAvatarInfo.PropMapEntr\
    yR\x07propMap\x12$\n\x0etalent_id_list\x18\x03\x20\x03(\rR\x0ctalentIdLi\
    st\x12G\n\x0efight_prop_map\x18\x04\x20\x03(\x0b2!.ShowAvatarInfo.FightP\
    ropMapEntryR\x0cfightPropMap\x12$\n\x0eskill_depot_id\x18\x05\x20\x01(\r\
    R\x0cskillDepotId\x123\n\x16core_proud_skill_level\x18\x06\x20\x01(\rR\
    \x13coreProudSkillLevel\x129\n\x19inherent_proud_skill_list\x18\x07\x20\
    \x03(\rR\x16inherentProudSkillList\x12J\n\x0fskill_level_map\x18\x08\x20\
    \x03(\x0b2\".ShowAvatarInfo.SkillLevelMapEntryR\rskillLevelMap\x12j\n\
    \x1bproud_skill_extra_level_map\x18\t\x20\x03(\x0b2,.ShowAvatarInfo.Prou\
    dSkillExtraLevelMapEntryR\x17proudSkillExtraLevelMap\x12)\n\nequip_list\
    \x18\n\x20\x03(\x0b2\n.ShowEquipR\tequipList\x122\n\x0bfetter_info\x18\
    \x0b\x20\x01(\x0b2\x11.AvatarFetterInfoR\nfetterInfo\x12\x1d\n\ncostume_\
    id\x18\x0c\x20\x01(\rR\tcostumeId\x12/\n\nexcel_info\x18\r\x20\x01(\x0b2\
    \x10.AvatarExcelInfoR\texcelInfo\x1aF\n\x0cPropMapEntry\x12\x10\n\x03key\
    \x18\x01\x20\x01(\rR\x03key\x12\x20\n\x05value\x18\x02\x20\x01(\x0b2\n.P\
    ropValueR\x05value:\x028\x01\x1a?\n\x11FightPropMapEntry\x12\x10\n\x03ke\
    y\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\x02R\
    \x05value:\x028\x01\x1a@\n\x12SkillLevelMapEntry\x12\x10\n\x03key\x18\
    \x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05value:\
    \x028\x01\x1aJ\n\x1cProudSkillExtraLevelMapEntry\x12\x10\n\x03key\x18\
    \x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05value:\
    \x028\x01B\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::PropValue::file_descriptor().clone());
            deps.push(super::ShowEquip::file_descriptor().clone());
            deps.push(super::AvatarFetterInfo::file_descriptor().clone());
            deps.push(super::AvatarExcelInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ShowAvatarInfo::generated_message_descriptor_data());
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
