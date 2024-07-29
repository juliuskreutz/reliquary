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

//! Generated file from `SeaLampActivityInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SeaLampActivityInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SeaLampActivityInfo {
    // message fields
    // @@protoc_insertion_point(field:SeaLampActivityInfo.section_info_list)
    pub section_info_list: ::std::vec::Vec<super::SeaLampSectionInfo::SeaLampSectionInfo>,
    // @@protoc_insertion_point(field:SeaLampActivityInfo.first_day_start_time)
    pub first_day_start_time: u32,
    // @@protoc_insertion_point(field:SeaLampActivityInfo.sea_lamp_coin)
    pub sea_lamp_coin: u32,
    // @@protoc_insertion_point(field:SeaLampActivityInfo.NAOOKLKKOBJ)
    pub NAOOKLKKOBJ: bool,
    // @@protoc_insertion_point(field:SeaLampActivityInfo.FFHKCOOGLCB)
    pub FFHKCOOGLCB: bool,
    // @@protoc_insertion_point(field:SeaLampActivityInfo.day_index)
    pub day_index: u32,
    // @@protoc_insertion_point(field:SeaLampActivityInfo.mechanicus_id)
    pub mechanicus_id: u32,
    // @@protoc_insertion_point(field:SeaLampActivityInfo.popularity)
    pub popularity: u32,
    // @@protoc_insertion_point(field:SeaLampActivityInfo.is_content_closed)
    pub is_content_closed: bool,
    // special fields
    // @@protoc_insertion_point(special_field:SeaLampActivityInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SeaLampActivityInfo {
    fn default() -> &'a SeaLampActivityInfo {
        <SeaLampActivityInfo as ::protobuf::Message>::default_instance()
    }
}

