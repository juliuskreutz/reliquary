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

//! Generated file from `RogueStageInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueStageInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueStageInfo {
    // message fields
    // @@protoc_insertion_point(field:RogueStageInfo.LJKJLAHOHKL)
    pub LJKJLAHOHKL: u32,
    // @@protoc_insertion_point(field:RogueStageInfo.is_open)
    pub is_open: bool,
    // @@protoc_insertion_point(field:RogueStageInfo.is_passed)
    pub is_passed: bool,
    // @@protoc_insertion_point(field:RogueStageInfo.stage_id)
    pub stage_id: u32,
    // @@protoc_insertion_point(field:RogueStageInfo.AGOJOHPCIGH)
    pub AGOJOHPCIGH: bool,
    // @@protoc_insertion_point(field:RogueStageInfo.EEKEPFIGAHK)
    pub EEKEPFIGAHK: u32,
    // @@protoc_insertion_point(field:RogueStageInfo.NMCOPOECMDN)
    pub NMCOPOECMDN: bool,
    // @@protoc_insertion_point(field:RogueStageInfo.ANIBHHPOKGM)
    pub ANIBHHPOKGM: u32,
    // @@protoc_insertion_point(field:RogueStageInfo.cur_level)
    pub cur_level: u32,
    // @@protoc_insertion_point(field:RogueStageInfo.is_taken_reward)
    pub is_taken_reward: bool,
    // @@protoc_insertion_point(field:RogueStageInfo.avatar_team)
    pub avatar_team: ::protobuf::MessageField<super::RogueShowAvatarTeamInfo::RogueShowAvatarTeamInfo>,
    // @@protoc_insertion_point(field:RogueStageInfo.FLFNNNPKPMI)
    pub FLFNNNPKPMI: u32,
    // @@protoc_insertion_point(field:RogueStageInfo.KIADBMBFJFK)
    pub KIADBMBFJFK: u32,
    // @@protoc_insertion_point(field:RogueStageInfo.rune_record_list)
    pub rune_record_list: ::std::vec::Vec<super::RoguelikeRuneRecord::RoguelikeRuneRecord>,
    // @@protoc_insertion_point(field:RogueStageInfo.JMMCGHNAINB)
    pub JMMCGHNAINB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RogueStageInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueStageInfo {
    fn default() -> &'a RogueStageInfo {
        <RogueStageInfo as ::protobuf::Message>::default_instance()
    }
}

