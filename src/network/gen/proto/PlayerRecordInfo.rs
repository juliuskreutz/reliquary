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

//! Generated file from `PlayerRecordInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlayerRecordInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerRecordInfo {
    // message fields
    // @@protoc_insertion_point(field:PlayerRecordInfo.DKDINDMGEJH)
    pub DKDINDMGEJH: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.BFGDPMKBHNB)
    pub BFGDPMKBHNB: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.PNIAMOINNJF)
    pub PNIAMOINNJF: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.LAAOICJFOPI)
    pub LAAOICJFOPI: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.JJOHCDLEDMO)
    pub JJOHCDLEDMO: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.EEJCCFCIGDF)
    pub EEJCCFCIGDF: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.PDHAFOAOMPB)
    pub PDHAFOAOMPB: u32,
    // @@protoc_insertion_point(field:PlayerRecordInfo.collection_info)
    pub collection_info: ::protobuf::MessageField<super::PlayerCollectionInfo::PlayerCollectionInfo>,
    // @@protoc_insertion_point(field:PlayerRecordInfo.IPOCLBEJIAF)
    pub IPOCLBEJIAF: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerRecordInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerRecordInfo {
    fn default() -> &'a PlayerRecordInfo {
        <PlayerRecordInfo as ::protobuf::Message>::default_instance()
    }
}

