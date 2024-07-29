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

//! Generated file from `SummerTimeV2DungeonStageInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SummerTimeV2DungeonStageInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SummerTimeV2DungeonStageInfo {
    // message fields
    // @@protoc_insertion_point(field:SummerTimeV2DungeonStageInfo.is_prev_dungeon_succeed)
    pub is_prev_dungeon_succeed: bool,
    // @@protoc_insertion_point(field:SummerTimeV2DungeonStageInfo.is_open)
    pub is_open: bool,
    // @@protoc_insertion_point(field:SummerTimeV2DungeonStageInfo.open_time)
    pub open_time: u32,
    // @@protoc_insertion_point(field:SummerTimeV2DungeonStageInfo.stage_id)
    pub stage_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SummerTimeV2DungeonStageInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SummerTimeV2DungeonStageInfo {
    fn default() -> &'a SummerTimeV2DungeonStageInfo {
        <SummerTimeV2DungeonStageInfo as ::protobuf::Message>::default_instance()
    }
}

impl SummerTimeV2DungeonStageInfo {
    pub fn new() -> SummerTimeV2DungeonStageInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_prev_dungeon_succeed",
            |m: &SummerTimeV2DungeonStageInfo| { &m.is_prev_dungeon_succeed },
            |m: &mut SummerTimeV2DungeonStageInfo| { &mut m.is_prev_dungeon_succeed },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_open",
            |m: &SummerTimeV2DungeonStageInfo| { &m.is_open },
            |m: &mut SummerTimeV2DungeonStageInfo| { &mut m.is_open },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "open_time",
            |m: &SummerTimeV2DungeonStageInfo| { &m.open_time },
            |m: &mut SummerTimeV2DungeonStageInfo| { &mut m.open_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stage_id",
            |m: &SummerTimeV2DungeonStageInfo| { &m.stage_id },
            |m: &mut SummerTimeV2DungeonStageInfo| { &mut m.stage_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SummerTimeV2DungeonStageInfo>(
            "SummerTimeV2DungeonStageInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SummerTimeV2DungeonStageInfo {
    const NAME: &'static str = "SummerTimeV2DungeonStageInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.is_prev_dungeon_succeed = is.read_bool()?;
                },
                8 => {
                    self.is_open = is.read_bool()?;
                },
                104 => {
                    self.open_time = is.read_uint32()?;
                },
                16 => {
                    self.stage_id = is.read_uint32()?;
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
        if self.is_prev_dungeon_succeed != false {
            my_size += 1 + 1;
        }
        if self.is_open != false {
            my_size += 1 + 1;
        }
        if self.open_time != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.open_time);
        }
        if self.stage_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.stage_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_prev_dungeon_succeed != false {
            os.write_bool(12, self.is_prev_dungeon_succeed)?;
        }
        if self.is_open != false {
            os.write_bool(1, self.is_open)?;
        }
        if self.open_time != 0 {
            os.write_uint32(13, self.open_time)?;
        }
        if self.stage_id != 0 {
            os.write_uint32(2, self.stage_id)?;
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

    fn new() -> SummerTimeV2DungeonStageInfo {
        SummerTimeV2DungeonStageInfo::new()
    }

    fn clear(&mut self) {
        self.is_prev_dungeon_succeed = false;
        self.is_open = false;
        self.open_time = 0;
        self.stage_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SummerTimeV2DungeonStageInfo {
        static instance: SummerTimeV2DungeonStageInfo = SummerTimeV2DungeonStageInfo {
            is_prev_dungeon_succeed: false,
            is_open: false,
            open_time: 0,
            stage_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SummerTimeV2DungeonStageInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SummerTimeV2DungeonStageInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SummerTimeV2DungeonStageInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SummerTimeV2DungeonStageInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"SummerTimeV2DungeonStageInfo.proto\"\xa6\x01\n\x1cSummerTimeV2Dungeo\
    nStageInfo\x125\n\x17is_prev_dungeon_succeed\x18\x0c\x20\x01(\x08R\x14is\
    PrevDungeonSucceed\x12\x17\n\x07is_open\x18\x01\x20\x01(\x08R\x06isOpen\
    \x12\x1b\n\topen_time\x18\r\x20\x01(\rR\x08openTime\x12\x19\n\x08stage_i\
    d\x18\x02\x20\x01(\rR\x07stageIdB\x1b\n\x19emu.grasscutter.net.protob\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SummerTimeV2DungeonStageInfo::generated_message_descriptor_data());
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
