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

//! Generated file from `PropValue.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PropValue)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PropValue {
    // message fields
    // @@protoc_insertion_point(field:PropValue.type)
    pub type_: u32,
    // @@protoc_insertion_point(field:PropValue.val)
    pub val: i64,
    // message oneof groups
    pub value: ::std::option::Option<prop_value::Value>,
    // special fields
    // @@protoc_insertion_point(special_field:PropValue.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PropValue {
    fn default() -> &'a PropValue {
        <PropValue as ::protobuf::Message>::default_instance()
    }
}

impl PropValue {
    pub fn new() -> PropValue {
        ::std::default::Default::default()
    }

    // int64 ival = 2;

    pub fn ival(&self) -> i64 {
        match self.value {
            ::std::option::Option::Some(prop_value::Value::Ival(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_ival(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_ival(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(prop_value::Value::Ival(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_ival(&mut self, v: i64) {
        self.value = ::std::option::Option::Some(prop_value::Value::Ival(v))
    }

    // float fval = 3;

    pub fn fval(&self) -> f32 {
        match self.value {
            ::std::option::Option::Some(prop_value::Value::Fval(v)) => v,
            _ => 0.,
        }
    }

    pub fn clear_fval(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_fval(&self) -> bool {
        match self.value {
            ::std::option::Option::Some(prop_value::Value::Fval(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_fval(&mut self, v: f32) {
        self.value = ::std::option::Option::Some(prop_value::Value::Fval(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &PropValue| { &m.type_ },
            |m: &mut PropValue| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "ival",
            PropValue::has_ival,
            PropValue::ival,
            PropValue::set_ival,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "fval",
            PropValue::has_fval,
            PropValue::fval,
            PropValue::set_fval,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "val",
            |m: &PropValue| { &m.val },
            |m: &mut PropValue| { &mut m.val },
        ));
        oneofs.push(prop_value::Value::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PropValue>(
            "PropValue",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PropValue {
    const NAME: &'static str = "PropValue";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.type_ = is.read_uint32()?;
                },
                16 => {
                    self.value = ::std::option::Option::Some(prop_value::Value::Ival(is.read_int64()?));
                },
                29 => {
                    self.value = ::std::option::Option::Some(prop_value::Value::Fval(is.read_float()?));
                },
                32 => {
                    self.val = is.read_int64()?;
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
        if self.type_ != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.type_);
        }
        if self.val != 0 {
            my_size += ::protobuf::rt::int64_size(4, self.val);
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &prop_value::Value::Ival(v) => {
                    my_size += ::protobuf::rt::int64_size(2, v);
                },
                &prop_value::Value::Fval(v) => {
                    my_size += 1 + 4;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.type_ != 0 {
            os.write_uint32(1, self.type_)?;
        }
        if self.val != 0 {
            os.write_int64(4, self.val)?;
        }
        if let ::std::option::Option::Some(ref v) = self.value {
            match v {
                &prop_value::Value::Ival(v) => {
                    os.write_int64(2, v)?;
                },
                &prop_value::Value::Fval(v) => {
                    os.write_float(3, v)?;
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

    fn new() -> PropValue {
        PropValue::new()
    }

    fn clear(&mut self) {
        self.type_ = 0;
        self.value = ::std::option::Option::None;
        self.value = ::std::option::Option::None;
        self.val = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PropValue {
        static instance: PropValue = PropValue {
            type_: 0,
            val: 0,
            value: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PropValue {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PropValue").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PropValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PropValue {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `PropValue`
pub mod prop_value {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:PropValue.value)
    pub enum Value {
        // @@protoc_insertion_point(oneof_field:PropValue.ival)
        Ival(i64),
        // @@protoc_insertion_point(oneof_field:PropValue.fval)
        Fval(f32),
    }

    impl ::protobuf::Oneof for Value {
    }

    impl ::protobuf::OneofFull for Value {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::PropValue as ::protobuf::MessageFull>::descriptor().oneof_by_name("value").unwrap()).clone()
        }
    }

    impl Value {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Value>("value")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fPropValue.proto\"f\n\tPropValue\x12\x12\n\x04type\x18\x01\x20\x01(\
    \rR\x04type\x12\x14\n\x04ival\x18\x02\x20\x01(\x03H\0R\x04ival\x12\x14\n\
    \x04fval\x18\x03\x20\x01(\x02H\0R\x04fval\x12\x10\n\x03val\x18\x04\x20\
    \x01(\x03R\x03valB\x07\n\x05valueB\x1b\n\x19emu.grasscutter.net.protob\
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
            messages.push(PropValue::generated_message_descriptor_data());
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