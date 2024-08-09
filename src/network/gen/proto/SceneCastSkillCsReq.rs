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

//! Generated file from `SceneCastSkillCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SceneCastSkillCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneCastSkillCsReq {
    // message fields
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.caster_id)
    pub caster_id: u32,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.hit_target_entity_id_list)
    pub hit_target_entity_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.target_motion)
    pub target_motion: ::protobuf::MessageField<super::MotionInfo::MotionInfo>,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.attacked_group_id)
    pub attacked_group_id: u32,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.skill_index)
    pub skill_index: u32,
    // @@protoc_insertion_point(field:SceneCastSkillCsReq.assist_monster_wave_list)
    pub assist_monster_wave_list: ::std::vec::Vec<super::AssistMonsterWave::AssistMonsterWave>,
    // special fields
    // @@protoc_insertion_point(special_field:SceneCastSkillCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneCastSkillCsReq {
    fn default() -> &'a SceneCastSkillCsReq {
        <SceneCastSkillCsReq as ::protobuf::Message>::default_instance()
    }
}

impl SceneCastSkillCsReq {
    pub fn new() -> SceneCastSkillCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "caster_id",
            |m: &SceneCastSkillCsReq| { &m.caster_id },
            |m: &mut SceneCastSkillCsReq| { &mut m.caster_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "hit_target_entity_id_list",
            |m: &SceneCastSkillCsReq| { &m.hit_target_entity_id_list },
            |m: &mut SceneCastSkillCsReq| { &mut m.hit_target_entity_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::MotionInfo::MotionInfo>(
            "target_motion",
            |m: &SceneCastSkillCsReq| { &m.target_motion },
            |m: &mut SceneCastSkillCsReq| { &mut m.target_motion },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "attacked_group_id",
            |m: &SceneCastSkillCsReq| { &m.attacked_group_id },
            |m: &mut SceneCastSkillCsReq| { &mut m.attacked_group_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "skill_index",
            |m: &SceneCastSkillCsReq| { &m.skill_index },
            |m: &mut SceneCastSkillCsReq| { &mut m.skill_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "assist_monster_wave_list",
            |m: &SceneCastSkillCsReq| { &m.assist_monster_wave_list },
            |m: &mut SceneCastSkillCsReq| { &mut m.assist_monster_wave_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneCastSkillCsReq>(
            "SceneCastSkillCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneCastSkillCsReq {
    const NAME: &'static str = "SceneCastSkillCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.caster_id = is.read_uint32()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.hit_target_entity_id_list)?;
                },
                16 => {
                    self.hit_target_entity_id_list.push(is.read_uint32()?);
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.target_motion)?;
                },
                80 => {
                    self.attacked_group_id = is.read_uint32()?;
                },
                120 => {
                    self.skill_index = is.read_uint32()?;
                },
                34 => {
                    self.assist_monster_wave_list.push(is.read_message()?);
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
        if self.caster_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.caster_id);
        }
        for value in &self.hit_target_entity_id_list {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        if let Some(v) = self.target_motion.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.attacked_group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.attacked_group_id);
        }
        if self.skill_index != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.skill_index);
        }
        for value in &self.assist_monster_wave_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.caster_id != 0 {
            os.write_uint32(13, self.caster_id)?;
        }
        for v in &self.hit_target_entity_id_list {
            os.write_uint32(2, *v)?;
        };
        if let Some(v) = self.target_motion.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if self.attacked_group_id != 0 {
            os.write_uint32(10, self.attacked_group_id)?;
        }
        if self.skill_index != 0 {
            os.write_uint32(15, self.skill_index)?;
        }
        for v in &self.assist_monster_wave_list {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> SceneCastSkillCsReq {
        SceneCastSkillCsReq::new()
    }

    fn clear(&mut self) {
        self.caster_id = 0;
        self.hit_target_entity_id_list.clear();
        self.target_motion.clear();
        self.attacked_group_id = 0;
        self.skill_index = 0;
        self.assist_monster_wave_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneCastSkillCsReq {
        static instance: SceneCastSkillCsReq = SceneCastSkillCsReq {
            caster_id: 0,
            hit_target_entity_id_list: ::std::vec::Vec::new(),
            target_motion: ::protobuf::MessageField::none(),
            attacked_group_id: 0,
            skill_index: 0,
            assist_monster_wave_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneCastSkillCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneCastSkillCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneCastSkillCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneCastSkillCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19SceneCastSkillCsReq.proto\x1a\x10MotionInfo.proto\x1a\x17AssistMon\
    sterWave.proto\"\xb8\x02\n\x13SceneCastSkillCsReq\x12\x1b\n\tcaster_id\
    \x18\r\x20\x01(\rR\x08casterId\x128\n\x19hit_target_entity_id_list\x18\
    \x02\x20\x03(\rR\x15hitTargetEntityIdList\x120\n\rtarget_motion\x18\x0c\
    \x20\x01(\x0b2\x0b.MotionInfoR\x0ctargetMotion\x12*\n\x11attacked_group_\
    id\x18\n\x20\x01(\rR\x0fattackedGroupId\x12\x1f\n\x0bskill_index\x18\x0f\
    \x20\x01(\rR\nskillIndex\x12K\n\x18assist_monster_wave_list\x18\x04\x20\
    \x03(\x0b2\x12.AssistMonsterWaveR\x15assistMonsterWaveListB\x15\n\x13emu\
    .lunarcore.protob\x06proto3\
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
            deps.push(super::MotionInfo::file_descriptor().clone());
            deps.push(super::AssistMonsterWave::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneCastSkillCsReq::generated_message_descriptor_data());
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