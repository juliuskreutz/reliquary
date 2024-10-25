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

//! Generated file from `FIPNNGPAFCA.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FIPNNGPAFCA)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FIPNNGPAFCA {
    // message fields
    // @@protoc_insertion_point(field:FIPNNGPAFCA.GFDGGCBGGGD)
    pub GFDGGCBGGGD: u32,
    // @@protoc_insertion_point(field:FIPNNGPAFCA.ICMDONMEJGE)
    pub ICMDONMEJGE: u32,
    // @@protoc_insertion_point(field:FIPNNGPAFCA.FJPEFPBFALC)
    pub FJPEFPBFALC: u32,
    // message oneof groups
    pub FPHJPCBPDHC: ::std::option::Option<fipnngpafca::FPHJPCBPDHC>,
    // special fields
    // @@protoc_insertion_point(special_field:FIPNNGPAFCA.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FIPNNGPAFCA {
    fn default() -> &'a FIPNNGPAFCA {
        <FIPNNGPAFCA as ::protobuf::Message>::default_instance()
    }
}

impl FIPNNGPAFCA {
    pub fn new() -> FIPNNGPAFCA {
        ::std::default::Default::default()
    }

    // .PNCEOHPFFON DGGLJECCBMB = 3;

    pub fn DGGLJECCBMB(&self) -> &super::PNCEOHPFFON::PNCEOHPFFON {
        match self.FPHJPCBPDHC {
            ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::DGGLJECCBMB(ref v)) => v,
            _ => <super::PNCEOHPFFON::PNCEOHPFFON as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_DGGLJECCBMB(&mut self) {
        self.FPHJPCBPDHC = ::std::option::Option::None;
    }

    pub fn has_DGGLJECCBMB(&self) -> bool {
        match self.FPHJPCBPDHC {
            ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::DGGLJECCBMB(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_DGGLJECCBMB(&mut self, v: super::PNCEOHPFFON::PNCEOHPFFON) {
        self.FPHJPCBPDHC = ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::DGGLJECCBMB(v))
    }

    // Mutable pointer to the field.
    pub fn mut_DGGLJECCBMB(&mut self) -> &mut super::PNCEOHPFFON::PNCEOHPFFON {
        if let ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::DGGLJECCBMB(_)) = self.FPHJPCBPDHC {
        } else {
            self.FPHJPCBPDHC = ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::DGGLJECCBMB(super::PNCEOHPFFON::PNCEOHPFFON::new()));
        }
        match self.FPHJPCBPDHC {
            ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::DGGLJECCBMB(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_DGGLJECCBMB(&mut self) -> super::PNCEOHPFFON::PNCEOHPFFON {
        if self.has_DGGLJECCBMB() {
            match self.FPHJPCBPDHC.take() {
                ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::DGGLJECCBMB(v)) => v,
                _ => panic!(),
            }
        } else {
            super::PNCEOHPFFON::PNCEOHPFFON::new()
        }
    }

    // .HELBEIHMGKK KIOAHBIDMPC = 9;

    pub fn KIOAHBIDMPC(&self) -> &super::HELBEIHMGKK::HELBEIHMGKK {
        match self.FPHJPCBPDHC {
            ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIOAHBIDMPC(ref v)) => v,
            _ => <super::HELBEIHMGKK::HELBEIHMGKK as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_KIOAHBIDMPC(&mut self) {
        self.FPHJPCBPDHC = ::std::option::Option::None;
    }

    pub fn has_KIOAHBIDMPC(&self) -> bool {
        match self.FPHJPCBPDHC {
            ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIOAHBIDMPC(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KIOAHBIDMPC(&mut self, v: super::HELBEIHMGKK::HELBEIHMGKK) {
        self.FPHJPCBPDHC = ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIOAHBIDMPC(v))
    }

    // Mutable pointer to the field.
    pub fn mut_KIOAHBIDMPC(&mut self) -> &mut super::HELBEIHMGKK::HELBEIHMGKK {
        if let ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIOAHBIDMPC(_)) = self.FPHJPCBPDHC {
        } else {
            self.FPHJPCBPDHC = ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIOAHBIDMPC(super::HELBEIHMGKK::HELBEIHMGKK::new()));
        }
        match self.FPHJPCBPDHC {
            ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIOAHBIDMPC(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_KIOAHBIDMPC(&mut self) -> super::HELBEIHMGKK::HELBEIHMGKK {
        if self.has_KIOAHBIDMPC() {
            match self.FPHJPCBPDHC.take() {
                ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIOAHBIDMPC(v)) => v,
                _ => panic!(),
            }
        } else {
            super::HELBEIHMGKK::HELBEIHMGKK::new()
        }
    }

    // .JCBAJJIFBKN KIODDJOCDML = 7;

    pub fn KIODDJOCDML(&self) -> &super::JCBAJJIFBKN::JCBAJJIFBKN {
        match self.FPHJPCBPDHC {
            ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIODDJOCDML(ref v)) => v,
            _ => <super::JCBAJJIFBKN::JCBAJJIFBKN as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_KIODDJOCDML(&mut self) {
        self.FPHJPCBPDHC = ::std::option::Option::None;
    }

    pub fn has_KIODDJOCDML(&self) -> bool {
        match self.FPHJPCBPDHC {
            ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIODDJOCDML(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KIODDJOCDML(&mut self, v: super::JCBAJJIFBKN::JCBAJJIFBKN) {
        self.FPHJPCBPDHC = ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIODDJOCDML(v))
    }

    // Mutable pointer to the field.
    pub fn mut_KIODDJOCDML(&mut self) -> &mut super::JCBAJJIFBKN::JCBAJJIFBKN {
        if let ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIODDJOCDML(_)) = self.FPHJPCBPDHC {
        } else {
            self.FPHJPCBPDHC = ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIODDJOCDML(super::JCBAJJIFBKN::JCBAJJIFBKN::new()));
        }
        match self.FPHJPCBPDHC {
            ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIODDJOCDML(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_KIODDJOCDML(&mut self) -> super::JCBAJJIFBKN::JCBAJJIFBKN {
        if self.has_KIODDJOCDML() {
            match self.FPHJPCBPDHC.take() {
                ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIODDJOCDML(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JCBAJJIFBKN::JCBAJJIFBKN::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GFDGGCBGGGD",
            |m: &FIPNNGPAFCA| { &m.GFDGGCBGGGD },
            |m: &mut FIPNNGPAFCA| { &mut m.GFDGGCBGGGD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ICMDONMEJGE",
            |m: &FIPNNGPAFCA| { &m.ICMDONMEJGE },
            |m: &mut FIPNNGPAFCA| { &mut m.ICMDONMEJGE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJPEFPBFALC",
            |m: &FIPNNGPAFCA| { &m.FJPEFPBFALC },
            |m: &mut FIPNNGPAFCA| { &mut m.FJPEFPBFALC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::PNCEOHPFFON::PNCEOHPFFON>(
            "DGGLJECCBMB",
            FIPNNGPAFCA::has_DGGLJECCBMB,
            FIPNNGPAFCA::DGGLJECCBMB,
            FIPNNGPAFCA::mut_DGGLJECCBMB,
            FIPNNGPAFCA::set_DGGLJECCBMB,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::HELBEIHMGKK::HELBEIHMGKK>(
            "KIOAHBIDMPC",
            FIPNNGPAFCA::has_KIOAHBIDMPC,
            FIPNNGPAFCA::KIOAHBIDMPC,
            FIPNNGPAFCA::mut_KIOAHBIDMPC,
            FIPNNGPAFCA::set_KIOAHBIDMPC,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JCBAJJIFBKN::JCBAJJIFBKN>(
            "KIODDJOCDML",
            FIPNNGPAFCA::has_KIODDJOCDML,
            FIPNNGPAFCA::KIODDJOCDML,
            FIPNNGPAFCA::mut_KIODDJOCDML,
            FIPNNGPAFCA::set_KIODDJOCDML,
        ));
        oneofs.push(fipnngpafca::FPHJPCBPDHC::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FIPNNGPAFCA>(
            "FIPNNGPAFCA",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FIPNNGPAFCA {
    const NAME: &'static str = "FIPNNGPAFCA";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.GFDGGCBGGGD = is.read_uint32()?;
                },
                96 => {
                    self.ICMDONMEJGE = is.read_uint32()?;
                },
                80 => {
                    self.FJPEFPBFALC = is.read_uint32()?;
                },
                26 => {
                    self.FPHJPCBPDHC = ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::DGGLJECCBMB(is.read_message()?));
                },
                74 => {
                    self.FPHJPCBPDHC = ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIOAHBIDMPC(is.read_message()?));
                },
                58 => {
                    self.FPHJPCBPDHC = ::std::option::Option::Some(fipnngpafca::FPHJPCBPDHC::KIODDJOCDML(is.read_message()?));
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
        if self.GFDGGCBGGGD != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.GFDGGCBGGGD);
        }
        if self.ICMDONMEJGE != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.ICMDONMEJGE);
        }
        if self.FJPEFPBFALC != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.FJPEFPBFALC);
        }
        if let ::std::option::Option::Some(ref v) = self.FPHJPCBPDHC {
            match v {
                &fipnngpafca::FPHJPCBPDHC::DGGLJECCBMB(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &fipnngpafca::FPHJPCBPDHC::KIOAHBIDMPC(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &fipnngpafca::FPHJPCBPDHC::KIODDJOCDML(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.GFDGGCBGGGD != 0 {
            os.write_uint32(14, self.GFDGGCBGGGD)?;
        }
        if self.ICMDONMEJGE != 0 {
            os.write_uint32(12, self.ICMDONMEJGE)?;
        }
        if self.FJPEFPBFALC != 0 {
            os.write_uint32(10, self.FJPEFPBFALC)?;
        }
        if let ::std::option::Option::Some(ref v) = self.FPHJPCBPDHC {
            match v {
                &fipnngpafca::FPHJPCBPDHC::DGGLJECCBMB(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
                },
                &fipnngpafca::FPHJPCBPDHC::KIOAHBIDMPC(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
                },
                &fipnngpafca::FPHJPCBPDHC::KIODDJOCDML(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
                },
            };
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

    fn new() -> FIPNNGPAFCA {
        FIPNNGPAFCA::new()
    }

    fn clear(&mut self) {
        self.GFDGGCBGGGD = 0;
        self.ICMDONMEJGE = 0;
        self.FJPEFPBFALC = 0;
        self.FPHJPCBPDHC = ::std::option::Option::None;
        self.FPHJPCBPDHC = ::std::option::Option::None;
        self.FPHJPCBPDHC = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FIPNNGPAFCA {
        static instance: FIPNNGPAFCA = FIPNNGPAFCA {
            GFDGGCBGGGD: 0,
            ICMDONMEJGE: 0,
            FJPEFPBFALC: 0,
            FPHJPCBPDHC: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FIPNNGPAFCA {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FIPNNGPAFCA").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FIPNNGPAFCA {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FIPNNGPAFCA {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `FIPNNGPAFCA`
pub mod fipnngpafca {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:FIPNNGPAFCA.FPHJPCBPDHC)
    pub enum FPHJPCBPDHC {
        // @@protoc_insertion_point(oneof_field:FIPNNGPAFCA.DGGLJECCBMB)
        DGGLJECCBMB(super::super::PNCEOHPFFON::PNCEOHPFFON),
        // @@protoc_insertion_point(oneof_field:FIPNNGPAFCA.KIOAHBIDMPC)
        KIOAHBIDMPC(super::super::HELBEIHMGKK::HELBEIHMGKK),
        // @@protoc_insertion_point(oneof_field:FIPNNGPAFCA.KIODDJOCDML)
        KIODDJOCDML(super::super::JCBAJJIFBKN::JCBAJJIFBKN),
    }

    impl ::protobuf::Oneof for FPHJPCBPDHC {
    }

    impl ::protobuf::OneofFull for FPHJPCBPDHC {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::FIPNNGPAFCA as ::protobuf::MessageFull>::descriptor().oneof_by_name("FPHJPCBPDHC").unwrap()).clone()
        }
    }

    impl FPHJPCBPDHC {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<FPHJPCBPDHC>("FPHJPCBPDHC")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FIPNNGPAFCA.proto\x1a\x11HELBEIHMGKK.proto\x1a\x11JCBAJJIFBKN.prot\
    o\x1a\x11PNCEOHPFFON.proto\"\x98\x02\n\x0bFIPNNGPAFCA\x12\x20\n\x0bGFDGG\
    CBGGGD\x18\x0e\x20\x01(\rR\x0bGFDGGCBGGGD\x12\x20\n\x0bICMDONMEJGE\x18\
    \x0c\x20\x01(\rR\x0bICMDONMEJGE\x12\x20\n\x0bFJPEFPBFALC\x18\n\x20\x01(\
    \rR\x0bFJPEFPBFALC\x120\n\x0bDGGLJECCBMB\x18\x03\x20\x01(\x0b2\x0c.PNCEO\
    HPFFONH\0R\x0bDGGLJECCBMB\x120\n\x0bKIOAHBIDMPC\x18\t\x20\x01(\x0b2\x0c.\
    HELBEIHMGKKH\0R\x0bKIOAHBIDMPC\x120\n\x0bKIODDJOCDML\x18\x07\x20\x01(\
    \x0b2\x0c.JCBAJJIFBKNH\0R\x0bKIODDJOCDMLB\r\n\x0bFPHJPCBPDHCb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::HELBEIHMGKK::file_descriptor().clone());
            deps.push(super::JCBAJJIFBKN::file_descriptor().clone());
            deps.push(super::PNCEOHPFFON::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FIPNNGPAFCA::generated_message_descriptor_data());
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