impl PlayerRecordInfo {
    pub fn new() -> PlayerRecordInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DKDINDMGEJH",
            |m: &PlayerRecordInfo| { &m.DKDINDMGEJH },
            |m: &mut PlayerRecordInfo| { &mut m.DKDINDMGEJH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BFGDPMKBHNB",
            |m: &PlayerRecordInfo| { &m.BFGDPMKBHNB },
            |m: &mut PlayerRecordInfo| { &mut m.BFGDPMKBHNB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PNIAMOINNJF",
            |m: &PlayerRecordInfo| { &m.PNIAMOINNJF },
            |m: &mut PlayerRecordInfo| { &mut m.PNIAMOINNJF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LAAOICJFOPI",
            |m: &PlayerRecordInfo| { &m.LAAOICJFOPI },
            |m: &mut PlayerRecordInfo| { &mut m.LAAOICJFOPI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JJOHCDLEDMO",
            |m: &PlayerRecordInfo| { &m.JJOHCDLEDMO },
            |m: &mut PlayerRecordInfo| { &mut m.JJOHCDLEDMO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EEJCCFCIGDF",
            |m: &PlayerRecordInfo| { &m.EEJCCFCIGDF },
            |m: &mut PlayerRecordInfo| { &mut m.EEJCCFCIGDF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PDHAFOAOMPB",
            |m: &PlayerRecordInfo| { &m.PDHAFOAOMPB },
            |m: &mut PlayerRecordInfo| { &mut m.PDHAFOAOMPB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::PlayerCollectionInfo::PlayerCollectionInfo>(
            "collection_info",
            |m: &PlayerRecordInfo| { &m.collection_info },
            |m: &mut PlayerRecordInfo| { &mut m.collection_info },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IPOCLBEJIAF",
            |m: &PlayerRecordInfo| { &m.IPOCLBEJIAF },
            |m: &mut PlayerRecordInfo| { &mut m.IPOCLBEJIAF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerRecordInfo>(
            "PlayerRecordInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerRecordInfo {
    const NAME: &'static str = "PlayerRecordInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.DKDINDMGEJH = is.read_uint32()?;
                },
                48 => {
                    self.BFGDPMKBHNB = is.read_uint32()?;
                },
                104 => {
                    self.PNIAMOINNJF = is.read_uint32()?;
                },
                8 => {
                    self.LAAOICJFOPI = is.read_uint32()?;
                },
                88 => {
                    self.JJOHCDLEDMO = is.read_uint32()?;
                },
                96 => {
                    self.EEJCCFCIGDF = is.read_uint32()?;
                },
                16 => {
                    self.PDHAFOAOMPB = is.read_uint32()?;
                },
                58 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.collection_info)?;
                },
                112 => {
                    self.IPOCLBEJIAF = is.read_uint32()?;
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
        if self.DKDINDMGEJH != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.DKDINDMGEJH);
        }
        if self.BFGDPMKBHNB != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.BFGDPMKBHNB);
        }
        if self.PNIAMOINNJF != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.PNIAMOINNJF);
        }
        if self.LAAOICJFOPI != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.LAAOICJFOPI);
        }
        if self.JJOHCDLEDMO != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.JJOHCDLEDMO);
        }
        if self.EEJCCFCIGDF != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.EEJCCFCIGDF);
        }
        if self.PDHAFOAOMPB != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.PDHAFOAOMPB);
        }
        if let Some(v) = self.collection_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.IPOCLBEJIAF != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.IPOCLBEJIAF);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.DKDINDMGEJH != 0 {
            os.write_uint32(5, self.DKDINDMGEJH)?;
        }
        if self.BFGDPMKBHNB != 0 {
            os.write_uint32(6, self.BFGDPMKBHNB)?;
        }
        if self.PNIAMOINNJF != 0 {
            os.write_uint32(13, self.PNIAMOINNJF)?;
        }
        if self.LAAOICJFOPI != 0 {
            os.write_uint32(1, self.LAAOICJFOPI)?;
        }
        if self.JJOHCDLEDMO != 0 {
            os.write_uint32(11, self.JJOHCDLEDMO)?;
        }
        if self.EEJCCFCIGDF != 0 {
            os.write_uint32(12, self.EEJCCFCIGDF)?;
        }
        if self.PDHAFOAOMPB != 0 {
            os.write_uint32(2, self.PDHAFOAOMPB)?;
        }
        if let Some(v) = self.collection_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        }
        if self.IPOCLBEJIAF != 0 {
            os.write_uint32(14, self.IPOCLBEJIAF)?;
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

    fn new() -> PlayerRecordInfo {
        PlayerRecordInfo::new()
    }

    fn clear(&mut self) {
        self.DKDINDMGEJH = 0;
        self.BFGDPMKBHNB = 0;
        self.PNIAMOINNJF = 0;
        self.LAAOICJFOPI = 0;
        self.JJOHCDLEDMO = 0;
        self.EEJCCFCIGDF = 0;
        self.PDHAFOAOMPB = 0;
        self.collection_info.clear();
        self.IPOCLBEJIAF = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerRecordInfo {
        static instance: PlayerRecordInfo = PlayerRecordInfo {
            DKDINDMGEJH: 0,
            BFGDPMKBHNB: 0,
            PNIAMOINNJF: 0,
            LAAOICJFOPI: 0,
            JJOHCDLEDMO: 0,
            EEJCCFCIGDF: 0,
            PDHAFOAOMPB: 0,
            collection_info: ::protobuf::MessageField::none(),
            IPOCLBEJIAF: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerRecordInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerRecordInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerRecordInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerRecordInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16PlayerRecordInfo.proto\x1a\x1aPlayerCollectionInfo.proto\"\xe2\x02\
    \n\x10PlayerRecordInfo\x12\x20\n\x0bDKDINDMGEJH\x18\x05\x20\x01(\rR\x0bD\
    KDINDMGEJH\x12\x20\n\x0bBFGDPMKBHNB\x18\x06\x20\x01(\rR\x0bBFGDPMKBHNB\
    \x12\x20\n\x0bPNIAMOINNJF\x18\r\x20\x01(\rR\x0bPNIAMOINNJF\x12\x20\n\x0b\
    LAAOICJFOPI\x18\x01\x20\x01(\rR\x0bLAAOICJFOPI\x12\x20\n\x0bJJOHCDLEDMO\
    \x18\x0b\x20\x01(\rR\x0bJJOHCDLEDMO\x12\x20\n\x0bEEJCCFCIGDF\x18\x0c\x20\
    \x01(\rR\x0bEEJCCFCIGDF\x12\x20\n\x0bPDHAFOAOMPB\x18\x02\x20\x01(\rR\x0b\
    PDHAFOAOMPB\x12>\n\x0fcollection_info\x18\x07\x20\x01(\x0b2\x15.PlayerCo\
    llectionInfoR\x0ecollectionInfo\x12\x20\n\x0bIPOCLBEJIAF\x18\x0e\x20\x01\
    (\rR\x0bIPOCLBEJIAFB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::PlayerCollectionInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlayerRecordInfo::generated_message_descriptor_data());
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
