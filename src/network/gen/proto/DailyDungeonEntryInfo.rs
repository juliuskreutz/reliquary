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

//! Generated file from `DailyDungeonEntryInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DailyDungeonEntryInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DailyDungeonEntryInfo {
    // message fields
    // @@protoc_insertion_point(field:DailyDungeonEntryInfo.recommend_dungeon_entry_info)
    pub recommend_dungeon_entry_info: ::protobuf::MessageField<super::DungeonEntryInfo::DungeonEntryInfo>,
    // @@protoc_insertion_point(field:DailyDungeonEntryInfo.recommend_dungeon_id)
    pub recommend_dungeon_id: u32,
    // @@protoc_insertion_point(field:DailyDungeonEntryInfo.dungeon_entry_id)
    pub dungeon_entry_id: u32,
    // @@protoc_insertion_point(field:DailyDungeonEntryInfo.dungeon_entry_config_id)
    pub dungeon_entry_config_id: u32,
    // @@protoc_insertion_point(field:DailyDungeonEntryInfo.is_point_unlocked)
    pub is_point_unlocked: bool,
    // @@protoc_insertion_point(field:DailyDungeonEntryInfo.is_quick_open)
    pub is_quick_open: bool,
    // special fields
    // @@protoc_insertion_point(special_field:DailyDungeonEntryInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DailyDungeonEntryInfo {
    fn default() -> &'a DailyDungeonEntryInfo {
        <DailyDungeonEntryInfo as ::protobuf::Message>::default_instance()
    }
}

impl DailyDungeonEntryInfo {
    pub fn new() -> DailyDungeonEntryInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DungeonEntryInfo::DungeonEntryInfo>(
            "recommend_dungeon_entry_info",
            |m: &DailyDungeonEntryInfo| { &m.recommend_dungeon_entry_info },
            |m: &mut DailyDungeonEntryInfo| { &mut m.recommend_dungeon_entry_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "recommend_dungeon_id",
            |m: &DailyDungeonEntryInfo| { &m.recommend_dungeon_id },
            |m: &mut DailyDungeonEntryInfo| { &mut m.recommend_dungeon_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dungeon_entry_id",
            |m: &DailyDungeonEntryInfo| { &m.dungeon_entry_id },
            |m: &mut DailyDungeonEntryInfo| { &mut m.dungeon_entry_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dungeon_entry_config_id",
            |m: &DailyDungeonEntryInfo| { &m.dungeon_entry_config_id },
            |m: &mut DailyDungeonEntryInfo| { &mut m.dungeon_entry_config_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_point_unlocked",
            |m: &DailyDungeonEntryInfo| { &m.is_point_unlocked },
            |m: &mut DailyDungeonEntryInfo| { &mut m.is_point_unlocked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_quick_open",
            |m: &DailyDungeonEntryInfo| { &m.is_quick_open },
            |m: &mut DailyDungeonEntryInfo| { &mut m.is_quick_open },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DailyDungeonEntryInfo>(
            "DailyDungeonEntryInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DailyDungeonEntryInfo {
    const NAME: &'static str = "DailyDungeonEntryInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.recommend_dungeon_entry_info)?;
                },
                48 => {
                    self.recommend_dungeon_id = is.read_uint32()?;
                },
                72 => {
                    self.dungeon_entry_id = is.read_uint32()?;
                },
                80 => {
                    self.dungeon_entry_config_id = is.read_uint32()?;
                },
                104 => {
                    self.is_point_unlocked = is.read_bool()?;
                },
                112 => {
                    self.is_quick_open = is.read_bool()?;
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
        if let Some(v) = self.recommend_dungeon_entry_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.recommend_dungeon_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.recommend_dungeon_id);
        }
        if self.dungeon_entry_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.dungeon_entry_id);
        }
        if self.dungeon_entry_config_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.dungeon_entry_config_id);
        }
        if self.is_point_unlocked != false {
            my_size += 1 + 1;
        }
        if self.is_quick_open != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.recommend_dungeon_entry_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.recommend_dungeon_id != 0 {
            os.write_uint32(6, self.recommend_dungeon_id)?;
        }
        if self.dungeon_entry_id != 0 {
            os.write_uint32(9, self.dungeon_entry_id)?;
        }
        if self.dungeon_entry_config_id != 0 {
            os.write_uint32(10, self.dungeon_entry_config_id)?;
        }
        if self.is_point_unlocked != false {
            os.write_bool(13, self.is_point_unlocked)?;
        }
        if self.is_quick_open != false {
            os.write_bool(14, self.is_quick_open)?;
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

    fn new() -> DailyDungeonEntryInfo {
        DailyDungeonEntryInfo::new()
    }

    fn clear(&mut self) {
        self.recommend_dungeon_entry_info.clear();
        self.recommend_dungeon_id = 0;
        self.dungeon_entry_id = 0;
        self.dungeon_entry_config_id = 0;
        self.is_point_unlocked = false;
        self.is_quick_open = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DailyDungeonEntryInfo {
        static instance: DailyDungeonEntryInfo = DailyDungeonEntryInfo {
            recommend_dungeon_entry_info: ::protobuf::MessageField::none(),
            recommend_dungeon_id: 0,
            dungeon_entry_id: 0,
            dungeon_entry_config_id: 0,
            is_point_unlocked: false,
            is_quick_open: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DailyDungeonEntryInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DailyDungeonEntryInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DailyDungeonEntryInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DailyDungeonEntryInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bDailyDungeonEntryInfo.proto\x1a\x16DungeonEntryInfo.proto\"\xce\
    \x02\n\x15DailyDungeonEntryInfo\x12R\n\x1crecommend_dungeon_entry_info\
    \x18\x02\x20\x01(\x0b2\x11.DungeonEntryInfoR\x19recommendDungeonEntryInf\
    o\x120\n\x14recommend_dungeon_id\x18\x06\x20\x01(\rR\x12recommendDungeon\
    Id\x12(\n\x10dungeon_entry_id\x18\t\x20\x01(\rR\x0edungeonEntryId\x125\n\
    \x17dungeon_entry_config_id\x18\n\x20\x01(\rR\x14dungeonEntryConfigId\
    \x12*\n\x11is_point_unlocked\x18\r\x20\x01(\x08R\x0fisPointUnlocked\x12\
    \"\n\ris_quick_open\x18\x0e\x20\x01(\x08R\x0bisQuickOpenB\x1b\n\x19emu.g\
    rasscutter.net.protob\x06proto3\
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
            deps.push(super::DungeonEntryInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(DailyDungeonEntryInfo::generated_message_descriptor_data());
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