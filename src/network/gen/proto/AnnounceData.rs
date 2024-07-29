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

//! Generated file from `AnnounceData.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AnnounceData)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AnnounceData {
    // message fields
    // @@protoc_insertion_point(field:AnnounceData.center_system_text)
    pub center_system_text: ::std::string::String,
    // @@protoc_insertion_point(field:AnnounceData.count_down_text)
    pub count_down_text: ::std::string::String,
    // @@protoc_insertion_point(field:AnnounceData.dungeon_confirm_text)
    pub dungeon_confirm_text: ::std::string::String,
    // @@protoc_insertion_point(field:AnnounceData.end_time)
    pub end_time: u32,
    // @@protoc_insertion_point(field:AnnounceData.count_down_frequency)
    pub count_down_frequency: u32,
    // @@protoc_insertion_point(field:AnnounceData.config_id)
    pub config_id: u32,
    // @@protoc_insertion_point(field:AnnounceData.begin_time)
    pub begin_time: u32,
    // @@protoc_insertion_point(field:AnnounceData.is_center_system_last_5_every_minutes)
    pub is_center_system_last_5_every_minutes: bool,
    // @@protoc_insertion_point(field:AnnounceData.center_system_frequency)
    pub center_system_frequency: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AnnounceData.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AnnounceData {
    fn default() -> &'a AnnounceData {
        <AnnounceData as ::protobuf::Message>::default_instance()
    }
}

impl AnnounceData {
    pub fn new() -> AnnounceData {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "center_system_text",
            |m: &AnnounceData| { &m.center_system_text },
            |m: &mut AnnounceData| { &mut m.center_system_text },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "count_down_text",
            |m: &AnnounceData| { &m.count_down_text },
            |m: &mut AnnounceData| { &mut m.count_down_text },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "dungeon_confirm_text",
            |m: &AnnounceData| { &m.dungeon_confirm_text },
            |m: &mut AnnounceData| { &mut m.dungeon_confirm_text },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "end_time",
            |m: &AnnounceData| { &m.end_time },
            |m: &mut AnnounceData| { &mut m.end_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "count_down_frequency",
            |m: &AnnounceData| { &m.count_down_frequency },
            |m: &mut AnnounceData| { &mut m.count_down_frequency },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "config_id",
            |m: &AnnounceData| { &m.config_id },
            |m: &mut AnnounceData| { &mut m.config_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "begin_time",
            |m: &AnnounceData| { &m.begin_time },
            |m: &mut AnnounceData| { &mut m.begin_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_center_system_last_5_every_minutes",
            |m: &AnnounceData| { &m.is_center_system_last_5_every_minutes },
            |m: &mut AnnounceData| { &mut m.is_center_system_last_5_every_minutes },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "center_system_frequency",
            |m: &AnnounceData| { &m.center_system_frequency },
            |m: &mut AnnounceData| { &mut m.center_system_frequency },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AnnounceData>(
            "AnnounceData",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AnnounceData {
    const NAME: &'static str = "AnnounceData";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    self.center_system_text = is.read_string()?;
                },
                26 => {
                    self.count_down_text = is.read_string()?;
                },
                122 => {
                    self.dungeon_confirm_text = is.read_string()?;
                },
                96 => {
                    self.end_time = is.read_uint32()?;
                },
                72 => {
                    self.count_down_frequency = is.read_uint32()?;
                },
                104 => {
                    self.config_id = is.read_uint32()?;
                },
                16 => {
                    self.begin_time = is.read_uint32()?;
                },
                48 => {
                    self.is_center_system_last_5_every_minutes = is.read_bool()?;
                },
                8 => {
                    self.center_system_frequency = is.read_uint32()?;
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
        if !self.center_system_text.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.center_system_text);
        }
        if !self.count_down_text.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.count_down_text);
        }
        if !self.dungeon_confirm_text.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.dungeon_confirm_text);
        }
        if self.end_time != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.end_time);
        }
        if self.count_down_frequency != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.count_down_frequency);
        }
        if self.config_id != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.config_id);
        }
        if self.begin_time != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.begin_time);
        }
        if self.is_center_system_last_5_every_minutes != false {
            my_size += 1 + 1;
        }
        if self.center_system_frequency != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.center_system_frequency);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.center_system_text.is_empty() {
            os.write_string(5, &self.center_system_text)?;
        }
        if !self.count_down_text.is_empty() {
            os.write_string(3, &self.count_down_text)?;
        }
        if !self.dungeon_confirm_text.is_empty() {
            os.write_string(15, &self.dungeon_confirm_text)?;
        }
        if self.end_time != 0 {
            os.write_uint32(12, self.end_time)?;
        }
        if self.count_down_frequency != 0 {
            os.write_uint32(9, self.count_down_frequency)?;
        }
        if self.config_id != 0 {
            os.write_uint32(13, self.config_id)?;
        }
        if self.begin_time != 0 {
            os.write_uint32(2, self.begin_time)?;
        }
        if self.is_center_system_last_5_every_minutes != false {
            os.write_bool(6, self.is_center_system_last_5_every_minutes)?;
        }
        if self.center_system_frequency != 0 {
            os.write_uint32(1, self.center_system_frequency)?;
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

    fn new() -> AnnounceData {
        AnnounceData::new()
    }

    fn clear(&mut self) {
        self.center_system_text.clear();
        self.count_down_text.clear();
        self.dungeon_confirm_text.clear();
        self.end_time = 0;
        self.count_down_frequency = 0;
        self.config_id = 0;
        self.begin_time = 0;
        self.is_center_system_last_5_every_minutes = false;
        self.center_system_frequency = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AnnounceData {
        static instance: AnnounceData = AnnounceData {
            center_system_text: ::std::string::String::new(),
            count_down_text: ::std::string::String::new(),
            dungeon_confirm_text: ::std::string::String::new(),
            end_time: 0,
            count_down_frequency: 0,
            config_id: 0,
            begin_time: 0,
            is_center_system_last_5_every_minutes: false,
            center_system_frequency: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AnnounceData {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AnnounceData").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AnnounceData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AnnounceData {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12AnnounceData.proto\"\xa7\x03\n\x0cAnnounceData\x12,\n\x12center_sy\
    stem_text\x18\x05\x20\x01(\tR\x10centerSystemText\x12&\n\x0fcount_down_t\
    ext\x18\x03\x20\x01(\tR\rcountDownText\x120\n\x14dungeon_confirm_text\
    \x18\x0f\x20\x01(\tR\x12dungeonConfirmText\x12\x19\n\x08end_time\x18\x0c\
    \x20\x01(\rR\x07endTime\x120\n\x14count_down_frequency\x18\t\x20\x01(\rR\
    \x12countDownFrequency\x12\x1b\n\tconfig_id\x18\r\x20\x01(\rR\x08configI\
    d\x12\x1d\n\nbegin_time\x18\x02\x20\x01(\rR\tbeginTime\x12N\n%is_center_\
    system_last_5_every_minutes\x18\x06\x20\x01(\x08R\x1fisCenterSystemLast5\
    EveryMinutes\x126\n\x17center_system_frequency\x18\x01\x20\x01(\rR\x15ce\
    nterSystemFrequencyB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(AnnounceData::generated_message_descriptor_data());
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
