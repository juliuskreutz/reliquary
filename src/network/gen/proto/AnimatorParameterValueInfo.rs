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

//! Generated file from `AnimatorParameterValueInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AnimatorParameterValueInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AnimatorParameterValueInfo {
    // message fields
    // @@protoc_insertion_point(field:AnimatorParameterValueInfo.para_type)
    pub para_type: u32,
    // message oneof groups
    pub paraVal: ::std::option::Option<animator_parameter_value_info::ParaVal>,
    // special fields
    // @@protoc_insertion_point(special_field:AnimatorParameterValueInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AnimatorParameterValueInfo {
    fn default() -> &'a AnimatorParameterValueInfo {
        <AnimatorParameterValueInfo as ::protobuf::Message>::default_instance()
    }
}

impl AnimatorParameterValueInfo {
    pub fn new() -> AnimatorParameterValueInfo {
        ::std::default::Default::default()
    }

    // int32 int_val = 2;

    pub fn int_val(&self) -> i32 {
        match self.paraVal {
            ::std::option::Option::Some(animator_parameter_value_info::ParaVal::IntVal(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_int_val(&mut self) {
        self.paraVal = ::std::option::Option::None;
    }

    pub fn has_int_val(&self) -> bool {
        match self.paraVal {
            ::std::option::Option::Some(animator_parameter_value_info::ParaVal::IntVal(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_int_val(&mut self, v: i32) {
        self.paraVal = ::std::option::Option::Some(animator_parameter_value_info::ParaVal::IntVal(v))
    }

    // float float_val = 3;

    pub fn float_val(&self) -> f32 {
        match self.paraVal {
            ::std::option::Option::Some(animator_parameter_value_info::ParaVal::FloatVal(v)) => v,
            _ => 0.,
        }
    }

    pub fn clear_float_val(&mut self) {
        self.paraVal = ::std::option::Option::None;
    }

    pub fn has_float_val(&self) -> bool {
        match self.paraVal {
            ::std::option::Option::Some(animator_parameter_value_info::ParaVal::FloatVal(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_float_val(&mut self, v: f32) {
        self.paraVal = ::std::option::Option::Some(animator_parameter_value_info::ParaVal::FloatVal(v))
    }

    // bool bool_val = 4;

    pub fn bool_val(&self) -> bool {
        match self.paraVal {
            ::std::option::Option::Some(animator_parameter_value_info::ParaVal::BoolVal(v)) => v,
            _ => false,
        }
    }

    pub fn clear_bool_val(&mut self) {
        self.paraVal = ::std::option::Option::None;
    }

    pub fn has_bool_val(&self) -> bool {
        match self.paraVal {
            ::std::option::Option::Some(animator_parameter_value_info::ParaVal::BoolVal(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_bool_val(&mut self, v: bool) {
        self.paraVal = ::std::option::Option::Some(animator_parameter_value_info::ParaVal::BoolVal(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "para_type",
            |m: &AnimatorParameterValueInfo| { &m.para_type },
            |m: &mut AnimatorParameterValueInfo| { &mut m.para_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "int_val",
            AnimatorParameterValueInfo::has_int_val,
            AnimatorParameterValueInfo::int_val,
            AnimatorParameterValueInfo::set_int_val,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "float_val",
            AnimatorParameterValueInfo::has_float_val,
            AnimatorParameterValueInfo::float_val,
            AnimatorParameterValueInfo::set_float_val,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "bool_val",
            AnimatorParameterValueInfo::has_bool_val,
            AnimatorParameterValueInfo::bool_val,
            AnimatorParameterValueInfo::set_bool_val,
        ));
        oneofs.push(animator_parameter_value_info::ParaVal::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AnimatorParameterValueInfo>(
            "AnimatorParameterValueInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AnimatorParameterValueInfo {
    const NAME: &'static str = "AnimatorParameterValueInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.para_type = is.read_uint32()?;
                },
                16 => {
                    self.paraVal = ::std::option::Option::Some(animator_parameter_value_info::ParaVal::IntVal(is.read_int32()?));
                },
                29 => {
                    self.paraVal = ::std::option::Option::Some(animator_parameter_value_info::ParaVal::FloatVal(is.read_float()?));
                },
                32 => {
                    self.paraVal = ::std::option::Option::Some(animator_parameter_value_info::ParaVal::BoolVal(is.read_bool()?));
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
        if self.para_type != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.para_type);
        }
        if let ::std::option::Option::Some(ref v) = self.paraVal {
            match v {
                &animator_parameter_value_info::ParaVal::IntVal(v) => {
                    my_size += ::protobuf::rt::int32_size(2, v);
                },
                &animator_parameter_value_info::ParaVal::FloatVal(v) => {
                    my_size += 1 + 4;
                },
                &animator_parameter_value_info::ParaVal::BoolVal(v) => {
                    my_size += 1 + 1;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.para_type != 0 {
            os.write_uint32(1, self.para_type)?;
        }
        if let ::std::option::Option::Some(ref v) = self.paraVal {
            match v {
                &animator_parameter_value_info::ParaVal::IntVal(v) => {
                    os.write_int32(2, v)?;
                },
                &animator_parameter_value_info::ParaVal::FloatVal(v) => {
                    os.write_float(3, v)?;
                },
                &animator_parameter_value_info::ParaVal::BoolVal(v) => {
                    os.write_bool(4, v)?;
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

    fn new() -> AnimatorParameterValueInfo {
        AnimatorParameterValueInfo::new()
    }

    fn clear(&mut self) {
        self.para_type = 0;
        self.paraVal = ::std::option::Option::None;
        self.paraVal = ::std::option::Option::None;
        self.paraVal = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AnimatorParameterValueInfo {
        static instance: AnimatorParameterValueInfo = AnimatorParameterValueInfo {
            para_type: 0,
            paraVal: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AnimatorParameterValueInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AnimatorParameterValueInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AnimatorParameterValueInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AnimatorParameterValueInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `AnimatorParameterValueInfo`
pub mod animator_parameter_value_info {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:AnimatorParameterValueInfo.paraVal)
    pub enum ParaVal {
        // @@protoc_insertion_point(oneof_field:AnimatorParameterValueInfo.int_val)
        IntVal(i32),
        // @@protoc_insertion_point(oneof_field:AnimatorParameterValueInfo.float_val)
        FloatVal(f32),
        // @@protoc_insertion_point(oneof_field:AnimatorParameterValueInfo.bool_val)
        BoolVal(bool),
    }

    impl ::protobuf::Oneof for ParaVal {
    }

    impl ::protobuf::OneofFull for ParaVal {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::AnimatorParameterValueInfo as ::protobuf::MessageFull>::descriptor().oneof_by_name("paraVal").unwrap()).clone()
        }
    }

    impl ParaVal {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<ParaVal>("paraVal")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20AnimatorParameterValueInfo.proto\"\x9b\x01\n\x1aAnimatorParameterV\
    alueInfo\x12\x1b\n\tpara_type\x18\x01\x20\x01(\rR\x08paraType\x12\x19\n\
    \x07int_val\x18\x02\x20\x01(\x05H\0R\x06intVal\x12\x1d\n\tfloat_val\x18\
    \x03\x20\x01(\x02H\0R\x08floatVal\x12\x1b\n\x08bool_val\x18\x04\x20\x01(\
    \x08H\0R\x07boolValB\t\n\x07paraValB\x1b\n\x19emu.grasscutter.net.protob\
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
            messages.push(AnimatorParameterValueInfo::generated_message_descriptor_data());
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