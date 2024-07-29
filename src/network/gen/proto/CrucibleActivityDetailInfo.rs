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

//! Generated file from `CrucibleActivityDetailInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CrucibleActivityDetailInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CrucibleActivityDetailInfo {
    // message fields
    // @@protoc_insertion_point(field:CrucibleActivityDetailInfo.pos)
    pub pos: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:CrucibleActivityDetailInfo.cost_time)
    pub cost_time: u32,
    // @@protoc_insertion_point(field:CrucibleActivityDetailInfo.battle_world_level)
    pub battle_world_level: u32,
    // @@protoc_insertion_point(field:CrucibleActivityDetailInfo.uid_info_list)
    pub uid_info_list: ::std::vec::Vec<super::CrucibleBattleUidInfo::CrucibleBattleUidInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:CrucibleActivityDetailInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CrucibleActivityDetailInfo {
    fn default() -> &'a CrucibleActivityDetailInfo {
        <CrucibleActivityDetailInfo as ::protobuf::Message>::default_instance()
    }
}

impl CrucibleActivityDetailInfo {
    pub fn new() -> CrucibleActivityDetailInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "pos",
            |m: &CrucibleActivityDetailInfo| { &m.pos },
            |m: &mut CrucibleActivityDetailInfo| { &mut m.pos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cost_time",
            |m: &CrucibleActivityDetailInfo| { &m.cost_time },
            |m: &mut CrucibleActivityDetailInfo| { &mut m.cost_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "battle_world_level",
            |m: &CrucibleActivityDetailInfo| { &m.battle_world_level },
            |m: &mut CrucibleActivityDetailInfo| { &mut m.battle_world_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "uid_info_list",
            |m: &CrucibleActivityDetailInfo| { &m.uid_info_list },
            |m: &mut CrucibleActivityDetailInfo| { &mut m.uid_info_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CrucibleActivityDetailInfo>(
            "CrucibleActivityDetailInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CrucibleActivityDetailInfo {
    const NAME: &'static str = "CrucibleActivityDetailInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pos)?;
                },
                16 => {
                    self.cost_time = is.read_uint32()?;
                },
                32 => {
                    self.battle_world_level = is.read_uint32()?;
                },
                74 => {
                    self.uid_info_list.push(is.read_message()?);
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
        if let Some(v) = self.pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.cost_time != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.cost_time);
        }
        if self.battle_world_level != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.battle_world_level);
        }
        for value in &self.uid_info_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.pos.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.cost_time != 0 {
            os.write_uint32(2, self.cost_time)?;
        }
        if self.battle_world_level != 0 {
            os.write_uint32(4, self.battle_world_level)?;
        }
        for v in &self.uid_info_list {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
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

    fn new() -> CrucibleActivityDetailInfo {
        CrucibleActivityDetailInfo::new()
    }

    fn clear(&mut self) {
        self.pos.clear();
        self.cost_time = 0;
        self.battle_world_level = 0;
        self.uid_info_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CrucibleActivityDetailInfo {
        static instance: CrucibleActivityDetailInfo = CrucibleActivityDetailInfo {
            pos: ::protobuf::MessageField::none(),
            cost_time: 0,
            battle_world_level: 0,
            uid_info_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CrucibleActivityDetailInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CrucibleActivityDetailInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CrucibleActivityDetailInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrucibleActivityDetailInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20CrucibleActivityDetailInfo.proto\x1a\x0cVector.proto\x1a\x1bCrucib\
    leBattleUidInfo.proto\"\xbe\x01\n\x1aCrucibleActivityDetailInfo\x12\x19\
    \n\x03pos\x18\x07\x20\x01(\x0b2\x07.VectorR\x03pos\x12\x1b\n\tcost_time\
    \x18\x02\x20\x01(\rR\x08costTime\x12,\n\x12battle_world_level\x18\x04\
    \x20\x01(\rR\x10battleWorldLevel\x12:\n\ruid_info_list\x18\t\x20\x03(\
    \x0b2\x16.CrucibleBattleUidInfoR\x0buidInfoListB\x1b\n\x19emu.grasscutte\
    r.net.protob\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::Vector::file_descriptor().clone());
            deps.push(super::CrucibleBattleUidInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CrucibleActivityDetailInfo::generated_message_descriptor_data());
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