impl RogueStageInfo {
    pub fn new() -> RogueStageInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(15);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LJKJLAHOHKL",
            |m: &RogueStageInfo| { &m.LJKJLAHOHKL },
            |m: &mut RogueStageInfo| { &mut m.LJKJLAHOHKL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_open",
            |m: &RogueStageInfo| { &m.is_open },
            |m: &mut RogueStageInfo| { &mut m.is_open },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_passed",
            |m: &RogueStageInfo| { &m.is_passed },
            |m: &mut RogueStageInfo| { &mut m.is_passed },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stage_id",
            |m: &RogueStageInfo| { &m.stage_id },
            |m: &mut RogueStageInfo| { &mut m.stage_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AGOJOHPCIGH",
            |m: &RogueStageInfo| { &m.AGOJOHPCIGH },
            |m: &mut RogueStageInfo| { &mut m.AGOJOHPCIGH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EEKEPFIGAHK",
            |m: &RogueStageInfo| { &m.EEKEPFIGAHK },
            |m: &mut RogueStageInfo| { &mut m.EEKEPFIGAHK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NMCOPOECMDN",
            |m: &RogueStageInfo| { &m.NMCOPOECMDN },
            |m: &mut RogueStageInfo| { &mut m.NMCOPOECMDN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ANIBHHPOKGM",
            |m: &RogueStageInfo| { &m.ANIBHHPOKGM },
            |m: &mut RogueStageInfo| { &mut m.ANIBHHPOKGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_level",
            |m: &RogueStageInfo| { &m.cur_level },
            |m: &mut RogueStageInfo| { &mut m.cur_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_taken_reward",
            |m: &RogueStageInfo| { &m.is_taken_reward },
            |m: &mut RogueStageInfo| { &mut m.is_taken_reward },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::RogueShowAvatarTeamInfo::RogueShowAvatarTeamInfo>(
            "avatar_team",
            |m: &RogueStageInfo| { &m.avatar_team },
            |m: &mut RogueStageInfo| { &mut m.avatar_team },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FLFNNNPKPMI",
            |m: &RogueStageInfo| { &m.FLFNNNPKPMI },
            |m: &mut RogueStageInfo| { &mut m.FLFNNNPKPMI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KIADBMBFJFK",
            |m: &RogueStageInfo| { &m.KIADBMBFJFK },
            |m: &mut RogueStageInfo| { &mut m.KIADBMBFJFK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "rune_record_list",
            |m: &RogueStageInfo| { &m.rune_record_list },
            |m: &mut RogueStageInfo| { &mut m.rune_record_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JMMCGHNAINB",
            |m: &RogueStageInfo| { &m.JMMCGHNAINB },
            |m: &mut RogueStageInfo| { &mut m.JMMCGHNAINB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueStageInfo>(
            "RogueStageInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueStageInfo {
    const NAME: &'static str = "RogueStageInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                56 => {
                    self.LJKJLAHOHKL = is.read_uint32()?;
                },
                112 => {
                    self.is_open = is.read_bool()?;
                },
                64 => {
                    self.is_passed = is.read_bool()?;
                },
                40 => {
                    self.stage_id = is.read_uint32()?;
                },
                24 => {
                    self.AGOJOHPCIGH = is.read_bool()?;
                },
                32 => {
                    self.EEKEPFIGAHK = is.read_uint32()?;
                },
                120 => {
                    self.NMCOPOECMDN = is.read_bool()?;
                },
                4072 => {
                    self.ANIBHHPOKGM = is.read_uint32()?;
                },
                8 => {
                    self.cur_level = is.read_uint32()?;
                },
                80 => {
                    self.is_taken_reward = is.read_bool()?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.avatar_team)?;
                },
                15032 => {
                    self.FLFNNNPKPMI = is.read_uint32()?;
                },
                48 => {
                    self.KIADBMBFJFK = is.read_uint32()?;
                },
                74 => {
                    self.rune_record_list.push(is.read_message()?);
                },
                104 => {
                    self.JMMCGHNAINB = is.read_uint32()?;
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
        if self.LJKJLAHOHKL != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LJKJLAHOHKL);
        }
        if self.is_open != false {
            my_size += 1 + 1;
        }
        if self.is_passed != false {
            my_size += 1 + 1;
        }
        if self.stage_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.stage_id);
        }
        if self.AGOJOHPCIGH != false {
            my_size += 1 + 1;
        }
        if self.EEKEPFIGAHK != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.EEKEPFIGAHK);
        }
        if self.NMCOPOECMDN != false {
            my_size += 1 + 1;
        }
        if self.ANIBHHPOKGM != 0 {
            my_size += ::protobuf::rt::uint32_size(509, self.ANIBHHPOKGM);
        }
        if self.cur_level != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.cur_level);
        }
        if self.is_taken_reward != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.avatar_team.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.FLFNNNPKPMI != 0 {
            my_size += ::protobuf::rt::uint32_size(1879, self.FLFNNNPKPMI);
        }
        if self.KIADBMBFJFK != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.KIADBMBFJFK);
        }
        for value in &self.rune_record_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.JMMCGHNAINB != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.JMMCGHNAINB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LJKJLAHOHKL != 0 {
            os.write_uint32(7, self.LJKJLAHOHKL)?;
        }
        if self.is_open != false {
            os.write_bool(14, self.is_open)?;
        }
        if self.is_passed != false {
            os.write_bool(8, self.is_passed)?;
        }
        if self.stage_id != 0 {
            os.write_uint32(5, self.stage_id)?;
        }
        if self.AGOJOHPCIGH != false {
            os.write_bool(3, self.AGOJOHPCIGH)?;
        }
        if self.EEKEPFIGAHK != 0 {
            os.write_uint32(4, self.EEKEPFIGAHK)?;
        }
        if self.NMCOPOECMDN != false {
            os.write_bool(15, self.NMCOPOECMDN)?;
        }
        if self.ANIBHHPOKGM != 0 {
            os.write_uint32(509, self.ANIBHHPOKGM)?;
        }
        if self.cur_level != 0 {
            os.write_uint32(1, self.cur_level)?;
        }
        if self.is_taken_reward != false {
            os.write_bool(10, self.is_taken_reward)?;
        }
        if let Some(v) = self.avatar_team.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if self.FLFNNNPKPMI != 0 {
            os.write_uint32(1879, self.FLFNNNPKPMI)?;
        }
        if self.KIADBMBFJFK != 0 {
            os.write_uint32(6, self.KIADBMBFJFK)?;
        }
        for v in &self.rune_record_list {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.JMMCGHNAINB != 0 {
            os.write_uint32(13, self.JMMCGHNAINB)?;
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

    fn new() -> RogueStageInfo {
        RogueStageInfo::new()
    }

    fn clear(&mut self) {
        self.LJKJLAHOHKL = 0;
        self.is_open = false;
        self.is_passed = false;
        self.stage_id = 0;
        self.AGOJOHPCIGH = false;
        self.EEKEPFIGAHK = 0;
        self.NMCOPOECMDN = false;
        self.ANIBHHPOKGM = 0;
        self.cur_level = 0;
        self.is_taken_reward = false;
        self.avatar_team.clear();
        self.FLFNNNPKPMI = 0;
        self.KIADBMBFJFK = 0;
        self.rune_record_list.clear();
        self.JMMCGHNAINB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueStageInfo {
        static instance: RogueStageInfo = RogueStageInfo {
            LJKJLAHOHKL: 0,
            is_open: false,
            is_passed: false,
            stage_id: 0,
            AGOJOHPCIGH: false,
            EEKEPFIGAHK: 0,
            NMCOPOECMDN: false,
            ANIBHHPOKGM: 0,
            cur_level: 0,
            is_taken_reward: false,
            avatar_team: ::protobuf::MessageField::none(),
            FLFNNNPKPMI: 0,
            KIADBMBFJFK: 0,
            rune_record_list: ::std::vec::Vec::new(),
            JMMCGHNAINB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueStageInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueStageInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueStageInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueStageInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14RogueStageInfo.proto\x1a\x1dRogueShowAvatarTeamInfo.proto\x1a\x19R\
    oguelikeRuneRecord.proto\"\xb3\x04\n\x0eRogueStageInfo\x12\x20\n\x0bLJKJ\
    LAHOHKL\x18\x07\x20\x01(\rR\x0bLJKJLAHOHKL\x12\x17\n\x07is_open\x18\x0e\
    \x20\x01(\x08R\x06isOpen\x12\x1b\n\tis_passed\x18\x08\x20\x01(\x08R\x08i\
    sPassed\x12\x19\n\x08stage_id\x18\x05\x20\x01(\rR\x07stageId\x12\x20\n\
    \x0bAGOJOHPCIGH\x18\x03\x20\x01(\x08R\x0bAGOJOHPCIGH\x12\x20\n\x0bEEKEPF\
    IGAHK\x18\x04\x20\x01(\rR\x0bEEKEPFIGAHK\x12\x20\n\x0bNMCOPOECMDN\x18\
    \x0f\x20\x01(\x08R\x0bNMCOPOECMDN\x12!\n\x0bANIBHHPOKGM\x18\xfd\x03\x20\
    \x01(\rR\x0bANIBHHPOKGM\x12\x1b\n\tcur_level\x18\x01\x20\x01(\rR\x08curL\
    evel\x12&\n\x0fis_taken_reward\x18\n\x20\x01(\x08R\risTakenReward\x129\n\
    \x0bavatar_team\x18\x0c\x20\x01(\x0b2\x18.RogueShowAvatarTeamInfoR\navat\
    arTeam\x12!\n\x0bFLFNNNPKPMI\x18\xd7\x0e\x20\x01(\rR\x0bFLFNNNPKPMI\x12\
    \x20\n\x0bKIADBMBFJFK\x18\x06\x20\x01(\rR\x0bKIADBMBFJFK\x12>\n\x10rune_\
    record_list\x18\t\x20\x03(\x0b2\x14.RoguelikeRuneRecordR\x0eruneRecordLi\
    st\x12\x20\n\x0bJMMCGHNAINB\x18\r\x20\x01(\rR\x0bJMMCGHNAINBB\x1b\n\x19e\
    mu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::RogueShowAvatarTeamInfo::file_descriptor().clone());
            deps.push(super::RoguelikeRuneRecord::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueStageInfo::generated_message_descriptor_data());
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