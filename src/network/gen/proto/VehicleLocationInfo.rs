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

//! Generated file from `VehicleLocationInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:VehicleLocationInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct VehicleLocationInfo {
    // message fields
    // @@protoc_insertion_point(field:VehicleLocationInfo.entity_id)
    pub entity_id: u32,
    // @@protoc_insertion_point(field:VehicleLocationInfo.MKHCMCLJKLN)
    pub MKHCMCLJKLN: f32,
    // @@protoc_insertion_point(field:VehicleLocationInfo.AACAIMICFPI)
    pub AACAIMICFPI: f32,
    // @@protoc_insertion_point(field:VehicleLocationInfo.owner_uid)
    pub owner_uid: u32,
    // @@protoc_insertion_point(field:VehicleLocationInfo.pos)
    pub pos: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:VehicleLocationInfo.uid_list)
    pub uid_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:VehicleLocationInfo.rot)
    pub rot: ::protobuf::MessageField<super::Vector::Vector>,
    // @@protoc_insertion_point(field:VehicleLocationInfo.gadget_id)
    pub gadget_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:VehicleLocationInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a VehicleLocationInfo {
    fn default() -> &'a VehicleLocationInfo {
        <VehicleLocationInfo as ::protobuf::Message>::default_instance()
    }
}

impl VehicleLocationInfo {
    pub fn new() -> VehicleLocationInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entity_id",
            |m: &VehicleLocationInfo| { &m.entity_id },
            |m: &mut VehicleLocationInfo| { &mut m.entity_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MKHCMCLJKLN",
            |m: &VehicleLocationInfo| { &m.MKHCMCLJKLN },
            |m: &mut VehicleLocationInfo| { &mut m.MKHCMCLJKLN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AACAIMICFPI",
            |m: &VehicleLocationInfo| { &m.AACAIMICFPI },
            |m: &mut VehicleLocationInfo| { &mut m.AACAIMICFPI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "owner_uid",
            |m: &VehicleLocationInfo| { &m.owner_uid },
            |m: &mut VehicleLocationInfo| { &mut m.owner_uid },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "pos",
            |m: &VehicleLocationInfo| { &m.pos },
            |m: &mut VehicleLocationInfo| { &mut m.pos },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "uid_list",
            |m: &VehicleLocationInfo| { &m.uid_list },
            |m: &mut VehicleLocationInfo| { &mut m.uid_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Vector::Vector>(
            "rot",
            |m: &VehicleLocationInfo| { &m.rot },
            |m: &mut VehicleLocationInfo| { &mut m.rot },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "gadget_id",
            |m: &VehicleLocationInfo| { &m.gadget_id },
            |m: &mut VehicleLocationInfo| { &mut m.gadget_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<VehicleLocationInfo>(
            "VehicleLocationInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for VehicleLocationInfo {
    const NAME: &'static str = "VehicleLocationInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.entity_id = is.read_uint32()?;
                },
                45 => {
                    self.MKHCMCLJKLN = is.read_float()?;
                },
                53 => {
                    self.AACAIMICFPI = is.read_float()?;
                },
                72 => {
                    self.owner_uid = is.read_uint32()?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pos)?;
                },
                98 => {
                    is.read_repeated_packed_uint32_into(&mut self.uid_list)?;
                },
                96 => {
                    self.uid_list.push(is.read_uint32()?);
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rot)?;
                },
                112 => {
                    self.gadget_id = is.read_uint32()?;
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
        if self.entity_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.entity_id);
        }
        if self.MKHCMCLJKLN != 0. {
            my_size += 1 + 4;
        }
        if self.AACAIMICFPI != 0. {
            my_size += 1 + 4;
        }
        if self.owner_uid != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.owner_uid);
        }
        if let Some(v) = self.pos.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.uid_list {
            my_size += ::protobuf::rt::uint32_size(12, *value);
        };
        if let Some(v) = self.rot.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.gadget_id != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.gadget_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.entity_id != 0 {
            os.write_uint32(2, self.entity_id)?;
        }
        if self.MKHCMCLJKLN != 0. {
            os.write_float(5, self.MKHCMCLJKLN)?;
        }
        if self.AACAIMICFPI != 0. {
            os.write_float(6, self.AACAIMICFPI)?;
        }
        if self.owner_uid != 0 {
            os.write_uint32(9, self.owner_uid)?;
        }
        if let Some(v) = self.pos.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        for v in &self.uid_list {
            os.write_uint32(12, *v)?;
        };
        if let Some(v) = self.rot.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if self.gadget_id != 0 {
            os.write_uint32(14, self.gadget_id)?;
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

    fn new() -> VehicleLocationInfo {
        VehicleLocationInfo::new()
    }

    fn clear(&mut self) {
        self.entity_id = 0;
        self.MKHCMCLJKLN = 0.;
        self.AACAIMICFPI = 0.;
        self.owner_uid = 0;
        self.pos.clear();
        self.uid_list.clear();
        self.rot.clear();
        self.gadget_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static VehicleLocationInfo {
        static instance: VehicleLocationInfo = VehicleLocationInfo {
            entity_id: 0,
            MKHCMCLJKLN: 0.,
            AACAIMICFPI: 0.,
            owner_uid: 0,
            pos: ::protobuf::MessageField::none(),
            uid_list: ::std::vec::Vec::new(),
            rot: ::protobuf::MessageField::none(),
            gadget_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for VehicleLocationInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("VehicleLocationInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for VehicleLocationInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VehicleLocationInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19VehicleLocationInfo.proto\x1a\x0cVector.proto\"\x81\x02\n\x13Vehic\
    leLocationInfo\x12\x1b\n\tentity_id\x18\x02\x20\x01(\rR\x08entityId\x12\
    \x20\n\x0bMKHCMCLJKLN\x18\x05\x20\x01(\x02R\x0bMKHCMCLJKLN\x12\x20\n\x0b\
    AACAIMICFPI\x18\x06\x20\x01(\x02R\x0bAACAIMICFPI\x12\x1b\n\towner_uid\
    \x18\t\x20\x01(\rR\x08ownerUid\x12\x19\n\x03pos\x18\x0b\x20\x01(\x0b2\
    \x07.VectorR\x03pos\x12\x19\n\x08uid_list\x18\x0c\x20\x03(\rR\x07uidList\
    \x12\x19\n\x03rot\x18\r\x20\x01(\x0b2\x07.VectorR\x03rot\x12\x1b\n\tgadg\
    et_id\x18\x0e\x20\x01(\rR\x08gadgetIdB\x1b\n\x19emu.grasscutter.net.prot\
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
            deps.push(super::Vector::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(VehicleLocationInfo::generated_message_descriptor_data());
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