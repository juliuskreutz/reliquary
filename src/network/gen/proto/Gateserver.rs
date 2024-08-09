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

//! Generated file from `Gateserver.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:Gateserver)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Gateserver {
    // message fields
    // @@protoc_insertion_point(field:Gateserver.lua_url)
    pub lua_url: ::std::string::String,
    // @@protoc_insertion_point(field:Gateserver.ex_resource_url)
    pub ex_resource_url: ::std::string::String,
    // @@protoc_insertion_point(field:Gateserver.asset_bundle_url)
    pub asset_bundle_url: ::std::string::String,
    // @@protoc_insertion_point(field:Gateserver.ifix_url)
    pub ifix_url: ::std::string::String,
    // @@protoc_insertion_point(field:Gateserver.region_name)
    pub region_name: ::std::string::String,
    // @@protoc_insertion_point(field:Gateserver.ip)
    pub ip: ::std::string::String,
    // @@protoc_insertion_point(field:Gateserver.port)
    pub port: u32,
    // @@protoc_insertion_point(field:Gateserver.msg)
    pub msg: ::std::string::String,
    // @@protoc_insertion_point(field:Gateserver.ifix_version)
    pub ifix_version: ::std::string::String,
    // @@protoc_insertion_point(field:Gateserver.mdk_res_version)
    pub mdk_res_version: ::std::string::String,
    // @@protoc_insertion_point(field:Gateserver.client_secret_key)
    pub client_secret_key: ::std::string::String,
    // @@protoc_insertion_point(field:Gateserver.unk1)
    pub unk1: bool,
    // @@protoc_insertion_point(field:Gateserver.unk2)
    pub unk2: bool,
    // @@protoc_insertion_point(field:Gateserver.unk3)
    pub unk3: bool,
    // @@protoc_insertion_point(field:Gateserver.unk4)
    pub unk4: bool,
    // @@protoc_insertion_point(field:Gateserver.unk5)
    pub unk5: bool,
    // @@protoc_insertion_point(field:Gateserver.unk6)
    pub unk6: bool,
    // @@protoc_insertion_point(field:Gateserver.unk7)
    pub unk7: bool,
    // special fields
    // @@protoc_insertion_point(special_field:Gateserver.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Gateserver {
    fn default() -> &'a Gateserver {
        <Gateserver as ::protobuf::Message>::default_instance()
    }
}

