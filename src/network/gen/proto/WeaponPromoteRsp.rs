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

//! Generated file from `WeaponPromoteRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:WeaponPromoteRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct WeaponPromoteRsp {
    // message fields
    // @@protoc_insertion_point(field:WeaponPromoteRsp.target_weapon_guid)
    pub target_weapon_guid: u64,
    // @@protoc_insertion_point(field:WeaponPromoteRsp.cur_promote_level)
    pub cur_promote_level: u32,
    // @@protoc_insertion_point(field:WeaponPromoteRsp.retcode)
    pub retcode: i32,
    // @@protoc_insertion_point(field:WeaponPromoteRsp.old_promote_level)
    pub old_promote_level: u32,
    // special fields
    // @@protoc_insertion_point(special_field:WeaponPromoteRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a WeaponPromoteRsp {
    fn default() -> &'a WeaponPromoteRsp {
        <WeaponPromoteRsp as ::protobuf::Message>::default_instance()
    }
}

impl WeaponPromoteRsp {
    pub fn new() -> WeaponPromoteRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "target_weapon_guid",
            |m: &WeaponPromoteRsp| { &m.target_weapon_guid },
            |m: &mut WeaponPromoteRsp| { &mut m.target_weapon_guid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_promote_level",
            |m: &WeaponPromoteRsp| { &m.cur_promote_level },
            |m: &mut WeaponPromoteRsp| { &mut m.cur_promote_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &WeaponPromoteRsp| { &m.retcode },
            |m: &mut WeaponPromoteRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "old_promote_level",
            |m: &WeaponPromoteRsp| { &m.old_promote_level },
            |m: &mut WeaponPromoteRsp| { &mut m.old_promote_level },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<WeaponPromoteRsp>(
            "WeaponPromoteRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for WeaponPromoteRsp {
    const NAME: &'static str = "WeaponPromoteRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.target_weapon_guid = is.read_uint64()?;
                },
                56 => {
                    self.cur_promote_level = is.read_uint32()?;
                },
                96 => {
                    self.retcode = is.read_int32()?;
                },
                120 => {
                    self.old_promote_level = is.read_uint32()?;
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
        if self.target_weapon_guid != 0 {
            my_size += ::protobuf::rt::uint64_size(5, self.target_weapon_guid);
        }
        if self.cur_promote_level != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.cur_promote_level);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::int32_size(12, self.retcode);
        }
        if self.old_promote_level != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.old_promote_level);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.target_weapon_guid != 0 {
            os.write_uint64(5, self.target_weapon_guid)?;
        }
        if self.cur_promote_level != 0 {
            os.write_uint32(7, self.cur_promote_level)?;
        }
        if self.retcode != 0 {
            os.write_int32(12, self.retcode)?;
        }
        if self.old_promote_level != 0 {
            os.write_uint32(15, self.old_promote_level)?;
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

    fn new() -> WeaponPromoteRsp {
        WeaponPromoteRsp::new()
    }

    fn clear(&mut self) {
        self.target_weapon_guid = 0;
        self.cur_promote_level = 0;
        self.retcode = 0;
        self.old_promote_level = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static WeaponPromoteRsp {
        static instance: WeaponPromoteRsp = WeaponPromoteRsp {
            target_weapon_guid: 0,
            cur_promote_level: 0,
            retcode: 0,
            old_promote_level: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for WeaponPromoteRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("WeaponPromoteRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for WeaponPromoteRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WeaponPromoteRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16WeaponPromoteRsp.proto\"\xb2\x01\n\x10WeaponPromoteRsp\x12,\n\x12t\
    arget_weapon_guid\x18\x05\x20\x01(\x04R\x10targetWeaponGuid\x12*\n\x11cu\
    r_promote_level\x18\x07\x20\x01(\rR\x0fcurPromoteLevel\x12\x18\n\x07retc\
    ode\x18\x0c\x20\x01(\x05R\x07retcode\x12*\n\x11old_promote_level\x18\x0f\
    \x20\x01(\rR\x0foldPromoteLevelB\x1b\n\x19emu.grasscutter.net.protob\x06\
    proto3\
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
            messages.push(WeaponPromoteRsp::generated_message_descriptor_data());
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