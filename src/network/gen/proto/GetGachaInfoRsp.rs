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

//! Generated file from `GetGachaInfoRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetGachaInfoRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetGachaInfoRsp {
    // message fields
    // @@protoc_insertion_point(field:GetGachaInfoRsp.gacha_random)
    pub gacha_random: u32,
    // @@protoc_insertion_point(field:GetGachaInfoRsp.is_under_minors_restrict)
    pub is_under_minors_restrict: bool,
    // @@protoc_insertion_point(field:GetGachaInfoRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:GetGachaInfoRsp.daily_gacha_times)
    pub daily_gacha_times: u32,
    // @@protoc_insertion_point(field:GetGachaInfoRsp.gacha_info_list)
    pub gacha_info_list: ::std::vec::Vec<super::GachaInfo::GachaInfo>,
    // @@protoc_insertion_point(field:GetGachaInfoRsp.is_under_general_restrict)
    pub is_under_general_restrict: bool,
    // special fields
    // @@protoc_insertion_point(special_field:GetGachaInfoRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetGachaInfoRsp {
    fn default() -> &'a GetGachaInfoRsp {
        <GetGachaInfoRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetGachaInfoRsp {
    pub fn new() -> GetGachaInfoRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gacha_random",
            |m: &GetGachaInfoRsp| { &m.gacha_random },
            |m: &mut GetGachaInfoRsp| { &mut m.gacha_random },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_under_minors_restrict",
            |m: &GetGachaInfoRsp| { &m.is_under_minors_restrict },
            |m: &mut GetGachaInfoRsp| { &mut m.is_under_minors_restrict },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetGachaInfoRsp| { &m.retcode },
            |m: &mut GetGachaInfoRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "daily_gacha_times",
            |m: &GetGachaInfoRsp| { &m.daily_gacha_times },
            |m: &mut GetGachaInfoRsp| { &mut m.daily_gacha_times },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "gacha_info_list",
            |m: &GetGachaInfoRsp| { &m.gacha_info_list },
            |m: &mut GetGachaInfoRsp| { &mut m.gacha_info_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_under_general_restrict",
            |m: &GetGachaInfoRsp| { &m.is_under_general_restrict },
            |m: &mut GetGachaInfoRsp| { &mut m.is_under_general_restrict },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetGachaInfoRsp>(
            "GetGachaInfoRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetGachaInfoRsp {
    const NAME: &'static str = "GetGachaInfoRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.gacha_random = is.read_uint32()?;
                },
                80 => {
                    self.is_under_minors_restrict = is.read_bool()?;
                },
                88 => {
                    self.retcode = is.read_int32()?;
                },
                96 => {
                    self.daily_gacha_times = is.read_uint32()?;
                },
                106 => {
                    self.gacha_info_list.push(is.read_message()?);
                },
                112 => {
                    self.is_under_general_restrict = is.read_bool()?;
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
        if self.gacha_random != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.gacha_random);
        }
        if self.is_under_minors_restrict != false {
            my_size += 1 + 1;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(11, self.retcode);
        }
        if self.daily_gacha_times != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.daily_gacha_times);
        }
        for value in &self.gacha_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.is_under_general_restrict != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.gacha_random != 0 {
            os.write_uint32(6, self.gacha_random)?;
        }
        if self.is_under_minors_restrict != false {
            os.write_bool(10, self.is_under_minors_restrict)?;
        }
        if self.retcode != 0 {
            os.write_int32(11, self.retcode)?;
        }
        if self.daily_gacha_times != 0 {
            os.write_uint32(12, self.daily_gacha_times)?;
        }
        for v in &self.gacha_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.is_under_general_restrict != false {
            os.write_bool(14, self.is_under_general_restrict)?;
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

    fn new() -> GetGachaInfoRsp {
        GetGachaInfoRsp::new()
    }

    fn clear(&mut self) {
        self.gacha_random = 0;
        self.is_under_minors_restrict = false;
        self.retcode = 0;
        self.daily_gacha_times = 0;
        self.gacha_info_list.clear();
        self.is_under_general_restrict = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetGachaInfoRsp {
        static instance: GetGachaInfoRsp = GetGachaInfoRsp {
            gacha_random: 0,
            is_under_minors_restrict: false,
            retcode: 0,
            daily_gacha_times: 0,
            gacha_info_list: ::std::vec::Vec::new(),
            is_under_general_restrict: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetGachaInfoRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetGachaInfoRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetGachaInfoRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetGachaInfoRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15GetGachaInfoRsp.proto\x1a\x0fGachaInfo.proto\"\xa2\x02\n\x0fGetGac\
    haInfoRsp\x12!\n\x0cgacha_random\x18\x06\x20\x01(\rR\x0bgachaRandom\x127\
    \n\x18is_under_minors_restrict\x18\n\x20\x01(\x08R\x15isUnderMinorsRestr\
    ict\x12\x18\n\x07retcode\x18\x0b\x20\x01(\x05R\x07retcode\x12*\n\x11dail\
    y_gacha_times\x18\x0c\x20\x01(\rR\x0fdailyGachaTimes\x122\n\x0fgacha_inf\
    o_list\x18\r\x20\x03(\x0b2\n.GachaInfoR\rgachaInfoList\x129\n\x19is_unde\
    r_general_restrict\x18\x0e\x20\x01(\x08R\x16isUnderGeneralRestrictB\x1b\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::GachaInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetGachaInfoRsp::generated_message_descriptor_data());
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