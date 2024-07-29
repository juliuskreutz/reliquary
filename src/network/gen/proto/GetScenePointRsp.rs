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

//! Generated file from `GetScenePointRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetScenePointRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetScenePointRsp {
    // message fields
    // @@protoc_insertion_point(field:GetScenePointRsp.unhide_point_list)
    pub unhide_point_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetScenePointRsp.unlocked_point_list)
    pub unlocked_point_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetScenePointRsp.unlock_area_list)
    pub unlock_area_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetScenePointRsp.scene_id)
    pub scene_id: u32,
    // @@protoc_insertion_point(field:GetScenePointRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:GetScenePointRsp.belong_uid)
    pub belong_uid: u32,
    // @@protoc_insertion_point(field:GetScenePointRsp.LEPDAGALLND)
    pub LEPDAGALLND: bool,
    // @@protoc_insertion_point(field:GetScenePointRsp.MLNFAFFPPOP)
    pub MLNFAFFPPOP: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetScenePointRsp.GLPBJFLCKCL)
    pub GLPBJFLCKCL: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetScenePointRsp.FJNNNLIJJBL)
    pub FJNNNLIJJBL: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetScenePointRsp.NEAHFHGMEKK)
    pub NEAHFHGMEKK: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetScenePointRsp.DGFPDBCEBHH)
    pub DGFPDBCEBHH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetScenePointRsp.CKNEJGMGCKL)
    pub CKNEJGMGCKL: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:GetScenePointRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetScenePointRsp {
    fn default() -> &'a GetScenePointRsp {
        <GetScenePointRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetScenePointRsp {
    pub fn new() -> GetScenePointRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unhide_point_list",
            |m: &GetScenePointRsp| { &m.unhide_point_list },
            |m: &mut GetScenePointRsp| { &mut m.unhide_point_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unlocked_point_list",
            |m: &GetScenePointRsp| { &m.unlocked_point_list },
            |m: &mut GetScenePointRsp| { &mut m.unlocked_point_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "unlock_area_list",
            |m: &GetScenePointRsp| { &m.unlock_area_list },
            |m: &mut GetScenePointRsp| { &mut m.unlock_area_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scene_id",
            |m: &GetScenePointRsp| { &m.scene_id },
            |m: &mut GetScenePointRsp| { &mut m.scene_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetScenePointRsp| { &m.retcode },
            |m: &mut GetScenePointRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "belong_uid",
            |m: &GetScenePointRsp| { &m.belong_uid },
            |m: &mut GetScenePointRsp| { &mut m.belong_uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LEPDAGALLND",
            |m: &GetScenePointRsp| { &m.LEPDAGALLND },
            |m: &mut GetScenePointRsp| { &mut m.LEPDAGALLND },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MLNFAFFPPOP",
            |m: &GetScenePointRsp| { &m.MLNFAFFPPOP },
            |m: &mut GetScenePointRsp| { &mut m.MLNFAFFPPOP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GLPBJFLCKCL",
            |m: &GetScenePointRsp| { &m.GLPBJFLCKCL },
            |m: &mut GetScenePointRsp| { &mut m.GLPBJFLCKCL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FJNNNLIJJBL",
            |m: &GetScenePointRsp| { &m.FJNNNLIJJBL },
            |m: &mut GetScenePointRsp| { &mut m.FJNNNLIJJBL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NEAHFHGMEKK",
            |m: &GetScenePointRsp| { &m.NEAHFHGMEKK },
            |m: &mut GetScenePointRsp| { &mut m.NEAHFHGMEKK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DGFPDBCEBHH",
            |m: &GetScenePointRsp| { &m.DGFPDBCEBHH },
            |m: &mut GetScenePointRsp| { &mut m.DGFPDBCEBHH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CKNEJGMGCKL",
            |m: &GetScenePointRsp| { &m.CKNEJGMGCKL },
            |m: &mut GetScenePointRsp| { &mut m.CKNEJGMGCKL },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetScenePointRsp>(
            "GetScenePointRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetScenePointRsp {
    const NAME: &'static str = "GetScenePointRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.unhide_point_list)?;
                },
                80 => {
                    self.unhide_point_list.push(is.read_uint32()?);
                },
                114 => {
                    is.read_repeated_packed_uint32_into(&mut self.unlocked_point_list)?;
                },
                112 => {
                    self.unlocked_point_list.push(is.read_uint32()?);
                },
                50 => {
                    is.read_repeated_packed_uint32_into(&mut self.unlock_area_list)?;
                },
                48 => {
                    self.unlock_area_list.push(is.read_uint32()?);
                },
                8 => {
                    self.scene_id = is.read_uint32()?;
                },
                16 => {
                    self.retcode = is.read_int32()?;
                },
                72 => {
                    self.belong_uid = is.read_uint32()?;
                },
                56 => {
                    self.LEPDAGALLND = is.read_bool()?;
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.MLNFAFFPPOP)?;
                },
                24 => {
                    self.MLNFAFFPPOP.push(is.read_uint32()?);
                },
                34 => {
                    is.read_repeated_packed_uint32_into(&mut self.GLPBJFLCKCL)?;
                },
                32 => {
                    self.GLPBJFLCKCL.push(is.read_uint32()?);
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.FJNNNLIJJBL)?;
                },
                64 => {
                    self.FJNNNLIJJBL.push(is.read_uint32()?);
                },
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.NEAHFHGMEKK)?;
                },
                88 => {
                    self.NEAHFHGMEKK.push(is.read_uint32()?);
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.DGFPDBCEBHH)?;
                },
                96 => {
                    self.DGFPDBCEBHH.push(is.read_uint32()?);
                },
                106 => {
                    is.read_repeated_packed_uint32_into(&mut self.CKNEJGMGCKL)?;
                },
                104 => {
                    self.CKNEJGMGCKL.push(is.read_uint32()?);
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
        for value in &self.unhide_point_list {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        for value in &self.unlocked_point_list {
            my_size += ::protobuf::rt::uint32_size(14, *value);
        };
        for value in &self.unlock_area_list {
            my_size += ::protobuf::rt::uint32_size(6, *value);
        };
        if self.scene_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.scene_id);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(2, self.retcode);
        }
        if self.belong_uid != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.belong_uid);
        }
        if self.LEPDAGALLND != false {
            my_size += 1 + 1;
        }
        for value in &self.MLNFAFFPPOP {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        for value in &self.GLPBJFLCKCL {
            my_size += ::protobuf::rt::uint32_size(4, *value);
        };
        for value in &self.FJNNNLIJJBL {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        for value in &self.NEAHFHGMEKK {
            my_size += ::protobuf::rt::uint32_size(11, *value);
        };
        for value in &self.DGFPDBCEBHH {
            my_size += ::protobuf::rt::uint32_size(12, *value);
        };
        for value in &self.CKNEJGMGCKL {
            my_size += ::protobuf::rt::uint32_size(13, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.unhide_point_list {
            os.write_uint32(10, *v)?;
        };
        for v in &self.unlocked_point_list {
            os.write_uint32(14, *v)?;
        };
        for v in &self.unlock_area_list {
            os.write_uint32(6, *v)?;
        };
        if self.scene_id != 0 {
            os.write_uint32(1, self.scene_id)?;
        }
        if self.retcode != 0 {
            os.write_int32(2, self.retcode)?;
        }
        if self.belong_uid != 0 {
            os.write_uint32(9, self.belong_uid)?;
        }
        if self.LEPDAGALLND != false {
            os.write_bool(7, self.LEPDAGALLND)?;
        }
        for v in &self.MLNFAFFPPOP {
            os.write_uint32(3, *v)?;
        };
        for v in &self.GLPBJFLCKCL {
            os.write_uint32(4, *v)?;
        };
        for v in &self.FJNNNLIJJBL {
            os.write_uint32(8, *v)?;
        };
        for v in &self.NEAHFHGMEKK {
            os.write_uint32(11, *v)?;
        };
        for v in &self.DGFPDBCEBHH {
            os.write_uint32(12, *v)?;
        };
        for v in &self.CKNEJGMGCKL {
            os.write_uint32(13, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GetScenePointRsp {
        GetScenePointRsp::new()
    }

    fn clear(&mut self) {
        self.unhide_point_list.clear();
        self.unlocked_point_list.clear();
        self.unlock_area_list.clear();
        self.scene_id = 0;
        self.retcode = 0;
        self.belong_uid = 0;
        self.LEPDAGALLND = false;
        self.MLNFAFFPPOP.clear();
        self.GLPBJFLCKCL.clear();
        self.FJNNNLIJJBL.clear();
        self.NEAHFHGMEKK.clear();
        self.DGFPDBCEBHH.clear();
        self.CKNEJGMGCKL.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetScenePointRsp {
        static instance: GetScenePointRsp = GetScenePointRsp {
            unhide_point_list: ::std::vec::Vec::new(),
            unlocked_point_list: ::std::vec::Vec::new(),
            unlock_area_list: ::std::vec::Vec::new(),
            scene_id: 0,
            retcode: 0,
            belong_uid: 0,
            LEPDAGALLND: false,
            MLNFAFFPPOP: ::std::vec::Vec::new(),
            GLPBJFLCKCL: ::std::vec::Vec::new(),
            FJNNNLIJJBL: ::std::vec::Vec::new(),
            NEAHFHGMEKK: ::std::vec::Vec::new(),
            DGFPDBCEBHH: ::std::vec::Vec::new(),
            CKNEJGMGCKL: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetScenePointRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetScenePointRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetScenePointRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetScenePointRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16GetScenePointRsp.proto\"\xda\x03\n\x10GetScenePointRsp\x12*\n\x11u\
    nhide_point_list\x18\n\x20\x03(\rR\x0funhidePointList\x12.\n\x13unlocked\
    _point_list\x18\x0e\x20\x03(\rR\x11unlockedPointList\x12(\n\x10unlock_ar\
    ea_list\x18\x06\x20\x03(\rR\x0eunlockAreaList\x12\x19\n\x08scene_id\x18\
    \x01\x20\x01(\rR\x07sceneId\x12\x18\n\x07retcode\x18\x02\x20\x01(\x05R\
    \x07retcode\x12\x1d\n\nbelong_uid\x18\t\x20\x01(\rR\tbelongUid\x12\x20\n\
    \x0bLEPDAGALLND\x18\x07\x20\x01(\x08R\x0bLEPDAGALLND\x12\x20\n\x0bMLNFAF\
    FPPOP\x18\x03\x20\x03(\rR\x0bMLNFAFFPPOP\x12\x20\n\x0bGLPBJFLCKCL\x18\
    \x04\x20\x03(\rR\x0bGLPBJFLCKCL\x12\x20\n\x0bFJNNNLIJJBL\x18\x08\x20\x03\
    (\rR\x0bFJNNNLIJJBL\x12\x20\n\x0bNEAHFHGMEKK\x18\x0b\x20\x03(\rR\x0bNEAH\
    FHGMEKK\x12\x20\n\x0bDGFPDBCEBHH\x18\x0c\x20\x03(\rR\x0bDGFPDBCEBHH\x12\
    \x20\n\x0bCKNEJGMGCKL\x18\r\x20\x03(\rR\x0bCKNEJGMGCKLB\x1b\n\x19emu.gra\
    sscutter.net.protob\x06proto3\
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
            messages.push(GetScenePointRsp::generated_message_descriptor_data());
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
