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

//! Generated file from `GetAuthkeyRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetAuthkeyRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetAuthkeyRsp {
    // message fields
    // @@protoc_insertion_point(field:GetAuthkeyRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:GetAuthkeyRsp.auth_appid)
    pub auth_appid: ::std::string::String,
    // @@protoc_insertion_point(field:GetAuthkeyRsp.game_biz)
    pub game_biz: ::std::string::String,
    // @@protoc_insertion_point(field:GetAuthkeyRsp.sign_type)
    pub sign_type: u32,
    // @@protoc_insertion_point(field:GetAuthkeyRsp.authkey_ver)
    pub authkey_ver: u32,
    // @@protoc_insertion_point(field:GetAuthkeyRsp.authkey)
    pub authkey: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:GetAuthkeyRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetAuthkeyRsp {
    fn default() -> &'a GetAuthkeyRsp {
        <GetAuthkeyRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetAuthkeyRsp {
    pub fn new() -> GetAuthkeyRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetAuthkeyRsp| { &m.retcode },
            |m: &mut GetAuthkeyRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "auth_appid",
            |m: &GetAuthkeyRsp| { &m.auth_appid },
            |m: &mut GetAuthkeyRsp| { &mut m.auth_appid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "game_biz",
            |m: &GetAuthkeyRsp| { &m.game_biz },
            |m: &mut GetAuthkeyRsp| { &mut m.game_biz },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sign_type",
            |m: &GetAuthkeyRsp| { &m.sign_type },
            |m: &mut GetAuthkeyRsp| { &mut m.sign_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "authkey_ver",
            |m: &GetAuthkeyRsp| { &m.authkey_ver },
            |m: &mut GetAuthkeyRsp| { &mut m.authkey_ver },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "authkey",
            |m: &GetAuthkeyRsp| { &m.authkey },
            |m: &mut GetAuthkeyRsp| { &mut m.authkey },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetAuthkeyRsp>(
            "GetAuthkeyRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetAuthkeyRsp {
    const NAME: &'static str = "GetAuthkeyRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.retcode = is.read_int32()?;
                },
                18 => {
                    self.auth_appid = is.read_string()?;
                },
                26 => {
                    self.game_biz = is.read_string()?;
                },
                32 => {
                    self.sign_type = is.read_uint32()?;
                },
                56 => {
                    self.authkey_ver = is.read_uint32()?;
                },
                106 => {
                    self.authkey = is.read_string()?;
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
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(1, self.retcode);
        }
        if !self.auth_appid.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.auth_appid);
        }
        if !self.game_biz.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.game_biz);
        }
        if self.sign_type != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.sign_type);
        }
        if self.authkey_ver != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.authkey_ver);
        }
        if !self.authkey.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.authkey);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.retcode != 0 {
            os.write_int32(1, self.retcode)?;
        }
        if !self.auth_appid.is_empty() {
            os.write_string(2, &self.auth_appid)?;
        }
        if !self.game_biz.is_empty() {
            os.write_string(3, &self.game_biz)?;
        }
        if self.sign_type != 0 {
            os.write_uint32(4, self.sign_type)?;
        }
        if self.authkey_ver != 0 {
            os.write_uint32(7, self.authkey_ver)?;
        }
        if !self.authkey.is_empty() {
            os.write_string(13, &self.authkey)?;
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

    fn new() -> GetAuthkeyRsp {
        GetAuthkeyRsp::new()
    }

    fn clear(&mut self) {
        self.retcode = 0;
        self.auth_appid.clear();
        self.game_biz.clear();
        self.sign_type = 0;
        self.authkey_ver = 0;
        self.authkey.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetAuthkeyRsp {
        static instance: GetAuthkeyRsp = GetAuthkeyRsp {
            retcode: 0,
            auth_appid: ::std::string::String::new(),
            game_biz: ::std::string::String::new(),
            sign_type: 0,
            authkey_ver: 0,
            authkey: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetAuthkeyRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetAuthkeyRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetAuthkeyRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetAuthkeyRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13GetAuthkeyRsp.proto\"\xbb\x01\n\rGetAuthkeyRsp\x12\x18\n\x07retcod\
    e\x18\x01\x20\x01(\x05R\x07retcode\x12\x1d\n\nauth_appid\x18\x02\x20\x01\
    (\tR\tauthAppid\x12\x19\n\x08game_biz\x18\x03\x20\x01(\tR\x07gameBiz\x12\
    \x1b\n\tsign_type\x18\x04\x20\x01(\rR\x08signType\x12\x1f\n\x0bauthkey_v\
    er\x18\x07\x20\x01(\rR\nauthkeyVer\x12\x18\n\x07authkey\x18\r\x20\x01(\t\
    R\x07authkeyB\x1b\n\x19emu.grasscutter.net.protob\x06proto3\
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
            messages.push(GetAuthkeyRsp::generated_message_descriptor_data());
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