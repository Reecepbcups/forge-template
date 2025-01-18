///Module containing a contract's types and functions.
/**

```solidity
library ILayerServiceManager {
    struct Payload { ILayerTrigger.TriggerId triggerId; bytes data; }
    struct SignedPayload { Payload payload; bytes signature; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ILayerServiceManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct Payload { ILayerTrigger.TriggerId triggerId; bytes data; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Payload {
        pub triggerId: <ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
        pub data: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            ILayerTrigger::TriggerId,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<Payload> for UnderlyingRustTuple<'_> {
            fn from(value: Payload) -> Self {
                (value.triggerId, value.data)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Payload {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    triggerId: tuple.0,
                    data: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Payload {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Payload {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <ILayerTrigger::TriggerId as alloy_sol_types::SolType>::tokenize(
                        &self.triggerId,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Payload {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for Payload {
            const NAME: &'static str = "Payload";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Payload(uint64 triggerId,bytes data)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <ILayerTrigger::TriggerId as alloy_sol_types::SolType>::eip712_data_word(
                            &self.triggerId,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.data,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Payload {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <ILayerTrigger::TriggerId as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.triggerId,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.data,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <ILayerTrigger::TriggerId as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.triggerId,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct SignedPayload { Payload payload; bytes signature; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignedPayload {
        pub payload: <Payload as alloy::sol_types::SolType>::RustType,
        pub signature: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (Payload, alloy::sol_types::sol_data::Bytes);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Payload as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Bytes,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignedPayload> for UnderlyingRustTuple<'_> {
            fn from(value: SignedPayload) -> Self {
                (value.payload, value.signature)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignedPayload {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    payload: tuple.0,
                    signature: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignedPayload {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SignedPayload {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <Payload as alloy_sol_types::SolType>::tokenize(&self.payload),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SignedPayload {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SignedPayload {
            const NAME: &'static str = "SignedPayload";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignedPayload(Payload payload,bytes signature)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(<Payload as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(
                        <Payload as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <Payload as alloy_sol_types::SolType>::eip712_data_word(
                            &self.payload,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignedPayload {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <Payload as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.payload,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <Payload as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.payload,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ILayerServiceManager`](self) contract instance.

See the [wrapper's documentation](`ILayerServiceManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ILayerServiceManagerInstance<T, P, N> {
        ILayerServiceManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ILayerServiceManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ILayerServiceManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ILayerServiceManagerInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ILayerServiceManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ILayerServiceManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ILayerServiceManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ILayerServiceManager`](self) contract instance.

See the [wrapper's documentation](`ILayerServiceManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ILayerServiceManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ILayerServiceManagerInstance<T, P, N> {
            ILayerServiceManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ILayerServiceManagerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ILayerServiceManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library ILayerTrigger {
    type TriggerId is uint64;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ILayerTrigger {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TriggerId(u64);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<TriggerId> for u64 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<64>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl TriggerId {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u64) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u64 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for TriggerId {
            type RustType = u64;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                64,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TriggerId {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ILayerTrigger`](self) contract instance.

See the [wrapper's documentation](`ILayerTriggerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ILayerTriggerInstance<T, P, N> {
        ILayerTriggerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ILayerTrigger`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ILayerTrigger`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ILayerTriggerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ILayerTriggerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ILayerTriggerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ILayerTriggerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ILayerTrigger`](self) contract instance.

See the [wrapper's documentation](`ILayerTriggerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ILayerTriggerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ILayerTriggerInstance<T, P, N> {
            ILayerTriggerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ILayerTriggerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ILayerTriggerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library IRewardsCoordinator {
    struct OperatorDirectedRewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; OperatorReward[] operatorRewards; uint32 startTimestamp; uint32 duration; string description; }
    struct OperatorReward { address operator; uint256 amount; }
    struct RewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; uint256 amount; uint32 startTimestamp; uint32 duration; }
    struct StrategyAndMultiplier { address strategy; uint96 multiplier; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IRewardsCoordinator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct OperatorDirectedRewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; OperatorReward[] operatorRewards; uint32 startTimestamp; uint32 duration; string description; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorDirectedRewardsSubmission {
        pub strategiesAndMultipliers: alloy::sol_types::private::Vec<
            <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
        >,
        pub token: alloy::sol_types::private::Address,
        pub operatorRewards: alloy::sol_types::private::Vec<
            <OperatorReward as alloy::sol_types::SolType>::RustType,
        >,
        pub startTimestamp: u32,
        pub duration: u32,
        pub description: alloy::sol_types::private::String,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<StrategyAndMultiplier>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<OperatorReward>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<
                <OperatorReward as alloy::sol_types::SolType>::RustType,
            >,
            u32,
            u32,
            alloy::sol_types::private::String,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorDirectedRewardsSubmission>
        for UnderlyingRustTuple<'_> {
            fn from(value: OperatorDirectedRewardsSubmission) -> Self {
                (
                    value.strategiesAndMultipliers,
                    value.token,
                    value.operatorRewards,
                    value.startTimestamp,
                    value.duration,
                    value.description,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OperatorDirectedRewardsSubmission {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategiesAndMultipliers: tuple.0,
                    token: tuple.1,
                    operatorRewards: tuple.2,
                    startTimestamp: tuple.3,
                    duration: tuple.4,
                    description: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorDirectedRewardsSubmission {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for OperatorDirectedRewardsSubmission {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.strategiesAndMultipliers,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        OperatorReward,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorRewards),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.duration),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.description,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OperatorDirectedRewardsSubmission {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for OperatorDirectedRewardsSubmission {
            const NAME: &'static str = "OperatorDirectedRewardsSubmission";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorDirectedRewardsSubmission(StrategyAndMultiplier[] strategiesAndMultipliers,address token,OperatorReward[] operatorRewards,uint32 startTimestamp,uint32 duration,string description)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(
                        <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <OperatorReward as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <OperatorReward as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategiesAndMultipliers,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        OperatorReward,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operatorRewards,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.duration)
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.description,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorDirectedRewardsSubmission {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategiesAndMultipliers,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        OperatorReward,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorRewards,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.duration,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.description,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    StrategyAndMultiplier,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategiesAndMultipliers,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    OperatorReward,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorRewards,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.duration,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.description,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct OperatorReward { address operator; uint256 amount; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorReward {
        pub operator: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<OperatorReward> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorReward) -> Self {
                (value.operator, value.amount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorReward {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operator: tuple.0,
                    amount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorReward {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorReward {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for OperatorReward {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for OperatorReward {
            const NAME: &'static str = "OperatorReward";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorReward(address operator,uint256 amount)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operator,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorReward {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operator,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct RewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; uint256 amount; uint32 startTimestamp; uint32 duration; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RewardsSubmission {
        pub strategiesAndMultipliers: alloy::sol_types::private::Vec<
            <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
        >,
        pub token: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub startTimestamp: u32,
        pub duration: u32,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Array<StrategyAndMultiplier>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            u32,
            u32,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<RewardsSubmission> for UnderlyingRustTuple<'_> {
            fn from(value: RewardsSubmission) -> Self {
                (
                    value.strategiesAndMultipliers,
                    value.token,
                    value.amount,
                    value.startTimestamp,
                    value.duration,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RewardsSubmission {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategiesAndMultipliers: tuple.0,
                    token: tuple.1,
                    amount: tuple.2,
                    startTimestamp: tuple.3,
                    duration: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RewardsSubmission {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RewardsSubmission {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.strategiesAndMultipliers,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.duration),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for RewardsSubmission {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for RewardsSubmission {
            const NAME: &'static str = "RewardsSubmission";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RewardsSubmission(StrategyAndMultiplier[] strategiesAndMultipliers,address token,uint256 amount,uint32 startTimestamp,uint32 duration)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategiesAndMultipliers,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.duration)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RewardsSubmission {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategiesAndMultipliers,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.duration,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Array<
                    StrategyAndMultiplier,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategiesAndMultipliers,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.duration,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct StrategyAndMultiplier { address strategy; uint96 multiplier; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyAndMultiplier {
        pub strategy: alloy::sol_types::private::Address,
        pub multiplier: alloy::sol_types::private::primitives::aliases::U96,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U96,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StrategyAndMultiplier> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyAndMultiplier) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyAndMultiplier {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyAndMultiplier {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyAndMultiplier {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.multiplier),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for StrategyAndMultiplier {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for StrategyAndMultiplier {
            const NAME: &'static str = "StrategyAndMultiplier";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyAndMultiplier(address strategy,uint96 multiplier)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.multiplier)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StrategyAndMultiplier {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategy,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.multiplier,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategy,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.multiplier,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IRewardsCoordinator`](self) contract instance.

See the [wrapper's documentation](`IRewardsCoordinatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRewardsCoordinatorInstance<T, P, N> {
        IRewardsCoordinatorInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IRewardsCoordinator`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IRewardsCoordinator`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRewardsCoordinatorInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRewardsCoordinatorInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRewardsCoordinatorInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRewardsCoordinatorInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IRewardsCoordinator`](self) contract instance.

See the [wrapper's documentation](`IRewardsCoordinatorInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IRewardsCoordinatorInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRewardsCoordinatorInstance<T, P, N> {
            IRewardsCoordinatorInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRewardsCoordinatorInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IRewardsCoordinatorInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
///Module containing a contract's types and functions.
/**

```solidity
library ISignatureUtils {
    struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISignatureUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureWithSaltAndExpiry {
        pub signature: alloy::sol_types::private::Bytes,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U256,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SignatureWithSaltAndExpiry>
        for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithSaltAndExpiry) -> Self {
                (value.signature, value.salt, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SignatureWithSaltAndExpiry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signature: tuple.0,
                    salt: tuple.1,
                    expiry: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignatureWithSaltAndExpiry {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for SignatureWithSaltAndExpiry {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SignatureWithSaltAndExpiry {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SignatureWithSaltAndExpiry {
            const NAME: &'static str = "SignatureWithSaltAndExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithSaltAndExpiry(bytes signature,bytes32 salt,uint256 expiry)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signature,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.salt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.expiry)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignatureWithSaltAndExpiry {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.salt)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expiry,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.salt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expiry,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISignatureUtilsInstance<T, P, N> {
        ISignatureUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISignatureUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ISignatureUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISignatureUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISignatureUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISignatureUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISignatureUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> ISignatureUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISignatureUtilsInstance<T, P, N> {
            ISignatureUtilsInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISignatureUtilsInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISignatureUtilsInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library ILayerServiceManager {
    struct Payload {
        ILayerTrigger.TriggerId triggerId;
        bytes data;
    }
    struct SignedPayload {
        Payload payload;
        bytes signature;
    }
}

library ILayerTrigger {
    type TriggerId is uint64;
}

library IRewardsCoordinator {
    struct OperatorDirectedRewardsSubmission {
        StrategyAndMultiplier[] strategiesAndMultipliers;
        address token;
        OperatorReward[] operatorRewards;
        uint32 startTimestamp;
        uint32 duration;
        string description;
    }
    struct OperatorReward {
        address operator;
        uint256 amount;
    }
    struct RewardsSubmission {
        StrategyAndMultiplier[] strategiesAndMultipliers;
        address token;
        uint256 amount;
        uint32 startTimestamp;
        uint32 duration;
    }
    struct StrategyAndMultiplier {
        address strategy;
        uint96 multiplier;
    }
}

library ISignatureUtils {
    struct SignatureWithSaltAndExpiry {
        bytes signature;
        bytes32 salt;
        uint256 expiry;
    }
}

interface WavsServiceManager {
    error InvalidSignature();

    event AddedSignedPayloadForTrigger(ILayerTrigger.TriggerId indexed triggerId);
    event Initialized(uint8 version);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator);

    constructor(address _avsDirectory, address _stakeRegistry, address _rewardsCoordinator, address _delegationManager);

    function addSignedPayloadForTrigger(ILayerServiceManager.SignedPayload memory signedPayload) external;
    function addSignedPayloadForTriggerMulti(ILayerServiceManager.SignedPayload[] memory signedPayloads) external;
    function avsDirectory() external view returns (address);
    function createAVSRewardsSubmission(IRewardsCoordinator.RewardsSubmission[] memory rewardsSubmissions) external;
    function createOperatorDirectedAVSRewardsSubmission(IRewardsCoordinator.OperatorDirectedRewardsSubmission[] memory operatorDirectedRewardsSubmissions) external;
    function deregisterOperatorFromAVS(address operator) external;
    function getOperatorRestakedStrategies(address _operator) external view returns (address[] memory);
    function getRestakeableStrategies() external view returns (address[] memory);
    function owner() external view returns (address);
    function registerOperatorToAVS(address operator, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function renounceOwnership() external;
    function rewardsInitiator() external view returns (address);
    function setClaimerFor(address claimer) external;
    function setRewardsInitiator(address newRewardsInitiator) external;
    function signedDataByTriggerId(ILayerTrigger.TriggerId) external view returns (bytes memory data, bytes memory signature);
    function stakeRegistry() external view returns (address);
    function transferOwnership(address newOwner) external;
    function updateAVSMetadataURI(string memory _metadataURI) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_avsDirectory",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_stakeRegistry",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_rewardsCoordinator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_delegationManager",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addSignedPayloadForTrigger",
    "inputs": [
      {
        "name": "signedPayload",
        "type": "tuple",
        "internalType": "struct ILayerServiceManager.SignedPayload",
        "components": [
          {
            "name": "payload",
            "type": "tuple",
            "internalType": "struct ILayerServiceManager.Payload",
            "components": [
              {
                "name": "triggerId",
                "type": "uint64",
                "internalType": "ILayerTrigger.TriggerId"
              },
              {
                "name": "data",
                "type": "bytes",
                "internalType": "bytes"
              }
            ]
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addSignedPayloadForTriggerMulti",
    "inputs": [
      {
        "name": "signedPayloads",
        "type": "tuple[]",
        "internalType": "struct ILayerServiceManager.SignedPayload[]",
        "components": [
          {
            "name": "payload",
            "type": "tuple",
            "internalType": "struct ILayerServiceManager.Payload",
            "components": [
              {
                "name": "triggerId",
                "type": "uint64",
                "internalType": "ILayerTrigger.TriggerId"
              },
              {
                "name": "data",
                "type": "bytes",
                "internalType": "bytes"
              }
            ]
          },
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "avsDirectory",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "createAVSRewardsSubmission",
    "inputs": [
      {
        "name": "rewardsSubmissions",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinator.RewardsSubmission[]",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinator.StrategyAndMultiplier[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          },
          {
            "name": "token",
            "type": "address",
            "internalType": "contract IERC20"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "duration",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createOperatorDirectedAVSRewardsSubmission",
    "inputs": [
      {
        "name": "operatorDirectedRewardsSubmissions",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinator.OperatorDirectedRewardsSubmission[]",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinator.StrategyAndMultiplier[]",
            "components": [
              {
                "name": "strategy",
                "type": "address",
                "internalType": "contract IStrategy"
              },
              {
                "name": "multiplier",
                "type": "uint96",
                "internalType": "uint96"
              }
            ]
          },
          {
            "name": "token",
            "type": "address",
            "internalType": "contract IERC20"
          },
          {
            "name": "operatorRewards",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinator.OperatorReward[]",
            "components": [
              {
                "name": "operator",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "startTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "duration",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "description",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperatorFromAVS",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getOperatorRestakedStrategies",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRestakeableStrategies",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registerOperatorToAVS",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSignature",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithSaltAndExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "expiry",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "rewardsInitiator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setClaimerFor",
    "inputs": [
      {
        "name": "claimer",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setRewardsInitiator",
    "inputs": [
      {
        "name": "newRewardsInitiator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "signedDataByTriggerId",
    "inputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "ILayerTrigger.TriggerId"
      }
    ],
    "outputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "stakeRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateAVSMetadataURI",
    "inputs": [
      {
        "name": "_metadataURI",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "AddedSignedPayloadForTrigger",
    "inputs": [
      {
        "name": "triggerId",
        "type": "uint64",
        "indexed": true,
        "internalType": "ILayerTrigger.TriggerId"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Initialized",
    "inputs": [
      {
        "name": "version",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "previousOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RewardsInitiatorUpdated",
    "inputs": [
      {
        "name": "prevRewardsInitiator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newRewardsInitiator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod WavsServiceManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610100604052348015610010575f80fd5b506040516126ad3803806126ad83398101604081905261002f9161013f565b6001600160a01b0380851660a05280841660805280831660c052811660e0528383838361005a610067565b5050505050505050610190565b5f54610100900460ff16156100d25760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b5f5460ff9081161015610122575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b80516001600160a01b038116811461013a575f80fd5b919050565b5f805f8060808587031215610152575f80fd5b61015b85610124565b935061016960208601610124565b925061017760408601610124565b915061018560608601610124565b905092959194509250565b60805160a05160c05160e0516124886102255f395f6109f701525f8181610d3c01528181610fd1015281816110930152818161113f015281816114610152818161152301526115f401525f818161019301528181610cbd0152818161118f01526111db01525f818161015f0152818161049201528181610583015281816107c4015281816108b7015261121501526124885ff3fe608060405234801561000f575f80fd5b50600436106100e4575f3560e01c8063276f85f2146100e857806333cfb7b7146101125780633bc28c8c1461013257806351caeaba14610147578063683048351461015a5780636b3aa72e1461018e578063715018a6146101b55780638da5cb5b146101bd5780639926ee7d146101c55780639d1e4053146101d8578063a0169ddd146101eb578063a20b99bf146101fe578063a364f4da14610211578063a98fb35514610224578063e481af9d14610237578063f2fde38b1461023f578063fc299dee14610252578063fce36c7d14610265575b5f80fd5b6100fb6100f6366004611646565b610278565b604051610109929190611694565b60405180910390f35b6101256101203660046116d5565b61039f565b60405161010991906116f0565b6101456101403660046116d5565b6103b0565b005b610145610155366004611782565b6103c4565b6101817f000000000000000000000000000000000000000000000000000000000000000081565b60405161010991906117c0565b6101817f000000000000000000000000000000000000000000000000000000000000000081565b610145610465565b610181610478565b6101456101d33660046118dc565b610487565b6101456101e6366004611984565b6104e6565b6101456101f93660046116d5565b610796565b61014561020c366004611782565b6107a7565b61014561021f3660046116d5565b6107b9565b6101456102323660046119ba565b61080a565b61012561081b565b61014561024d3660046116d5565b61082a565b606554610181906001600160a01b031681565b610145610273366004611782565b6108a0565b60976020525f908152604090208054819061029290611a06565b80601f01602080910402602001604051908101604052809291908181526020018280546102be90611a06565b80156103095780601f106102e057610100808354040283529160200191610309565b820191905f5260205f20905b8154815290600101906020018083116102ec57829003601f168201915b50505050509080600101805461031e90611a06565b80601f016020809104026020016040519081016040528092919081815260200182805461034a90611a06565b80156103955780601f1061036c57610100808354040283529160200191610395565b820191905f5260205f20905b81548152906001019060200180831161037857829003601f168201915b5050505050905082565b60606103aa826108b2565b92915050565b6103b8610b8d565b6103c181610bec565b50565b5f5b63ffffffff81168211156104605730639d1e4053848463ffffffff85168181106103f2576103f2611a3e565b90506020028101906104049190611a52565b6040518263ffffffff1660e01b81526004016104209190611b10565b5f604051808303815f87803b158015610437575f80fd5b505af1158015610449573d5f803e3d5ffd5b50505050808061045890611b82565b9150506103c6565b505050565b61046d610b8d565b6104765f610c55565b565b6033546001600160a01b031690565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146104d85760405162461bcd60e51b81526004016104cf90611ba6565b60405180910390fd5b6104e28282610ca6565b5050565b5f6104f18280611a52565b6040516020016105019190611c16565b6040516020818303038152906040528051906020012090505f61056f826040517b0ca2ba3432b932bab69029b4b3b732b21026b2b9b9b0b3b29d05199960211b6020820152603c81018290525f90605c01604051602081830303815290604052805190602001209050919050565b9050630b135d3f60e11b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016631626ba7e836105b66020880188611c28565b6040518463ffffffff1660e01b81526004016105d493929190611c6a565b602060405180830381865afa1580156105ef573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106139190611c83565b6001600160e01b031982811691161461063f57604051638baa579f60e01b815260040160405180910390fd5b604080518082019091525f90806106568780611a52565b610664906020810190611c28565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152505050908252506020908101906106ac90880188611c28565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201829052509390945250929350839250609791506106f48880611a52565b610702906020810190611646565b6001600160401b0316815260208101919091526040015f20815181906107289082611cee565b506020820151600182019061073d9082611cee565b5061074c915086905080611a52565b61075a906020810190611646565b6001600160401b03167f41b42d39b4c7441eedf9cb7c1a70d2f7771846ab84d9f245ec091d4dfa14855b60405160405180910390a25050505050565b61079e610b8d565b6103c181610d25565b6107af610da1565b6104e28282610e3b565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146108015760405162461bcd60e51b81526004016104cf90611ba6565b6103c181611178565b610812610b8d565b6103c1816111c4565b6060610825611210565b905090565b610832610b8d565b6001600160a01b0381166108975760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016104cf565b6103c181610c55565b6108a8610da1565b6104e28282611344565b60605f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316631703a0186040518163ffffffff1660e01b81526004015f60405180830381865afa158015610910573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526109379190810190611dde565b8051519091505f816001600160401b03811115610956576109566117d4565b60405190808252806020026020018201604052801561097f578160200160208202803683370190505b5090505f5b828110156109dd5783518051829081106109a0576109a0611a3e565b60200260200101515f01518282815181106109bd576109bd611a3e565b6001600160a01b0390921660209283029190910190910152600101610984565b50604051639004134760e01b81525f906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639004134790610a2e9089908690600401611ee3565b5f60405180830381865afa158015610a48573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610a6f9190810190611f41565b90505f805b84811015610ab3575f838281518110610a8f57610a8f611a3e565b60200260200101511115610aab5781610aa781611fc7565b9250505b600101610a74565b505f816001600160401b03811115610acd57610acd6117d4565b604051908082528060200260200182016040528015610af6578160200160208202803683370190505b5090505f805b86811015610b7f575f858281518110610b1757610b17611a3e565b60200260200101511115610b7757858181518110610b3757610b37611a3e565b6020026020010151838381518110610b5157610b51611a3e565b6001600160a01b039092166020928302919091019091015281610b7381611fc7565b9250505b600101610afc565b509098975050505050505050565b33610b96610478565b6001600160a01b0316146104765760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016104cf565b6065546040517fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e391610c2b916001600160a01b03909116908490611fdf565b60405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b604051639926ee7d60e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639926ee7d90610cf49085908590600401611ff9565b5f604051808303815f87803b158015610d0b575f80fd5b505af1158015610d1d573d5f803e3d5ffd5b505050505050565b60405163a0169ddd60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a0169ddd90610d719084906004016117c0565b5f604051808303815f87803b158015610d88575f80fd5b505af1158015610d9a573d5f803e3d5ffd5b5050505050565b6065546001600160a01b031633146104765760405162461bcd60e51b815260206004820152605160248201527f4543445341536572766963654d616e61676572426173652e6f6e6c795265776160448201527f726473496e69746961746f723a2063616c6c6572206973206e6f7420746865206064820152703932bbb0b932399034b734ba34b0ba37b960791b608482015260a4016104cf565b5f5b81811015611127575f805b848484818110610e5a57610e5a611a3e565b9050602002810190610e6c9190612043565b610e7a906040810190612057565b9050811015610ee457848484818110610e9557610e95611a3e565b9050602002810190610ea79190612043565b610eb5906040810190612057565b82818110610ec557610ec5611a3e565b9050604002016020013582610eda919061209c565b9150600101610e48565b50838383818110610ef757610ef7611a3e565b9050602002810190610f099190612043565b610f1a9060408101906020016116d5565b6001600160a01b03166323b872dd3330846040518463ffffffff1660e01b8152600401610f49939291906120af565b6020604051808303815f875af1158015610f65573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f8991906120d3565b505f848484818110610f9d57610f9d611a3e565b9050602002810190610faf9190612043565b610fc09060408101906020016116d5565b6001600160a01b031663dd62ed3e307f00000000000000000000000000000000000000000000000000000000000000006040518363ffffffff1660e01b815260040161100d929190611fdf565b602060405180830381865afa158015611028573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061104c91906120f2565b905084848481811061106057611060611a3e565b90506020028101906110729190612043565b6110839060408101906020016116d5565b6001600160a01b031663095ea7b37f00000000000000000000000000000000000000000000000000000000000000006110bc848661209c565b6040518363ffffffff1660e01b81526004016110d9929190612109565b6020604051808303815f875af11580156110f5573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061111991906120d3565b505050806001019050610e3d565b50604051634e5cd2fd60e11b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639cb9a5fa90610cf4903090869086906004016121dd565b6040516351b27a6d60e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a364f4da90610d719084906004016117c0565b60405163a98fb35560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a98fb35590610d7190849060040161234c565b60605f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316631703a0186040518163ffffffff1660e01b81526004015f60405180830381865afa15801561126e573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526112959190810190611dde565b90505f815f0151516001600160401b038111156112b4576112b46117d4565b6040519080825280602002602001820160405280156112dd578160200160208202803683370190505b5090505f5b82515181101561133d57825180518290811061130057611300611a3e565b60200260200101515f015182828151811061131d5761131d611a3e565b6001600160a01b03909216602092830291909101909101526001016112e2565b5092915050565b5f5b818110156115dc5782828281811061136057611360611a3e565b9050602002810190611372919061235e565b6113839060408101906020016116d5565b6001600160a01b03166323b872dd33308686868181106113a5576113a5611a3e565b90506020028101906113b7919061235e565b604001356040518463ffffffff1660e01b81526004016113d9939291906120af565b6020604051808303815f875af11580156113f5573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061141991906120d3565b505f83838381811061142d5761142d611a3e565b905060200281019061143f919061235e565b6114509060408101906020016116d5565b6001600160a01b031663dd62ed3e307f00000000000000000000000000000000000000000000000000000000000000006040518363ffffffff1660e01b815260040161149d929190611fdf565b602060405180830381865afa1580156114b8573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114dc91906120f2565b90508383838181106114f0576114f0611a3e565b9050602002810190611502919061235e565b6115139060408101906020016116d5565b6001600160a01b031663095ea7b37f00000000000000000000000000000000000000000000000000000000000000008387878781811061155557611555611a3e565b9050602002810190611567919061235e565b60400135611575919061209c565b6040518363ffffffff1660e01b8152600401611592929190612109565b6020604051808303815f875af11580156115ae573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115d291906120d3565b5050600101611346565b5060405163fce36c7d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063fce36c7d90610cf49085908590600401612372565b80356001600160401b0381168114611641575f80fd5b919050565b5f60208284031215611656575f80fd5b61165f8261162b565b9392505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b604081525f6116a66040830185611666565b82810360208401526116b88185611666565b95945050505050565b6001600160a01b03811681146103c1575f80fd5b5f602082840312156116e5575f80fd5b813561165f816116c1565b602080825282518282018190525f918401906040840190835b818110156117305783516001600160a01b0316835260209384019390920191600101611709565b509095945050505050565b5f8083601f84011261174b575f80fd5b5081356001600160401b03811115611761575f80fd5b6020830191508360208260051b850101111561177b575f80fd5b9250929050565b5f8060208385031215611793575f80fd5b82356001600160401b038111156117a8575f80fd5b6117b48582860161173b565b90969095509350505050565b6001600160a01b0391909116815260200190565b634e487b7160e01b5f52604160045260245ffd5b604051606081016001600160401b038111828210171561180a5761180a6117d4565b60405290565b604051602081016001600160401b038111828210171561180a5761180a6117d4565b604080519081016001600160401b038111828210171561180a5761180a6117d4565b604051601f8201601f191681016001600160401b038111828210171561187c5761187c6117d4565b604052919050565b5f806001600160401b0384111561189d5761189d6117d4565b50601f8301601f19166020016118b281611854565b9150508281528383830111156118c6575f80fd5b828260208301375f602084830101529392505050565b5f80604083850312156118ed575f80fd5b82356118f8816116c1565b915060208301356001600160401b03811115611912575f80fd5b830160608186031215611923575f80fd5b61192b6117e8565b81356001600160401b03811115611940575f80fd5b8201601f81018713611950575f80fd5b61195f87823560208401611884565b8252506020828101359082015260409182013591810191909152919491935090915050565b5f60208284031215611994575f80fd5b81356001600160401b038111156119a9575f80fd5b82016040818503121561165f575f80fd5b5f602082840312156119ca575f80fd5b81356001600160401b038111156119df575f80fd5b8201601f810184136119ef575f80fd5b6119fe84823560208401611884565b949350505050565b600181811c90821680611a1a57607f821691505b602082108103611a3857634e487b7160e01b5f52602260045260245ffd5b50919050565b634e487b7160e01b5f52603260045260245ffd5b5f8235603e19833603018112611a66575f80fd5b9190910192915050565b5f808335601e19843603018112611a85575f80fd5b83016020810192503590506001600160401b03811115611aa3575f80fd5b80360382131561177b575f80fd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b6001600160401b03611aea8261162b565b1682525f611afb6020830183611a70565b604060208601526116b8604086018284611ab1565b602081525f8235603e19843603018112611b28575f80fd5b60406020840152611b3e60608401858301611ad9565b9050611b4d6020850185611a70565b848303601f19016040860152611b64838284611ab1565b9695505050505050565b634e487b7160e01b5f52601160045260245ffd5b5f63ffffffff821663ffffffff8103611b9d57611b9d611b6e565b60010192915050565b6020808252604a908201527f4543445341536572766963654d616e61676572426173652e6f6e6c795374616b60408201527f6552656769737472793a2063616c6c6572206973206e6f7420746865207374616060820152696b65526567697374727960b01b608082015260a00190565b602081525f61165f6020830184611ad9565b5f808335601e19843603018112611c3d575f80fd5b8301803591506001600160401b03821115611c56575f80fd5b60200191503681900382131561177b575f80fd5b838152604060208201525f6116b8604083018486611ab1565b5f60208284031215611c93575f80fd5b81516001600160e01b03198116811461165f575f80fd5b601f82111561046057805f5260205f20601f840160051c81016020851015611ccf5750805b601f840160051c820191505b81811015610d9a575f8155600101611cdb565b81516001600160401b03811115611d0757611d076117d4565b611d1b81611d158454611a06565b84611caa565b6020601f821160018114611d4d575f8315611d365750848201515b5f19600385901b1c1916600184901b178455610d9a565b5f84815260208120601f198516915b82811015611d7c5787850151825560209485019460019092019101611d5c565b5084821015611d9957868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b5f6001600160401b03821115611dc057611dc06117d4565b5060051b60200190565b6001600160601b03811681146103c1575f80fd5b5f60208284031215611dee575f80fd5b81516001600160401b03811115611e03575f80fd5b820160208185031215611e14575f80fd5b611e1c611810565b81516001600160401b03811115611e31575f80fd5b80830192505084601f830112611e45575f80fd5b8151611e58611e5382611da8565b611854565b8082825260208201915060208360061b860101925087831115611e79575f80fd5b6020850194505b82851015611ed65760408589031215611e97575f80fd5b611e9f611832565b8551611eaa816116c1565b81526020860151611eba81611dca565b8060208301525080835250602082019150604085019450611e80565b8352509095945050505050565b6001600160a01b03831681526040602080830182905283519183018290525f91908401906060840190835b81811015611f355783516001600160a01b0316835260209384019390920191600101611f0e565b50909695505050505050565b5f60208284031215611f51575f80fd5b81516001600160401b03811115611f66575f80fd5b8201601f81018413611f76575f80fd5b8051611f84611e5382611da8565b8082825260208201915060208360051b850101925086831115611fa5575f80fd5b6020840193505b82841015611b64578351825260209384019390910190611fac565b5f60018201611fd857611fd8611b6e565b5060010190565b6001600160a01b0392831681529116602082015260400190565b60018060a01b0383168152604060208201525f82516060604084015261202260a0840182611666565b90506020840151606084015260408401516080840152809150509392505050565b5f823560be19833603018112611a66575f80fd5b5f808335601e1984360301811261206c575f80fd5b8301803591506001600160401b03821115612085575f80fd5b6020019150600681901b360382131561177b575f80fd5b808201808211156103aa576103aa611b6e565b6001600160a01b039384168152919092166020820152604081019190915260600190565b5f602082840312156120e3575f80fd5b8151801515811461165f575f80fd5b5f60208284031215612102575f80fd5b5051919050565b6001600160a01b03929092168252602082015260400190565b5f808335601e19843603018112612137575f80fd5b83016020810192503590506001600160401b03811115612155575f80fd5b8060061b360382131561177b575f80fd5b8183526020830192505f815f5b848110156121c0578135612186816116c1565b6001600160a01b03168652602082013561219f81611dca565b6001600160601b031660208701526040958601959190910190600101612173565b5093949350505050565b803563ffffffff81168114611641575f80fd5b6001600160a01b038416815260406020820181905281018290525f6060600584901b83018101908301858360be1936839003015b8782101561233e57868503605f19018452823581811261222f575f80fd5b890161223b8180612122565b60c0885261224d60c089018284612166565b915050602082013561225e816116c1565b6001600160a01b031660208801526122796040830183612122565b88830360408a015280835290915f91906020015b818310156122c85783356122a0816116c1565b6001600160a01b0316815260208481013590820152604093840193600193909301920161228d565b6122d4606086016121ca565b63ffffffff811660608c015293506122ee608086016121ca565b63ffffffff811660808c0152935061230960a0860186611a70565b9550935089810360a08b0152612320818686611ab1565b99505050505050602083019250602084019350600182019150612211565b509298975050505050505050565b602081525f61165f6020830184611666565b5f8235609e19833603018112611a66575f80fd5b602080825281018290525f6040600584901b830181019083018583609e1936839003015b8782101561244557868503603f1901845282358181126123b4575f80fd5b89016123c08180612122565b60a088526123d260a089018284612166565b91505060208201356123e3816116c1565b6001600160a01b031660208801526040828101359088015263ffffffff61240c606084016121ca565b16606088015263ffffffff612423608084016121ca565b1660808801528096505050602083019250602084019350600182019150612396565b509297965050505050505056fea2646970667358221220f47e5322e4fb51b75c1c9b2bafea1f62eff57e491547a9d903ed8e8d3c289a5064736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15a\0\x10W_\x80\xFD[P`@Qa&\xAD8\x03\x80a&\xAD\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01?V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\xA0R\x80\x84\x16`\x80R\x80\x83\x16`\xC0R\x81\x16`\xE0R\x83\x83\x83\x83a\0Za\0gV[PPPPPPPPa\x01\x90V[_Ta\x01\0\x90\x04`\xFF\x16\x15a\0\xD2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_T`\xFF\x90\x81\x16\x10\x15a\x01\"W_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01:W_\x80\xFD[\x91\x90PV[_\x80_\x80`\x80\x85\x87\x03\x12\x15a\x01RW_\x80\xFD[a\x01[\x85a\x01$V[\x93Pa\x01i` \x86\x01a\x01$V[\x92Pa\x01w`@\x86\x01a\x01$V[\x91Pa\x01\x85``\x86\x01a\x01$V[\x90P\x92\x95\x91\x94P\x92PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa$\x88a\x02%_9_a\t\xF7\x01R_\x81\x81a\r<\x01R\x81\x81a\x0F\xD1\x01R\x81\x81a\x10\x93\x01R\x81\x81a\x11?\x01R\x81\x81a\x14a\x01R\x81\x81a\x15#\x01Ra\x15\xF4\x01R_\x81\x81a\x01\x93\x01R\x81\x81a\x0C\xBD\x01R\x81\x81a\x11\x8F\x01Ra\x11\xDB\x01R_\x81\x81a\x01_\x01R\x81\x81a\x04\x92\x01R\x81\x81a\x05\x83\x01R\x81\x81a\x07\xC4\x01R\x81\x81a\x08\xB7\x01Ra\x12\x15\x01Ra$\x88_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xE4W_5`\xE0\x1C\x80c'o\x85\xF2\x14a\0\xE8W\x80c3\xCF\xB7\xB7\x14a\x01\x12W\x80c;\xC2\x8C\x8C\x14a\x012W\x80cQ\xCA\xEA\xBA\x14a\x01GW\x80ch0H5\x14a\x01ZW\x80ck:\xA7.\x14a\x01\x8EW\x80cqP\x18\xA6\x14a\x01\xB5W\x80c\x8D\xA5\xCB[\x14a\x01\xBDW\x80c\x99&\xEE}\x14a\x01\xC5W\x80c\x9D\x1E@S\x14a\x01\xD8W\x80c\xA0\x16\x9D\xDD\x14a\x01\xEBW\x80c\xA2\x0B\x99\xBF\x14a\x01\xFEW\x80c\xA3d\xF4\xDA\x14a\x02\x11W\x80c\xA9\x8F\xB3U\x14a\x02$W\x80c\xE4\x81\xAF\x9D\x14a\x027W\x80c\xF2\xFD\xE3\x8B\x14a\x02?W\x80c\xFC)\x9D\xEE\x14a\x02RW\x80c\xFC\xE3l}\x14a\x02eW[_\x80\xFD[a\0\xFBa\0\xF66`\x04a\x16FV[a\x02xV[`@Qa\x01\t\x92\x91\x90a\x16\x94V[`@Q\x80\x91\x03\x90\xF3[a\x01%a\x01 6`\x04a\x16\xD5V[a\x03\x9FV[`@Qa\x01\t\x91\x90a\x16\xF0V[a\x01Ea\x01@6`\x04a\x16\xD5V[a\x03\xB0V[\0[a\x01Ea\x01U6`\x04a\x17\x82V[a\x03\xC4V[a\x01\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qa\x01\t\x91\x90a\x17\xC0V[a\x01\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01Ea\x04eV[a\x01\x81a\x04xV[a\x01Ea\x01\xD36`\x04a\x18\xDCV[a\x04\x87V[a\x01Ea\x01\xE66`\x04a\x19\x84V[a\x04\xE6V[a\x01Ea\x01\xF96`\x04a\x16\xD5V[a\x07\x96V[a\x01Ea\x02\x0C6`\x04a\x17\x82V[a\x07\xA7V[a\x01Ea\x02\x1F6`\x04a\x16\xD5V[a\x07\xB9V[a\x01Ea\x0226`\x04a\x19\xBAV[a\x08\nV[a\x01%a\x08\x1BV[a\x01Ea\x02M6`\x04a\x16\xD5V[a\x08*V[`eTa\x01\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01Ea\x02s6`\x04a\x17\x82V[a\x08\xA0V[`\x97` R_\x90\x81R`@\x90 \x80T\x81\x90a\x02\x92\x90a\x1A\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xBE\x90a\x1A\x06V[\x80\x15a\x03\tW\x80`\x1F\x10a\x02\xE0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\tV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xECW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x03\x1E\x90a\x1A\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03J\x90a\x1A\x06V[\x80\x15a\x03\x95W\x80`\x1F\x10a\x03lWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\x95V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03xW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82V[``a\x03\xAA\x82a\x08\xB2V[\x92\x91PPV[a\x03\xB8a\x0B\x8DV[a\x03\xC1\x81a\x0B\xECV[PV[_[c\xFF\xFF\xFF\xFF\x81\x16\x82\x11\x15a\x04`W0c\x9D\x1E@S\x84\x84c\xFF\xFF\xFF\xFF\x85\x16\x81\x81\x10a\x03\xF2Wa\x03\xF2a\x1A>V[\x90P` \x02\x81\x01\x90a\x04\x04\x91\x90a\x1ARV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04 \x91\x90a\x1B\x10V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x047W_\x80\xFD[PZ\xF1\x15\x80\x15a\x04IW=_\x80>=_\xFD[PPPP\x80\x80a\x04X\x90a\x1B\x82V[\x91PPa\x03\xC6V[PPPV[a\x04ma\x0B\x8DV[a\x04v_a\x0CUV[V[`3T`\x01`\x01`\xA0\x1B\x03\x16\x90V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\xCF\x90a\x1B\xA6V[`@Q\x80\x91\x03\x90\xFD[a\x04\xE2\x82\x82a\x0C\xA6V[PPV[_a\x04\xF1\x82\x80a\x1ARV[`@Q` \x01a\x05\x01\x91\x90a\x1C\x16V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_a\x05o\x82`@Q{\x0C\xA2\xBA42\xB92\xBA\xB6\x90)\xB4\xB3\xB72\xB2\x10&\xB2\xB9\xB9\xB0\xB3\xB2\x9D\x05\x19\x99`!\x1B` \x82\x01R`<\x81\x01\x82\x90R_\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90Pc\x0B\x13]?`\xE1\x1B`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x16&\xBA~\x83a\x05\xB6` \x88\x01\x88a\x1C(V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xD4\x93\x92\x91\x90a\x1CjV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xEFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x13\x91\x90a\x1C\x83V[`\x01`\x01`\xE0\x1B\x03\x19\x82\x81\x16\x91\x16\x14a\x06?W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R_\x90\x80a\x06V\x87\x80a\x1ARV[a\x06d\x90` \x81\x01\x90a\x1C(V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x90\x81\x01\x90a\x06\xAC\x90\x88\x01\x88a\x1C(V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x82\x90RP\x93\x90\x94RP\x92\x93P\x83\x92P`\x97\x91Pa\x06\xF4\x88\x80a\x1ARV[a\x07\x02\x90` \x81\x01\x90a\x16FV[`\x01`\x01`@\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x81Q\x81\x90a\x07(\x90\x82a\x1C\xEEV[P` \x82\x01Q`\x01\x82\x01\x90a\x07=\x90\x82a\x1C\xEEV[Pa\x07L\x91P\x86\x90P\x80a\x1ARV[a\x07Z\x90` \x81\x01\x90a\x16FV[`\x01`\x01`@\x1B\x03\x16\x7FA\xB4-9\xB4\xC7D\x1E\xED\xF9\xCB|\x1Ap\xD2\xF7w\x18F\xAB\x84\xD9\xF2E\xEC\t\x1DM\xFA\x14\x85[`@Q`@Q\x80\x91\x03\x90\xA2PPPPPV[a\x07\x9Ea\x0B\x8DV[a\x03\xC1\x81a\r%V[a\x07\xAFa\r\xA1V[a\x04\xE2\x82\x82a\x0E;V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x08\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\xCF\x90a\x1B\xA6V[a\x03\xC1\x81a\x11xV[a\x08\x12a\x0B\x8DV[a\x03\xC1\x81a\x11\xC4V[``a\x08%a\x12\x10V[\x90P\x90V[a\x082a\x0B\x8DV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x08\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x04\xCFV[a\x03\xC1\x81a\x0CUV[a\x08\xA8a\r\xA1V[a\x04\xE2\x82\x82a\x13DV[``_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x17\x03\xA0\x18`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x10W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t7\x91\x90\x81\x01\x90a\x1D\xDEV[\x80QQ\x90\x91P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\tVWa\tVa\x17\xD4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\x7FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a\t\xDDW\x83Q\x80Q\x82\x90\x81\x10a\t\xA0Wa\t\xA0a\x1A>V[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a\t\xBDWa\t\xBDa\x1A>V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\t\x84V[P`@Qc\x90\x04\x13G`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x90\x04\x13G\x90a\n.\x90\x89\x90\x86\x90`\x04\x01a\x1E\xE3V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nHW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\no\x91\x90\x81\x01\x90a\x1FAV[\x90P_\x80[\x84\x81\x10\x15a\n\xB3W_\x83\x82\x81Q\x81\x10a\n\x8FWa\n\x8Fa\x1A>V[` \x02` \x01\x01Q\x11\x15a\n\xABW\x81a\n\xA7\x81a\x1F\xC7V[\x92PP[`\x01\x01a\ntV[P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xCDWa\n\xCDa\x17\xD4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xF6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x86\x81\x10\x15a\x0B\x7FW_\x85\x82\x81Q\x81\x10a\x0B\x17Wa\x0B\x17a\x1A>V[` \x02` \x01\x01Q\x11\x15a\x0BwW\x85\x81\x81Q\x81\x10a\x0B7Wa\x0B7a\x1A>V[` \x02` \x01\x01Q\x83\x83\x81Q\x81\x10a\x0BQWa\x0BQa\x1A>V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x81a\x0Bs\x81a\x1F\xC7V[\x92PP[`\x01\x01a\n\xFCV[P\x90\x98\x97PPPPPPPPV[3a\x0B\x96a\x04xV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x04\xCFV[`eT`@Q\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91a\x0C+\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x84\x90a\x1F\xDFV[`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x99&\xEE}\x90a\x0C\xF4\x90\x85\x90\x85\x90`\x04\x01a\x1F\xF9V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\r\x0BW_\x80\xFD[PZ\xF1\x15\x80\x15a\r\x1DW=_\x80>=_\xFD[PPPPPPV[`@Qc\xA0\x16\x9D\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA0\x16\x9D\xDD\x90a\rq\x90\x84\x90`\x04\x01a\x17\xC0V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\r\x88W_\x80\xFD[PZ\xF1\x15\x80\x15a\r\x9AW=_\x80>=_\xFD[PPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FECDSAServiceManagerBase.onlyRewa`D\x82\x01R\x7FrdsInitiator: caller is not the `d\x82\x01Rp92\xBB\xB0\xB929\x904\xB74\xBA4\xB0\xBA7\xB9`y\x1B`\x84\x82\x01R`\xA4\x01a\x04\xCFV[_[\x81\x81\x10\x15a\x11'W_\x80[\x84\x84\x84\x81\x81\x10a\x0EZWa\x0EZa\x1A>V[\x90P` \x02\x81\x01\x90a\x0El\x91\x90a CV[a\x0Ez\x90`@\x81\x01\x90a WV[\x90P\x81\x10\x15a\x0E\xE4W\x84\x84\x84\x81\x81\x10a\x0E\x95Wa\x0E\x95a\x1A>V[\x90P` \x02\x81\x01\x90a\x0E\xA7\x91\x90a CV[a\x0E\xB5\x90`@\x81\x01\x90a WV[\x82\x81\x81\x10a\x0E\xC5Wa\x0E\xC5a\x1A>V[\x90P`@\x02\x01` \x015\x82a\x0E\xDA\x91\x90a \x9CV[\x91P`\x01\x01a\x0EHV[P\x83\x83\x83\x81\x81\x10a\x0E\xF7Wa\x0E\xF7a\x1A>V[\x90P` \x02\x81\x01\x90a\x0F\t\x91\x90a CV[a\x0F\x1A\x90`@\x81\x01\x90` \x01a\x16\xD5V[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x84`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0FI\x93\x92\x91\x90a \xAFV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0FeW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x89\x91\x90a \xD3V[P_\x84\x84\x84\x81\x81\x10a\x0F\x9DWa\x0F\x9Da\x1A>V[\x90P` \x02\x81\x01\x90a\x0F\xAF\x91\x90a CV[a\x0F\xC0\x90`@\x81\x01\x90` \x01a\x16\xD5V[`\x01`\x01`\xA0\x1B\x03\x16c\xDDb\xED>0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\r\x92\x91\x90a\x1F\xDFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10(W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10L\x91\x90a \xF2V[\x90P\x84\x84\x84\x81\x81\x10a\x10`Wa\x10`a\x1A>V[\x90P` \x02\x81\x01\x90a\x10r\x91\x90a CV[a\x10\x83\x90`@\x81\x01\x90` \x01a\x16\xD5V[`\x01`\x01`\xA0\x1B\x03\x16c\t^\xA7\xB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\xBC\x84\x86a \x9CV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xD9\x92\x91\x90a!\tV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10\xF5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x19\x91\x90a \xD3V[PPP\x80`\x01\x01\x90Pa\x0E=V[P`@QcN\\\xD2\xFD`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x9C\xB9\xA5\xFA\x90a\x0C\xF4\x900\x90\x86\x90\x86\x90`\x04\x01a!\xDDV[`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90a\rq\x90\x84\x90`\x04\x01a\x17\xC0V[`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x8F\xB3U\x90a\rq\x90\x84\x90`\x04\x01a#LV[``_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x17\x03\xA0\x18`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12nW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\x95\x91\x90\x81\x01\x90a\x1D\xDEV[\x90P_\x81_\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xB4Wa\x12\xB4a\x17\xD4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xDDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82QQ\x81\x10\x15a\x13=W\x82Q\x80Q\x82\x90\x81\x10a\x13\0Wa\x13\0a\x1A>V[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a\x13\x1DWa\x13\x1Da\x1A>V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x12\xE2V[P\x92\x91PPV[_[\x81\x81\x10\x15a\x15\xDCW\x82\x82\x82\x81\x81\x10a\x13`Wa\x13`a\x1A>V[\x90P` \x02\x81\x01\x90a\x13r\x91\x90a#^V[a\x13\x83\x90`@\x81\x01\x90` \x01a\x16\xD5V[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x86\x86\x86\x81\x81\x10a\x13\xA5Wa\x13\xA5a\x1A>V[\x90P` \x02\x81\x01\x90a\x13\xB7\x91\x90a#^V[`@\x015`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xD9\x93\x92\x91\x90a \xAFV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\xF5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x19\x91\x90a \xD3V[P_\x83\x83\x83\x81\x81\x10a\x14-Wa\x14-a\x1A>V[\x90P` \x02\x81\x01\x90a\x14?\x91\x90a#^V[a\x14P\x90`@\x81\x01\x90` \x01a\x16\xD5V[`\x01`\x01`\xA0\x1B\x03\x16c\xDDb\xED>0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x9D\x92\x91\x90a\x1F\xDFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xB8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xDC\x91\x90a \xF2V[\x90P\x83\x83\x83\x81\x81\x10a\x14\xF0Wa\x14\xF0a\x1A>V[\x90P` \x02\x81\x01\x90a\x15\x02\x91\x90a#^V[a\x15\x13\x90`@\x81\x01\x90` \x01a\x16\xD5V[`\x01`\x01`\xA0\x1B\x03\x16c\t^\xA7\xB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x87\x87\x87\x81\x81\x10a\x15UWa\x15Ua\x1A>V[\x90P` \x02\x81\x01\x90a\x15g\x91\x90a#^V[`@\x015a\x15u\x91\x90a \x9CV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\x92\x92\x91\x90a!\tV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15\xAEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xD2\x91\x90a \xD3V[PP`\x01\x01a\x13FV[P`@Qc\xFC\xE3l}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFC\xE3l}\x90a\x0C\xF4\x90\x85\x90\x85\x90`\x04\x01a#rV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x16AW_\x80\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x16VW_\x80\xFD[a\x16_\x82a\x16+V[\x93\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`@\x81R_a\x16\xA6`@\x83\x01\x85a\x16fV[\x82\x81\x03` \x84\x01Ra\x16\xB8\x81\x85a\x16fV[\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xC1W_\x80\xFD[_` \x82\x84\x03\x12\x15a\x16\xE5W_\x80\xFD[\x815a\x16_\x81a\x16\xC1V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x170W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x17\tV[P\x90\x95\x94PPPPPV[_\x80\x83`\x1F\x84\x01\x12a\x17KW_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17aW_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x17{W_\x80\xFD[\x92P\x92\x90PV[_\x80` \x83\x85\x03\x12\x15a\x17\x93W_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xA8W_\x80\xFD[a\x17\xB4\x85\x82\x86\x01a\x17;V[\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x18\nWa\x18\na\x17\xD4V[`@R\x90V[`@Q` \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x18\nWa\x18\na\x17\xD4V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x18\nWa\x18\na\x17\xD4V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x18|Wa\x18|a\x17\xD4V[`@R\x91\x90PV[_\x80`\x01`\x01`@\x1B\x03\x84\x11\x15a\x18\x9DWa\x18\x9Da\x17\xD4V[P`\x1F\x83\x01`\x1F\x19\x16` \x01a\x18\xB2\x81a\x18TV[\x91PP\x82\x81R\x83\x83\x83\x01\x11\x15a\x18\xC6W_\x80\xFD[\x82\x82` \x83\x017_` \x84\x83\x01\x01R\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\x18\xEDW_\x80\xFD[\x825a\x18\xF8\x81a\x16\xC1V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\x12W_\x80\xFD[\x83\x01``\x81\x86\x03\x12\x15a\x19#W_\x80\xFD[a\x19+a\x17\xE8V[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19@W_\x80\xFD[\x82\x01`\x1F\x81\x01\x87\x13a\x19PW_\x80\xFD[a\x19_\x87\x825` \x84\x01a\x18\x84V[\x82RP` \x82\x81\x015\x90\x82\x01R`@\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x91\x94\x91\x93P\x90\x91PPV[_` \x82\x84\x03\x12\x15a\x19\x94W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xA9W_\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x16_W_\x80\xFD[_` \x82\x84\x03\x12\x15a\x19\xCAW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xDFW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x19\xEFW_\x80\xFD[a\x19\xFE\x84\x825` \x84\x01a\x18\x84V[\x94\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1A\x1AW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1A8WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12a\x1AfW_\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1A\x85W_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xA3W_\x80\xFD[\x806\x03\x82\x13\x15a\x17{W_\x80\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`@\x1B\x03a\x1A\xEA\x82a\x16+V[\x16\x82R_a\x1A\xFB` \x83\x01\x83a\x1ApV[`@` \x86\x01Ra\x16\xB8`@\x86\x01\x82\x84a\x1A\xB1V[` \x81R_\x825`>\x19\x846\x03\x01\x81\x12a\x1B(W_\x80\xFD[`@` \x84\x01Ra\x1B>``\x84\x01\x85\x83\x01a\x1A\xD9V[\x90Pa\x1BM` \x85\x01\x85a\x1ApV[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x1Bd\x83\x82\x84a\x1A\xB1V[\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a\x1B\x9DWa\x1B\x9Da\x1BnV[`\x01\x01\x92\x91PPV[` \x80\x82R`J\x90\x82\x01R\x7FECDSAServiceManagerBase.onlyStak`@\x82\x01R\x7FeRegistry: caller is not the sta``\x82\x01RikeRegistry`\xB0\x1B`\x80\x82\x01R`\xA0\x01\x90V[` \x81R_a\x16_` \x83\x01\x84a\x1A\xD9V[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1C=W_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1CVW_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x17{W_\x80\xFD[\x83\x81R`@` \x82\x01R_a\x16\xB8`@\x83\x01\x84\x86a\x1A\xB1V[_` \x82\x84\x03\x12\x15a\x1C\x93W_\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x16_W_\x80\xFD[`\x1F\x82\x11\x15a\x04`W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x1C\xCFWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\r\x9AW_\x81U`\x01\x01a\x1C\xDBV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x07Wa\x1D\x07a\x17\xD4V[a\x1D\x1B\x81a\x1D\x15\x84Ta\x1A\x06V[\x84a\x1C\xAAV[` `\x1F\x82\x11`\x01\x81\x14a\x1DMW_\x83\x15a\x1D6WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\r\x9AV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x1D|W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1D\\V[P\x84\x82\x10\x15a\x1D\x99W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1D\xC0Wa\x1D\xC0a\x17\xD4V[P`\x05\x1B` \x01\x90V[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x03\xC1W_\x80\xFD[_` \x82\x84\x03\x12\x15a\x1D\xEEW_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\x03W_\x80\xFD[\x82\x01` \x81\x85\x03\x12\x15a\x1E\x14W_\x80\xFD[a\x1E\x1Ca\x18\x10V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E1W_\x80\xFD[\x80\x83\x01\x92PP\x84`\x1F\x83\x01\x12a\x1EEW_\x80\xFD[\x81Qa\x1EXa\x1ES\x82a\x1D\xA8V[a\x18TV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x87\x83\x11\x15a\x1EyW_\x80\xFD[` \x85\x01\x94P[\x82\x85\x10\x15a\x1E\xD6W`@\x85\x89\x03\x12\x15a\x1E\x97W_\x80\xFD[a\x1E\x9Fa\x182V[\x85Qa\x1E\xAA\x81a\x16\xC1V[\x81R` \x86\x01Qa\x1E\xBA\x81a\x1D\xCAV[\x80` \x83\x01RP\x80\x83RP` \x82\x01\x91P`@\x85\x01\x94Pa\x1E\x80V[\x83RP\x90\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x80\x83\x01\x82\x90R\x83Q\x91\x83\x01\x82\x90R_\x91\x90\x84\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a\x1F5W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1F\x0EV[P\x90\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\x1FQW_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FfW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x1FvW_\x80\xFD[\x80Qa\x1F\x84a\x1ES\x82a\x1D\xA8V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a\x1F\xA5W_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x1BdW\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a\x1F\xACV[_`\x01\x82\x01a\x1F\xD8Wa\x1F\xD8a\x1BnV[P`\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_\x82Q```@\x84\x01Ra \"`\xA0\x84\x01\x82a\x16fV[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[_\x825`\xBE\x19\x836\x03\x01\x81\x12a\x1AfW_\x80\xFD[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a lW_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a \x85W_\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a\x17{W_\x80\xFD[\x80\x82\x01\x80\x82\x11\x15a\x03\xAAWa\x03\xAAa\x1BnV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[_` \x82\x84\x03\x12\x15a \xE3W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x16_W_\x80\xFD[_` \x82\x84\x03\x12\x15a!\x02W_\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a!7W_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a!UW_\x80\xFD[\x80`\x06\x1B6\x03\x82\x13\x15a\x17{W_\x80\xFD[\x81\x83R` \x83\x01\x92P_\x81_[\x84\x81\x10\x15a!\xC0W\x815a!\x86\x81a\x16\xC1V[`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x82\x015a!\x9F\x81a\x1D\xCAV[`\x01`\x01``\x1B\x03\x16` \x87\x01R`@\x95\x86\x01\x95\x91\x90\x91\x01\x90`\x01\x01a!sV[P\x93\x94\x93PPPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x16AW_\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R_```\x05\x84\x90\x1B\x83\x01\x81\x01\x90\x83\x01\x85\x83`\xBE\x196\x83\x90\x03\x01[\x87\x82\x10\x15a#>W\x86\x85\x03`_\x19\x01\x84R\x825\x81\x81\x12a\"/W_\x80\xFD[\x89\x01a\";\x81\x80a!\"V[`\xC0\x88Ra\"M`\xC0\x89\x01\x82\x84a!fV[\x91PP` \x82\x015a\"^\x81a\x16\xC1V[`\x01`\x01`\xA0\x1B\x03\x16` \x88\x01Ra\"y`@\x83\x01\x83a!\"V[\x88\x83\x03`@\x8A\x01R\x80\x83R\x90\x91_\x91\x90` \x01[\x81\x83\x10\x15a\"\xC8W\x835a\"\xA0\x81a\x16\xC1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x84\x81\x015\x90\x82\x01R`@\x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x01a\"\x8DV[a\"\xD4``\x86\x01a!\xCAV[c\xFF\xFF\xFF\xFF\x81\x16``\x8C\x01R\x93Pa\"\xEE`\x80\x86\x01a!\xCAV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x8C\x01R\x93Pa#\t`\xA0\x86\x01\x86a\x1ApV[\x95P\x93P\x89\x81\x03`\xA0\x8B\x01Ra# \x81\x86\x86a\x1A\xB1V[\x99PPPPPP` \x83\x01\x92P` \x84\x01\x93P`\x01\x82\x01\x91Pa\"\x11V[P\x92\x98\x97PPPPPPPPV[` \x81R_a\x16_` \x83\x01\x84a\x16fV[_\x825`\x9E\x19\x836\x03\x01\x81\x12a\x1AfW_\x80\xFD[` \x80\x82R\x81\x01\x82\x90R_`@`\x05\x84\x90\x1B\x83\x01\x81\x01\x90\x83\x01\x85\x83`\x9E\x196\x83\x90\x03\x01[\x87\x82\x10\x15a$EW\x86\x85\x03`?\x19\x01\x84R\x825\x81\x81\x12a#\xB4W_\x80\xFD[\x89\x01a#\xC0\x81\x80a!\"V[`\xA0\x88Ra#\xD2`\xA0\x89\x01\x82\x84a!fV[\x91PP` \x82\x015a#\xE3\x81a\x16\xC1V[`\x01`\x01`\xA0\x1B\x03\x16` \x88\x01R`@\x82\x81\x015\x90\x88\x01Rc\xFF\xFF\xFF\xFFa$\x0C``\x84\x01a!\xCAV[\x16``\x88\x01Rc\xFF\xFF\xFF\xFFa$#`\x80\x84\x01a!\xCAV[\x16`\x80\x88\x01R\x80\x96PPP` \x83\x01\x92P` \x84\x01\x93P`\x01\x82\x01\x91Pa#\x96V[P\x92\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \xF4~S\"\xE4\xFBQ\xB7\\\x1C\x9B+\xAF\xEA\x1Fb\xEF\xF5~I\x15G\xA9\xD9\x03\xED\x8E\x8D<(\x9APdsolcC\0\x08\x1A\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b50600436106100e4575f3560e01c8063276f85f2146100e857806333cfb7b7146101125780633bc28c8c1461013257806351caeaba14610147578063683048351461015a5780636b3aa72e1461018e578063715018a6146101b55780638da5cb5b146101bd5780639926ee7d146101c55780639d1e4053146101d8578063a0169ddd146101eb578063a20b99bf146101fe578063a364f4da14610211578063a98fb35514610224578063e481af9d14610237578063f2fde38b1461023f578063fc299dee14610252578063fce36c7d14610265575b5f80fd5b6100fb6100f6366004611646565b610278565b604051610109929190611694565b60405180910390f35b6101256101203660046116d5565b61039f565b60405161010991906116f0565b6101456101403660046116d5565b6103b0565b005b610145610155366004611782565b6103c4565b6101817f000000000000000000000000000000000000000000000000000000000000000081565b60405161010991906117c0565b6101817f000000000000000000000000000000000000000000000000000000000000000081565b610145610465565b610181610478565b6101456101d33660046118dc565b610487565b6101456101e6366004611984565b6104e6565b6101456101f93660046116d5565b610796565b61014561020c366004611782565b6107a7565b61014561021f3660046116d5565b6107b9565b6101456102323660046119ba565b61080a565b61012561081b565b61014561024d3660046116d5565b61082a565b606554610181906001600160a01b031681565b610145610273366004611782565b6108a0565b60976020525f908152604090208054819061029290611a06565b80601f01602080910402602001604051908101604052809291908181526020018280546102be90611a06565b80156103095780601f106102e057610100808354040283529160200191610309565b820191905f5260205f20905b8154815290600101906020018083116102ec57829003601f168201915b50505050509080600101805461031e90611a06565b80601f016020809104026020016040519081016040528092919081815260200182805461034a90611a06565b80156103955780601f1061036c57610100808354040283529160200191610395565b820191905f5260205f20905b81548152906001019060200180831161037857829003601f168201915b5050505050905082565b60606103aa826108b2565b92915050565b6103b8610b8d565b6103c181610bec565b50565b5f5b63ffffffff81168211156104605730639d1e4053848463ffffffff85168181106103f2576103f2611a3e565b90506020028101906104049190611a52565b6040518263ffffffff1660e01b81526004016104209190611b10565b5f604051808303815f87803b158015610437575f80fd5b505af1158015610449573d5f803e3d5ffd5b50505050808061045890611b82565b9150506103c6565b505050565b61046d610b8d565b6104765f610c55565b565b6033546001600160a01b031690565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146104d85760405162461bcd60e51b81526004016104cf90611ba6565b60405180910390fd5b6104e28282610ca6565b5050565b5f6104f18280611a52565b6040516020016105019190611c16565b6040516020818303038152906040528051906020012090505f61056f826040517b0ca2ba3432b932bab69029b4b3b732b21026b2b9b9b0b3b29d05199960211b6020820152603c81018290525f90605c01604051602081830303815290604052805190602001209050919050565b9050630b135d3f60e11b6001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016631626ba7e836105b66020880188611c28565b6040518463ffffffff1660e01b81526004016105d493929190611c6a565b602060405180830381865afa1580156105ef573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106139190611c83565b6001600160e01b031982811691161461063f57604051638baa579f60e01b815260040160405180910390fd5b604080518082019091525f90806106568780611a52565b610664906020810190611c28565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f920191909152505050908252506020908101906106ac90880188611c28565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284375f9201829052509390945250929350839250609791506106f48880611a52565b610702906020810190611646565b6001600160401b0316815260208101919091526040015f20815181906107289082611cee565b506020820151600182019061073d9082611cee565b5061074c915086905080611a52565b61075a906020810190611646565b6001600160401b03167f41b42d39b4c7441eedf9cb7c1a70d2f7771846ab84d9f245ec091d4dfa14855b60405160405180910390a25050505050565b61079e610b8d565b6103c181610d25565b6107af610da1565b6104e28282610e3b565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146108015760405162461bcd60e51b81526004016104cf90611ba6565b6103c181611178565b610812610b8d565b6103c1816111c4565b6060610825611210565b905090565b610832610b8d565b6001600160a01b0381166108975760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016104cf565b6103c181610c55565b6108a8610da1565b6104e28282611344565b60605f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316631703a0186040518163ffffffff1660e01b81526004015f60405180830381865afa158015610910573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526109379190810190611dde565b8051519091505f816001600160401b03811115610956576109566117d4565b60405190808252806020026020018201604052801561097f578160200160208202803683370190505b5090505f5b828110156109dd5783518051829081106109a0576109a0611a3e565b60200260200101515f01518282815181106109bd576109bd611a3e565b6001600160a01b0390921660209283029190910190910152600101610984565b50604051639004134760e01b81525f906001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639004134790610a2e9089908690600401611ee3565b5f60405180830381865afa158015610a48573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610a6f9190810190611f41565b90505f805b84811015610ab3575f838281518110610a8f57610a8f611a3e565b60200260200101511115610aab5781610aa781611fc7565b9250505b600101610a74565b505f816001600160401b03811115610acd57610acd6117d4565b604051908082528060200260200182016040528015610af6578160200160208202803683370190505b5090505f805b86811015610b7f575f858281518110610b1757610b17611a3e565b60200260200101511115610b7757858181518110610b3757610b37611a3e565b6020026020010151838381518110610b5157610b51611a3e565b6001600160a01b039092166020928302919091019091015281610b7381611fc7565b9250505b600101610afc565b509098975050505050505050565b33610b96610478565b6001600160a01b0316146104765760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016104cf565b6065546040517fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e391610c2b916001600160a01b03909116908490611fdf565b60405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b604051639926ee7d60e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639926ee7d90610cf49085908590600401611ff9565b5f604051808303815f87803b158015610d0b575f80fd5b505af1158015610d1d573d5f803e3d5ffd5b505050505050565b60405163a0169ddd60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a0169ddd90610d719084906004016117c0565b5f604051808303815f87803b158015610d88575f80fd5b505af1158015610d9a573d5f803e3d5ffd5b5050505050565b6065546001600160a01b031633146104765760405162461bcd60e51b815260206004820152605160248201527f4543445341536572766963654d616e61676572426173652e6f6e6c795265776160448201527f726473496e69746961746f723a2063616c6c6572206973206e6f7420746865206064820152703932bbb0b932399034b734ba34b0ba37b960791b608482015260a4016104cf565b5f5b81811015611127575f805b848484818110610e5a57610e5a611a3e565b9050602002810190610e6c9190612043565b610e7a906040810190612057565b9050811015610ee457848484818110610e9557610e95611a3e565b9050602002810190610ea79190612043565b610eb5906040810190612057565b82818110610ec557610ec5611a3e565b9050604002016020013582610eda919061209c565b9150600101610e48565b50838383818110610ef757610ef7611a3e565b9050602002810190610f099190612043565b610f1a9060408101906020016116d5565b6001600160a01b03166323b872dd3330846040518463ffffffff1660e01b8152600401610f49939291906120af565b6020604051808303815f875af1158015610f65573d5f803e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610f8991906120d3565b505f848484818110610f9d57610f9d611a3e565b9050602002810190610faf9190612043565b610fc09060408101906020016116d5565b6001600160a01b031663dd62ed3e307f00000000000000000000000000000000000000000000000000000000000000006040518363ffffffff1660e01b815260040161100d929190611fdf565b602060405180830381865afa158015611028573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061104c91906120f2565b905084848481811061106057611060611a3e565b90506020028101906110729190612043565b6110839060408101906020016116d5565b6001600160a01b031663095ea7b37f00000000000000000000000000000000000000000000000000000000000000006110bc848661209c565b6040518363ffffffff1660e01b81526004016110d9929190612109565b6020604051808303815f875af11580156110f5573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061111991906120d3565b505050806001019050610e3d565b50604051634e5cd2fd60e11b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001690639cb9a5fa90610cf4903090869086906004016121dd565b6040516351b27a6d60e11b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a364f4da90610d719084906004016117c0565b60405163a98fb35560e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a98fb35590610d7190849060040161234c565b60605f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316631703a0186040518163ffffffff1660e01b81526004015f60405180830381865afa15801561126e573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526112959190810190611dde565b90505f815f0151516001600160401b038111156112b4576112b46117d4565b6040519080825280602002602001820160405280156112dd578160200160208202803683370190505b5090505f5b82515181101561133d57825180518290811061130057611300611a3e565b60200260200101515f015182828151811061131d5761131d611a3e565b6001600160a01b03909216602092830291909101909101526001016112e2565b5092915050565b5f5b818110156115dc5782828281811061136057611360611a3e565b9050602002810190611372919061235e565b6113839060408101906020016116d5565b6001600160a01b03166323b872dd33308686868181106113a5576113a5611a3e565b90506020028101906113b7919061235e565b604001356040518463ffffffff1660e01b81526004016113d9939291906120af565b6020604051808303815f875af11580156113f5573d5f803e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061141991906120d3565b505f83838381811061142d5761142d611a3e565b905060200281019061143f919061235e565b6114509060408101906020016116d5565b6001600160a01b031663dd62ed3e307f00000000000000000000000000000000000000000000000000000000000000006040518363ffffffff1660e01b815260040161149d929190611fdf565b602060405180830381865afa1580156114b8573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906114dc91906120f2565b90508383838181106114f0576114f0611a3e565b9050602002810190611502919061235e565b6115139060408101906020016116d5565b6001600160a01b031663095ea7b37f00000000000000000000000000000000000000000000000000000000000000008387878781811061155557611555611a3e565b9050602002810190611567919061235e565b60400135611575919061209c565b6040518363ffffffff1660e01b8152600401611592929190612109565b6020604051808303815f875af11580156115ae573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115d291906120d3565b5050600101611346565b5060405163fce36c7d60e01b81526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063fce36c7d90610cf49085908590600401612372565b80356001600160401b0381168114611641575f80fd5b919050565b5f60208284031215611656575f80fd5b61165f8261162b565b9392505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b604081525f6116a66040830185611666565b82810360208401526116b88185611666565b95945050505050565b6001600160a01b03811681146103c1575f80fd5b5f602082840312156116e5575f80fd5b813561165f816116c1565b602080825282518282018190525f918401906040840190835b818110156117305783516001600160a01b0316835260209384019390920191600101611709565b509095945050505050565b5f8083601f84011261174b575f80fd5b5081356001600160401b03811115611761575f80fd5b6020830191508360208260051b850101111561177b575f80fd5b9250929050565b5f8060208385031215611793575f80fd5b82356001600160401b038111156117a8575f80fd5b6117b48582860161173b565b90969095509350505050565b6001600160a01b0391909116815260200190565b634e487b7160e01b5f52604160045260245ffd5b604051606081016001600160401b038111828210171561180a5761180a6117d4565b60405290565b604051602081016001600160401b038111828210171561180a5761180a6117d4565b604080519081016001600160401b038111828210171561180a5761180a6117d4565b604051601f8201601f191681016001600160401b038111828210171561187c5761187c6117d4565b604052919050565b5f806001600160401b0384111561189d5761189d6117d4565b50601f8301601f19166020016118b281611854565b9150508281528383830111156118c6575f80fd5b828260208301375f602084830101529392505050565b5f80604083850312156118ed575f80fd5b82356118f8816116c1565b915060208301356001600160401b03811115611912575f80fd5b830160608186031215611923575f80fd5b61192b6117e8565b81356001600160401b03811115611940575f80fd5b8201601f81018713611950575f80fd5b61195f87823560208401611884565b8252506020828101359082015260409182013591810191909152919491935090915050565b5f60208284031215611994575f80fd5b81356001600160401b038111156119a9575f80fd5b82016040818503121561165f575f80fd5b5f602082840312156119ca575f80fd5b81356001600160401b038111156119df575f80fd5b8201601f810184136119ef575f80fd5b6119fe84823560208401611884565b949350505050565b600181811c90821680611a1a57607f821691505b602082108103611a3857634e487b7160e01b5f52602260045260245ffd5b50919050565b634e487b7160e01b5f52603260045260245ffd5b5f8235603e19833603018112611a66575f80fd5b9190910192915050565b5f808335601e19843603018112611a85575f80fd5b83016020810192503590506001600160401b03811115611aa3575f80fd5b80360382131561177b575f80fd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b6001600160401b03611aea8261162b565b1682525f611afb6020830183611a70565b604060208601526116b8604086018284611ab1565b602081525f8235603e19843603018112611b28575f80fd5b60406020840152611b3e60608401858301611ad9565b9050611b4d6020850185611a70565b848303601f19016040860152611b64838284611ab1565b9695505050505050565b634e487b7160e01b5f52601160045260245ffd5b5f63ffffffff821663ffffffff8103611b9d57611b9d611b6e565b60010192915050565b6020808252604a908201527f4543445341536572766963654d616e61676572426173652e6f6e6c795374616b60408201527f6552656769737472793a2063616c6c6572206973206e6f7420746865207374616060820152696b65526567697374727960b01b608082015260a00190565b602081525f61165f6020830184611ad9565b5f808335601e19843603018112611c3d575f80fd5b8301803591506001600160401b03821115611c56575f80fd5b60200191503681900382131561177b575f80fd5b838152604060208201525f6116b8604083018486611ab1565b5f60208284031215611c93575f80fd5b81516001600160e01b03198116811461165f575f80fd5b601f82111561046057805f5260205f20601f840160051c81016020851015611ccf5750805b601f840160051c820191505b81811015610d9a575f8155600101611cdb565b81516001600160401b03811115611d0757611d076117d4565b611d1b81611d158454611a06565b84611caa565b6020601f821160018114611d4d575f8315611d365750848201515b5f19600385901b1c1916600184901b178455610d9a565b5f84815260208120601f198516915b82811015611d7c5787850151825560209485019460019092019101611d5c565b5084821015611d9957868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b5f6001600160401b03821115611dc057611dc06117d4565b5060051b60200190565b6001600160601b03811681146103c1575f80fd5b5f60208284031215611dee575f80fd5b81516001600160401b03811115611e03575f80fd5b820160208185031215611e14575f80fd5b611e1c611810565b81516001600160401b03811115611e31575f80fd5b80830192505084601f830112611e45575f80fd5b8151611e58611e5382611da8565b611854565b8082825260208201915060208360061b860101925087831115611e79575f80fd5b6020850194505b82851015611ed65760408589031215611e97575f80fd5b611e9f611832565b8551611eaa816116c1565b81526020860151611eba81611dca565b8060208301525080835250602082019150604085019450611e80565b8352509095945050505050565b6001600160a01b03831681526040602080830182905283519183018290525f91908401906060840190835b81811015611f355783516001600160a01b0316835260209384019390920191600101611f0e565b50909695505050505050565b5f60208284031215611f51575f80fd5b81516001600160401b03811115611f66575f80fd5b8201601f81018413611f76575f80fd5b8051611f84611e5382611da8565b8082825260208201915060208360051b850101925086831115611fa5575f80fd5b6020840193505b82841015611b64578351825260209384019390910190611fac565b5f60018201611fd857611fd8611b6e565b5060010190565b6001600160a01b0392831681529116602082015260400190565b60018060a01b0383168152604060208201525f82516060604084015261202260a0840182611666565b90506020840151606084015260408401516080840152809150509392505050565b5f823560be19833603018112611a66575f80fd5b5f808335601e1984360301811261206c575f80fd5b8301803591506001600160401b03821115612085575f80fd5b6020019150600681901b360382131561177b575f80fd5b808201808211156103aa576103aa611b6e565b6001600160a01b039384168152919092166020820152604081019190915260600190565b5f602082840312156120e3575f80fd5b8151801515811461165f575f80fd5b5f60208284031215612102575f80fd5b5051919050565b6001600160a01b03929092168252602082015260400190565b5f808335601e19843603018112612137575f80fd5b83016020810192503590506001600160401b03811115612155575f80fd5b8060061b360382131561177b575f80fd5b8183526020830192505f815f5b848110156121c0578135612186816116c1565b6001600160a01b03168652602082013561219f81611dca565b6001600160601b031660208701526040958601959190910190600101612173565b5093949350505050565b803563ffffffff81168114611641575f80fd5b6001600160a01b038416815260406020820181905281018290525f6060600584901b83018101908301858360be1936839003015b8782101561233e57868503605f19018452823581811261222f575f80fd5b890161223b8180612122565b60c0885261224d60c089018284612166565b915050602082013561225e816116c1565b6001600160a01b031660208801526122796040830183612122565b88830360408a015280835290915f91906020015b818310156122c85783356122a0816116c1565b6001600160a01b0316815260208481013590820152604093840193600193909301920161228d565b6122d4606086016121ca565b63ffffffff811660608c015293506122ee608086016121ca565b63ffffffff811660808c0152935061230960a0860186611a70565b9550935089810360a08b0152612320818686611ab1565b99505050505050602083019250602084019350600182019150612211565b509298975050505050505050565b602081525f61165f6020830184611666565b5f8235609e19833603018112611a66575f80fd5b602080825281018290525f6040600584901b830181019083018583609e1936839003015b8782101561244557868503603f1901845282358181126123b4575f80fd5b89016123c08180612122565b60a088526123d260a089018284612166565b91505060208201356123e3816116c1565b6001600160a01b031660208801526040828101359088015263ffffffff61240c606084016121ca565b16606088015263ffffffff612423608084016121ca565b1660808801528096505050602083019250602084019350600182019150612396565b509297965050505050505056fea2646970667358221220f47e5322e4fb51b75c1c9b2bafea1f62eff57e491547a9d903ed8e8d3c289a5064736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\0\xE4W_5`\xE0\x1C\x80c'o\x85\xF2\x14a\0\xE8W\x80c3\xCF\xB7\xB7\x14a\x01\x12W\x80c;\xC2\x8C\x8C\x14a\x012W\x80cQ\xCA\xEA\xBA\x14a\x01GW\x80ch0H5\x14a\x01ZW\x80ck:\xA7.\x14a\x01\x8EW\x80cqP\x18\xA6\x14a\x01\xB5W\x80c\x8D\xA5\xCB[\x14a\x01\xBDW\x80c\x99&\xEE}\x14a\x01\xC5W\x80c\x9D\x1E@S\x14a\x01\xD8W\x80c\xA0\x16\x9D\xDD\x14a\x01\xEBW\x80c\xA2\x0B\x99\xBF\x14a\x01\xFEW\x80c\xA3d\xF4\xDA\x14a\x02\x11W\x80c\xA9\x8F\xB3U\x14a\x02$W\x80c\xE4\x81\xAF\x9D\x14a\x027W\x80c\xF2\xFD\xE3\x8B\x14a\x02?W\x80c\xFC)\x9D\xEE\x14a\x02RW\x80c\xFC\xE3l}\x14a\x02eW[_\x80\xFD[a\0\xFBa\0\xF66`\x04a\x16FV[a\x02xV[`@Qa\x01\t\x92\x91\x90a\x16\x94V[`@Q\x80\x91\x03\x90\xF3[a\x01%a\x01 6`\x04a\x16\xD5V[a\x03\x9FV[`@Qa\x01\t\x91\x90a\x16\xF0V[a\x01Ea\x01@6`\x04a\x16\xD5V[a\x03\xB0V[\0[a\x01Ea\x01U6`\x04a\x17\x82V[a\x03\xC4V[a\x01\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qa\x01\t\x91\x90a\x17\xC0V[a\x01\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x01Ea\x04eV[a\x01\x81a\x04xV[a\x01Ea\x01\xD36`\x04a\x18\xDCV[a\x04\x87V[a\x01Ea\x01\xE66`\x04a\x19\x84V[a\x04\xE6V[a\x01Ea\x01\xF96`\x04a\x16\xD5V[a\x07\x96V[a\x01Ea\x02\x0C6`\x04a\x17\x82V[a\x07\xA7V[a\x01Ea\x02\x1F6`\x04a\x16\xD5V[a\x07\xB9V[a\x01Ea\x0226`\x04a\x19\xBAV[a\x08\nV[a\x01%a\x08\x1BV[a\x01Ea\x02M6`\x04a\x16\xD5V[a\x08*V[`eTa\x01\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01Ea\x02s6`\x04a\x17\x82V[a\x08\xA0V[`\x97` R_\x90\x81R`@\x90 \x80T\x81\x90a\x02\x92\x90a\x1A\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xBE\x90a\x1A\x06V[\x80\x15a\x03\tW\x80`\x1F\x10a\x02\xE0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\tV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\xECW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90\x80`\x01\x01\x80Ta\x03\x1E\x90a\x1A\x06V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x03J\x90a\x1A\x06V[\x80\x15a\x03\x95W\x80`\x1F\x10a\x03lWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03\x95V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03xW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x82V[``a\x03\xAA\x82a\x08\xB2V[\x92\x91PPV[a\x03\xB8a\x0B\x8DV[a\x03\xC1\x81a\x0B\xECV[PV[_[c\xFF\xFF\xFF\xFF\x81\x16\x82\x11\x15a\x04`W0c\x9D\x1E@S\x84\x84c\xFF\xFF\xFF\xFF\x85\x16\x81\x81\x10a\x03\xF2Wa\x03\xF2a\x1A>V[\x90P` \x02\x81\x01\x90a\x04\x04\x91\x90a\x1ARV[`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04 \x91\x90a\x1B\x10V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x047W_\x80\xFD[PZ\xF1\x15\x80\x15a\x04IW=_\x80>=_\xFD[PPPP\x80\x80a\x04X\x90a\x1B\x82V[\x91PPa\x03\xC6V[PPPV[a\x04ma\x0B\x8DV[a\x04v_a\x0CUV[V[`3T`\x01`\x01`\xA0\x1B\x03\x16\x90V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\xCF\x90a\x1B\xA6V[`@Q\x80\x91\x03\x90\xFD[a\x04\xE2\x82\x82a\x0C\xA6V[PPV[_a\x04\xF1\x82\x80a\x1ARV[`@Q` \x01a\x05\x01\x91\x90a\x1C\x16V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P_a\x05o\x82`@Q{\x0C\xA2\xBA42\xB92\xBA\xB6\x90)\xB4\xB3\xB72\xB2\x10&\xB2\xB9\xB9\xB0\xB3\xB2\x9D\x05\x19\x99`!\x1B` \x82\x01R`<\x81\x01\x82\x90R_\x90`\\\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[\x90Pc\x0B\x13]?`\xE1\x1B`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x16&\xBA~\x83a\x05\xB6` \x88\x01\x88a\x1C(V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x05\xD4\x93\x92\x91\x90a\x1CjV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xEFW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x13\x91\x90a\x1C\x83V[`\x01`\x01`\xE0\x1B\x03\x19\x82\x81\x16\x91\x16\x14a\x06?W`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R_\x90\x80a\x06V\x87\x80a\x1ARV[a\x06d\x90` \x81\x01\x90a\x1C(V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x90\x81\x01\x90a\x06\xAC\x90\x88\x01\x88a\x1C(V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847_\x92\x01\x82\x90RP\x93\x90\x94RP\x92\x93P\x83\x92P`\x97\x91Pa\x06\xF4\x88\x80a\x1ARV[a\x07\x02\x90` \x81\x01\x90a\x16FV[`\x01`\x01`@\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x81Q\x81\x90a\x07(\x90\x82a\x1C\xEEV[P` \x82\x01Q`\x01\x82\x01\x90a\x07=\x90\x82a\x1C\xEEV[Pa\x07L\x91P\x86\x90P\x80a\x1ARV[a\x07Z\x90` \x81\x01\x90a\x16FV[`\x01`\x01`@\x1B\x03\x16\x7FA\xB4-9\xB4\xC7D\x1E\xED\xF9\xCB|\x1Ap\xD2\xF7w\x18F\xAB\x84\xD9\xF2E\xEC\t\x1DM\xFA\x14\x85[`@Q`@Q\x80\x91\x03\x90\xA2PPPPPV[a\x07\x9Ea\x0B\x8DV[a\x03\xC1\x81a\r%V[a\x07\xAFa\r\xA1V[a\x04\xE2\x82\x82a\x0E;V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x08\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x04\xCF\x90a\x1B\xA6V[a\x03\xC1\x81a\x11xV[a\x08\x12a\x0B\x8DV[a\x03\xC1\x81a\x11\xC4V[``a\x08%a\x12\x10V[\x90P\x90V[a\x082a\x0B\x8DV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x08\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x04\xCFV[a\x03\xC1\x81a\x0CUV[a\x08\xA8a\r\xA1V[a\x04\xE2\x82\x82a\x13DV[``_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x17\x03\xA0\x18`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x10W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t7\x91\x90\x81\x01\x90a\x1D\xDEV[\x80QQ\x90\x91P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\tVWa\tVa\x17\xD4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\x7FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a\t\xDDW\x83Q\x80Q\x82\x90\x81\x10a\t\xA0Wa\t\xA0a\x1A>V[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a\t\xBDWa\t\xBDa\x1A>V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\t\x84V[P`@Qc\x90\x04\x13G`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x90\x04\x13G\x90a\n.\x90\x89\x90\x86\x90`\x04\x01a\x1E\xE3V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nHW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\no\x91\x90\x81\x01\x90a\x1FAV[\x90P_\x80[\x84\x81\x10\x15a\n\xB3W_\x83\x82\x81Q\x81\x10a\n\x8FWa\n\x8Fa\x1A>V[` \x02` \x01\x01Q\x11\x15a\n\xABW\x81a\n\xA7\x81a\x1F\xC7V[\x92PP[`\x01\x01a\ntV[P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xCDWa\n\xCDa\x17\xD4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xF6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x80[\x86\x81\x10\x15a\x0B\x7FW_\x85\x82\x81Q\x81\x10a\x0B\x17Wa\x0B\x17a\x1A>V[` \x02` \x01\x01Q\x11\x15a\x0BwW\x85\x81\x81Q\x81\x10a\x0B7Wa\x0B7a\x1A>V[` \x02` \x01\x01Q\x83\x83\x81Q\x81\x10a\x0BQWa\x0BQa\x1A>V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x81a\x0Bs\x81a\x1F\xC7V[\x92PP[`\x01\x01a\n\xFCV[P\x90\x98\x97PPPPPPPPV[3a\x0B\x96a\x04xV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x04\xCFV[`eT`@Q\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91a\x0C+\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x84\x90a\x1F\xDFV[`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x99&\xEE}\x90a\x0C\xF4\x90\x85\x90\x85\x90`\x04\x01a\x1F\xF9V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\r\x0BW_\x80\xFD[PZ\xF1\x15\x80\x15a\r\x1DW=_\x80>=_\xFD[PPPPPPV[`@Qc\xA0\x16\x9D\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA0\x16\x9D\xDD\x90a\rq\x90\x84\x90`\x04\x01a\x17\xC0V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\r\x88W_\x80\xFD[PZ\xF1\x15\x80\x15a\r\x9AW=_\x80>=_\xFD[PPPPPV[`eT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FECDSAServiceManagerBase.onlyRewa`D\x82\x01R\x7FrdsInitiator: caller is not the `d\x82\x01Rp92\xBB\xB0\xB929\x904\xB74\xBA4\xB0\xBA7\xB9`y\x1B`\x84\x82\x01R`\xA4\x01a\x04\xCFV[_[\x81\x81\x10\x15a\x11'W_\x80[\x84\x84\x84\x81\x81\x10a\x0EZWa\x0EZa\x1A>V[\x90P` \x02\x81\x01\x90a\x0El\x91\x90a CV[a\x0Ez\x90`@\x81\x01\x90a WV[\x90P\x81\x10\x15a\x0E\xE4W\x84\x84\x84\x81\x81\x10a\x0E\x95Wa\x0E\x95a\x1A>V[\x90P` \x02\x81\x01\x90a\x0E\xA7\x91\x90a CV[a\x0E\xB5\x90`@\x81\x01\x90a WV[\x82\x81\x81\x10a\x0E\xC5Wa\x0E\xC5a\x1A>V[\x90P`@\x02\x01` \x015\x82a\x0E\xDA\x91\x90a \x9CV[\x91P`\x01\x01a\x0EHV[P\x83\x83\x83\x81\x81\x10a\x0E\xF7Wa\x0E\xF7a\x1A>V[\x90P` \x02\x81\x01\x90a\x0F\t\x91\x90a CV[a\x0F\x1A\x90`@\x81\x01\x90` \x01a\x16\xD5V[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x84`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0FI\x93\x92\x91\x90a \xAFV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x0FeW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\x89\x91\x90a \xD3V[P_\x84\x84\x84\x81\x81\x10a\x0F\x9DWa\x0F\x9Da\x1A>V[\x90P` \x02\x81\x01\x90a\x0F\xAF\x91\x90a CV[a\x0F\xC0\x90`@\x81\x01\x90` \x01a\x16\xD5V[`\x01`\x01`\xA0\x1B\x03\x16c\xDDb\xED>0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\r\x92\x91\x90a\x1F\xDFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10(W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10L\x91\x90a \xF2V[\x90P\x84\x84\x84\x81\x81\x10a\x10`Wa\x10`a\x1A>V[\x90P` \x02\x81\x01\x90a\x10r\x91\x90a CV[a\x10\x83\x90`@\x81\x01\x90` \x01a\x16\xD5V[`\x01`\x01`\xA0\x1B\x03\x16c\t^\xA7\xB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x10\xBC\x84\x86a \x9CV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10\xD9\x92\x91\x90a!\tV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x10\xF5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x19\x91\x90a \xD3V[PPP\x80`\x01\x01\x90Pa\x0E=V[P`@QcN\\\xD2\xFD`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x9C\xB9\xA5\xFA\x90a\x0C\xF4\x900\x90\x86\x90\x86\x90`\x04\x01a!\xDDV[`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3d\xF4\xDA\x90a\rq\x90\x84\x90`\x04\x01a\x17\xC0V[`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA9\x8F\xB3U\x90a\rq\x90\x84\x90`\x04\x01a#LV[``_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x17\x03\xA0\x18`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12nW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\x95\x91\x90\x81\x01\x90a\x1D\xDEV[\x90P_\x81_\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\xB4Wa\x12\xB4a\x17\xD4V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xDDW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82QQ\x81\x10\x15a\x13=W\x82Q\x80Q\x82\x90\x81\x10a\x13\0Wa\x13\0a\x1A>V[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a\x13\x1DWa\x13\x1Da\x1A>V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x12\xE2V[P\x92\x91PPV[_[\x81\x81\x10\x15a\x15\xDCW\x82\x82\x82\x81\x81\x10a\x13`Wa\x13`a\x1A>V[\x90P` \x02\x81\x01\x90a\x13r\x91\x90a#^V[a\x13\x83\x90`@\x81\x01\x90` \x01a\x16\xD5V[`\x01`\x01`\xA0\x1B\x03\x16c#\xB8r\xDD30\x86\x86\x86\x81\x81\x10a\x13\xA5Wa\x13\xA5a\x1A>V[\x90P` \x02\x81\x01\x90a\x13\xB7\x91\x90a#^V[`@\x015`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\xD9\x93\x92\x91\x90a \xAFV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x13\xF5W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x19\x91\x90a \xD3V[P_\x83\x83\x83\x81\x81\x10a\x14-Wa\x14-a\x1A>V[\x90P` \x02\x81\x01\x90a\x14?\x91\x90a#^V[a\x14P\x90`@\x81\x01\x90` \x01a\x16\xD5V[`\x01`\x01`\xA0\x1B\x03\x16c\xDDb\xED>0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\x9D\x92\x91\x90a\x1F\xDFV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xB8W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\xDC\x91\x90a \xF2V[\x90P\x83\x83\x83\x81\x81\x10a\x14\xF0Wa\x14\xF0a\x1A>V[\x90P` \x02\x81\x01\x90a\x15\x02\x91\x90a#^V[a\x15\x13\x90`@\x81\x01\x90` \x01a\x16\xD5V[`\x01`\x01`\xA0\x1B\x03\x16c\t^\xA7\xB3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x87\x87\x87\x81\x81\x10a\x15UWa\x15Ua\x1A>V[\x90P` \x02\x81\x01\x90a\x15g\x91\x90a#^V[`@\x015a\x15u\x91\x90a \x9CV[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15\x92\x92\x91\x90a!\tV[` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\x15\xAEW=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xD2\x91\x90a \xD3V[PP`\x01\x01a\x13FV[P`@Qc\xFC\xE3l}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xFC\xE3l}\x90a\x0C\xF4\x90\x85\x90\x85\x90`\x04\x01a#rV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x16AW_\x80\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a\x16VW_\x80\xFD[a\x16_\x82a\x16+V[\x93\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`@\x81R_a\x16\xA6`@\x83\x01\x85a\x16fV[\x82\x81\x03` \x84\x01Ra\x16\xB8\x81\x85a\x16fV[\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xC1W_\x80\xFD[_` \x82\x84\x03\x12\x15a\x16\xE5W_\x80\xFD[\x815a\x16_\x81a\x16\xC1V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a\x170W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x17\tV[P\x90\x95\x94PPPPPV[_\x80\x83`\x1F\x84\x01\x12a\x17KW_\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17aW_\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x17{W_\x80\xFD[\x92P\x92\x90PV[_\x80` \x83\x85\x03\x12\x15a\x17\x93W_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xA8W_\x80\xFD[a\x17\xB4\x85\x82\x86\x01a\x17;V[\x90\x96\x90\x95P\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x18\nWa\x18\na\x17\xD4V[`@R\x90V[`@Q` \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x18\nWa\x18\na\x17\xD4V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x18\nWa\x18\na\x17\xD4V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x18|Wa\x18|a\x17\xD4V[`@R\x91\x90PV[_\x80`\x01`\x01`@\x1B\x03\x84\x11\x15a\x18\x9DWa\x18\x9Da\x17\xD4V[P`\x1F\x83\x01`\x1F\x19\x16` \x01a\x18\xB2\x81a\x18TV[\x91PP\x82\x81R\x83\x83\x83\x01\x11\x15a\x18\xC6W_\x80\xFD[\x82\x82` \x83\x017_` \x84\x83\x01\x01R\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\x18\xEDW_\x80\xFD[\x825a\x18\xF8\x81a\x16\xC1V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\x12W_\x80\xFD[\x83\x01``\x81\x86\x03\x12\x15a\x19#W_\x80\xFD[a\x19+a\x17\xE8V[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19@W_\x80\xFD[\x82\x01`\x1F\x81\x01\x87\x13a\x19PW_\x80\xFD[a\x19_\x87\x825` \x84\x01a\x18\x84V[\x82RP` \x82\x81\x015\x90\x82\x01R`@\x91\x82\x015\x91\x81\x01\x91\x90\x91R\x91\x94\x91\x93P\x90\x91PPV[_` \x82\x84\x03\x12\x15a\x19\x94W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xA9W_\x80\xFD[\x82\x01`@\x81\x85\x03\x12\x15a\x16_W_\x80\xFD[_` \x82\x84\x03\x12\x15a\x19\xCAW_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xDFW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x19\xEFW_\x80\xFD[a\x19\xFE\x84\x825` \x84\x01a\x18\x84V[\x94\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x1A\x1AW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x1A8WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[P\x91\x90PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12a\x1AfW_\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1A\x85W_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1A\xA3W_\x80\xFD[\x806\x03\x82\x13\x15a\x17{W_\x80\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`@\x1B\x03a\x1A\xEA\x82a\x16+V[\x16\x82R_a\x1A\xFB` \x83\x01\x83a\x1ApV[`@` \x86\x01Ra\x16\xB8`@\x86\x01\x82\x84a\x1A\xB1V[` \x81R_\x825`>\x19\x846\x03\x01\x81\x12a\x1B(W_\x80\xFD[`@` \x84\x01Ra\x1B>``\x84\x01\x85\x83\x01a\x1A\xD9V[\x90Pa\x1BM` \x85\x01\x85a\x1ApV[\x84\x83\x03`\x1F\x19\x01`@\x86\x01Ra\x1Bd\x83\x82\x84a\x1A\xB1V[\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_c\xFF\xFF\xFF\xFF\x82\x16c\xFF\xFF\xFF\xFF\x81\x03a\x1B\x9DWa\x1B\x9Da\x1BnV[`\x01\x01\x92\x91PPV[` \x80\x82R`J\x90\x82\x01R\x7FECDSAServiceManagerBase.onlyStak`@\x82\x01R\x7FeRegistry: caller is not the sta``\x82\x01RikeRegistry`\xB0\x1B`\x80\x82\x01R`\xA0\x01\x90V[` \x81R_a\x16_` \x83\x01\x84a\x1A\xD9V[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1C=W_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1CVW_\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x17{W_\x80\xFD[\x83\x81R`@` \x82\x01R_a\x16\xB8`@\x83\x01\x84\x86a\x1A\xB1V[_` \x82\x84\x03\x12\x15a\x1C\x93W_\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x16_W_\x80\xFD[`\x1F\x82\x11\x15a\x04`W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a\x1C\xCFWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\r\x9AW_\x81U`\x01\x01a\x1C\xDBV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x07Wa\x1D\x07a\x17\xD4V[a\x1D\x1B\x81a\x1D\x15\x84Ta\x1A\x06V[\x84a\x1C\xAAV[` `\x1F\x82\x11`\x01\x81\x14a\x1DMW_\x83\x15a\x1D6WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\r\x9AV[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a\x1D|W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a\x1D\\V[P\x84\x82\x10\x15a\x1D\x99W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1D\xC0Wa\x1D\xC0a\x17\xD4V[P`\x05\x1B` \x01\x90V[`\x01`\x01``\x1B\x03\x81\x16\x81\x14a\x03\xC1W_\x80\xFD[_` \x82\x84\x03\x12\x15a\x1D\xEEW_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\x03W_\x80\xFD[\x82\x01` \x81\x85\x03\x12\x15a\x1E\x14W_\x80\xFD[a\x1E\x1Ca\x18\x10V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E1W_\x80\xFD[\x80\x83\x01\x92PP\x84`\x1F\x83\x01\x12a\x1EEW_\x80\xFD[\x81Qa\x1EXa\x1ES\x82a\x1D\xA8V[a\x18TV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x87\x83\x11\x15a\x1EyW_\x80\xFD[` \x85\x01\x94P[\x82\x85\x10\x15a\x1E\xD6W`@\x85\x89\x03\x12\x15a\x1E\x97W_\x80\xFD[a\x1E\x9Fa\x182V[\x85Qa\x1E\xAA\x81a\x16\xC1V[\x81R` \x86\x01Qa\x1E\xBA\x81a\x1D\xCAV[\x80` \x83\x01RP\x80\x83RP` \x82\x01\x91P`@\x85\x01\x94Pa\x1E\x80V[\x83RP\x90\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x80\x83\x01\x82\x90R\x83Q\x91\x83\x01\x82\x90R_\x91\x90\x84\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a\x1F5W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x1F\x0EV[P\x90\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a\x1FQW_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FfW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a\x1FvW_\x80\xFD[\x80Qa\x1F\x84a\x1ES\x82a\x1D\xA8V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a\x1F\xA5W_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a\x1BdW\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a\x1F\xACV[_`\x01\x82\x01a\x1F\xD8Wa\x1F\xD8a\x1BnV[P`\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x16` \x82\x01R`@\x01\x90V[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_\x82Q```@\x84\x01Ra \"`\xA0\x84\x01\x82a\x16fV[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[_\x825`\xBE\x19\x836\x03\x01\x81\x12a\x1AfW_\x80\xFD[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a lW_\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a \x85W_\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a\x17{W_\x80\xFD[\x80\x82\x01\x80\x82\x11\x15a\x03\xAAWa\x03\xAAa\x1BnV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`@\x81\x01\x91\x90\x91R``\x01\x90V[_` \x82\x84\x03\x12\x15a \xE3W_\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x16_W_\x80\xFD[_` \x82\x84\x03\x12\x15a!\x02W_\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R` \x82\x01R`@\x01\x90V[_\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a!7W_\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a!UW_\x80\xFD[\x80`\x06\x1B6\x03\x82\x13\x15a\x17{W_\x80\xFD[\x81\x83R` \x83\x01\x92P_\x81_[\x84\x81\x10\x15a!\xC0W\x815a!\x86\x81a\x16\xC1V[`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x82\x015a!\x9F\x81a\x1D\xCAV[`\x01`\x01``\x1B\x03\x16` \x87\x01R`@\x95\x86\x01\x95\x91\x90\x91\x01\x90`\x01\x01a!sV[P\x93\x94\x93PPPPV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x16AW_\x80\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x82\x90R_```\x05\x84\x90\x1B\x83\x01\x81\x01\x90\x83\x01\x85\x83`\xBE\x196\x83\x90\x03\x01[\x87\x82\x10\x15a#>W\x86\x85\x03`_\x19\x01\x84R\x825\x81\x81\x12a\"/W_\x80\xFD[\x89\x01a\";\x81\x80a!\"V[`\xC0\x88Ra\"M`\xC0\x89\x01\x82\x84a!fV[\x91PP` \x82\x015a\"^\x81a\x16\xC1V[`\x01`\x01`\xA0\x1B\x03\x16` \x88\x01Ra\"y`@\x83\x01\x83a!\"V[\x88\x83\x03`@\x8A\x01R\x80\x83R\x90\x91_\x91\x90` \x01[\x81\x83\x10\x15a\"\xC8W\x835a\"\xA0\x81a\x16\xC1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x84\x81\x015\x90\x82\x01R`@\x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x01a\"\x8DV[a\"\xD4``\x86\x01a!\xCAV[c\xFF\xFF\xFF\xFF\x81\x16``\x8C\x01R\x93Pa\"\xEE`\x80\x86\x01a!\xCAV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x8C\x01R\x93Pa#\t`\xA0\x86\x01\x86a\x1ApV[\x95P\x93P\x89\x81\x03`\xA0\x8B\x01Ra# \x81\x86\x86a\x1A\xB1V[\x99PPPPPP` \x83\x01\x92P` \x84\x01\x93P`\x01\x82\x01\x91Pa\"\x11V[P\x92\x98\x97PPPPPPPPV[` \x81R_a\x16_` \x83\x01\x84a\x16fV[_\x825`\x9E\x19\x836\x03\x01\x81\x12a\x1AfW_\x80\xFD[` \x80\x82R\x81\x01\x82\x90R_`@`\x05\x84\x90\x1B\x83\x01\x81\x01\x90\x83\x01\x85\x83`\x9E\x196\x83\x90\x03\x01[\x87\x82\x10\x15a$EW\x86\x85\x03`?\x19\x01\x84R\x825\x81\x81\x12a#\xB4W_\x80\xFD[\x89\x01a#\xC0\x81\x80a!\"V[`\xA0\x88Ra#\xD2`\xA0\x89\x01\x82\x84a!fV[\x91PP` \x82\x015a#\xE3\x81a\x16\xC1V[`\x01`\x01`\xA0\x1B\x03\x16` \x88\x01R`@\x82\x81\x015\x90\x88\x01Rc\xFF\xFF\xFF\xFFa$\x0C``\x84\x01a!\xCAV[\x16``\x88\x01Rc\xFF\xFF\xFF\xFFa$#`\x80\x84\x01a!\xCAV[\x16`\x80\x88\x01R\x80\x96PPP` \x83\x01\x92P` \x84\x01\x93P`\x01\x82\x01\x91Pa#\x96V[P\x92\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 \xF4~S\"\xE4\xFBQ\xB7\\\x1C\x9B+\xAF\xEA\x1Fb\xEF\xF5~I\x15G\xA9\xD9\x03\xED\x8E\x8D<(\x9APdsolcC\0\x08\x1A\x003",
    );
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
```solidity
error InvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignature {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature()";
            const SELECTOR: [u8; 4] = [139u8, 170u8, 87u8, 159u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Event with signature `AddedSignedPayloadForTrigger(uint64)` and selector `0x41b42d39b4c7441eedf9cb7c1a70d2f7771846ab84d9f245ec091d4dfa14855b`.
```solidity
event AddedSignedPayloadForTrigger(ILayerTrigger.TriggerId indexed triggerId);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AddedSignedPayloadForTrigger {
        #[allow(missing_docs)]
        pub triggerId: <ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for AddedSignedPayloadForTrigger {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                ILayerTrigger::TriggerId,
            );
            const SIGNATURE: &'static str = "AddedSignedPayloadForTrigger(uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                65u8,
                180u8,
                45u8,
                57u8,
                180u8,
                199u8,
                68u8,
                30u8,
                237u8,
                249u8,
                203u8,
                124u8,
                26u8,
                112u8,
                210u8,
                247u8,
                119u8,
                24u8,
                70u8,
                171u8,
                132u8,
                217u8,
                242u8,
                69u8,
                236u8,
                9u8,
                29u8,
                77u8,
                250u8,
                20u8,
                133u8,
                91u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { triggerId: topics.1 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.triggerId.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <ILayerTrigger::TriggerId as alloy_sol_types::EventTopic>::encode_topic(
                    &self.triggerId,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for AddedSignedPayloadForTrigger {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AddedSignedPayloadForTrigger> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &AddedSignedPayloadForTrigger,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Initialized(uint8)` and selector `0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498`.
```solidity
event Initialized(uint8 version);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct Initialized {
        #[allow(missing_docs)]
        pub version: u8,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "Initialized(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { version: data.0 }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.version),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub previousOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousOwner: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.previousOwner.clone(),
                    self.newOwner.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.previousOwner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RewardsInitiatorUpdated(address,address)` and selector `0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3`.
```solidity
event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RewardsInitiatorUpdated {
        #[allow(missing_docs)]
        pub prevRewardsInitiator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newRewardsInitiator: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RewardsInitiatorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "RewardsInitiatorUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                225u8,
                28u8,
                221u8,
                241u8,
                129u8,
                106u8,
                67u8,
                49u8,
                140u8,
                161u8,
                117u8,
                187u8,
                197u8,
                44u8,
                208u8,
                24u8,
                84u8,
                54u8,
                233u8,
                203u8,
                234u8,
                215u8,
                200u8,
                58u8,
                204u8,
                84u8,
                167u8,
                62u8,
                70u8,
                23u8,
                23u8,
                227u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevRewardsInitiator: data.0,
                    newRewardsInitiator: data.1,
                }
            }
            #[inline]
            fn check_signature(
                topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
            ) -> alloy_sol_types::Result<()> {
                if topics.0 != Self::SIGNATURE_HASH {
                    return Err(
                        alloy_sol_types::Error::invalid_event_signature_hash(
                            Self::SIGNATURE,
                            topics.0,
                            Self::SIGNATURE_HASH,
                        ),
                    );
                }
                Ok(())
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.prevRewardsInitiator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newRewardsInitiator,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(),)
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RewardsInitiatorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RewardsInitiatorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RewardsInitiatorUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _avsDirectory, address _stakeRegistry, address _rewardsCoordinator, address _delegationManager);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _avsDirectory: alloy::sol_types::private::Address,
        pub _stakeRegistry: alloy::sol_types::private::Address,
        pub _rewardsCoordinator: alloy::sol_types::private::Address,
        pub _delegationManager: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._avsDirectory,
                        value._stakeRegistry,
                        value._rewardsCoordinator,
                        value._delegationManager,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _avsDirectory: tuple.0,
                        _stakeRegistry: tuple.1,
                        _rewardsCoordinator: tuple.2,
                        _delegationManager: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._avsDirectory,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._stakeRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._rewardsCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._delegationManager,
                    ),
                )
            }
        }
    };
    /**Function with signature `addSignedPayloadForTrigger(((uint64,bytes),bytes))` and selector `0x9d1e4053`.
```solidity
function addSignedPayloadForTrigger(ILayerServiceManager.SignedPayload memory signedPayload) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSignedPayloadForTriggerCall {
        pub signedPayload: <ILayerServiceManager::SignedPayload as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`addSignedPayloadForTrigger(((uint64,bytes),bytes))`](addSignedPayloadForTriggerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSignedPayloadForTriggerReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ILayerServiceManager::SignedPayload,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ILayerServiceManager::SignedPayload as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addSignedPayloadForTriggerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSignedPayloadForTriggerCall) -> Self {
                    (value.signedPayload,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSignedPayloadForTriggerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { signedPayload: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addSignedPayloadForTriggerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSignedPayloadForTriggerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSignedPayloadForTriggerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addSignedPayloadForTriggerCall {
            type Parameters<'a> = (ILayerServiceManager::SignedPayload,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addSignedPayloadForTriggerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addSignedPayloadForTrigger(((uint64,bytes),bytes))";
            const SELECTOR: [u8; 4] = [157u8, 30u8, 64u8, 83u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ILayerServiceManager::SignedPayload as alloy_sol_types::SolType>::tokenize(
                        &self.signedPayload,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `addSignedPayloadForTriggerMulti(((uint64,bytes),bytes)[])` and selector `0x51caeaba`.
```solidity
function addSignedPayloadForTriggerMulti(ILayerServiceManager.SignedPayload[] memory signedPayloads) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSignedPayloadForTriggerMultiCall {
        pub signedPayloads: alloy::sol_types::private::Vec<
            <ILayerServiceManager::SignedPayload as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`addSignedPayloadForTriggerMulti(((uint64,bytes),bytes)[])`](addSignedPayloadForTriggerMultiCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addSignedPayloadForTriggerMultiReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<ILayerServiceManager::SignedPayload>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <ILayerServiceManager::SignedPayload as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addSignedPayloadForTriggerMultiCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSignedPayloadForTriggerMultiCall) -> Self {
                    (value.signedPayloads,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSignedPayloadForTriggerMultiCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { signedPayloads: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addSignedPayloadForTriggerMultiReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addSignedPayloadForTriggerMultiReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addSignedPayloadForTriggerMultiReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addSignedPayloadForTriggerMultiCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<ILayerServiceManager::SignedPayload>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addSignedPayloadForTriggerMultiReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addSignedPayloadForTriggerMulti(((uint64,bytes),bytes)[])";
            const SELECTOR: [u8; 4] = [81u8, 202u8, 234u8, 186u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        ILayerServiceManager::SignedPayload,
                    > as alloy_sol_types::SolType>::tokenize(&self.signedPayloads),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `avsDirectory()` and selector `0x6b3aa72e`.
```solidity
function avsDirectory() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryCall {}
    ///Container type for the return parameters of the [`avsDirectory()`](avsDirectoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<avsDirectoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsDirectoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<avsDirectoryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsDirectoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for avsDirectoryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsDirectoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "avsDirectory()";
            const SELECTOR: [u8; 4] = [107u8, 58u8, 167u8, 46u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])` and selector `0xfce36c7d`.
```solidity
function createAVSRewardsSubmission(IRewardsCoordinator.RewardsSubmission[] memory rewardsSubmissions) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createAVSRewardsSubmissionCall {
        pub rewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])`](createAVSRewardsSubmissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createAVSRewardsSubmissionReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinator::RewardsSubmission,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createAVSRewardsSubmissionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionCall) -> Self {
                    (value.rewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createAVSRewardsSubmissionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        rewardsSubmissions: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createAVSRewardsSubmissionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createAVSRewardsSubmissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createAVSRewardsSubmissionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinator::RewardsSubmission,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])";
            const SELECTOR: [u8; 4] = [252u8, 227u8, 108u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        IRewardsCoordinator::RewardsSubmission,
                    > as alloy_sol_types::SolType>::tokenize(&self.rewardsSubmissions),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])` and selector `0xa20b99bf`.
```solidity
function createOperatorDirectedAVSRewardsSubmission(IRewardsCoordinator.OperatorDirectedRewardsSubmission[] memory operatorDirectedRewardsSubmissions) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorDirectedAVSRewardsSubmissionCall {
        pub operatorDirectedRewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinator::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])`](createOperatorDirectedAVSRewardsSubmissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorDirectedAVSRewardsSubmissionReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinator::OperatorDirectedRewardsSubmission,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinator::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createOperatorDirectedAVSRewardsSubmissionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorDirectedAVSRewardsSubmissionCall) -> Self {
                    (value.operatorDirectedRewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorDirectedAVSRewardsSubmissionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorDirectedRewardsSubmissions: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createOperatorDirectedAVSRewardsSubmissionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: createOperatorDirectedAVSRewardsSubmissionReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorDirectedAVSRewardsSubmissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for createOperatorDirectedAVSRewardsSubmissionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinator::OperatorDirectedRewardsSubmission,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorDirectedAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])";
            const SELECTOR: [u8; 4] = [162u8, 11u8, 153u8, 191u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        IRewardsCoordinator::OperatorDirectedRewardsSubmission,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.operatorDirectedRewardsSubmissions,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `deregisterOperatorFromAVS(address)` and selector `0xa364f4da`.
```solidity
function deregisterOperatorFromAVS(address operator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromAVSCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`deregisterOperatorFromAVS(address)`](deregisterOperatorFromAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromAVSReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromAVSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromAVSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromAVSCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperatorFromAVS(address)";
            const SELECTOR: [u8; 4] = [163u8, 100u8, 244u8, 218u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorRestakedStrategies(address)` and selector `0x33cfb7b7`.
```solidity
function getOperatorRestakedStrategies(address _operator) external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorRestakedStrategiesCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorRestakedStrategies(address)`](getOperatorRestakedStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorRestakedStrategiesReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorRestakedStrategiesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorRestakedStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorRestakedStrategiesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorRestakedStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorRestakedStrategiesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorRestakedStrategiesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorRestakedStrategies(address)";
            const SELECTOR: [u8; 4] = [51u8, 207u8, 183u8, 183u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getRestakeableStrategies()` and selector `0xe481af9d`.
```solidity
function getRestakeableStrategies() external view returns (address[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRestakeableStrategiesCall {}
    ///Container type for the return parameters of the [`getRestakeableStrategies()`](getRestakeableStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRestakeableStrategiesReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRestakeableStrategiesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRestakeableStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRestakeableStrategiesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRestakeableStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRestakeableStrategiesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRestakeableStrategiesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRestakeableStrategies()";
            const SELECTOR: [u8; 4] = [228u8, 129u8, 175u8, 157u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ownerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `registerOperatorToAVS(address,(bytes,bytes32,uint256))` and selector `0x9926ee7d`.
```solidity
function registerOperatorToAVS(address operator, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerOperatorToAVS(address,(bytes,bytes32,uint256))`](registerOperatorToAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorToAVSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSCall) -> Self {
                    (value.operator, value.operatorSignature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorToAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSignature: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorToAVSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorToAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorToAVSCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorToAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperatorToAVS(address,(bytes,bytes32,uint256))";
            const SELECTOR: [u8; 4] = [153u8, 38u8, 238u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <ISignatureUtils::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSignature,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `renounceOwnership()` and selector `0x715018a6`.
```solidity
function renounceOwnership() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipCall {}
    ///Container type for the return parameters of the [`renounceOwnership()`](renounceOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct renounceOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<renounceOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: renounceOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for renounceOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for renounceOwnershipCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = renounceOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "renounceOwnership()";
            const SELECTOR: [u8; 4] = [113u8, 80u8, 24u8, 166u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `rewardsInitiator()` and selector `0xfc299dee`.
```solidity
function rewardsInitiator() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsInitiatorCall {}
    ///Container type for the return parameters of the [`rewardsInitiator()`](rewardsInitiatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsInitiatorReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rewardsInitiatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rewardsInitiatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rewardsInitiatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rewardsInitiatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rewardsInitiatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rewardsInitiator()";
            const SELECTOR: [u8; 4] = [252u8, 41u8, 157u8, 238u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setClaimerFor(address)` and selector `0xa0169ddd`.
```solidity
function setClaimerFor(address claimer) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setClaimerForCall {
        pub claimer: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setClaimerFor(address)`](setClaimerForCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setClaimerForReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setClaimerForCall> for UnderlyingRustTuple<'_> {
                fn from(value: setClaimerForCall) -> Self {
                    (value.claimer,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setClaimerForCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { claimer: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setClaimerForReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setClaimerForReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setClaimerForReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setClaimerForCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setClaimerForReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setClaimerFor(address)";
            const SELECTOR: [u8; 4] = [160u8, 22u8, 157u8, 221u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.claimer,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setRewardsInitiator(address)` and selector `0x3bc28c8c`.
```solidity
function setRewardsInitiator(address newRewardsInitiator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsInitiatorCall {
        pub newRewardsInitiator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setRewardsInitiator(address)`](setRewardsInitiatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsInitiatorReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setRewardsInitiatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorCall) -> Self {
                    (value.newRewardsInitiator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setRewardsInitiatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newRewardsInitiator: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setRewardsInitiatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setRewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setRewardsInitiatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRewardsInitiatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setRewardsInitiator(address)";
            const SELECTOR: [u8; 4] = [59u8, 194u8, 140u8, 140u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newRewardsInitiator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `signedDataByTriggerId(uint64)` and selector `0x276f85f2`.
```solidity
function signedDataByTriggerId(ILayerTrigger.TriggerId) external view returns (bytes memory data, bytes memory signature);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct signedDataByTriggerIdCall {
        pub _0: <ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`signedDataByTriggerId(uint64)`](signedDataByTriggerIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct signedDataByTriggerIdReturn {
        pub data: alloy::sol_types::private::Bytes,
        pub signature: alloy::sol_types::private::Bytes,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (ILayerTrigger::TriggerId,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<signedDataByTriggerIdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: signedDataByTriggerIdCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for signedDataByTriggerIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<signedDataByTriggerIdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: signedDataByTriggerIdReturn) -> Self {
                    (value.data, value.signature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for signedDataByTriggerIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        data: tuple.0,
                        signature: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for signedDataByTriggerIdCall {
            type Parameters<'a> = (ILayerTrigger::TriggerId,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = signedDataByTriggerIdReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "signedDataByTriggerId(uint64)";
            const SELECTOR: [u8; 4] = [39u8, 111u8, 133u8, 242u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ILayerTrigger::TriggerId as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `stakeRegistry()` and selector `0x68304835`.
```solidity
function stakeRegistry() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryCall {}
    ///Container type for the return parameters of the [`stakeRegistry()`](stakeRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeRegistry()";
            const SELECTOR: [u8; 4] = [104u8, 48u8, 72u8, 53u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateAVSMetadataURI(string)` and selector `0xa98fb355`.
```solidity
function updateAVSMetadataURI(string memory _metadataURI) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURICall {
        pub _metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateAVSMetadataURI(string)`](updateAVSMetadataURICall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURIReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateAVSMetadataURICall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURICall) -> Self {
                    (value._metadataURI,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAVSMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _metadataURI: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateAVSMetadataURIReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURIReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAVSMetadataURIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateAVSMetadataURICall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateAVSMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateAVSMetadataURI(string)";
            const SELECTOR: [u8; 4] = [169u8, 143u8, 179u8, 85u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self._metadataURI,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`WavsServiceManager`](self) function calls.
    pub enum WavsServiceManagerCalls {
        addSignedPayloadForTrigger(addSignedPayloadForTriggerCall),
        addSignedPayloadForTriggerMulti(addSignedPayloadForTriggerMultiCall),
        avsDirectory(avsDirectoryCall),
        createAVSRewardsSubmission(createAVSRewardsSubmissionCall),
        createOperatorDirectedAVSRewardsSubmission(
            createOperatorDirectedAVSRewardsSubmissionCall,
        ),
        deregisterOperatorFromAVS(deregisterOperatorFromAVSCall),
        getOperatorRestakedStrategies(getOperatorRestakedStrategiesCall),
        getRestakeableStrategies(getRestakeableStrategiesCall),
        owner(ownerCall),
        registerOperatorToAVS(registerOperatorToAVSCall),
        renounceOwnership(renounceOwnershipCall),
        rewardsInitiator(rewardsInitiatorCall),
        setClaimerFor(setClaimerForCall),
        setRewardsInitiator(setRewardsInitiatorCall),
        signedDataByTriggerId(signedDataByTriggerIdCall),
        stakeRegistry(stakeRegistryCall),
        transferOwnership(transferOwnershipCall),
        updateAVSMetadataURI(updateAVSMetadataURICall),
    }
    #[automatically_derived]
    impl WavsServiceManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [39u8, 111u8, 133u8, 242u8],
            [51u8, 207u8, 183u8, 183u8],
            [59u8, 194u8, 140u8, 140u8],
            [81u8, 202u8, 234u8, 186u8],
            [104u8, 48u8, 72u8, 53u8],
            [107u8, 58u8, 167u8, 46u8],
            [113u8, 80u8, 24u8, 166u8],
            [141u8, 165u8, 203u8, 91u8],
            [153u8, 38u8, 238u8, 125u8],
            [157u8, 30u8, 64u8, 83u8],
            [160u8, 22u8, 157u8, 221u8],
            [162u8, 11u8, 153u8, 191u8],
            [163u8, 100u8, 244u8, 218u8],
            [169u8, 143u8, 179u8, 85u8],
            [228u8, 129u8, 175u8, 157u8],
            [242u8, 253u8, 227u8, 139u8],
            [252u8, 41u8, 157u8, 238u8],
            [252u8, 227u8, 108u8, 125u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for WavsServiceManagerCalls {
        const NAME: &'static str = "WavsServiceManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 18usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::addSignedPayloadForTrigger(_) => {
                    <addSignedPayloadForTriggerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addSignedPayloadForTriggerMulti(_) => {
                    <addSignedPayloadForTriggerMultiCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectory(_) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createAVSRewardsSubmission(_) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createOperatorDirectedAVSRewardsSubmission(_) => {
                    <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperatorFromAVS(_) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorRestakedStrategies(_) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRestakeableStrategies(_) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::registerOperatorToAVS(_) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rewardsInitiator(_) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setClaimerFor(_) => {
                    <setClaimerForCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setRewardsInitiator(_) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::signedDataByTriggerId(_) => {
                    <signedDataByTriggerIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateAVSMetadataURI(_) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<WavsServiceManagerCalls>] = &[
                {
                    fn signedDataByTriggerId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <signedDataByTriggerIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::signedDataByTriggerId)
                    }
                    signedDataByTriggerId
                },
                {
                    fn getOperatorRestakedStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::getOperatorRestakedStrategies)
                    }
                    getOperatorRestakedStrategies
                },
                {
                    fn setRewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::setRewardsInitiator)
                    }
                    setRewardsInitiator
                },
                {
                    fn addSignedPayloadForTriggerMulti(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <addSignedPayloadForTriggerMultiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                WavsServiceManagerCalls::addSignedPayloadForTriggerMulti,
                            )
                    }
                    addSignedPayloadForTriggerMulti
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn avsDirectory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::avsDirectory)
                    }
                    avsDirectory
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn registerOperatorToAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::registerOperatorToAVS)
                    }
                    registerOperatorToAVS
                },
                {
                    fn addSignedPayloadForTrigger(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <addSignedPayloadForTriggerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::addSignedPayloadForTrigger)
                    }
                    addSignedPayloadForTrigger
                },
                {
                    fn setClaimerFor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <setClaimerForCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::setClaimerFor)
                    }
                    setClaimerFor
                },
                {
                    fn createOperatorDirectedAVSRewardsSubmission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                WavsServiceManagerCalls::createOperatorDirectedAVSRewardsSubmission,
                            )
                    }
                    createOperatorDirectedAVSRewardsSubmission
                },
                {
                    fn deregisterOperatorFromAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::deregisterOperatorFromAVS)
                    }
                    deregisterOperatorFromAVS
                },
                {
                    fn updateAVSMetadataURI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::updateAVSMetadataURI)
                    }
                    updateAVSMetadataURI
                },
                {
                    fn getRestakeableStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::getRestakeableStrategies)
                    }
                    getRestakeableStrategies
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn rewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::rewardsInitiator)
                    }
                    rewardsInitiator
                },
                {
                    fn createAVSRewardsSubmission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerCalls> {
                        <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerCalls::createAVSRewardsSubmission)
                    }
                    createAVSRewardsSubmission
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::addSignedPayloadForTrigger(inner) => {
                    <addSignedPayloadForTriggerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addSignedPayloadForTriggerMulti(inner) => {
                    <addSignedPayloadForTriggerMultiCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createAVSRewardsSubmission(inner) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createOperatorDirectedAVSRewardsSubmission(inner) => {
                    <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperatorFromAVS(inner) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorRestakedStrategies(inner) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRestakeableStrategies(inner) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerOperatorToAVS(inner) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rewardsInitiator(inner) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setClaimerFor(inner) => {
                    <setClaimerForCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setRewardsInitiator(inner) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::signedDataByTriggerId(inner) => {
                    <signedDataByTriggerIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::addSignedPayloadForTrigger(inner) => {
                    <addSignedPayloadForTriggerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addSignedPayloadForTriggerMulti(inner) => {
                    <addSignedPayloadForTriggerMultiCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createAVSRewardsSubmission(inner) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createOperatorDirectedAVSRewardsSubmission(inner) => {
                    <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperatorFromAVS(inner) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorRestakedStrategies(inner) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRestakeableStrategies(inner) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registerOperatorToAVS(inner) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::rewardsInitiator(inner) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setClaimerFor(inner) => {
                    <setClaimerForCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setRewardsInitiator(inner) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::signedDataByTriggerId(inner) => {
                    <signedDataByTriggerIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`WavsServiceManager`](self) custom errors.
    pub enum WavsServiceManagerErrors {
        InvalidSignature(InvalidSignature),
    }
    #[automatically_derived]
    impl WavsServiceManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[139u8, 170u8, 87u8, 159u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for WavsServiceManagerErrors {
        const NAME: &'static str = "WavsServiceManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<WavsServiceManagerErrors>] = &[
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<WavsServiceManagerErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(WavsServiceManagerErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`WavsServiceManager`](self) events.
    pub enum WavsServiceManagerEvents {
        AddedSignedPayloadForTrigger(AddedSignedPayloadForTrigger),
        Initialized(Initialized),
        OwnershipTransferred(OwnershipTransferred),
        RewardsInitiatorUpdated(RewardsInitiatorUpdated),
    }
    #[automatically_derived]
    impl WavsServiceManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                65u8,
                180u8,
                45u8,
                57u8,
                180u8,
                199u8,
                68u8,
                30u8,
                237u8,
                249u8,
                203u8,
                124u8,
                26u8,
                112u8,
                210u8,
                247u8,
                119u8,
                24u8,
                70u8,
                171u8,
                132u8,
                217u8,
                242u8,
                69u8,
                236u8,
                9u8,
                29u8,
                77u8,
                250u8,
                20u8,
                133u8,
                91u8,
            ],
            [
                127u8,
                38u8,
                184u8,
                63u8,
                249u8,
                110u8,
                31u8,
                43u8,
                106u8,
                104u8,
                47u8,
                19u8,
                56u8,
                82u8,
                246u8,
                121u8,
                138u8,
                9u8,
                196u8,
                101u8,
                218u8,
                149u8,
                146u8,
                20u8,
                96u8,
                206u8,
                251u8,
                56u8,
                71u8,
                64u8,
                36u8,
                152u8,
            ],
            [
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ],
            [
                225u8,
                28u8,
                221u8,
                241u8,
                129u8,
                106u8,
                67u8,
                49u8,
                140u8,
                161u8,
                117u8,
                187u8,
                197u8,
                44u8,
                208u8,
                24u8,
                84u8,
                54u8,
                233u8,
                203u8,
                234u8,
                215u8,
                200u8,
                58u8,
                204u8,
                84u8,
                167u8,
                62u8,
                70u8,
                23u8,
                23u8,
                227u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for WavsServiceManagerEvents {
        const NAME: &'static str = "WavsServiceManagerEvents";
        const COUNT: usize = 4usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <AddedSignedPayloadForTrigger as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <AddedSignedPayloadForTrigger as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AddedSignedPayloadForTrigger)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(
                    <RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RewardsInitiatorUpdated)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for WavsServiceManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AddedSignedPayloadForTrigger(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RewardsInitiatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AddedSignedPayloadForTrigger(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RewardsInitiatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`WavsServiceManager`](self) contract instance.

See the [wrapper's documentation](`WavsServiceManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> WavsServiceManagerInstance<T, P, N> {
        WavsServiceManagerInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _avsDirectory: alloy::sol_types::private::Address,
        _stakeRegistry: alloy::sol_types::private::Address,
        _rewardsCoordinator: alloy::sol_types::private::Address,
        _delegationManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<WavsServiceManagerInstance<T, P, N>>,
    > {
        WavsServiceManagerInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _avsDirectory,
            _stakeRegistry,
            _rewardsCoordinator,
            _delegationManager,
        )
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
        _avsDirectory: alloy::sol_types::private::Address,
        _stakeRegistry: alloy::sol_types::private::Address,
        _rewardsCoordinator: alloy::sol_types::private::Address,
        _delegationManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        WavsServiceManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _avsDirectory,
            _stakeRegistry,
            _rewardsCoordinator,
            _delegationManager,
        )
    }
    /**A [`WavsServiceManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`WavsServiceManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct WavsServiceManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for WavsServiceManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("WavsServiceManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > WavsServiceManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`WavsServiceManager`](self) contract instance.

See the [wrapper's documentation](`WavsServiceManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
            _avsDirectory: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
            _rewardsCoordinator: alloy::sol_types::private::Address,
            _delegationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<WavsServiceManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _avsDirectory,
                _stakeRegistry,
                _rewardsCoordinator,
                _delegationManager,
            );
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(
            provider: P,
            _avsDirectory: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
            _rewardsCoordinator: alloy::sol_types::private::Address,
            _delegationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _avsDirectory,
                            _stakeRegistry,
                            _rewardsCoordinator,
                            _delegationManager,
                        },
                    )[..],
                ]
                    .concat()
                    .into(),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> WavsServiceManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> WavsServiceManagerInstance<T, P, N> {
            WavsServiceManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > WavsServiceManagerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`addSignedPayloadForTrigger`] function.
        pub fn addSignedPayloadForTrigger(
            &self,
            signedPayload: <ILayerServiceManager::SignedPayload as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, addSignedPayloadForTriggerCall, N> {
            self.call_builder(
                &addSignedPayloadForTriggerCall {
                    signedPayload,
                },
            )
        }
        ///Creates a new call builder for the [`addSignedPayloadForTriggerMulti`] function.
        pub fn addSignedPayloadForTriggerMulti(
            &self,
            signedPayloads: alloy::sol_types::private::Vec<
                <ILayerServiceManager::SignedPayload as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            addSignedPayloadForTriggerMultiCall,
            N,
        > {
            self.call_builder(
                &addSignedPayloadForTriggerMultiCall {
                    signedPayloads,
                },
            )
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
        }
        ///Creates a new call builder for the [`createAVSRewardsSubmission`] function.
        pub fn createAVSRewardsSubmission(
            &self,
            rewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createAVSRewardsSubmissionCall, N> {
            self.call_builder(
                &createAVSRewardsSubmissionCall {
                    rewardsSubmissions,
                },
            )
        }
        ///Creates a new call builder for the [`createOperatorDirectedAVSRewardsSubmission`] function.
        pub fn createOperatorDirectedAVSRewardsSubmission(
            &self,
            operatorDirectedRewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinator::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            createOperatorDirectedAVSRewardsSubmissionCall,
            N,
        > {
            self.call_builder(
                &createOperatorDirectedAVSRewardsSubmissionCall {
                    operatorDirectedRewardsSubmissions,
                },
            )
        }
        ///Creates a new call builder for the [`deregisterOperatorFromAVS`] function.
        pub fn deregisterOperatorFromAVS(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorFromAVSCall, N> {
            self.call_builder(
                &deregisterOperatorFromAVSCall {
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorRestakedStrategies`] function.
        pub fn getOperatorRestakedStrategies(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getOperatorRestakedStrategiesCall,
            N,
        > {
            self.call_builder(
                &getOperatorRestakedStrategiesCall {
                    _operator,
                },
            )
        }
        ///Creates a new call builder for the [`getRestakeableStrategies`] function.
        pub fn getRestakeableStrategies(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRestakeableStrategiesCall, N> {
            self.call_builder(&getRestakeableStrategiesCall {})
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`registerOperatorToAVS`] function.
        pub fn registerOperatorToAVS(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorToAVSCall, N> {
            self.call_builder(
                &registerOperatorToAVSCall {
                    operator,
                    operatorSignature,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`rewardsInitiator`] function.
        pub fn rewardsInitiator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, rewardsInitiatorCall, N> {
            self.call_builder(&rewardsInitiatorCall {})
        }
        ///Creates a new call builder for the [`setClaimerFor`] function.
        pub fn setClaimerFor(
            &self,
            claimer: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setClaimerForCall, N> {
            self.call_builder(&setClaimerForCall { claimer })
        }
        ///Creates a new call builder for the [`setRewardsInitiator`] function.
        pub fn setRewardsInitiator(
            &self,
            newRewardsInitiator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setRewardsInitiatorCall, N> {
            self.call_builder(
                &setRewardsInitiatorCall {
                    newRewardsInitiator,
                },
            )
        }
        ///Creates a new call builder for the [`signedDataByTriggerId`] function.
        pub fn signedDataByTriggerId(
            &self,
            _0: <ILayerTrigger::TriggerId as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, signedDataByTriggerIdCall, N> {
            self.call_builder(&signedDataByTriggerIdCall { _0 })
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
            self.call_builder(&stakeRegistryCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`updateAVSMetadataURI`] function.
        pub fn updateAVSMetadataURI(
            &self,
            _metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateAVSMetadataURICall, N> {
            self.call_builder(
                &updateAVSMetadataURICall {
                    _metadataURI,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > WavsServiceManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`AddedSignedPayloadForTrigger`] event.
        pub fn AddedSignedPayloadForTrigger_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AddedSignedPayloadForTrigger, N> {
            self.event_filter::<AddedSignedPayloadForTrigger>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`RewardsInitiatorUpdated`] event.
        pub fn RewardsInitiatorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RewardsInitiatorUpdated, N> {
            self.event_filter::<RewardsInitiatorUpdated>()
        }
    }
}
