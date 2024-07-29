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

//! Generated file from `MichiaeMatsuriActivityDetailInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MichiaeMatsuriActivityDetailInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MichiaeMatsuriActivityDetailInfo {
    // message fields
    // @@protoc_insertion_point(field:MichiaeMatsuriActivityDetailInfo.gain_crystal_exp)
    pub gain_crystal_exp: u32,
    // @@protoc_insertion_point(field:MichiaeMatsuriActivityDetailInfo.unlocked_crystal_skill_list)
    pub unlocked_crystal_skill_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:MichiaeMatsuriActivityDetailInfo.chest_pos_list)
    pub chest_pos_list: ::std::vec::Vec<super::MichiaeMatsuriChestPositionInfo::MichiaeMatsuriChestPositionInfo>,
    // @@protoc_insertion_point(field:MichiaeMatsuriActivityDetailInfo.challenge_pos_list)
    pub challenge_pos_list: ::std::vec::Vec<super::MichiaeMatsuriChallengePositionInfo::MichiaeMatsuriChallengePositionInfo>,
    // @@protoc_insertion_point(field:MichiaeMatsuriActivityDetailInfo.stage_list)
    pub stage_list: ::std::vec::Vec<super::MichiaeMatsuriStage::MichiaeMatsuriStage>,
    // special fields
    // @@protoc_insertion_point(special_field:MichiaeMatsuriActivityDetailInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MichiaeMatsuriActivityDetailInfo {
    fn default() -> &'a MichiaeMatsuriActivityDetailInfo {
        <MichiaeMatsuriActivityDetailInfo as ::protobuf::Message>::default_instance()
    }
}

impl MichiaeMatsuriActivityDetailInfo {
    pub fn new() -> MichiaeMatsuriActivityDetailInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gain_crystal_exp",
            |m: &MichiaeMatsuriActivityDetailInfo| { &m.gain_crystal_exp },
            |m: &mut MichiaeMatsuriActivityDetailInfo| { &mut m.gain_crystal_exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unlocked_crystal_skill_list",
            |m: &MichiaeMatsuriActivityDetailInfo| { &m.unlocked_crystal_skill_list },
            |m: &mut MichiaeMatsuriActivityDetailInfo| { &mut m.unlocked_crystal_skill_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "chest_pos_list",
            |m: &MichiaeMatsuriActivityDetailInfo| { &m.chest_pos_list },
            |m: &mut MichiaeMatsuriActivityDetailInfo| { &mut m.chest_pos_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "challenge_pos_list",
            |m: &MichiaeMatsuriActivityDetailInfo| { &m.challenge_pos_list },
            |m: &mut MichiaeMatsuriActivityDetailInfo| { &mut m.challenge_pos_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "stage_list",
            |m: &MichiaeMatsuriActivityDetailInfo| { &m.stage_list },
            |m: &mut MichiaeMatsuriActivityDetailInfo| { &mut m.stage_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MichiaeMatsuriActivityDetailInfo>(
            "MichiaeMatsuriActivityDetailInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MichiaeMatsuriActivityDetailInfo {
    const NAME: &'static str = "MichiaeMatsuriActivityDetailInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.gain_crystal_exp = is.read_uint32()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.unlocked_crystal_skill_list)?;
                },
                16 => {
                    self.unlocked_crystal_skill_list.push(is.read_uint32()?);
                },
                74 => {
                    self.chest_pos_list.push(is.read_message()?);
                },
                90 => {
                    self.challenge_pos_list.push(is.read_message()?);
                },
                10 => {
                    self.stage_list.push(is.read_message()?);
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
        if self.gain_crystal_exp != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.gain_crystal_exp);
        }
        for value in &self.unlocked_crystal_skill_list {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        for value in &self.chest_pos_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.challenge_pos_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.stage_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.gain_crystal_exp != 0 {
            os.write_uint32(15, self.gain_crystal_exp)?;
        }
        for v in &self.unlocked_crystal_skill_list {
            os.write_uint32(2, *v)?;
        };
        for v in &self.chest_pos_list {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        for v in &self.challenge_pos_list {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        for v in &self.stage_list {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
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

    fn new() -> MichiaeMatsuriActivityDetailInfo {
        MichiaeMatsuriActivityDetailInfo::new()
    }

    fn clear(&mut self) {
        self.gain_crystal_exp = 0;
        self.unlocked_crystal_skill_list.clear();
        self.chest_pos_list.clear();
        self.challenge_pos_list.clear();
        self.stage_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MichiaeMatsuriActivityDetailInfo {
        static instance: MichiaeMatsuriActivityDetailInfo = MichiaeMatsuriActivityDetailInfo {
            gain_crystal_exp: 0,
            unlocked_crystal_skill_list: ::std::vec::Vec::new(),
            chest_pos_list: ::std::vec::Vec::new(),
            challenge_pos_list: ::std::vec::Vec::new(),
            stage_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MichiaeMatsuriActivityDetailInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MichiaeMatsuriActivityDetailInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MichiaeMatsuriActivityDetailInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MichiaeMatsuriActivityDetailInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&MichiaeMatsuriActivityDetailInfo.proto\x1a%MichiaeMatsuriChestPositio\
    nInfo.proto\x1a)MichiaeMatsuriChallengePositionInfo.proto\x1a\x19Michiae\
    MatsuriStage.proto\"\xdc\x02\n\x20MichiaeMatsuriActivityDetailInfo\x12(\
    \n\x10gain_crystal_exp\x18\x0f\x20\x01(\rR\x0egainCrystalExp\x12=\n\x1bu\
    nlocked_crystal_skill_list\x18\x02\x20\x03(\rR\x18unlockedCrystalSkillLi\
    st\x12F\n\x0echest_pos_list\x18\t\x20\x03(\x0b2\x20.MichiaeMatsuriChestP\
    ositionInfoR\x0cchestPosList\x12R\n\x12challenge_pos_list\x18\x0b\x20\
    \x03(\x0b2$.MichiaeMatsuriChallengePositionInfoR\x10challengePosList\x12\
    3\n\nstage_list\x18\x01\x20\x03(\x0b2\x14.MichiaeMatsuriStageR\tstageLis\
    tB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::MichiaeMatsuriChestPositionInfo::file_descriptor().clone());
            deps.push(super::MichiaeMatsuriChallengePositionInfo::file_descriptor().clone());
            deps.push(super::MichiaeMatsuriStage::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MichiaeMatsuriActivityDetailInfo::generated_message_descriptor_data());
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
