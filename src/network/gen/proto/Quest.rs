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

//! Generated file from `Quest.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:Quest)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Quest {
    // message fields
    // @@protoc_insertion_point(field:Quest.quest_id)
    pub quest_id: u32,
    // @@protoc_insertion_point(field:Quest.state)
    pub state: u32,
    // @@protoc_insertion_point(field:Quest.start_time)
    pub start_time: u32,
    // @@protoc_insertion_point(field:Quest.is_random)
    pub is_random: bool,
    // @@protoc_insertion_point(field:Quest.parent_quest_id)
    pub parent_quest_id: u32,
    // @@protoc_insertion_point(field:Quest.quest_config_id)
    pub quest_config_id: u32,
    // @@protoc_insertion_point(field:Quest.start_game_time)
    pub start_game_time: u32,
    // @@protoc_insertion_point(field:Quest.accept_time)
    pub accept_time: u32,
    // @@protoc_insertion_point(field:Quest.finish_progress_list)
    pub finish_progress_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:Quest.fail_progress_list)
    pub fail_progress_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:Quest.FMILFAINACE)
    pub FMILFAINACE: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:Quest.BCOAJLAHNNM)
    pub BCOAJLAHNNM: ::protobuf::MessageField<super::LackingResourceInfo::LackingResourceInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:Quest.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Quest {
    fn default() -> &'a Quest {
        <Quest as ::protobuf::Message>::default_instance()
    }
}

