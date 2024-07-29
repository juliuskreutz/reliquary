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

//! Generated file from `LackingResourceInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LackingResourceInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LackingResourceInfo {
    // message fields
    // @@protoc_insertion_point(field:LackingResourceInfo.PDFBJCPKGAD)
    pub PDFBJCPKGAD: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LackingResourceInfo.JHOJDJGELLK)
    pub JHOJDJGELLK: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:LackingResourceInfo.GEPBOPLIEMB)
    pub GEPBOPLIEMB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LackingResourceInfo.KBEAGMAGFPD)
    pub KBEAGMAGFPD: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:LackingResourceInfo.NGBJMLNHLBB)
    pub NGBJMLNHLBB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LackingResourceInfo.GJBFBFOCLJL)
    pub GJBFBFOCLJL: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:LackingResourceInfo.IFLMBHBHLMC)
    pub IFLMBHBHLMC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LackingResourceInfo.PLGICBELJNB)
    pub PLGICBELJNB: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:LackingResourceInfo.FOIHNLFBPFI)
    pub FOIHNLFBPFI: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:LackingResourceInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LackingResourceInfo {
    fn default() -> &'a LackingResourceInfo {
        <LackingResourceInfo as ::protobuf::Message>::default_instance()
    }
}

impl LackingResourceInfo {
    pub fn new() -> LackingResourceInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PDFBJCPKGAD",
            |m: &LackingResourceInfo| { &m.PDFBJCPKGAD },
            |m: &mut LackingResourceInfo| { &mut m.PDFBJCPKGAD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "JHOJDJGELLK",
            |m: &LackingResourceInfo| { &m.JHOJDJGELLK },
            |m: &mut LackingResourceInfo| { &mut m.JHOJDJGELLK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GEPBOPLIEMB",
            |m: &LackingResourceInfo| { &m.GEPBOPLIEMB },
            |m: &mut LackingResourceInfo| { &mut m.GEPBOPLIEMB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "KBEAGMAGFPD",
            |m: &LackingResourceInfo| { &m.KBEAGMAGFPD },
            |m: &mut LackingResourceInfo| { &mut m.KBEAGMAGFPD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NGBJMLNHLBB",
            |m: &LackingResourceInfo| { &m.NGBJMLNHLBB },
            |m: &mut LackingResourceInfo| { &mut m.NGBJMLNHLBB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "GJBFBFOCLJL",
            |m: &LackingResourceInfo| { &m.GJBFBFOCLJL },
            |m: &mut LackingResourceInfo| { &mut m.GJBFBFOCLJL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "IFLMBHBHLMC",
            |m: &LackingResourceInfo| { &m.IFLMBHBHLMC },
            |m: &mut LackingResourceInfo| { &mut m.IFLMBHBHLMC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PLGICBELJNB",
            |m: &LackingResourceInfo| { &m.PLGICBELJNB },
            |m: &mut LackingResourceInfo| { &mut m.PLGICBELJNB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FOIHNLFBPFI",
            |m: &LackingResourceInfo| { &m.FOIHNLFBPFI },
            |m: &mut LackingResourceInfo| { &mut m.FOIHNLFBPFI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LackingResourceInfo>(
            "LackingResourceInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LackingResourceInfo {
    const NAME: &'static str = "LackingResourceInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.PDFBJCPKGAD)?;
                },
                8 => {
                    self.PDFBJCPKGAD.push(is.read_uint32()?);
                },
                18 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.JHOJDJGELLK.insert(key, value);
                },
                26 => {
                    is.read_repeated_packed_uint32_into(&mut self.GEPBOPLIEMB)?;
                },
                24 => {
                    self.GEPBOPLIEMB.push(is.read_uint32()?);
                },
                34 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.KBEAGMAGFPD.insert(key, value);
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.NGBJMLNHLBB)?;
                },
                40 => {
                    self.NGBJMLNHLBB.push(is.read_uint32()?);
                },
                50 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.GJBFBFOCLJL.insert(key, value);
                },
                58 => {
                    is.read_repeated_packed_uint32_into(&mut self.IFLMBHBHLMC)?;
                },
                56 => {
                    self.IFLMBHBHLMC.push(is.read_uint32()?);
                },
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.PLGICBELJNB)?;
                },
                64 => {
                    self.PLGICBELJNB.push(is.read_uint32()?);
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.FOIHNLFBPFI)?;
                },
                72 => {
                    self.FOIHNLFBPFI.push(is.read_uint32()?);
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
        for value in &self.PDFBJCPKGAD {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        for (k, v) in &self.JHOJDJGELLK {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.GEPBOPLIEMB {
            my_size += ::protobuf::rt::uint32_size(3, *value);
        };
        for (k, v) in &self.KBEAGMAGFPD {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.NGBJMLNHLBB {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        for (k, v) in &self.GJBFBFOCLJL {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.IFLMBHBHLMC {
            my_size += ::protobuf::rt::uint32_size(7, *value);
        };
        for value in &self.PLGICBELJNB {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        for value in &self.FOIHNLFBPFI {
            my_size += ::protobuf::rt::uint32_size(9, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.PDFBJCPKGAD {
            os.write_uint32(1, *v)?;
        };
        for (k, v) in &self.JHOJDJGELLK {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(18)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        for v in &self.GEPBOPLIEMB {
            os.write_uint32(3, *v)?;
        };
        for (k, v) in &self.KBEAGMAGFPD {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(34)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        for v in &self.NGBJMLNHLBB {
            os.write_uint32(5, *v)?;
        };
        for (k, v) in &self.GJBFBFOCLJL {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(50)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        for v in &self.IFLMBHBHLMC {
            os.write_uint32(7, *v)?;
        };
        for v in &self.PLGICBELJNB {
            os.write_uint32(8, *v)?;
        };
        for v in &self.FOIHNLFBPFI {
            os.write_uint32(9, *v)?;
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

    fn new() -> LackingResourceInfo {
        LackingResourceInfo::new()
    }

    fn clear(&mut self) {
        self.PDFBJCPKGAD.clear();
        self.JHOJDJGELLK.clear();
        self.GEPBOPLIEMB.clear();
        self.KBEAGMAGFPD.clear();
        self.NGBJMLNHLBB.clear();
        self.GJBFBFOCLJL.clear();
        self.IFLMBHBHLMC.clear();
        self.PLGICBELJNB.clear();
        self.FOIHNLFBPFI.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LackingResourceInfo {
        static instance: ::protobuf::rt::Lazy<LackingResourceInfo> = ::protobuf::rt::Lazy::new();
        instance.get(LackingResourceInfo::new)
    }
}

impl ::protobuf::MessageFull for LackingResourceInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LackingResourceInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LackingResourceInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LackingResourceInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19LackingResourceInfo.proto\"\xfc\x04\n\x13LackingResourceInfo\x12\
    \x20\n\x0bPDFBJCPKGAD\x18\x01\x20\x03(\rR\x0bPDFBJCPKGAD\x12G\n\x0bJHOJD\
    JGELLK\x18\x02\x20\x03(\x0b2%.LackingResourceInfo.JHOJDJGELLKEntryR\x0bJ\
    HOJDJGELLK\x12\x20\n\x0bGEPBOPLIEMB\x18\x03\x20\x03(\rR\x0bGEPBOPLIEMB\
    \x12G\n\x0bKBEAGMAGFPD\x18\x04\x20\x03(\x0b2%.LackingResourceInfo.KBEAGM\
    AGFPDEntryR\x0bKBEAGMAGFPD\x12\x20\n\x0bNGBJMLNHLBB\x18\x05\x20\x03(\rR\
    \x0bNGBJMLNHLBB\x12G\n\x0bGJBFBFOCLJL\x18\x06\x20\x03(\x0b2%.LackingReso\
    urceInfo.GJBFBFOCLJLEntryR\x0bGJBFBFOCLJL\x12\x20\n\x0bIFLMBHBHLMC\x18\
    \x07\x20\x03(\rR\x0bIFLMBHBHLMC\x12\x20\n\x0bPLGICBELJNB\x18\x08\x20\x03\
    (\rR\x0bPLGICBELJNB\x12\x20\n\x0bFOIHNLFBPFI\x18\t\x20\x03(\rR\x0bFOIHNL\
    FBPFI\x1a>\n\x10JHOJDJGELLKEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\
    \x03key\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05value:\x028\x01\x1a>\n\
    \x10KBEAGMAGFPDEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\
    \n\x05value\x18\x02\x20\x01(\rR\x05value:\x028\x01\x1a>\n\x10GJBFBFOCLJL\
    Entry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\n\x05value\x18\
    \x02\x20\x01(\rR\x05value:\x028\x01B\x1b\n\x19emu.grasscutter.net.protob\
    \x06proto3\
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
            messages.push(LackingResourceInfo::generated_message_descriptor_data());
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
