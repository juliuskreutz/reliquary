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

//! Generated file from `DragonSpineChapterInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DragonSpineChapterInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DragonSpineChapterInfo {
    // message fields
    // @@protoc_insertion_point(field:DragonSpineChapterInfo.finished_mission_num)
    pub finished_mission_num: u32,
    // @@protoc_insertion_point(field:DragonSpineChapterInfo.progress)
    pub progress: u32,
    // @@protoc_insertion_point(field:DragonSpineChapterInfo.open_time)
    pub open_time: u32,
    // @@protoc_insertion_point(field:DragonSpineChapterInfo.is_open)
    pub is_open: bool,
    // @@protoc_insertion_point(field:DragonSpineChapterInfo.chapter_id)
    pub chapter_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:DragonSpineChapterInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DragonSpineChapterInfo {
    fn default() -> &'a DragonSpineChapterInfo {
        <DragonSpineChapterInfo as ::protobuf::Message>::default_instance()
    }
}

impl DragonSpineChapterInfo {
    pub fn new() -> DragonSpineChapterInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "finished_mission_num",
            |m: &DragonSpineChapterInfo| { &m.finished_mission_num },
            |m: &mut DragonSpineChapterInfo| { &mut m.finished_mission_num },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "progress",
            |m: &DragonSpineChapterInfo| { &m.progress },
            |m: &mut DragonSpineChapterInfo| { &mut m.progress },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "open_time",
            |m: &DragonSpineChapterInfo| { &m.open_time },
            |m: &mut DragonSpineChapterInfo| { &mut m.open_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_open",
            |m: &DragonSpineChapterInfo| { &m.is_open },
            |m: &mut DragonSpineChapterInfo| { &mut m.is_open },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "chapter_id",
            |m: &DragonSpineChapterInfo| { &m.chapter_id },
            |m: &mut DragonSpineChapterInfo| { &mut m.chapter_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DragonSpineChapterInfo>(
            "DragonSpineChapterInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DragonSpineChapterInfo {
    const NAME: &'static str = "DragonSpineChapterInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.finished_mission_num = is.read_uint32()?;
                },
                24 => {
                    self.progress = is.read_uint32()?;
                },
                40 => {
                    self.open_time = is.read_uint32()?;
                },
                96 => {
                    self.is_open = is.read_bool()?;
                },
                104 => {
                    self.chapter_id = is.read_uint32()?;
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
        if self.finished_mission_num != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.finished_mission_num);
        }
        if self.progress != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.progress);
        }
        if self.open_time != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.open_time);
        }
        if self.is_open != false {
            my_size += 1 + 1;
        }
        if self.chapter_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.chapter_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.finished_mission_num != 0 {
            os.write_uint32(10, self.finished_mission_num)?;
        }
        if self.progress != 0 {
            os.write_uint32(3, self.progress)?;
        }
        if self.open_time != 0 {
            os.write_uint32(5, self.open_time)?;
        }
        if self.is_open != false {
            os.write_bool(12, self.is_open)?;
        }
        if self.chapter_id != 0 {
            os.write_uint32(13, self.chapter_id)?;
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

    fn new() -> DragonSpineChapterInfo {
        DragonSpineChapterInfo::new()
    }

    fn clear(&mut self) {
        self.finished_mission_num = 0;
        self.progress = 0;
        self.open_time = 0;
        self.is_open = false;
        self.chapter_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DragonSpineChapterInfo {
        static instance: DragonSpineChapterInfo = DragonSpineChapterInfo {
            finished_mission_num: 0,
            progress: 0,
            open_time: 0,
            is_open: false,
            chapter_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DragonSpineChapterInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DragonSpineChapterInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DragonSpineChapterInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DragonSpineChapterInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cDragonSpineChapterInfo.proto\"\xbb\x01\n\x16DragonSpineChapterInfo\
    \x120\n\x14finished_mission_num\x18\n\x20\x01(\rR\x12finishedMissionNum\
    \x12\x1a\n\x08progress\x18\x03\x20\x01(\rR\x08progress\x12\x1b\n\topen_t\
    ime\x18\x05\x20\x01(\rR\x08openTime\x12\x17\n\x07is_open\x18\x0c\x20\x01\
    (\x08R\x06isOpen\x12\x1d\n\nchapter_id\x18\r\x20\x01(\rR\tchapterIdB\x1b\
    \n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(DragonSpineChapterInfo::generated_message_descriptor_data());
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
