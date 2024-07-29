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

//! Generated file from `RogueDiaryProgress.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueDiaryProgress)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueDiaryProgress {
    // message fields
    // @@protoc_insertion_point(field:RogueDiaryProgress.optional_card_list)
    pub optional_card_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:RogueDiaryProgress.cur_round)
    pub cur_round: u32,
    // @@protoc_insertion_point(field:RogueDiaryProgress.stage_id)
    pub stage_id: u32,
    // @@protoc_insertion_point(field:RogueDiaryProgress.is_enter_dungeon)
    pub is_enter_dungeon: bool,
    // @@protoc_insertion_point(field:RogueDiaryProgress.difficulty)
    pub difficulty: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RogueDiaryProgress.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueDiaryProgress {
    fn default() -> &'a RogueDiaryProgress {
        <RogueDiaryProgress as ::protobuf::Message>::default_instance()
    }
}

impl RogueDiaryProgress {
    pub fn new() -> RogueDiaryProgress {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "optional_card_list",
            |m: &RogueDiaryProgress| { &m.optional_card_list },
            |m: &mut RogueDiaryProgress| { &mut m.optional_card_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_round",
            |m: &RogueDiaryProgress| { &m.cur_round },
            |m: &mut RogueDiaryProgress| { &mut m.cur_round },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stage_id",
            |m: &RogueDiaryProgress| { &m.stage_id },
            |m: &mut RogueDiaryProgress| { &mut m.stage_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_enter_dungeon",
            |m: &RogueDiaryProgress| { &m.is_enter_dungeon },
            |m: &mut RogueDiaryProgress| { &mut m.is_enter_dungeon },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "difficulty",
            |m: &RogueDiaryProgress| { &m.difficulty },
            |m: &mut RogueDiaryProgress| { &mut m.difficulty },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueDiaryProgress>(
            "RogueDiaryProgress",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueDiaryProgress {
    const NAME: &'static str = "RogueDiaryProgress";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.optional_card_list)?;
                },
                40 => {
                    self.optional_card_list.push(is.read_uint32()?);
                },
                8 => {
                    self.cur_round = is.read_uint32()?;
                },
                24 => {
                    self.stage_id = is.read_uint32()?;
                },
                56 => {
                    self.is_enter_dungeon = is.read_bool()?;
                },
                16 => {
                    self.difficulty = is.read_uint32()?;
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
        for value in &self.optional_card_list {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        if self.cur_round != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.cur_round);
        }
        if self.stage_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.stage_id);
        }
        if self.is_enter_dungeon != false {
            my_size += 1 + 1;
        }
        if self.difficulty != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.difficulty);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.optional_card_list {
            os.write_uint32(5, *v)?;
        };
        if self.cur_round != 0 {
            os.write_uint32(1, self.cur_round)?;
        }
        if self.stage_id != 0 {
            os.write_uint32(3, self.stage_id)?;
        }
        if self.is_enter_dungeon != false {
            os.write_bool(7, self.is_enter_dungeon)?;
        }
        if self.difficulty != 0 {
            os.write_uint32(2, self.difficulty)?;
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

    fn new() -> RogueDiaryProgress {
        RogueDiaryProgress::new()
    }

    fn clear(&mut self) {
        self.optional_card_list.clear();
        self.cur_round = 0;
        self.stage_id = 0;
        self.is_enter_dungeon = false;
        self.difficulty = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueDiaryProgress {
        static instance: RogueDiaryProgress = RogueDiaryProgress {
            optional_card_list: ::std::vec::Vec::new(),
            cur_round: 0,
            stage_id: 0,
            is_enter_dungeon: false,
            difficulty: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueDiaryProgress {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueDiaryProgress").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueDiaryProgress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueDiaryProgress {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18RogueDiaryProgress.proto\"\xc4\x01\n\x12RogueDiaryProgress\x12,\n\
    \x12optional_card_list\x18\x05\x20\x03(\rR\x10optionalCardList\x12\x1b\n\
    \tcur_round\x18\x01\x20\x01(\rR\x08curRound\x12\x19\n\x08stage_id\x18\
    \x03\x20\x01(\rR\x07stageId\x12(\n\x10is_enter_dungeon\x18\x07\x20\x01(\
    \x08R\x0eisEnterDungeon\x12\x1e\n\ndifficulty\x18\x02\x20\x01(\rR\ndiffi\
    cultyB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueDiaryProgress::generated_message_descriptor_data());
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