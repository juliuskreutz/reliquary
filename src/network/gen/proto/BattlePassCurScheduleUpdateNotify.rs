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

//! Generated file from `BattlePassCurScheduleUpdateNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BattlePassCurScheduleUpdateNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BattlePassCurScheduleUpdateNotify {
    // message fields
    // @@protoc_insertion_point(field:BattlePassCurScheduleUpdateNotify.have_cur_schedule)
    pub have_cur_schedule: bool,
    // @@protoc_insertion_point(field:BattlePassCurScheduleUpdateNotify.cur_schedule)
    pub cur_schedule: ::protobuf::MessageField<super::BattlePassSchedule::BattlePassSchedule>,
    // special fields
    // @@protoc_insertion_point(special_field:BattlePassCurScheduleUpdateNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BattlePassCurScheduleUpdateNotify {
    fn default() -> &'a BattlePassCurScheduleUpdateNotify {
        <BattlePassCurScheduleUpdateNotify as ::protobuf::Message>::default_instance()
    }
}

impl BattlePassCurScheduleUpdateNotify {
    pub fn new() -> BattlePassCurScheduleUpdateNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "have_cur_schedule",
            |m: &BattlePassCurScheduleUpdateNotify| { &m.have_cur_schedule },
            |m: &mut BattlePassCurScheduleUpdateNotify| { &mut m.have_cur_schedule },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::BattlePassSchedule::BattlePassSchedule>(
            "cur_schedule",
            |m: &BattlePassCurScheduleUpdateNotify| { &m.cur_schedule },
            |m: &mut BattlePassCurScheduleUpdateNotify| { &mut m.cur_schedule },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BattlePassCurScheduleUpdateNotify>(
            "BattlePassCurScheduleUpdateNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BattlePassCurScheduleUpdateNotify {
    const NAME: &'static str = "BattlePassCurScheduleUpdateNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.have_cur_schedule = is.read_bool()?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.cur_schedule)?;
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
        if self.have_cur_schedule != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.cur_schedule.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.have_cur_schedule != false {
            os.write_bool(14, self.have_cur_schedule)?;
        }
        if let Some(v) = self.cur_schedule.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
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

    fn new() -> BattlePassCurScheduleUpdateNotify {
        BattlePassCurScheduleUpdateNotify::new()
    }

    fn clear(&mut self) {
        self.have_cur_schedule = false;
        self.cur_schedule.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BattlePassCurScheduleUpdateNotify {
        static instance: BattlePassCurScheduleUpdateNotify = BattlePassCurScheduleUpdateNotify {
            have_cur_schedule: false,
            cur_schedule: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BattlePassCurScheduleUpdateNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BattlePassCurScheduleUpdateNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BattlePassCurScheduleUpdateNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BattlePassCurScheduleUpdateNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'BattlePassCurScheduleUpdateNotify.proto\x1a\x18BattlePassSchedule.pro\
    to\"\x87\x01\n!BattlePassCurScheduleUpdateNotify\x12*\n\x11have_cur_sche\
    dule\x18\x0e\x20\x01(\x08R\x0fhaveCurSchedule\x126\n\x0ccur_schedule\x18\
    \x0f\x20\x01(\x0b2\x13.BattlePassScheduleR\x0bcurScheduleB\x1b\n\x19emu.\
    grasscutter.net.protob\x06proto3\
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
            deps.push(super::BattlePassSchedule::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BattlePassCurScheduleUpdateNotify::generated_message_descriptor_data());
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