impl Gateserver {
    pub fn new() -> Gateserver {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(18);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "lua_url",
            |m: &Gateserver| { &m.lua_url },
            |m: &mut Gateserver| { &mut m.lua_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ex_resource_url",
            |m: &Gateserver| { &m.ex_resource_url },
            |m: &mut Gateserver| { &mut m.ex_resource_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "asset_bundle_url",
            |m: &Gateserver| { &m.asset_bundle_url },
            |m: &mut Gateserver| { &mut m.asset_bundle_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ifix_url",
            |m: &Gateserver| { &m.ifix_url },
            |m: &mut Gateserver| { &mut m.ifix_url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "region_name",
            |m: &Gateserver| { &m.region_name },
            |m: &mut Gateserver| { &mut m.region_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ip",
            |m: &Gateserver| { &m.ip },
            |m: &mut Gateserver| { &mut m.ip },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "port",
            |m: &Gateserver| { &m.port },
            |m: &mut Gateserver| { &mut m.port },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "msg",
            |m: &Gateserver| { &m.msg },
            |m: &mut Gateserver| { &mut m.msg },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ifix_version",
            |m: &Gateserver| { &m.ifix_version },
            |m: &mut Gateserver| { &mut m.ifix_version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "mdk_res_version",
            |m: &Gateserver| { &m.mdk_res_version },
            |m: &mut Gateserver| { &mut m.mdk_res_version },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "client_secret_key",
            |m: &Gateserver| { &m.client_secret_key },
            |m: &mut Gateserver| { &mut m.client_secret_key },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unk1",
            |m: &Gateserver| { &m.unk1 },
            |m: &mut Gateserver| { &mut m.unk1 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unk2",
            |m: &Gateserver| { &m.unk2 },
            |m: &mut Gateserver| { &mut m.unk2 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unk3",
            |m: &Gateserver| { &m.unk3 },
            |m: &mut Gateserver| { &mut m.unk3 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unk4",
            |m: &Gateserver| { &m.unk4 },
            |m: &mut Gateserver| { &mut m.unk4 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unk5",
            |m: &Gateserver| { &m.unk5 },
            |m: &mut Gateserver| { &mut m.unk5 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unk6",
            |m: &Gateserver| { &m.unk6 },
            |m: &mut Gateserver| { &mut m.unk6 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unk7",
            |m: &Gateserver| { &m.unk7 },
            |m: &mut Gateserver| { &mut m.unk7 },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Gateserver>(
            "Gateserver",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Gateserver {
    const NAME: &'static str = "Gateserver";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                18 => {
                    self.lua_url = is.read_string()?;
                },
                114 => {
                    self.ex_resource_url = is.read_string()?;
                },
                122 => {
                    self.asset_bundle_url = is.read_string()?;
                },
                15162 => {
                    self.ifix_url = is.read_string()?;
                },
                82 => {
                    self.region_name = is.read_string()?;
                },
                10 => {
                    self.ip = is.read_string()?;
                },
                96 => {
                    self.port = is.read_uint32()?;
                },
                3066 => {
                    self.msg = is.read_string()?;
                },
                9674 => {
                    self.ifix_version = is.read_string()?;
                },
                14386 => {
                    self.mdk_res_version = is.read_string()?;
                },
                12778 => {
                    self.client_secret_key = is.read_string()?;
                },
                48 => {
                    self.unk1 = is.read_bool()?;
                },
                104 => {
                    self.unk2 = is.read_bool()?;
                },
                656 => {
                    self.unk3 = is.read_bool()?;
                },
                5896 => {
                    self.unk4 = is.read_bool()?;
                },
                8296 => {
                    self.unk5 = is.read_bool()?;
                },
                11208 => {
                    self.unk6 = is.read_bool()?;
                },
                14808 => {
                    self.unk7 = is.read_bool()?;
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
        if !self.lua_url.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.lua_url);
        }
        if !self.ex_resource_url.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.ex_resource_url);
        }
        if !self.asset_bundle_url.is_empty() {
            my_size += ::protobuf::rt::string_size(15, &self.asset_bundle_url);
        }
        if !self.ifix_url.is_empty() {
            my_size += ::protobuf::rt::string_size(1895, &self.ifix_url);
        }
        if !self.region_name.is_empty() {
            my_size += ::protobuf::rt::string_size(10, &self.region_name);
        }
        if !self.ip.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.ip);
        }
        if self.port != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.port);
        }
        if !self.msg.is_empty() {
            my_size += ::protobuf::rt::string_size(383, &self.msg);
        }
        if !self.ifix_version.is_empty() {
            my_size += ::protobuf::rt::string_size(1209, &self.ifix_version);
        }
        if !self.mdk_res_version.is_empty() {
            my_size += ::protobuf::rt::string_size(1798, &self.mdk_res_version);
        }
        if !self.client_secret_key.is_empty() {
            my_size += ::protobuf::rt::string_size(1597, &self.client_secret_key);
        }
        if self.unk1 != false {
            my_size += 1 + 1;
        }
        if self.unk2 != false {
            my_size += 1 + 1;
        }
        if self.unk3 != false {
            my_size += 2 + 1;
        }
        if self.unk4 != false {
            my_size += 2 + 1;
        }
        if self.unk5 != false {
            my_size += 2 + 1;
        }
        if self.unk6 != false {
            my_size += 2 + 1;
        }
        if self.unk7 != false {
            my_size += 2 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.lua_url.is_empty() {
            os.write_string(2, &self.lua_url)?;
        }
        if !self.ex_resource_url.is_empty() {
            os.write_string(14, &self.ex_resource_url)?;
        }
        if !self.asset_bundle_url.is_empty() {
            os.write_string(15, &self.asset_bundle_url)?;
        }
        if !self.ifix_url.is_empty() {
            os.write_string(1895, &self.ifix_url)?;
        }
        if !self.region_name.is_empty() {
            os.write_string(10, &self.region_name)?;
        }
        if !self.ip.is_empty() {
            os.write_string(1, &self.ip)?;
        }
        if self.port != 0 {
            os.write_uint32(12, self.port)?;
        }
        if !self.msg.is_empty() {
            os.write_string(383, &self.msg)?;
        }
        if !self.ifix_version.is_empty() {
            os.write_string(1209, &self.ifix_version)?;
        }
        if !self.mdk_res_version.is_empty() {
            os.write_string(1798, &self.mdk_res_version)?;
        }
        if !self.client_secret_key.is_empty() {
            os.write_string(1597, &self.client_secret_key)?;
        }
        if self.unk1 != false {
            os.write_bool(6, self.unk1)?;
        }
        if self.unk2 != false {
            os.write_bool(13, self.unk2)?;
        }
        if self.unk3 != false {
            os.write_bool(82, self.unk3)?;
        }
        if self.unk4 != false {
            os.write_bool(737, self.unk4)?;
        }
        if self.unk5 != false {
            os.write_bool(1037, self.unk5)?;
        }
        if self.unk6 != false {
            os.write_bool(1401, self.unk6)?;
        }
        if self.unk7 != false {
            os.write_bool(1851, self.unk7)?;
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

    fn new() -> Gateserver {
        Gateserver::new()
    }

    fn clear(&mut self) {
        self.lua_url.clear();
        self.ex_resource_url.clear();
        self.asset_bundle_url.clear();
        self.ifix_url.clear();
        self.region_name.clear();
        self.ip.clear();
        self.port = 0;
        self.msg.clear();
        self.ifix_version.clear();
        self.mdk_res_version.clear();
        self.client_secret_key.clear();
        self.unk1 = false;
        self.unk2 = false;
        self.unk3 = false;
        self.unk4 = false;
        self.unk5 = false;
        self.unk6 = false;
        self.unk7 = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Gateserver {
        static instance: Gateserver = Gateserver {
            lua_url: ::std::string::String::new(),
            ex_resource_url: ::std::string::String::new(),
            asset_bundle_url: ::std::string::String::new(),
            ifix_url: ::std::string::String::new(),
            region_name: ::std::string::String::new(),
            ip: ::std::string::String::new(),
            port: 0,
            msg: ::std::string::String::new(),
            ifix_version: ::std::string::String::new(),
            mdk_res_version: ::std::string::String::new(),
            client_secret_key: ::std::string::String::new(),
            unk1: false,
            unk2: false,
            unk3: false,
            unk4: false,
            unk5: false,
            unk6: false,
            unk7: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Gateserver {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Gateserver").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Gateserver {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Gateserver {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10Gateserver.proto\"\xf5\x03\n\nGateserver\x12\x17\n\x07lua_url\x18\
    \x02\x20\x01(\tR\x06luaUrl\x12&\n\x0fex_resource_url\x18\x0e\x20\x01(\tR\
    \rexResourceUrl\x12(\n\x10asset_bundle_url\x18\x0f\x20\x01(\tR\x0eassetB\
    undleUrl\x12\x1a\n\x08ifix_url\x18\xe7\x0e\x20\x01(\tR\x07ifixUrl\x12\
    \x1f\n\x0bregion_name\x18\n\x20\x01(\tR\nregionName\x12\x0e\n\x02ip\x18\
    \x01\x20\x01(\tR\x02ip\x12\x12\n\x04port\x18\x0c\x20\x01(\rR\x04port\x12\
    \x11\n\x03msg\x18\xff\x02\x20\x01(\tR\x03msg\x12\"\n\x0cifix_version\x18\
    \xb9\t\x20\x01(\tR\x0bifixVersion\x12'\n\x0fmdk_res_version\x18\x86\x0e\
    \x20\x01(\tR\rmdkResVersion\x12+\n\x11client_secret_key\x18\xbd\x0c\x20\
    \x01(\tR\x0fclientSecretKey\x12\x12\n\x04unk1\x18\x06\x20\x01(\x08R\x04u\
    nk1\x12\x12\n\x04unk2\x18\r\x20\x01(\x08R\x04unk2\x12\x12\n\x04unk3\x18R\
    \x20\x01(\x08R\x04unk3\x12\x13\n\x04unk4\x18\xe1\x05\x20\x01(\x08R\x04un\
    k4\x12\x13\n\x04unk5\x18\x8d\x08\x20\x01(\x08R\x04unk5\x12\x13\n\x04unk6\
    \x18\xf9\n\x20\x01(\x08R\x04unk6\x12\x13\n\x04unk7\x18\xbb\x0e\x20\x01(\
    \x08R\x04unk7B\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            messages.push(Gateserver::generated_message_descriptor_data());
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