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

//! Generated file from `GachaActivityDetailInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GachaActivityDetailInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GachaActivityDetailInfo {
    // message fields
    // @@protoc_insertion_point(field:GachaActivityDetailInfo.gacha_stage_data_list)
    pub gacha_stage_data_list: ::std::vec::Vec<super::GachaStageData::GachaStageData>,
    // @@protoc_insertion_point(field:GachaActivityDetailInfo.robot_num_map)
    pub robot_num_map: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:GachaActivityDetailInfo.have_reward_robot_num_map)
    pub have_reward_robot_num_map: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:GachaActivityDetailInfo.FOLDOMNGPJM)
    pub FOLDOMNGPJM: u32,
    // @@protoc_insertion_point(field:GachaActivityDetailInfo.have_get_robot_list)
    pub have_get_robot_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GachaActivityDetailInfo.FGLFAJBJEEA)
    pub FGLFAJBJEEA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:GachaActivityDetailInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GachaActivityDetailInfo {
    fn default() -> &'a GachaActivityDetailInfo {
        <GachaActivityDetailInfo as ::protobuf::Message>::default_instance()
    }
}

impl GachaActivityDetailInfo {
    pub fn new() -> GachaActivityDetailInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "gacha_stage_data_list",
            |m: &GachaActivityDetailInfo| { &m.gacha_stage_data_list },
            |m: &mut GachaActivityDetailInfo| { &mut m.gacha_stage_data_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "robot_num_map",
            |m: &GachaActivityDetailInfo| { &m.robot_num_map },
            |m: &mut GachaActivityDetailInfo| { &mut m.robot_num_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "have_reward_robot_num_map",
            |m: &GachaActivityDetailInfo| { &m.have_reward_robot_num_map },
            |m: &mut GachaActivityDetailInfo| { &mut m.have_reward_robot_num_map },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FOLDOMNGPJM",
            |m: &GachaActivityDetailInfo| { &m.FOLDOMNGPJM },
            |m: &mut GachaActivityDetailInfo| { &mut m.FOLDOMNGPJM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "have_get_robot_list",
            |m: &GachaActivityDetailInfo| { &m.have_get_robot_list },
            |m: &mut GachaActivityDetailInfo| { &mut m.have_get_robot_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FGLFAJBJEEA",
            |m: &GachaActivityDetailInfo| { &m.FGLFAJBJEEA },
            |m: &mut GachaActivityDetailInfo| { &mut m.FGLFAJBJEEA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GachaActivityDetailInfo>(
            "GachaActivityDetailInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GachaActivityDetailInfo {
    const NAME: &'static str = "GachaActivityDetailInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    self.gacha_stage_data_list.push(is.read_message()?);
                },
                114 => {
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
                    self.robot_num_map.insert(key, value);
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
                    self.have_reward_robot_num_map.insert(key, value);
                },
                80 => {
                    self.FOLDOMNGPJM = is.read_uint32()?;
                },
                122 => {
                    is.read_repeated_packed_uint32_into(&mut self.have_get_robot_list)?;
                },
                120 => {
                    self.have_get_robot_list.push(is.read_uint32()?);
                },
                96 => {
                    self.FGLFAJBJEEA = is.read_uint32()?;
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
        for value in &self.gacha_stage_data_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for (k, v) in &self.robot_num_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for (k, v) in &self.have_reward_robot_num_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.FOLDOMNGPJM != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.FOLDOMNGPJM);
        }
        for value in &self.have_get_robot_list {
            my_size += ::protobuf::rt::uint32_size(15, *value);
        };
        if self.FGLFAJBJEEA != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.FGLFAJBJEEA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.gacha_stage_data_list {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        for (k, v) in &self.robot_num_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(114)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        for (k, v) in &self.have_reward_robot_num_map {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(74)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        if self.FOLDOMNGPJM != 0 {
            os.write_uint32(10, self.FOLDOMNGPJM)?;
        }
        for v in &self.have_get_robot_list {
            os.write_uint32(15, *v)?;
        };
        if self.FGLFAJBJEEA != 0 {
            os.write_uint32(12, self.FGLFAJBJEEA)?;
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

    fn new() -> GachaActivityDetailInfo {
        GachaActivityDetailInfo::new()
    }

    fn clear(&mut self) {
        self.gacha_stage_data_list.clear();
        self.robot_num_map.clear();
        self.have_reward_robot_num_map.clear();
        self.FOLDOMNGPJM = 0;
        self.have_get_robot_list.clear();
        self.FGLFAJBJEEA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GachaActivityDetailInfo {
        static instance: ::protobuf::rt::Lazy<GachaActivityDetailInfo> = ::protobuf::rt::Lazy::new();
        instance.get(GachaActivityDetailInfo::new)
    }
}

impl ::protobuf::MessageFull for GachaActivityDetailInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GachaActivityDetailInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GachaActivityDetailInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GachaActivityDetailInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dGachaActivityDetailInfo.proto\x1a\x14GachaStageData.proto\"\x98\
    \x04\n\x17GachaActivityDetailInfo\x12B\n\x15gacha_stage_data_list\x18\
    \x08\x20\x03(\x0b2\x0f.GachaStageDataR\x12gachaStageDataList\x12M\n\rrob\
    ot_num_map\x18\x0e\x20\x03(\x0b2).GachaActivityDetailInfo.RobotNumMapEnt\
    ryR\x0brobotNumMap\x12m\n\x19have_reward_robot_num_map\x18\t\x20\x03(\
    \x0b23.GachaActivityDetailInfo.HaveRewardRobotNumMapEntryR\x15haveReward\
    RobotNumMap\x12\x20\n\x0bFOLDOMNGPJM\x18\n\x20\x01(\rR\x0bFOLDOMNGPJM\
    \x12-\n\x13have_get_robot_list\x18\x0f\x20\x03(\rR\x10haveGetRobotList\
    \x12\x20\n\x0bFGLFAJBJEEA\x18\x0c\x20\x01(\rR\x0bFGLFAJBJEEA\x1a>\n\x10R\
    obotNumMapEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\
    \x05value\x18\x02\x20\x01(\rR\x05value:\x028\x01\x1aH\n\x1aHaveRewardRob\
    otNumMapEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05v\
    alue\x18\x02\x20\x01(\rR\x05value:\x028\x01B\x1b\n\x19emu.grasscutter.ne\
    t.protob\x06proto3\
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
            deps.push(super::GachaStageData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GachaActivityDetailInfo::generated_message_descriptor_data());
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