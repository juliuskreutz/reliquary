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

//! Generated file from `HachiStageInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:HachiStageInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HachiStageInfo {
    // message fields
    // @@protoc_insertion_point(field:HachiStageInfo.open_time)
    pub open_time: u32,
    // @@protoc_insertion_point(field:HachiStageInfo.is_open)
    pub is_open: bool,
    // @@protoc_insertion_point(field:HachiStageInfo.stage_id)
    pub stage_id: u32,
    // @@protoc_insertion_point(field:HachiStageInfo.is_finished)
    pub is_finished: bool,
    // special fields
    // @@protoc_insertion_point(special_field:HachiStageInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HachiStageInfo {
    fn default() -> &'a HachiStageInfo {
        <HachiStageInfo as ::protobuf::Message>::default_instance()
    }
}

impl HachiStageInfo {
    pub fn new() -> HachiStageInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "open_time",
            |m: &HachiStageInfo| { &m.open_time },
            |m: &mut HachiStageInfo| { &mut m.open_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_open",
            |m: &HachiStageInfo| { &m.is_open },
            |m: &mut HachiStageInfo| { &mut m.is_open },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stage_id",
            |m: &HachiStageInfo| { &m.stage_id },
            |m: &mut HachiStageInfo| { &mut m.stage_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_finished",
            |m: &HachiStageInfo| { &m.is_finished },
            |m: &mut HachiStageInfo| { &mut m.is_finished },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HachiStageInfo>(
            "HachiStageInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HachiStageInfo {
    const NAME: &'static str = "HachiStageInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.open_time = is.read_uint32()?;
                },
                48 => {
                    self.is_open = is.read_bool()?;
                },
                24 => {
                    self.stage_id = is.read_uint32()?;
                },
                112 => {
                    self.is_finished = is.read_bool()?;
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
        if self.open_time != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.open_time);
        }
        if self.is_open != false {
            my_size += 1 + 1;
        }
        if self.stage_id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.stage_id);
        }
        if self.is_finished != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.open_time != 0 {
            os.write_uint32(9, self.open_time)?;
        }
        if self.is_open != false {
            os.write_bool(6, self.is_open)?;
        }
        if self.stage_id != 0 {
            os.write_uint32(3, self.stage_id)?;
        }
        if self.is_finished != false {
            os.write_bool(14, self.is_finished)?;
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

    fn new() -> HachiStageInfo {
        HachiStageInfo::new()
    }

    fn clear(&mut self) {
        self.open_time = 0;
        self.is_open = false;
        self.stage_id = 0;
        self.is_finished = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HachiStageInfo {
        static instance: HachiStageInfo = HachiStageInfo {
            open_time: 0,
            is_open: false,
            stage_id: 0,
            is_finished: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HachiStageInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HachiStageInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HachiStageInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HachiStageInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14HachiStageInfo.proto\"\x82\x01\n\x0eHachiStageInfo\x12\x1b\n\topen\
    _time\x18\t\x20\x01(\rR\x08openTime\x12\x17\n\x07is_open\x18\x06\x20\x01\
    (\x08R\x06isOpen\x12\x19\n\x08stage_id\x18\x03\x20\x01(\rR\x07stageId\
    \x12\x1f\n\x0bis_finished\x18\x0e\x20\x01(\x08R\nisFinishedB\x1b\n\x19em\
    u.grasscutter.net.protob\x06proto3\
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
            messages.push(HachiStageInfo::generated_message_descriptor_data());
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
