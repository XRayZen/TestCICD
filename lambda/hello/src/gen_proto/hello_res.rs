// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc 3.21.12
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

//! Generated file from `hello_res.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:hello_api.response.HelloResponse)
pub struct HelloResponse {
    // message fields
    // @@protoc_insertion_point(field:hello_api.response.HelloResponse.api_res_type)
    pub api_res_type: ::protobuf::EnumOrUnknown<ApiResType>,
    // message oneof groups
    pub api_res: ::std::option::Option<hello_response::Api_res>,
    // special fields
    // @@protoc_insertion_point(special_field:hello_api.response.HelloResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HelloResponse {
    fn default() -> &'a HelloResponse {
        <HelloResponse as ::protobuf::Message>::default_instance()
    }
}

impl HelloResponse {
    pub fn new() -> HelloResponse {
        ::std::default::Default::default()
    }

    // .hello_api.response.AddUserResponse add_user_response = 2;

    pub fn add_user_response(&self) -> &AddUserResponse {
        match self.api_res {
            ::std::option::Option::Some(hello_response::Api_res::AddUserResponse(ref v)) => v,
            _ => <AddUserResponse as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_add_user_response(&mut self) {
        self.api_res = ::std::option::Option::None;
    }

    pub fn has_add_user_response(&self) -> bool {
        match self.api_res {
            ::std::option::Option::Some(hello_response::Api_res::AddUserResponse(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_add_user_response(&mut self, v: AddUserResponse) {
        self.api_res = ::std::option::Option::Some(hello_response::Api_res::AddUserResponse(v))
    }

    // Mutable pointer to the field.
    pub fn mut_add_user_response(&mut self) -> &mut AddUserResponse {
        if let ::std::option::Option::Some(hello_response::Api_res::AddUserResponse(_)) = self.api_res {
        } else {
            self.api_res = ::std::option::Option::Some(hello_response::Api_res::AddUserResponse(AddUserResponse::new()));
        }
        match self.api_res {
            ::std::option::Option::Some(hello_response::Api_res::AddUserResponse(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_add_user_response(&mut self) -> AddUserResponse {
        if self.has_add_user_response() {
            match self.api_res.take() {
                ::std::option::Option::Some(hello_response::Api_res::AddUserResponse(v)) => v,
                _ => panic!(),
            }
        } else {
            AddUserResponse::new()
        }
    }

    // .hello_api.response.GetUserResponse get_user_response = 3;

    pub fn get_user_response(&self) -> &GetUserResponse {
        match self.api_res {
            ::std::option::Option::Some(hello_response::Api_res::GetUserResponse(ref v)) => v,
            _ => <GetUserResponse as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_get_user_response(&mut self) {
        self.api_res = ::std::option::Option::None;
    }

    pub fn has_get_user_response(&self) -> bool {
        match self.api_res {
            ::std::option::Option::Some(hello_response::Api_res::GetUserResponse(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_get_user_response(&mut self, v: GetUserResponse) {
        self.api_res = ::std::option::Option::Some(hello_response::Api_res::GetUserResponse(v))
    }

    // Mutable pointer to the field.
    pub fn mut_get_user_response(&mut self) -> &mut GetUserResponse {
        if let ::std::option::Option::Some(hello_response::Api_res::GetUserResponse(_)) = self.api_res {
        } else {
            self.api_res = ::std::option::Option::Some(hello_response::Api_res::GetUserResponse(GetUserResponse::new()));
        }
        match self.api_res {
            ::std::option::Option::Some(hello_response::Api_res::GetUserResponse(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_get_user_response(&mut self) -> GetUserResponse {
        if self.has_get_user_response() {
            match self.api_res.take() {
                ::std::option::Option::Some(hello_response::Api_res::GetUserResponse(v)) => v,
                _ => panic!(),
            }
        } else {
            GetUserResponse::new()
        }
    }

    // .hello_api.response.ErrorResponse error_response = 4;

    pub fn error_response(&self) -> &ErrorResponse {
        match self.api_res {
            ::std::option::Option::Some(hello_response::Api_res::ErrorResponse(ref v)) => v,
            _ => <ErrorResponse as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_error_response(&mut self) {
        self.api_res = ::std::option::Option::None;
    }

    pub fn has_error_response(&self) -> bool {
        match self.api_res {
            ::std::option::Option::Some(hello_response::Api_res::ErrorResponse(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_error_response(&mut self, v: ErrorResponse) {
        self.api_res = ::std::option::Option::Some(hello_response::Api_res::ErrorResponse(v))
    }

    // Mutable pointer to the field.
    pub fn mut_error_response(&mut self) -> &mut ErrorResponse {
        if let ::std::option::Option::Some(hello_response::Api_res::ErrorResponse(_)) = self.api_res {
        } else {
            self.api_res = ::std::option::Option::Some(hello_response::Api_res::ErrorResponse(ErrorResponse::new()));
        }
        match self.api_res {
            ::std::option::Option::Some(hello_response::Api_res::ErrorResponse(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_error_response(&mut self) -> ErrorResponse {
        if self.has_error_response() {
            match self.api_res.take() {
                ::std::option::Option::Some(hello_response::Api_res::ErrorResponse(v)) => v,
                _ => panic!(),
            }
        } else {
            ErrorResponse::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "api_res_type",
            |m: &HelloResponse| { &m.api_res_type },
            |m: &mut HelloResponse| { &mut m.api_res_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, AddUserResponse>(
            "add_user_response",
            HelloResponse::has_add_user_response,
            HelloResponse::add_user_response,
            HelloResponse::mut_add_user_response,
            HelloResponse::set_add_user_response,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, GetUserResponse>(
            "get_user_response",
            HelloResponse::has_get_user_response,
            HelloResponse::get_user_response,
            HelloResponse::mut_get_user_response,
            HelloResponse::set_get_user_response,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, ErrorResponse>(
            "error_response",
            HelloResponse::has_error_response,
            HelloResponse::error_response,
            HelloResponse::mut_error_response,
            HelloResponse::set_error_response,
        ));
        oneofs.push(hello_response::Api_res::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HelloResponse>(
            "HelloResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HelloResponse {
    const NAME: &'static str = "HelloResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.api_res_type = is.read_enum_or_unknown()?;
                },
                18 => {
                    self.api_res = ::std::option::Option::Some(hello_response::Api_res::AddUserResponse(is.read_message()?));
                },
                26 => {
                    self.api_res = ::std::option::Option::Some(hello_response::Api_res::GetUserResponse(is.read_message()?));
                },
                34 => {
                    self.api_res = ::std::option::Option::Some(hello_response::Api_res::ErrorResponse(is.read_message()?));
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
        if self.api_res_type != ::protobuf::EnumOrUnknown::new(ApiResType::API_RES_TYPE_UNSPECIFIED) {
            my_size += ::protobuf::rt::int32_size(1, self.api_res_type.value());
        }
        if let ::std::option::Option::Some(ref v) = self.api_res {
            match v {
                &hello_response::Api_res::AddUserResponse(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &hello_response::Api_res::GetUserResponse(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &hello_response::Api_res::ErrorResponse(ref v) => {
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
        if self.api_res_type != ::protobuf::EnumOrUnknown::new(ApiResType::API_RES_TYPE_UNSPECIFIED) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.api_res_type))?;
        }
        if let ::std::option::Option::Some(ref v) = self.api_res {
            match v {
                &hello_response::Api_res::AddUserResponse(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
                },
                &hello_response::Api_res::GetUserResponse(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
                },
                &hello_response::Api_res::ErrorResponse(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> HelloResponse {
        HelloResponse::new()
    }

    fn clear(&mut self) {
        self.api_res_type = ::protobuf::EnumOrUnknown::new(ApiResType::API_RES_TYPE_UNSPECIFIED);
        self.api_res = ::std::option::Option::None;
        self.api_res = ::std::option::Option::None;
        self.api_res = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HelloResponse {
        static instance: HelloResponse = HelloResponse {
            api_res_type: ::protobuf::EnumOrUnknown::from_i32(0),
            api_res: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HelloResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HelloResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HelloResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HelloResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `HelloResponse`
pub mod hello_response {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:hello_api.response.HelloResponse.api_res)
    pub enum Api_res {
        // @@protoc_insertion_point(oneof_field:hello_api.response.HelloResponse.add_user_response)
        AddUserResponse(super::AddUserResponse),
        // @@protoc_insertion_point(oneof_field:hello_api.response.HelloResponse.get_user_response)
        GetUserResponse(super::GetUserResponse),
        // @@protoc_insertion_point(oneof_field:hello_api.response.HelloResponse.error_response)
        ErrorResponse(super::ErrorResponse),
    }

    impl ::protobuf::Oneof for Api_res {
    }

    impl ::protobuf::OneofFull for Api_res {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::HelloResponse as ::protobuf::MessageFull>::descriptor().oneof_by_name("api_res").unwrap()).clone()
        }
    }

    impl Api_res {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Api_res>("api_res")
        }
    }
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:hello_api.response.AddUserResponse)
pub struct AddUserResponse {
    // message fields
    // @@protoc_insertion_point(field:hello_api.response.AddUserResponse.message)
    pub message: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:hello_api.response.AddUserResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AddUserResponse {
    fn default() -> &'a AddUserResponse {
        <AddUserResponse as ::protobuf::Message>::default_instance()
    }
}

impl AddUserResponse {
    pub fn new() -> AddUserResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "message",
            |m: &AddUserResponse| { &m.message },
            |m: &mut AddUserResponse| { &mut m.message },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AddUserResponse>(
            "AddUserResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AddUserResponse {
    const NAME: &'static str = "AddUserResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.message = is.read_string()?;
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
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.message.is_empty() {
            os.write_string(1, &self.message)?;
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

    fn new() -> AddUserResponse {
        AddUserResponse::new()
    }

    fn clear(&mut self) {
        self.message.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AddUserResponse {
        static instance: AddUserResponse = AddUserResponse {
            message: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AddUserResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AddUserResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AddUserResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AddUserResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:hello_api.response.GetUserResponse)
pub struct GetUserResponse {
    // message fields
    // @@protoc_insertion_point(field:hello_api.response.GetUserResponse.message)
    pub message: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:hello_api.response.GetUserResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetUserResponse {
    fn default() -> &'a GetUserResponse {
        <GetUserResponse as ::protobuf::Message>::default_instance()
    }
}

impl GetUserResponse {
    pub fn new() -> GetUserResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "message",
            |m: &GetUserResponse| { &m.message },
            |m: &mut GetUserResponse| { &mut m.message },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetUserResponse>(
            "GetUserResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetUserResponse {
    const NAME: &'static str = "GetUserResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.message = is.read_string()?;
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
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.message.is_empty() {
            os.write_string(1, &self.message)?;
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

    fn new() -> GetUserResponse {
        GetUserResponse::new()
    }

    fn clear(&mut self) {
        self.message.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetUserResponse {
        static instance: GetUserResponse = GetUserResponse {
            message: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetUserResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetUserResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetUserResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetUserResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:hello_api.response.ErrorResponse)
pub struct ErrorResponse {
    // message fields
    // @@protoc_insertion_point(field:hello_api.response.ErrorResponse.message)
    pub message: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:hello_api.response.ErrorResponse.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ErrorResponse {
    fn default() -> &'a ErrorResponse {
        <ErrorResponse as ::protobuf::Message>::default_instance()
    }
}

impl ErrorResponse {
    pub fn new() -> ErrorResponse {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "message",
            |m: &ErrorResponse| { &m.message },
            |m: &mut ErrorResponse| { &mut m.message },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ErrorResponse>(
            "ErrorResponse",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ErrorResponse {
    const NAME: &'static str = "ErrorResponse";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.message = is.read_string()?;
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
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.message);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.message.is_empty() {
            os.write_string(1, &self.message)?;
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

    fn new() -> ErrorResponse {
        ErrorResponse::new()
    }

    fn clear(&mut self) {
        self.message.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ErrorResponse {
        static instance: ErrorResponse = ErrorResponse {
            message: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ErrorResponse {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ErrorResponse").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ErrorResponse {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:hello_api.response.ApiResType)
pub enum ApiResType {
    // @@protoc_insertion_point(enum_value:hello_api.response.ApiResType.API_RES_TYPE_UNSPECIFIED)
    API_RES_TYPE_UNSPECIFIED = 0,
    // @@protoc_insertion_point(enum_value:hello_api.response.ApiResType.API_RES_TYPE_ERROR)
    API_RES_TYPE_ERROR = 1,
    // @@protoc_insertion_point(enum_value:hello_api.response.ApiResType.API_RES_TYPE_SUCCESS)
    API_RES_TYPE_SUCCESS = 2,
}

impl ::protobuf::Enum for ApiResType {
    const NAME: &'static str = "ApiResType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ApiResType> {
        match value {
            0 => ::std::option::Option::Some(ApiResType::API_RES_TYPE_UNSPECIFIED),
            1 => ::std::option::Option::Some(ApiResType::API_RES_TYPE_ERROR),
            2 => ::std::option::Option::Some(ApiResType::API_RES_TYPE_SUCCESS),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [ApiResType] = &[
        ApiResType::API_RES_TYPE_UNSPECIFIED,
        ApiResType::API_RES_TYPE_ERROR,
        ApiResType::API_RES_TYPE_SUCCESS,
    ];
}

impl ::protobuf::EnumFull for ApiResType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("ApiResType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for ApiResType {
    fn default() -> Self {
        ApiResType::API_RES_TYPE_UNSPECIFIED
    }
}

impl ApiResType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<ApiResType>("ApiResType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fhello_res.proto\x12\x12hello_api.response\"\xce\x02\n\rHelloRespon\
    se\x12@\n\x0capi_res_type\x18\x01\x20\x01(\x0e2\x1e.hello_api.response.A\
    piResTypeR\napiResType\x12Q\n\x11add_user_response\x18\x02\x20\x01(\x0b2\
    #.hello_api.response.AddUserResponseH\0R\x0faddUserResponse\x12Q\n\x11ge\
    t_user_response\x18\x03\x20\x01(\x0b2#.hello_api.response.GetUserRespons\
    eH\0R\x0fgetUserResponse\x12J\n\x0eerror_response\x18\x04\x20\x01(\x0b2!\
    .hello_api.response.ErrorResponseH\0R\rerrorResponseB\t\n\x07api_res\"+\
    \n\x0fAddUserResponse\x12\x18\n\x07message\x18\x01\x20\x01(\tR\x07messag\
    e\"+\n\x0fGetUserResponse\x12\x18\n\x07message\x18\x01\x20\x01(\tR\x07me\
    ssage\")\n\rErrorResponse\x12\x18\n\x07message\x18\x01\x20\x01(\tR\x07me\
    ssage*\\\n\nApiResType\x12\x1c\n\x18API_RES_TYPE_UNSPECIFIED\x10\0\x12\
    \x16\n\x12API_RES_TYPE_ERROR\x10\x01\x12\x18\n\x14API_RES_TYPE_SUCCESS\
    \x10\x02b\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(HelloResponse::generated_message_descriptor_data());
            messages.push(AddUserResponse::generated_message_descriptor_data());
            messages.push(GetUserResponse::generated_message_descriptor_data());
            messages.push(ErrorResponse::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(ApiResType::generated_enum_descriptor_data());
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