impl Quest {
    pub fn new() -> Quest {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(12);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "quest_id",
            |m: &Quest| { &m.quest_id },
            |m: &mut Quest| { &mut m.quest_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "state",
            |m: &Quest| { &m.state },
            |m: &mut Quest| { &mut m.state },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "start_time",
            |m: &Quest| { &m.start_time },
            |m: &mut Quest| { &mut m.start_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_random",
            |m: &Quest| { &m.is_random },
            |m: &mut Quest| { &mut m.is_random },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "parent_quest_id",
            |m: &Quest| { &m.parent_quest_id },
            |m: &mut Quest| { &mut m.parent_quest_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "quest_config_id",
            |m: &Quest| { &m.quest_config_id },
            |m: &mut Quest| { &mut m.quest_config_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "start_game_time",
            |m: &Quest| { &m.start_game_time },
            |m: &mut Quest| { &mut m.start_game_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "accept_time",
            |m: &Quest| { &m.accept_time },
            |m: &mut Quest| { &mut m.accept_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "finish_progress_list",
            |m: &Quest| { &m.finish_progress_list },
            |m: &mut Quest| { &mut m.finish_progress_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "fail_progress_list",
            |m: &Quest| { &m.fail_progress_list },
            |m: &mut Quest| { &mut m.fail_progress_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FMILFAINACE",
            |m: &Quest| { &m.FMILFAINACE },
            |m: &mut Quest| { &mut m.FMILFAINACE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LackingResourceInfo::LackingResourceInfo>(
            "BCOAJLAHNNM",
            |m: &Quest| { &m.BCOAJLAHNNM },
            |m: &mut Quest| { &mut m.BCOAJLAHNNM },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Quest>(
            "Quest",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Quest {
    const NAME: &'static str = "Quest";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.quest_id = is.read_uint32()?;
                },
                16 => {
                    self.state = is.read_uint32()?;
                },
                32 => {
                    self.start_time = is.read_uint32()?;
                },
                40 => {
                    self.is_random = is.read_bool()?;
                },
                48 => {
                    self.parent_quest_id = is.read_uint32()?;
                },
                56 => {
                    self.quest_config_id = is.read_uint32()?;
                },
                64 => {
                    self.start_game_time = is.read_uint32()?;
                },
                72 => {
                    self.accept_time = is.read_uint32()?;
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.finish_progress_list)?;
                },
                80 => {
                    self.finish_progress_list.push(is.read_uint32()?);
                },
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.fail_progress_list)?;
                },
                88 => {
                    self.fail_progress_list.push(is.read_uint32()?);
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.FMILFAINACE)?;
                },
                96 => {
                    self.FMILFAINACE.push(is.read_uint32()?);
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BCOAJLAHNNM)?;
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
        if self.quest_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.quest_id);
        }
        if self.state != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.state);
        }
        if self.start_time != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.start_time);
        }
        if self.is_random != false {
            my_size += 1 + 1;
        }
        if self.parent_quest_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.parent_quest_id);
        }
        if self.quest_config_id != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.quest_config_id);
        }
        if self.start_game_time != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.start_game_time);
        }
        if self.accept_time != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.accept_time);
        }
        for value in &self.finish_progress_list {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        for value in &self.fail_progress_list {
            my_size += ::protobuf::rt::uint32_size(11, *value);
        };
        for value in &self.FMILFAINACE {
            my_size += ::protobuf::rt::uint32_size(12, *value);
        };
        if let Some(v) = self.BCOAJLAHNNM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.quest_id != 0 {
            os.write_uint32(1, self.quest_id)?;
        }
        if self.state != 0 {
            os.write_uint32(2, self.state)?;
        }
        if self.start_time != 0 {
            os.write_uint32(4, self.start_time)?;
        }
        if self.is_random != false {
            os.write_bool(5, self.is_random)?;
        }
        if self.parent_quest_id != 0 {
            os.write_uint32(6, self.parent_quest_id)?;
        }
        if self.quest_config_id != 0 {
            os.write_uint32(7, self.quest_config_id)?;
        }
        if self.start_game_time != 0 {
            os.write_uint32(8, self.start_game_time)?;
        }
        if self.accept_time != 0 {
            os.write_uint32(9, self.accept_time)?;
        }
        for v in &self.finish_progress_list {
            os.write_uint32(10, *v)?;
        };
        for v in &self.fail_progress_list {
            os.write_uint32(11, *v)?;
        };
        for v in &self.FMILFAINACE {
            os.write_uint32(12, *v)?;
        };
        if let Some(v) = self.BCOAJLAHNNM.as_ref() {
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

    fn new() -> Quest {
        Quest::new()
    }

    fn clear(&mut self) {
        self.quest_id = 0;
        self.state = 0;
        self.start_time = 0;
        self.is_random = false;
        self.parent_quest_id = 0;
        self.quest_config_id = 0;
        self.start_game_time = 0;
        self.accept_time = 0;
        self.finish_progress_list.clear();
        self.fail_progress_list.clear();
        self.FMILFAINACE.clear();
        self.BCOAJLAHNNM.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Quest {
        static instance: Quest = Quest {
            quest_id: 0,
            state: 0,
            start_time: 0,
            is_random: false,
            parent_quest_id: 0,
            quest_config_id: 0,
            start_game_time: 0,
            accept_time: 0,
            finish_progress_list: ::std::vec::Vec::new(),
            fail_progress_list: ::std::vec::Vec::new(),
            FMILFAINACE: ::std::vec::Vec::new(),
            BCOAJLAHNNM: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Quest {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Quest").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Quest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Quest {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0bQuest.proto\x1a\x19LackingResourceInfo.proto\"\xc7\x03\n\x05Quest\
    \x12\x19\n\x08quest_id\x18\x01\x20\x01(\rR\x07questId\x12\x14\n\x05state\
    \x18\x02\x20\x01(\rR\x05state\x12\x1d\n\nstart_time\x18\x04\x20\x01(\rR\
    \tstartTime\x12\x1b\n\tis_random\x18\x05\x20\x01(\x08R\x08isRandom\x12&\
    \n\x0fparent_quest_id\x18\x06\x20\x01(\rR\rparentQuestId\x12&\n\x0fquest\
    _config_id\x18\x07\x20\x01(\rR\rquestConfigId\x12&\n\x0fstart_game_time\
    \x18\x08\x20\x01(\rR\rstartGameTime\x12\x1f\n\x0baccept_time\x18\t\x20\
    \x01(\rR\nacceptTime\x120\n\x14finish_progress_list\x18\n\x20\x03(\rR\
    \x12finishProgressList\x12,\n\x12fail_progress_list\x18\x0b\x20\x03(\rR\
    \x10failProgressList\x12\x20\n\x0bFMILFAINACE\x18\x0c\x20\x03(\rR\x0bFMI\
    LFAINACE\x126\n\x0bBCOAJLAHNNM\x18\r\x20\x01(\x0b2\x14.LackingResourceIn\
    foR\x0bBCOAJLAHNNMB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            deps.push(super::LackingResourceInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Quest::generated_message_descriptor_data());
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