impl SeaLampActivityInfo {
    pub fn new() -> SeaLampActivityInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "section_info_list",
            |m: &SeaLampActivityInfo| { &m.section_info_list },
            |m: &mut SeaLampActivityInfo| { &mut m.section_info_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "first_day_start_time",
            |m: &SeaLampActivityInfo| { &m.first_day_start_time },
            |m: &mut SeaLampActivityInfo| { &mut m.first_day_start_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sea_lamp_coin",
            |m: &SeaLampActivityInfo| { &m.sea_lamp_coin },
            |m: &mut SeaLampActivityInfo| { &mut m.sea_lamp_coin },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NAOOKLKKOBJ",
            |m: &SeaLampActivityInfo| { &m.NAOOKLKKOBJ },
            |m: &mut SeaLampActivityInfo| { &mut m.NAOOKLKKOBJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FFHKCOOGLCB",
            |m: &SeaLampActivityInfo| { &m.FFHKCOOGLCB },
            |m: &mut SeaLampActivityInfo| { &mut m.FFHKCOOGLCB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "day_index",
            |m: &SeaLampActivityInfo| { &m.day_index },
            |m: &mut SeaLampActivityInfo| { &mut m.day_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "mechanicus_id",
            |m: &SeaLampActivityInfo| { &m.mechanicus_id },
            |m: &mut SeaLampActivityInfo| { &mut m.mechanicus_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "popularity",
            |m: &SeaLampActivityInfo| { &m.popularity },
            |m: &mut SeaLampActivityInfo| { &mut m.popularity },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_content_closed",
            |m: &SeaLampActivityInfo| { &m.is_content_closed },
            |m: &mut SeaLampActivityInfo| { &mut m.is_content_closed },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SeaLampActivityInfo>(
            "SeaLampActivityInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SeaLampActivityInfo {
    const NAME: &'static str = "SeaLampActivityInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    self.section_info_list.push(is.read_message()?);
                },
                112 => {
                    self.first_day_start_time = is.read_uint32()?;
                },
                120 => {
                    self.sea_lamp_coin = is.read_uint32()?;
                },
                16 => {
                    self.NAOOKLKKOBJ = is.read_bool()?;
                },
                40 => {
                    self.FFHKCOOGLCB = is.read_bool()?;
                },
                56 => {
                    self.day_index = is.read_uint32()?;
                },
                72 => {
                    self.mechanicus_id = is.read_uint32()?;
                },
                32 => {
                    self.popularity = is.read_uint32()?;
                },
                88 => {
                    self.is_content_closed = is.read_bool()?;
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
        for value in &self.section_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.first_day_start_time != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.first_day_start_time);
        }
        if self.sea_lamp_coin != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.sea_lamp_coin);
        }
        if self.NAOOKLKKOBJ != false {
            my_size += 1 + 1;
        }
        if self.FFHKCOOGLCB != false {
            my_size += 1 + 1;
        }
        if self.day_index != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.day_index);
        }
        if self.mechanicus_id != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.mechanicus_id);
        }
        if self.popularity != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.popularity);
        }
        if self.is_content_closed != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.section_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        if self.first_day_start_time != 0 {
            os.write_uint32(14, self.first_day_start_time)?;
        }
        if self.sea_lamp_coin != 0 {
            os.write_uint32(15, self.sea_lamp_coin)?;
        }
        if self.NAOOKLKKOBJ != false {
            os.write_bool(2, self.NAOOKLKKOBJ)?;
        }
        if self.FFHKCOOGLCB != false {
            os.write_bool(5, self.FFHKCOOGLCB)?;
        }
        if self.day_index != 0 {
            os.write_uint32(7, self.day_index)?;
        }
        if self.mechanicus_id != 0 {
            os.write_uint32(9, self.mechanicus_id)?;
        }
        if self.popularity != 0 {
            os.write_uint32(4, self.popularity)?;
        }
        if self.is_content_closed != false {
            os.write_bool(11, self.is_content_closed)?;
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

    fn new() -> SeaLampActivityInfo {
        SeaLampActivityInfo::new()
    }

    fn clear(&mut self) {
        self.section_info_list.clear();
        self.first_day_start_time = 0;
        self.sea_lamp_coin = 0;
        self.NAOOKLKKOBJ = false;
        self.FFHKCOOGLCB = false;
        self.day_index = 0;
        self.mechanicus_id = 0;
        self.popularity = 0;
        self.is_content_closed = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SeaLampActivityInfo {
        static instance: SeaLampActivityInfo = SeaLampActivityInfo {
            section_info_list: ::std::vec::Vec::new(),
            first_day_start_time: 0,
            sea_lamp_coin: 0,
            NAOOKLKKOBJ: false,
            FFHKCOOGLCB: false,
            day_index: 0,
            mechanicus_id: 0,
            popularity: 0,
            is_content_closed: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SeaLampActivityInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SeaLampActivityInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SeaLampActivityInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SeaLampActivityInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19SeaLampActivityInfo.proto\x1a\x18SeaLampSectionInfo.proto\"\xfd\
    \x02\n\x13SeaLampActivityInfo\x12?\n\x11section_info_list\x18\x08\x20\
    \x03(\x0b2\x13.SeaLampSectionInfoR\x0fsectionInfoList\x12/\n\x14first_da\
    y_start_time\x18\x0e\x20\x01(\rR\x11firstDayStartTime\x12\"\n\rsea_lamp_\
    coin\x18\x0f\x20\x01(\rR\x0bseaLampCoin\x12\x20\n\x0bNAOOKLKKOBJ\x18\x02\
    \x20\x01(\x08R\x0bNAOOKLKKOBJ\x12\x20\n\x0bFFHKCOOGLCB\x18\x05\x20\x01(\
    \x08R\x0bFFHKCOOGLCB\x12\x1b\n\tday_index\x18\x07\x20\x01(\rR\x08dayInde\
    x\x12#\n\rmechanicus_id\x18\t\x20\x01(\rR\x0cmechanicusId\x12\x1e\n\npop\
    ularity\x18\x04\x20\x01(\rR\npopularity\x12*\n\x11is_content_closed\x18\
    \x0b\x20\x01(\x08R\x0fisContentClosedB\x1b\n\x19emu.grasscutter.net.prot\
    ob\x06proto3\
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
            deps.push(super::SeaLampSectionInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SeaLampActivityInfo::generated_message_descriptor_data());
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