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
library ISignatureUtils {
    struct SignatureWithSaltAndExpiry {
        bytes signature;
        bytes32 salt;
        uint256 expiry;
    }
}

interface ECDSAStakeRegistry {
    struct Quorum {
        StrategyParams[] strategies;
    }
    struct StrategyParams {
        address strategy;
        uint96 multiplier;
    }

    error InsufficientSignedStake();
    error InsufficientWeight();
    error InvalidLength();
    error InvalidQuorum();
    error InvalidReferenceBlock();
    error InvalidSignature();
    error InvalidSignedWeight();
    error InvalidThreshold();
    error LengthMismatch();
    error MustUpdateAllOperators();
    error NotSorted();
    error OperatorAlreadyRegistered();
    error OperatorNotRegistered();

    event Initialized(uint8 version);
    event MinimumWeightUpdated(uint256 _old, uint256 _new);
    event OperatorDeregistered(address indexed _operator, address indexed _avs);
    event OperatorRegistered(address indexed _operator, address indexed _avs);
    event OperatorWeightUpdated(address indexed _operator, uint256 oldWeight, uint256 newWeight);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event QuorumUpdated(Quorum _old, Quorum _new);
    event SigningKeyUpdate(address indexed operator, uint256 indexed updateBlock, address indexed newSigningKey, address oldSigningKey);
    event ThresholdWeightUpdated(uint256 _thresholdWeight);
    event TotalWeightUpdated(uint256 oldTotalWeight, uint256 newTotalWeight);
    event UpdateMinimumWeight(uint256 oldMinimumWeight, uint256 newMinimumWeight);

    constructor(address _delegationManager);

    function deregisterOperator() external;
    function getLastCheckpointOperatorWeight(address _operator) external view returns (uint256);
    function getLastCheckpointThresholdWeight() external view returns (uint256);
    function getLastCheckpointThresholdWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
    function getLastCheckpointTotalWeight() external view returns (uint256);
    function getLastCheckpointTotalWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
    function getLastestOperatorSigningKey(address _operator) external view returns (address);
    function getOperatorSigningKeyAtBlock(address _operator, uint256 _blockNumber) external view returns (address);
    function getOperatorWeight(address _operator) external view returns (uint256);
    function getOperatorWeightAtBlock(address _operator, uint32 _blockNumber) external view returns (uint256);
    function initialize(address _serviceManager, uint256 _thresholdWeight, Quorum memory _quorum) external;
    function isValidSignature(bytes32 _dataHash, bytes memory _signatureData) external view returns (bytes4);
    function minimumWeight() external view returns (uint256);
    function operatorRegistered(address _operator) external view returns (bool);
    function owner() external view returns (address);
    function quorum() external view returns (Quorum memory);
    function registerOperatorWithSignature(ISignatureUtils.SignatureWithSaltAndExpiry memory _operatorSignature, address _signingKey) external;
    function renounceOwnership() external;
    function transferOwnership(address newOwner) external;
    function updateMinimumWeight(uint256 _newMinimumWeight, address[] memory _operators) external;
    function updateOperatorSigningKey(address _newSigningKey) external;
    function updateOperators(address[] memory _operators) external;
    function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory) external;
    function updateQuorumConfig(Quorum memory _quorum, address[] memory _operators) external;
    function updateStakeThreshold(uint256 _thresholdWeight) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_delegationManager",
        "type": "address",
        "internalType": "contract IDelegationManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperator",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getLastCheckpointOperatorWeight",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointThresholdWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointThresholdWeightAtBlock",
    "inputs": [
      {
        "name": "_blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointTotalWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastCheckpointTotalWeightAtBlock",
    "inputs": [
      {
        "name": "_blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLastestOperatorSigningKey",
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
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorSigningKeyAtBlock",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_blockNumber",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "getOperatorWeight",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorWeightAtBlock",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "_serviceManager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_thresholdWeight",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_quorum",
        "type": "tuple",
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
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
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isValidSignature",
    "inputs": [
      {
        "name": "_dataHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "_signatureData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "minimumWeight",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorRegistered",
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
        "type": "bool",
        "internalType": "bool"
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
    "name": "quorum",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
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
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "registerOperatorWithSignature",
    "inputs": [
      {
        "name": "_operatorSignature",
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
      },
      {
        "name": "_signingKey",
        "type": "address",
        "internalType": "address"
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
    "name": "updateMinimumWeight",
    "inputs": [
      {
        "name": "_newMinimumWeight",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorSigningKey",
    "inputs": [
      {
        "name": "_newSigningKey",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperators",
    "inputs": [
      {
        "name": "_operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateOperatorsForQuorum",
    "inputs": [
      {
        "name": "operatorsPerQuorum",
        "type": "address[][]",
        "internalType": "address[][]"
      },
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateQuorumConfig",
    "inputs": [
      {
        "name": "_quorum",
        "type": "tuple",
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
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
          }
        ]
      },
      {
        "name": "_operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateStakeThreshold",
    "inputs": [
      {
        "name": "_thresholdWeight",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "MinimumWeightUpdated",
    "inputs": [
      {
        "name": "_old",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "_new",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorDeregistered",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "_avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorRegistered",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "_avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorWeightUpdated",
    "inputs": [
      {
        "name": "_operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
    "name": "QuorumUpdated",
    "inputs": [
      {
        "name": "_old",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
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
          }
        ]
      },
      {
        "name": "_new",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct Quorum",
        "components": [
          {
            "name": "strategies",
            "type": "tuple[]",
            "internalType": "struct StrategyParams[]",
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
          }
        ]
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "SigningKeyUpdate",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "updateBlock",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "newSigningKey",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldSigningKey",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ThresholdWeightUpdated",
    "inputs": [
      {
        "name": "_thresholdWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TotalWeightUpdated",
    "inputs": [
      {
        "name": "oldTotalWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newTotalWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "UpdateMinimumWeight",
    "inputs": [
      {
        "name": "oldMinimumWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newMinimumWeight",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "InsufficientSignedStake",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InsufficientWeight",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidQuorum",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidReferenceBlock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignedWeight",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidThreshold",
    "inputs": []
  },
  {
    "type": "error",
    "name": "LengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MustUpdateAllOperators",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NotSorted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorAlreadyRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorNotRegistered",
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
pub mod ECDSAStakeRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a0604052348015600e575f80fd5b506040516127d63803806127d6833981016040819052602b91603b565b6001600160a01b03166080526066565b5f60208284031215604a575f80fd5b81516001600160a01b0381168114605f575f80fd5b9392505050565b60805161275861007e5f395f6106b101526127585ff3fe608060405234801561000f575f80fd5b5060043610610130575f3560e01c8062cf2ab5146101345780630dba3394146101495780631626ba7e1461016f5780631703a0181461019b5780631e4cd85e146101b0578063314f3a49146101c35780633b242e4a146101cb5780633d5611f6146101de57806340bf2fb7146101f15780635140a548146101f95780635e1042e81461020c5780635ef533291461022c578063696255be1461023f578063715018a614610252578063743c31f41461025a578063857dc1901461026d5780638da5cb5b14610275578063955f2d901461027d57806398ec1ac914610290578063ab118995146102a3578063b933fa74146102b6578063cdcd3581146102be578063dec5d1f6146102d1578063ec7fbb31146102e4578063f2fde38b1461031f575b5f80fd5b610147610142366004611ce6565b610332565b005b61015c610157366004611d30565b61033e565b6040519081526020015b60405180910390f35b61018261017d366004611dbe565b610359565b6040516001600160e01b03199091168152602001610166565b6101a3610395565b6040516101669190611e5e565b61015c6101be366004611d30565b610426565b61015c61043b565b61015c6101d9366004611e70565b61044b565b6101476101ec366004611e8b565b61046b565b60675461015c565b610147610207366004611f3b565b61047a565b61021f61021a366004611ffe565b61049c565b6040516101669190612028565b61014761023a36600461203c565b6104c4565b61014761024d366004612053565b6104d5565b6101476104ef565b610147610268366004611e70565b610502565b61014761053b565b61021f610544565b61015c61028b36600461208c565b610553565b61015c61029e366004611e70565b61057d565b6101476102b136600461219a565b6107c2565b61015c6108d8565b61021f6102cc366004611e70565b6108e3565b6101476102df3660046121ee565b610903565b61030f6102f2366004611e70565b6001600160a01b03165f908152606e602052604090205460ff1690565b6040519015158152602001610166565b61014761032d366004611e70565b610914565b61033b8161098a565b50565b5f610353606b63ffffffff808516906109d616565b92915050565b5f805f80848060200190518101906103719190612309565b92509250925061038386848484610add565b50630b135d3f60e11b95945050505050565b604080516020810190915260608152604080516066805460208181028401850185528301818152929391928492909184915f9085015b82821015610419575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b0316818301528252600190920191016103cb565b5050505081525050905090565b5f610353606c63ffffffff808516906109d616565b5f610446606b610b8c565b905090565b6001600160a01b0381165f908152606d6020526040812061035390610b8c565b610476338383610be4565b5050565b610476825f8151811061048f5761048f6123dd565b6020026020010151610d13565b6001600160a01b0382165f908152606a602052604081206104bd90836109d6565b9392505050565b6104cc610d36565b61033b81610d95565b6104dd610d36565b6104e682610dd8565b6104768161098a565b6104f7610d36565b6105005f610e1e565b565b335f908152606e602052604090205460ff16610531576040516325ec6c1f60e01b815260040160405180910390fd5b61033b3382610e6f565b61050033610f25565b6033546001600160a01b031690565b6001600160a01b0382165f908152606d602052604081206104bd9063ffffffff808516906109d616565b5f8060665f01805480602002602001604051908101604052809291908181526020015f905b828210156105f0575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b0316818301528252600190920191016105a2565b5050505090505f8082516001600160401b0381111561061157610611611baf565b60405190808252806020026020018201604052801561063a578160200160208202803683370190505b5090505f5b83518110156106975783818151811061065a5761065a6123dd565b60200260200101515f0151828281518110610677576106776123dd565b6001600160a01b039092166020928302919091019091015260010161063f565b50604051639004134760e01b81525f906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906390041347906106e890899086906004016123f1565b5f60405180830381865afa158015610702573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610729919081019061244f565b90505f5b845181101561079557848181518110610748576107486123dd565b6020026020010151602001516001600160601b031682828151811061076f5761076f6123dd565b602002602001015161078191906124f3565b61078b908561250a565b935060010161072d565b506107a26127108461251d565b925060675483106107b7575090949350505050565b505f95945050505050565b5f54610100900460ff16158080156107e057505f54600160ff909116105b806107f95750303b1580156107f957505f5460ff166001145b6108615760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff191660011790558015610882575f805461ff0019166101001790555b61088d848484611042565b80156108d2575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b5f610446606c610b8c565b6001600160a01b0381165f908152606a6020526040812061035390610b8c565b61090b610d36565b6104e6826110a2565b61091c610d36565b6001600160a01b0381166109815760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610858565b61033b81610e1e565b5f805b82518110156109cc576109b88382815181106109ab576109ab6123dd565b60200260200101516111f4565b6109c2908361253c565b915060010161098d565b506108d281611317565b5f438210610a265760405162461bcd60e51b815260206004820181905260248201527f436865636b706f696e74733a20626c6f636b206e6f7420796574206d696e65646044820152606401610858565b82545f5b81811015610a87575f610a3d8284611381565b905084865f018281548110610a5457610a546123dd565b5f9182526020909120015463ffffffff161115610a7357809250610a81565b610a7e81600161250a565b91505b50610a2a565b8115610ac95784610a99600184612563565b81548110610aa957610aa96123dd565b5f91825260209091200154600160201b90046001600160e01b0316610acb565b5f5b6001600160e01b031695945050505050565b5f835190505f805f80610af185885161139b565b5f5b85811015610b7657888181518110610b0d57610b0d6123dd565b60200260200101519450610b2185886113de565b9250610b2d848661142f565b610b51838b8a8481518110610b4457610b446123dd565b6020026020010151611461565b8493505f610b5f8689611492565b9050610b6b818461250a565b925050600101610af3565b50610b8181876114e3565b505050505050505050565b80545f908015610bd25782610ba2600183612563565b81548110610bb257610bb26123dd565b5f91825260209091200154600160201b90046001600160e01b0316610bd4565b5f5b6001600160e01b03169392505050565b6001600160a01b0383165f908152606e602052604090205460ff1615610c1d576040516342ee68b560e01b815260040160405180910390fd5b60658054905f610c2c83612576565b90915550506001600160a01b0383165f908152606e60205260408120805460ff19166001179055610c5c846111f4565b9050610c6781611317565b5050610c738483610e6f565b606854604051639926ee7d60e01b81526001600160a01b0390911690639926ee7d90610ca590879087906004016125bc565b5f604051808303815f87803b158015610cbc575f80fd5b505af1158015610cce573d5f803e3d5ffd5b50506068546040516001600160a01b03918216935090871691507fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c1905f90a350505050565b6065548151146103325760405163169efb5b60e11b815260040160405180910390fd5b33610d3f610544565b6001600160a01b0316146105005760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610858565b610da0606c8261153d565b50506040518181527f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b9060200160405180910390a150565b606780549082905560408051828152602081018490527f713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f91015b60405180910390a15050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b6001600160a01b0382165f908152606a60205260408120610e8f90610b8c565b9050806001600160a01b0316826001600160a01b031603610eaf57505050565b6001600160a01b038381165f908152606a60205260409020610ed291841661153d565b5050816001600160a01b031643846001600160a01b03167fd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea131500284604051610f189190612028565b60405180910390a4505050565b6001600160a01b0381165f908152606e602052604090205460ff16610f5d576040516325ec6c1f60e01b815260040160405180910390fd5b60658054905f610f6c83612606565b90915550506001600160a01b0381165f908152606e60205260408120805460ff19169055610f99826111f4565b9050610fa481611317565b50506068546040516351b27a6d60e11b81526001600160a01b039091169063a364f4da90610fd6908590600401612028565b5f604051808303815f87803b158015610fed575f80fd5b505af1158015610fff573d5f803e3d5ffd5b50506068546040516001600160a01b03918216935090851691507f31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed580905f90a35050565b5f54610100900460ff166110685760405162461bcd60e51b81526004016108589061261b565b606880546001600160a01b0319166001600160a01b03851617905561108c82610d95565b611095816110a2565b61109d61165f565b505050565b6110ab8161168d565b6110c85760405163d173577960e01b815260040160405180910390fd5b6040805160668054602081810284018501855283018181525f9484928491879085015b82821015611139575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b0316818301528252600190920191016110eb565b505050915250909150606690505f6111518282611b85565b50505f5b8251518110156111c257825180516066919083908110611177576111776123dd565b6020908102919091018101518254600181810185555f94855293839020825192909301516001600160601b0316600160a01b026001600160a01b039092169190911791015501611155565b507f23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e8183604051610e12929190612666565b6001600160a01b0381165f908152606d6020526040812081908190819061121a90610b8c565b6001600160a01b0386165f908152606e602052604090205490915060ff1661127f576112468184612693565b9250825f036112585750909392505050565b6001600160a01b0385165f908152606d602052604081206112789161153d565b50506112ca565b6112888561057d565b91506112948183612693565b9250825f036112a65750909392505050565b6001600160a01b0385165f908152606d602052604090206112c7908361153d565b50505b60408051828152602081018490526001600160a01b038716917f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe594910160405180910390a250909392505050565b5f80611323606b610b8c565b91505f611330848461253c565b9150819050611340606b8261153d565b505060408051848152602081018490527f86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b910160405180910390a150915091565b5f61138f600284841861251d565b6104bd9084841661250a565b8082146113be576040516001621398b960e31b0319815260040160405180910390fd5b815f036104765760405163251f56a160e21b815260040160405180910390fd5b5f438263ffffffff16106114055760405163e64f180f60e01b815260040160405180910390fd5b6001600160a01b0383165f908152606a602052604090206104bd9063ffffffff808516906109d616565b806001600160a01b0316826001600160a01b0316106104765760405163ba50f91160e01b815260040160405180910390fd5b6114756001600160a01b0384168383611750565b61109d57604051638baa579f60e01b815260040160405180910390fd5b5f438263ffffffff16106114b95760405163e64f180f60e01b815260040160405180910390fd5b6001600160a01b0383165f908152606d602052604090206104bd9063ffffffff808516906109d616565b5f6114ed82611895565b90508083111561151057604051634b05a0f760e11b815260040160405180910390fd5b5f61151a836118d0565b9050838111156108d25760405163e121632f60e01b815260040160405180910390fd5b81545f9081908161154d86610b8c565b90505f8211801561158957504386611566600185612563565b81548110611576576115766123dd565b5f9182526020909120015463ffffffff16145b156115e6576115978561190b565b866115a3600185612563565b815481106115b3576115b36123dd565b905f5260205f20015f0160046101000a8154816001600160e01b0302191690836001600160e01b03160217905550611651565b855f0160405180604001604052806115fd43611977565b63ffffffff1681526020016116118861190b565b6001600160e01b0390811690915282546001810184555f93845260209384902083519490930151909116600160201b0263ffffffff909316929092179101555b9250839150505b9250929050565b5f54610100900460ff166116855760405162461bcd60e51b81526004016108589061261b565b6105006119db565b80515f90818080805b845181101561172f578481815181106116b1576116b16123dd565b60200260200101515f01519250826001600160a01b0316846001600160a01b0316106116f05760405163ba50f91160e01b815260040160405180910390fd5b829350848181518110611705576117056123dd565b6020026020010151602001516001600160601b031682611725919061250a565b9150600101611696565b50612710811461174457505f95945050505050565b50600195945050505050565b5f805f61175d8585611a0a565b90925090505f816004811115611775576117756126b9565b1480156117935750856001600160a01b0316826001600160a01b0316145b156117a3576001925050506104bd565b5f80876001600160a01b0316631626ba7e60e01b88886040516024016117ca9291906126cd565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905161180891906126e5565b5f60405180830381855afa9150503d805f8114611840576040519150601f19603f3d011682016040523d82523d5f602084013e611845565b606091505b5091509150818015611858575080516020145b801561188957508051630b135d3f60e11b9061187d90830160209081019084016126fb565b6001600160e01b031916145b98975050505050505050565b5f438263ffffffff16106118bc5760405163e64f180f60e01b815260040160405180910390fd5b610353606b63ffffffff808516906109d616565b5f438263ffffffff16106118f75760405163e64f180f60e01b815260040160405180910390fd5b610353606c63ffffffff808516906109d616565b5f6001600160e01b038211156119735760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b6064820152608401610858565b5090565b5f63ffffffff8211156119735760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608401610858565b5f54610100900460ff16611a015760405162461bcd60e51b81526004016108589061261b565b61050033610e1e565b5f808251604103611a3e576020830151604084015160608501515f1a611a3287828585611a72565b94509450505050611658565b8251604003611a675760208301516040840151611a5c868383611b4d565b935093505050611658565b505f90506002611658565b5f806fa2a8918ca85bafe22016d0b997e4df60600160ff1b03831115611a9d57505f90506003611b44565b8460ff16601b14158015611ab557508460ff16601c14155b15611ac557505f90506004611b44565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015611b16573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b038116611b3e575f60019250925050611b44565b91505f90505b94509492505050565b5f806001600160ff1b03831681611b6960ff86901c601b61250a565b9050611b7787828885611a72565b935093505050935093915050565b5080545f8255905f5260205f209081019061033b91905b80821115611973575f8155600101611b9c565b634e487b7160e01b5f52604160045260245ffd5b604051602081016001600160401b0381118282101715611be557611be5611baf565b60405290565b604080519081016001600160401b0381118282101715611be557611be5611baf565b604051601f8201601f191681016001600160401b0381118282101715611c3557611c35611baf565b604052919050565b5f6001600160401b03821115611c5557611c55611baf565b5060051b60200190565b6001600160a01b038116811461033b575f80fd5b5f82601f830112611c82575f80fd5b8135611c95611c9082611c3d565b611c0d565b8082825260208201915060208360051b860101925085831115611cb6575f80fd5b602085015b83811015611cdc578035611cce81611c5f565b835260209283019201611cbb565b5095945050505050565b5f60208284031215611cf6575f80fd5b81356001600160401b03811115611d0b575f80fd5b611d1784828501611c73565b949350505050565b63ffffffff8116811461033b575f80fd5b5f60208284031215611d40575f80fd5b81356104bd81611d1f565b5f6001600160401b03821115611d6357611d63611baf565b50601f01601f191660200190565b5f82601f830112611d80575f80fd5b8135611d8e611c9082611d4b565b818152846020838601011115611da2575f80fd5b816020850160208301375f918101602001919091529392505050565b5f8060408385031215611dcf575f80fd5b8235915060208301356001600160401b03811115611deb575f80fd5b611df785828601611d71565b9150509250929050565b8051602080845281518482018190525f9290910190829060408601905b80831015611cdc57835180516001600160a01b031683526020908101516001600160601b0316818401529093019260019290920191604090910190611e1e565b602081525f6104bd6020830184611e01565b5f60208284031215611e80575f80fd5b81356104bd81611c5f565b5f8060408385031215611e9c575f80fd5b82356001600160401b03811115611eb1575f80fd5b830160608186031215611ec2575f80fd5b604051606081016001600160401b0381118282101715611ee457611ee4611baf565b60405281356001600160401b03811115611efc575f80fd5b611f0887828501611d71565b82525060208281013581830152604092830135928201929092529250830135611f3081611c5f565b809150509250929050565b5f8060408385031215611f4c575f80fd5b82356001600160401b03811115611f61575f80fd5b8301601f81018513611f71575f80fd5b8035611f7f611c9082611c3d565b8082825260208201915060208360051b850101925087831115611fa0575f80fd5b602084015b83811015611fe05780356001600160401b03811115611fc2575f80fd5b611fd18a602083890101611c73565b84525060209283019201611fa5565b50945050505060208301356001600160401b03811115611deb575f80fd5b5f806040838503121561200f575f80fd5b823561201a81611c5f565b946020939093013593505050565b6001600160a01b0391909116815260200190565b5f6020828403121561204c575f80fd5b5035919050565b5f8060408385031215612064575f80fd5b8235915060208301356001600160401b03811115612080575f80fd5b611df785828601611c73565b5f806040838503121561209d575f80fd5b82356120a881611c5f565b91506020830135611f3081611d1f565b5f602082840312156120c8575f80fd5b6120d0611bc3565b905081356001600160401b038111156120e7575f80fd5b8201601f810184136120f7575f80fd5b8035612105611c9082611c3d565b8082825260208201915060208360061b850101925086831115612126575f80fd5b6020840193505b8284101561218e5760408488031215612144575f80fd5b61214c611beb565b843561215781611c5f565b815260208501356001600160601b0381168114612172575f80fd5b806020830152508083525060208201915060408401935061212d565b84525091949350505050565b5f805f606084860312156121ac575f80fd5b83356121b781611c5f565b92506020840135915060408401356001600160401b038111156121d8575f80fd5b6121e4868287016120b8565b9150509250925092565b5f80604083850312156121ff575f80fd5b82356001600160401b03811115612214575f80fd5b612220858286016120b8565b92505060208301356001600160401b03811115612080575f80fd5b5f82601f83011261224a575f80fd5b8151612258611c9082611c3d565b8082825260208201915060208360051b860101925085831115612279575f80fd5b602085015b83811015611cdc5780516001600160401b0381111561229b575f80fd5b8601603f810188136122ab575f80fd5b60208101516122bc611c9082611d4b565b8181526040838301018a10156122d0575f80fd5b8160408401602083015e5f6020838301015280865250505060208301925060208101905061227e565b805161230481611d1f565b919050565b5f805f6060848603121561231b575f80fd5b83516001600160401b03811115612330575f80fd5b8401601f81018613612340575f80fd5b805161234e611c9082611c3d565b8082825260208201915060208360051b85010192508883111561236f575f80fd5b6020840193505b8284101561239a57835161238981611c5f565b825260209384019390910190612376565b6020880151909650925050506001600160401b038111156123b9575f80fd5b6123c58682870161223b565b9250506123d4604085016122f9565b90509250925092565b634e487b7160e01b5f52603260045260245ffd5b6001600160a01b03831681526040602080830182905283519183018290525f91908401906060840190835b818110156124435783516001600160a01b031683526020938401939092019160010161241c565b50909695505050505050565b5f6020828403121561245f575f80fd5b81516001600160401b03811115612474575f80fd5b8201601f81018413612484575f80fd5b8051612492611c9082611c3d565b8082825260208201915060208360051b8501019250868311156124b3575f80fd5b6020840193505b828410156124d55783518252602093840193909101906124ba565b9695505050505050565b634e487b7160e01b5f52601160045260245ffd5b8082028115828204841417610353576103536124df565b80820180821115610353576103536124df565b5f8261253757634e487b7160e01b5f52601260045260245ffd5b500490565b8082018281125f83128015821682158216171561255b5761255b6124df565b505092915050565b81810381811115610353576103536124df565b5f60018201612587576125876124df565b5060010190565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b60018060a01b0383168152604060208201525f8251606060408401526125e560a084018261258e565b90506020840151606084015260408401516080840152809150509392505050565b5f81612614576126146124df565b505f190190565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b604081525f6126786040830185611e01565b828103602084015261268a8185611e01565b95945050505050565b8181035f8312801583831316838312821617156126b2576126b26124df565b5092915050565b634e487b7160e01b5f52602160045260245ffd5b828152604060208201525f611d17604083018461258e565b5f82518060208501845e5f920191825250919050565b5f6020828403121561270b575f80fd5b81516001600160e01b0319811681146104bd575f80fdfea264697066735822122082e9a76615d2262622df7ac8410170614df2238dc5c61021d1b2d7afa2b2101f64736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15`\x0EW_\x80\xFD[P`@Qa'\xD68\x03\x80a'\xD6\x839\x81\x01`@\x81\x90R`+\x91`;V[`\x01`\x01`\xA0\x1B\x03\x16`\x80R`fV[_` \x82\x84\x03\x12\x15`JW_\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`_W_\x80\xFD[\x93\x92PPPV[`\x80Qa'Xa\0~_9_a\x06\xB1\x01Ra'X_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x010W_5`\xE0\x1C\x80b\xCF*\xB5\x14a\x014W\x80c\r\xBA3\x94\x14a\x01IW\x80c\x16&\xBA~\x14a\x01oW\x80c\x17\x03\xA0\x18\x14a\x01\x9BW\x80c\x1EL\xD8^\x14a\x01\xB0W\x80c1O:I\x14a\x01\xC3W\x80c;$.J\x14a\x01\xCBW\x80c=V\x11\xF6\x14a\x01\xDEW\x80c@\xBF/\xB7\x14a\x01\xF1W\x80cQ@\xA5H\x14a\x01\xF9W\x80c^\x10B\xE8\x14a\x02\x0CW\x80c^\xF53)\x14a\x02,W\x80cibU\xBE\x14a\x02?W\x80cqP\x18\xA6\x14a\x02RW\x80ct<1\xF4\x14a\x02ZW\x80c\x85}\xC1\x90\x14a\x02mW\x80c\x8D\xA5\xCB[\x14a\x02uW\x80c\x95_-\x90\x14a\x02}W\x80c\x98\xEC\x1A\xC9\x14a\x02\x90W\x80c\xAB\x11\x89\x95\x14a\x02\xA3W\x80c\xB93\xFAt\x14a\x02\xB6W\x80c\xCD\xCD5\x81\x14a\x02\xBEW\x80c\xDE\xC5\xD1\xF6\x14a\x02\xD1W\x80c\xEC\x7F\xBB1\x14a\x02\xE4W\x80c\xF2\xFD\xE3\x8B\x14a\x03\x1FW[_\x80\xFD[a\x01Ga\x01B6`\x04a\x1C\xE6V[a\x032V[\0[a\x01\\a\x01W6`\x04a\x1D0V[a\x03>V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x82a\x01}6`\x04a\x1D\xBEV[a\x03YV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x01fV[a\x01\xA3a\x03\x95V[`@Qa\x01f\x91\x90a\x1E^V[a\x01\\a\x01\xBE6`\x04a\x1D0V[a\x04&V[a\x01\\a\x04;V[a\x01\\a\x01\xD96`\x04a\x1EpV[a\x04KV[a\x01Ga\x01\xEC6`\x04a\x1E\x8BV[a\x04kV[`gTa\x01\\V[a\x01Ga\x02\x076`\x04a\x1F;V[a\x04zV[a\x02\x1Fa\x02\x1A6`\x04a\x1F\xFEV[a\x04\x9CV[`@Qa\x01f\x91\x90a (V[a\x01Ga\x02:6`\x04a <V[a\x04\xC4V[a\x01Ga\x02M6`\x04a SV[a\x04\xD5V[a\x01Ga\x04\xEFV[a\x01Ga\x02h6`\x04a\x1EpV[a\x05\x02V[a\x01Ga\x05;V[a\x02\x1Fa\x05DV[a\x01\\a\x02\x8B6`\x04a \x8CV[a\x05SV[a\x01\\a\x02\x9E6`\x04a\x1EpV[a\x05}V[a\x01Ga\x02\xB16`\x04a!\x9AV[a\x07\xC2V[a\x01\\a\x08\xD8V[a\x02\x1Fa\x02\xCC6`\x04a\x1EpV[a\x08\xE3V[a\x01Ga\x02\xDF6`\x04a!\xEEV[a\t\x03V[a\x03\x0Fa\x02\xF26`\x04a\x1EpV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`n` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01fV[a\x01Ga\x03-6`\x04a\x1EpV[a\t\x14V[a\x03;\x81a\t\x8AV[PV[_a\x03S`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\t\xD6\x16V[\x92\x91PPV[_\x80_\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03q\x91\x90a#\tV[\x92P\x92P\x92Pa\x03\x83\x86\x84\x84\x84a\n\xDDV[Pc\x0B\x13]?`\xE1\x1B\x95\x94PPPPPV[`@\x80Q` \x81\x01\x90\x91R``\x81R`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R\x92\x93\x91\x92\x84\x92\x90\x91\x84\x91_\x90\x85\x01[\x82\x82\x10\x15a\x04\x19W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x03\xCBV[PPPP\x81RPP\x90P\x90V[_a\x03S`lc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\t\xD6\x16V[_a\x04F`ka\x0B\x8CV[\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`m` R`@\x81 a\x03S\x90a\x0B\x8CV[a\x04v3\x83\x83a\x0B\xE4V[PPV[a\x04v\x82_\x81Q\x81\x10a\x04\x8FWa\x04\x8Fa#\xDDV[` \x02` \x01\x01Qa\r\x13V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`j` R`@\x81 a\x04\xBD\x90\x83a\t\xD6V[\x93\x92PPPV[a\x04\xCCa\r6V[a\x03;\x81a\r\x95V[a\x04\xDDa\r6V[a\x04\xE6\x82a\r\xD8V[a\x04v\x81a\t\x8AV[a\x04\xF7a\r6V[a\x05\0_a\x0E\x1EV[V[3_\x90\x81R`n` R`@\x90 T`\xFF\x16a\x051W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03;3\x82a\x0EoV[a\x05\x003a\x0F%V[`3T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`m` R`@\x81 a\x04\xBD\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\t\xD6\x16V[_\x80`f_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\xF0W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x05\xA2V[PPPP\x90P_\x80\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\x11Wa\x06\x11a\x1B\xAFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06:W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x06\x97W\x83\x81\x81Q\x81\x10a\x06ZWa\x06Za#\xDDV[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a\x06wWa\x06wa#\xDDV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x06?V[P`@Qc\x90\x04\x13G`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x90\x04\x13G\x90a\x06\xE8\x90\x89\x90\x86\x90`\x04\x01a#\xF1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x02W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07)\x91\x90\x81\x01\x90a$OV[\x90P_[\x84Q\x81\x10\x15a\x07\x95W\x84\x81\x81Q\x81\x10a\x07HWa\x07Ha#\xDDV[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x07oWa\x07oa#\xDDV[` \x02` \x01\x01Qa\x07\x81\x91\x90a$\xF3V[a\x07\x8B\x90\x85a%\nV[\x93P`\x01\x01a\x07-V[Pa\x07\xA2a'\x10\x84a%\x1DV[\x92P`gT\x83\x10a\x07\xB7WP\x90\x94\x93PPPPV[P_\x95\x94PPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x07\xE0WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x07\xF9WP0;\x15\x80\x15a\x07\xF9WP_T`\xFF\x16`\x01\x14[a\x08aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x08\x82W_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x08\x8D\x84\x84\x84a\x10BV[\x80\x15a\x08\xD2W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[_a\x04F`la\x0B\x8CV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`j` R`@\x81 a\x03S\x90a\x0B\x8CV[a\t\x0Ba\r6V[a\x04\xE6\x82a\x10\xA2V[a\t\x1Ca\r6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\t\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08XV[a\x03;\x81a\x0E\x1EV[_\x80[\x82Q\x81\x10\x15a\t\xCCWa\t\xB8\x83\x82\x81Q\x81\x10a\t\xABWa\t\xABa#\xDDV[` \x02` \x01\x01Qa\x11\xF4V[a\t\xC2\x90\x83a%<V[\x91P`\x01\x01a\t\x8DV[Pa\x08\xD2\x81a\x13\x17V[_C\x82\x10a\n&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R`d\x01a\x08XV[\x82T_[\x81\x81\x10\x15a\n\x87W_a\n=\x82\x84a\x13\x81V[\x90P\x84\x86_\x01\x82\x81T\x81\x10a\nTWa\nTa#\xDDV[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\nsW\x80\x92Pa\n\x81V[a\n~\x81`\x01a%\nV[\x91P[Pa\n*V[\x81\x15a\n\xC9W\x84a\n\x99`\x01\x84a%cV[\x81T\x81\x10a\n\xA9Wa\n\xA9a#\xDDV[_\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\n\xCBV[_[`\x01`\x01`\xE0\x1B\x03\x16\x95\x94PPPPPV[_\x83Q\x90P_\x80_\x80a\n\xF1\x85\x88Qa\x13\x9BV[_[\x85\x81\x10\x15a\x0BvW\x88\x81\x81Q\x81\x10a\x0B\rWa\x0B\ra#\xDDV[` \x02` \x01\x01Q\x94Pa\x0B!\x85\x88a\x13\xDEV[\x92Pa\x0B-\x84\x86a\x14/V[a\x0BQ\x83\x8B\x8A\x84\x81Q\x81\x10a\x0BDWa\x0BDa#\xDDV[` \x02` \x01\x01Qa\x14aV[\x84\x93P_a\x0B_\x86\x89a\x14\x92V[\x90Pa\x0Bk\x81\x84a%\nV[\x92PP`\x01\x01a\n\xF3V[Pa\x0B\x81\x81\x87a\x14\xE3V[PPPPPPPPPV[\x80T_\x90\x80\x15a\x0B\xD2W\x82a\x0B\xA2`\x01\x83a%cV[\x81T\x81\x10a\x0B\xB2Wa\x0B\xB2a#\xDDV[_\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0B\xD4V[_[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`n` R`@\x90 T`\xFF\x16\x15a\x0C\x1DW`@QcB\xEEh\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90_a\x0C,\x83a%vV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`n` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x0C\\\x84a\x11\xF4V[\x90Pa\x0Cg\x81a\x13\x17V[PPa\x0Cs\x84\x83a\x0EoV[`hT`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x99&\xEE}\x90a\x0C\xA5\x90\x87\x90\x87\x90`\x04\x01a%\xBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\xBCW_\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\xCEW=_\x80>=_\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x87\x16\x91P\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x90_\x90\xA3PPPPV[`eT\x81Q\x14a\x032W`@Qc\x16\x9E\xFB[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\r?a\x05DV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08XV[a\r\xA0`l\x82a\x15=V[PP`@Q\x81\x81R\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`g\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7Fq<\xA5;\x88\xD6\xEBc\xF5\xB1\x85L\xB8\xCB\xDDsn\xC5\x1E\xDA\"^Fy\x1A\xA9)\x8B\x01`d\x8F\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`j` R`@\x81 a\x0E\x8F\x90a\x0B\x8CV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x0E\xAFWPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`j` R`@\x90 a\x0E\xD2\x91\x84\x16a\x15=V[PP\x81`\x01`\x01`\xA0\x1B\x03\x16C\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD0a\x16\x82R\xF4As6X\xF0\x9EM\x8F[-\x99\x8E\xD4\xEF$\xA2\xBB\xFDl\xEC\xA5.\xA11P\x02\x84`@Qa\x0F\x18\x91\x90a (V[`@Q\x80\x91\x03\x90\xA4PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`n` R`@\x90 T`\xFF\x16a\x0F]W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90_a\x0Fl\x83a&\x06V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`n` R`@\x81 \x80T`\xFF\x19\x16\x90Ua\x0F\x99\x82a\x11\xF4V[\x90Pa\x0F\xA4\x81a\x13\x17V[PP`hT`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA3d\xF4\xDA\x90a\x0F\xD6\x90\x85\x90`\x04\x01a (V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\xEDW_\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xFFW=_\x80>=_\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x85\x16\x91P\x7F1\xE0\xAD\xFE\xC7\x1B\xCC\xEE7\xB6\xE8:\x90\xC2\xFE\xDB\x17\xD8\xF1i?\xEE\x86<Gq\xE7\xBF\xE2\xAE\xD5\x80\x90_\x90\xA3PPV[_Ta\x01\0\x90\x04`\xFF\x16a\x10hW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08X\x90a&\x1BV[`h\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ua\x10\x8C\x82a\r\x95V[a\x10\x95\x81a\x10\xA2V[a\x10\x9Da\x16_V[PPPV[a\x10\xAB\x81a\x16\x8DV[a\x10\xC8W`@Qc\xD1sWy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R_\x94\x84\x92\x84\x91\x87\x90\x85\x01[\x82\x82\x10\x15a\x119W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x10\xEBV[PPP\x91RP\x90\x91P`f\x90P_a\x11Q\x82\x82a\x1B\x85V[PP_[\x82QQ\x81\x10\x15a\x11\xC2W\x82Q\x80Q`f\x91\x90\x83\x90\x81\x10a\x11wWa\x11wa#\xDDV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x81\x01\x85U_\x94\x85R\x93\x83\x90 \x82Q\x92\x90\x93\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x91\x01U\x01a\x11UV[P\x7F#\xAA\xD4\xE6\x17D\xEC\xE1d\x13\n\xA4\x15\xC1an\x80\x13k\x0F\x07p\xE5e\x89C\x8B\x90\xB2i&^\x81\x83`@Qa\x0E\x12\x92\x91\x90a&fV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`m` R`@\x81 \x81\x90\x81\x90\x81\x90a\x12\x1A\x90a\x0B\x8CV[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`n` R`@\x90 T\x90\x91P`\xFF\x16a\x12\x7FWa\x12F\x81\x84a&\x93V[\x92P\x82_\x03a\x12XWP\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`m` R`@\x81 a\x12x\x91a\x15=V[PPa\x12\xCAV[a\x12\x88\x85a\x05}V[\x91Pa\x12\x94\x81\x83a&\x93V[\x92P\x82_\x03a\x12\xA6WP\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`m` R`@\x90 a\x12\xC7\x90\x83a\x15=V[PP[`@\x80Q\x82\x81R` \x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16\x91\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x91\x01`@Q\x80\x91\x03\x90\xA2P\x90\x93\x92PPPV[_\x80a\x13#`ka\x0B\x8CV[\x91P_a\x130\x84\x84a%<V[\x91P\x81\x90Pa\x13@`k\x82a\x15=V[PP`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x7F\x86\xDC\xF8k\x12\xDF\xEE\xDE\xA7J\xE90\r\xBD\xAA\x19;\xCC\xE5\x80\x93i\xC8\x17~\xA2\xF4\xEA\xAAer\x9B\x91\x01`@Q\x80\x91\x03\x90\xA1P\x91P\x91V[_a\x13\x8F`\x02\x84\x84\x18a%\x1DV[a\x04\xBD\x90\x84\x84\x16a%\nV[\x80\x82\x14a\x13\xBEW`@Q`\x01b\x13\x98\xB9`\xE3\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81_\x03a\x04vW`@Qc%\x1FV\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x14\x05W`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`j` R`@\x90 a\x04\xBD\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\t\xD6\x16V[\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x10a\x04vW`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14u`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83a\x17PV[a\x10\x9DW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x14\xB9W`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`m` R`@\x90 a\x04\xBD\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\t\xD6\x16V[_a\x14\xED\x82a\x18\x95V[\x90P\x80\x83\x11\x15a\x15\x10W`@QcK\x05\xA0\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x15\x1A\x83a\x18\xD0V[\x90P\x83\x81\x11\x15a\x08\xD2W`@Qc\xE1!c/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81T_\x90\x81\x90\x81a\x15M\x86a\x0B\x8CV[\x90P_\x82\x11\x80\x15a\x15\x89WPC\x86a\x15f`\x01\x85a%cV[\x81T\x81\x10a\x15vWa\x15va#\xDDV[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x14[\x15a\x15\xE6Wa\x15\x97\x85a\x19\x0BV[\x86a\x15\xA3`\x01\x85a%cV[\x81T\x81\x10a\x15\xB3Wa\x15\xB3a#\xDDV[\x90_R` _ \x01_\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`\xE0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xE0\x1B\x03\x16\x02\x17\x90UPa\x16QV[\x85_\x01`@Q\x80`@\x01`@R\x80a\x15\xFDCa\x19wV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x16\x11\x88a\x19\x0BV[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U_\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[\x92P\x83\x91PP[\x92P\x92\x90PV[_Ta\x01\0\x90\x04`\xFF\x16a\x16\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08X\x90a&\x1BV[a\x05\0a\x19\xDBV[\x80Q_\x90\x81\x80\x80\x80[\x84Q\x81\x10\x15a\x17/W\x84\x81\x81Q\x81\x10a\x16\xB1Wa\x16\xB1a#\xDDV[` \x02` \x01\x01Q_\x01Q\x92P\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x16\xF0W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x93P\x84\x81\x81Q\x81\x10a\x17\x05Wa\x17\x05a#\xDDV[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82a\x17%\x91\x90a%\nV[\x91P`\x01\x01a\x16\x96V[Pa'\x10\x81\x14a\x17DWP_\x95\x94PPPPPV[P`\x01\x95\x94PPPPPV[_\x80_a\x17]\x85\x85a\x1A\nV[\x90\x92P\x90P_\x81`\x04\x81\x11\x15a\x17uWa\x17ua&\xB9V[\x14\x80\x15a\x17\x93WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x17\xA3W`\x01\x92PPPa\x04\xBDV[_\x80\x87`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x88\x88`@Q`$\x01a\x17\xCA\x92\x91\x90a&\xCDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x18\x08\x91\x90a&\xE5V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a\x18@W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x18EV[``\x91P[P\x91P\x91P\x81\x80\x15a\x18XWP\x80Q` \x14[\x80\x15a\x18\x89WP\x80Qc\x0B\x13]?`\xE1\x1B\x90a\x18}\x90\x83\x01` \x90\x81\x01\x90\x84\x01a&\xFBV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14[\x98\x97PPPPPPPPV[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x18\xBCW`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03S`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\t\xD6\x16V[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x18\xF7W`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03S`lc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\t\xD6\x16V[_`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x19sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x08XV[P\x90V[_c\xFF\xFF\xFF\xFF\x82\x11\x15a\x19sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x08XV[_Ta\x01\0\x90\x04`\xFF\x16a\x1A\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08X\x90a&\x1BV[a\x05\x003a\x0E\x1EV[_\x80\x82Q`A\x03a\x1A>W` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1Aa\x1A2\x87\x82\x85\x85a\x1ArV[\x94P\x94PPPPa\x16XV[\x82Q`@\x03a\x1AgW` \x83\x01Q`@\x84\x01Qa\x1A\\\x86\x83\x83a\x1BMV[\x93P\x93PPPa\x16XV[P_\x90P`\x02a\x16XV[_\x80o\xA2\xA8\x91\x8C\xA8[\xAF\xE2 \x16\xD0\xB9\x97\xE4\xDF``\x01`\xFF\x1B\x03\x83\x11\x15a\x1A\x9DWP_\x90P`\x03a\x1BDV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a\x1A\xB5WP\x84`\xFF\x16`\x1C\x14\x15[\x15a\x1A\xC5WP_\x90P`\x04a\x1BDV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1B\x16W=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1B>W_`\x01\x92P\x92PPa\x1BDV[\x91P_\x90P[\x94P\x94\x92PPPV[_\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a\x1Bi`\xFF\x86\x90\x1C`\x1Ba%\nV[\x90Pa\x1Bw\x87\x82\x88\x85a\x1ArV[\x93P\x93PPP\x93P\x93\x91PPV[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a\x03;\x91\x90[\x80\x82\x11\x15a\x19sW_\x81U`\x01\x01a\x1B\x9CV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q` \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1B\xE5Wa\x1B\xE5a\x1B\xAFV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1B\xE5Wa\x1B\xE5a\x1B\xAFV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C5Wa\x1C5a\x1B\xAFV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1CUWa\x1CUa\x1B\xAFV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03;W_\x80\xFD[_\x82`\x1F\x83\x01\x12a\x1C\x82W_\x80\xFD[\x815a\x1C\x95a\x1C\x90\x82a\x1C=V[a\x1C\rV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x1C\xB6W_\x80\xFD[` \x85\x01[\x83\x81\x10\x15a\x1C\xDCW\x805a\x1C\xCE\x81a\x1C_V[\x83R` \x92\x83\x01\x92\x01a\x1C\xBBV[P\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x1C\xF6W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x0BW_\x80\xFD[a\x1D\x17\x84\x82\x85\x01a\x1CsV[\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03;W_\x80\xFD[_` \x82\x84\x03\x12\x15a\x1D@W_\x80\xFD[\x815a\x04\xBD\x81a\x1D\x1FV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1DcWa\x1Dca\x1B\xAFV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x1D\x80W_\x80\xFD[\x815a\x1D\x8Ea\x1C\x90\x82a\x1DKV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1D\xA2W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\x1D\xCFW_\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xEBW_\x80\xFD[a\x1D\xF7\x85\x82\x86\x01a\x1DqV[\x91PP\x92P\x92\x90PV[\x80Q` \x80\x84R\x81Q\x84\x82\x01\x81\x90R_\x92\x90\x91\x01\x90\x82\x90`@\x86\x01\x90[\x80\x83\x10\x15a\x1C\xDCW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x84\x01R\x90\x93\x01\x92`\x01\x92\x90\x92\x01\x91`@\x90\x91\x01\x90a\x1E\x1EV[` \x81R_a\x04\xBD` \x83\x01\x84a\x1E\x01V[_` \x82\x84\x03\x12\x15a\x1E\x80W_\x80\xFD[\x815a\x04\xBD\x81a\x1C_V[_\x80`@\x83\x85\x03\x12\x15a\x1E\x9CW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xB1W_\x80\xFD[\x83\x01``\x81\x86\x03\x12\x15a\x1E\xC2W_\x80\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E\xE4Wa\x1E\xE4a\x1B\xAFV[`@R\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xFCW_\x80\xFD[a\x1F\x08\x87\x82\x85\x01a\x1DqV[\x82RP` \x82\x81\x015\x81\x83\x01R`@\x92\x83\x015\x92\x82\x01\x92\x90\x92R\x92P\x83\x015a\x1F0\x81a\x1C_V[\x80\x91PP\x92P\x92\x90PV[_\x80`@\x83\x85\x03\x12\x15a\x1FLW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FaW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x1FqW_\x80\xFD[\x805a\x1F\x7Fa\x1C\x90\x82a\x1C=V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a\x1F\xA0W_\x80\xFD[` \x84\x01[\x83\x81\x10\x15a\x1F\xE0W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xC2W_\x80\xFD[a\x1F\xD1\x8A` \x83\x89\x01\x01a\x1CsV[\x84RP` \x92\x83\x01\x92\x01a\x1F\xA5V[P\x94PPPP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xEBW_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a \x0FW_\x80\xFD[\x825a \x1A\x81a\x1C_V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[_` \x82\x84\x03\x12\x15a LW_\x80\xFD[P5\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a dW_\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \x80W_\x80\xFD[a\x1D\xF7\x85\x82\x86\x01a\x1CsV[_\x80`@\x83\x85\x03\x12\x15a \x9DW_\x80\xFD[\x825a \xA8\x81a\x1C_V[\x91P` \x83\x015a\x1F0\x81a\x1D\x1FV[_` \x82\x84\x03\x12\x15a \xC8W_\x80\xFD[a \xD0a\x1B\xC3V[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a \xE7W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a \xF7W_\x80\xFD[\x805a!\x05a\x1C\x90\x82a\x1C=V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a!&W_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a!\x8EW`@\x84\x88\x03\x12\x15a!DW_\x80\xFD[a!La\x1B\xEBV[\x845a!W\x81a\x1C_V[\x81R` \x85\x015`\x01`\x01``\x1B\x03\x81\x16\x81\x14a!rW_\x80\xFD[\x80` \x83\x01RP\x80\x83RP` \x82\x01\x91P`@\x84\x01\x93Pa!-V[\x84RP\x91\x94\x93PPPPV[_\x80_``\x84\x86\x03\x12\x15a!\xACW_\x80\xFD[\x835a!\xB7\x81a\x1C_V[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xD8W_\x80\xFD[a!\xE4\x86\x82\x87\x01a \xB8V[\x91PP\x92P\x92P\x92V[_\x80`@\x83\x85\x03\x12\x15a!\xFFW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x14W_\x80\xFD[a\" \x85\x82\x86\x01a \xB8V[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \x80W_\x80\xFD[_\x82`\x1F\x83\x01\x12a\"JW_\x80\xFD[\x81Qa\"Xa\x1C\x90\x82a\x1C=V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\"yW_\x80\xFD[` \x85\x01[\x83\x81\x10\x15a\x1C\xDCW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x9BW_\x80\xFD[\x86\x01`?\x81\x01\x88\x13a\"\xABW_\x80\xFD[` \x81\x01Qa\"\xBCa\x1C\x90\x82a\x1DKV[\x81\x81R`@\x83\x83\x01\x01\x8A\x10\x15a\"\xD0W_\x80\xFD[\x81`@\x84\x01` \x83\x01^_` \x83\x83\x01\x01R\x80\x86RPPP` \x83\x01\x92P` \x81\x01\x90Pa\"~V[\x80Qa#\x04\x81a\x1D\x1FV[\x91\x90PV[_\x80_``\x84\x86\x03\x12\x15a#\x1BW_\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#0W_\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a#@W_\x80\xFD[\x80Qa#Na\x1C\x90\x82a\x1C=V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a#oW_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a#\x9AW\x83Qa#\x89\x81a\x1C_V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a#vV[` \x88\x01Q\x90\x96P\x92PPP`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xB9W_\x80\xFD[a#\xC5\x86\x82\x87\x01a\";V[\x92PPa#\xD4`@\x85\x01a\"\xF9V[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x80\x83\x01\x82\x90R\x83Q\x91\x83\x01\x82\x90R_\x91\x90\x84\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a$CW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a$\x1CV[P\x90\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a$_W_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a$tW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a$\x84W_\x80\xFD[\x80Qa$\x92a\x1C\x90\x82a\x1C=V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a$\xB3W_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a$\xD5W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a$\xBAV[\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03SWa\x03Sa$\xDFV[\x80\x82\x01\x80\x82\x11\x15a\x03SWa\x03Sa$\xDFV[_\x82a%7WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a%[Wa%[a$\xDFV[PP\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x03SWa\x03Sa$\xDFV[_`\x01\x82\x01a%\x87Wa%\x87a$\xDFV[P`\x01\x01\x90V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_\x82Q```@\x84\x01Ra%\xE5`\xA0\x84\x01\x82a%\x8EV[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[_\x81a&\x14Wa&\x14a$\xDFV[P_\x19\x01\x90V[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_a&x`@\x83\x01\x85a\x1E\x01V[\x82\x81\x03` \x84\x01Ra&\x8A\x81\x85a\x1E\x01V[\x95\x94PPPPPV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&\xB2Wa&\xB2a$\xDFV[P\x92\x91PPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x82\x81R`@` \x82\x01R_a\x1D\x17`@\x83\x01\x84a%\x8EV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a'\x0BW_\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x04\xBDW_\x80\xFD\xFE\xA2dipfsX\"\x12 \x82\xE9\xA7f\x15\xD2&&\"\xDFz\xC8A\x01paM\xF2#\x8D\xC5\xC6\x10!\xD1\xB2\xD7\xAF\xA2\xB2\x10\x1FdsolcC\0\x08\x1A\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f80fd5b5060043610610130575f3560e01c8062cf2ab5146101345780630dba3394146101495780631626ba7e1461016f5780631703a0181461019b5780631e4cd85e146101b0578063314f3a49146101c35780633b242e4a146101cb5780633d5611f6146101de57806340bf2fb7146101f15780635140a548146101f95780635e1042e81461020c5780635ef533291461022c578063696255be1461023f578063715018a614610252578063743c31f41461025a578063857dc1901461026d5780638da5cb5b14610275578063955f2d901461027d57806398ec1ac914610290578063ab118995146102a3578063b933fa74146102b6578063cdcd3581146102be578063dec5d1f6146102d1578063ec7fbb31146102e4578063f2fde38b1461031f575b5f80fd5b610147610142366004611ce6565b610332565b005b61015c610157366004611d30565b61033e565b6040519081526020015b60405180910390f35b61018261017d366004611dbe565b610359565b6040516001600160e01b03199091168152602001610166565b6101a3610395565b6040516101669190611e5e565b61015c6101be366004611d30565b610426565b61015c61043b565b61015c6101d9366004611e70565b61044b565b6101476101ec366004611e8b565b61046b565b60675461015c565b610147610207366004611f3b565b61047a565b61021f61021a366004611ffe565b61049c565b6040516101669190612028565b61014761023a36600461203c565b6104c4565b61014761024d366004612053565b6104d5565b6101476104ef565b610147610268366004611e70565b610502565b61014761053b565b61021f610544565b61015c61028b36600461208c565b610553565b61015c61029e366004611e70565b61057d565b6101476102b136600461219a565b6107c2565b61015c6108d8565b61021f6102cc366004611e70565b6108e3565b6101476102df3660046121ee565b610903565b61030f6102f2366004611e70565b6001600160a01b03165f908152606e602052604090205460ff1690565b6040519015158152602001610166565b61014761032d366004611e70565b610914565b61033b8161098a565b50565b5f610353606b63ffffffff808516906109d616565b92915050565b5f805f80848060200190518101906103719190612309565b92509250925061038386848484610add565b50630b135d3f60e11b95945050505050565b604080516020810190915260608152604080516066805460208181028401850185528301818152929391928492909184915f9085015b82821015610419575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b0316818301528252600190920191016103cb565b5050505081525050905090565b5f610353606c63ffffffff808516906109d616565b5f610446606b610b8c565b905090565b6001600160a01b0381165f908152606d6020526040812061035390610b8c565b610476338383610be4565b5050565b610476825f8151811061048f5761048f6123dd565b6020026020010151610d13565b6001600160a01b0382165f908152606a602052604081206104bd90836109d6565b9392505050565b6104cc610d36565b61033b81610d95565b6104dd610d36565b6104e682610dd8565b6104768161098a565b6104f7610d36565b6105005f610e1e565b565b335f908152606e602052604090205460ff16610531576040516325ec6c1f60e01b815260040160405180910390fd5b61033b3382610e6f565b61050033610f25565b6033546001600160a01b031690565b6001600160a01b0382165f908152606d602052604081206104bd9063ffffffff808516906109d616565b5f8060665f01805480602002602001604051908101604052809291908181526020015f905b828210156105f0575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b0316818301528252600190920191016105a2565b5050505090505f8082516001600160401b0381111561061157610611611baf565b60405190808252806020026020018201604052801561063a578160200160208202803683370190505b5090505f5b83518110156106975783818151811061065a5761065a6123dd565b60200260200101515f0151828281518110610677576106776123dd565b6001600160a01b039092166020928302919091019091015260010161063f565b50604051639004134760e01b81525f906001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906390041347906106e890899086906004016123f1565b5f60405180830381865afa158015610702573d5f803e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610729919081019061244f565b90505f5b845181101561079557848181518110610748576107486123dd565b6020026020010151602001516001600160601b031682828151811061076f5761076f6123dd565b602002602001015161078191906124f3565b61078b908561250a565b935060010161072d565b506107a26127108461251d565b925060675483106107b7575090949350505050565b505f95945050505050565b5f54610100900460ff16158080156107e057505f54600160ff909116105b806107f95750303b1580156107f957505f5460ff166001145b6108615760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff191660011790558015610882575f805461ff0019166101001790555b61088d848484611042565b80156108d2575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050565b5f610446606c610b8c565b6001600160a01b0381165f908152606a6020526040812061035390610b8c565b61090b610d36565b6104e6826110a2565b61091c610d36565b6001600160a01b0381166109815760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610858565b61033b81610e1e565b5f805b82518110156109cc576109b88382815181106109ab576109ab6123dd565b60200260200101516111f4565b6109c2908361253c565b915060010161098d565b506108d281611317565b5f438210610a265760405162461bcd60e51b815260206004820181905260248201527f436865636b706f696e74733a20626c6f636b206e6f7420796574206d696e65646044820152606401610858565b82545f5b81811015610a87575f610a3d8284611381565b905084865f018281548110610a5457610a546123dd565b5f9182526020909120015463ffffffff161115610a7357809250610a81565b610a7e81600161250a565b91505b50610a2a565b8115610ac95784610a99600184612563565b81548110610aa957610aa96123dd565b5f91825260209091200154600160201b90046001600160e01b0316610acb565b5f5b6001600160e01b031695945050505050565b5f835190505f805f80610af185885161139b565b5f5b85811015610b7657888181518110610b0d57610b0d6123dd565b60200260200101519450610b2185886113de565b9250610b2d848661142f565b610b51838b8a8481518110610b4457610b446123dd565b6020026020010151611461565b8493505f610b5f8689611492565b9050610b6b818461250a565b925050600101610af3565b50610b8181876114e3565b505050505050505050565b80545f908015610bd25782610ba2600183612563565b81548110610bb257610bb26123dd565b5f91825260209091200154600160201b90046001600160e01b0316610bd4565b5f5b6001600160e01b03169392505050565b6001600160a01b0383165f908152606e602052604090205460ff1615610c1d576040516342ee68b560e01b815260040160405180910390fd5b60658054905f610c2c83612576565b90915550506001600160a01b0383165f908152606e60205260408120805460ff19166001179055610c5c846111f4565b9050610c6781611317565b5050610c738483610e6f565b606854604051639926ee7d60e01b81526001600160a01b0390911690639926ee7d90610ca590879087906004016125bc565b5f604051808303815f87803b158015610cbc575f80fd5b505af1158015610cce573d5f803e3d5ffd5b50506068546040516001600160a01b03918216935090871691507fa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c1905f90a350505050565b6065548151146103325760405163169efb5b60e11b815260040160405180910390fd5b33610d3f610544565b6001600160a01b0316146105005760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610858565b610da0606c8261153d565b50506040518181527f9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b9060200160405180910390a150565b606780549082905560408051828152602081018490527f713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f91015b60405180910390a15050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b6001600160a01b0382165f908152606a60205260408120610e8f90610b8c565b9050806001600160a01b0316826001600160a01b031603610eaf57505050565b6001600160a01b038381165f908152606a60205260409020610ed291841661153d565b5050816001600160a01b031643846001600160a01b03167fd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea131500284604051610f189190612028565b60405180910390a4505050565b6001600160a01b0381165f908152606e602052604090205460ff16610f5d576040516325ec6c1f60e01b815260040160405180910390fd5b60658054905f610f6c83612606565b90915550506001600160a01b0381165f908152606e60205260408120805460ff19169055610f99826111f4565b9050610fa481611317565b50506068546040516351b27a6d60e11b81526001600160a01b039091169063a364f4da90610fd6908590600401612028565b5f604051808303815f87803b158015610fed575f80fd5b505af1158015610fff573d5f803e3d5ffd5b50506068546040516001600160a01b03918216935090851691507f31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed580905f90a35050565b5f54610100900460ff166110685760405162461bcd60e51b81526004016108589061261b565b606880546001600160a01b0319166001600160a01b03851617905561108c82610d95565b611095816110a2565b61109d61165f565b505050565b6110ab8161168d565b6110c85760405163d173577960e01b815260040160405180910390fd5b6040805160668054602081810284018501855283018181525f9484928491879085015b82821015611139575f84815260209081902060408051808201909152908401546001600160a01b0381168252600160a01b90046001600160601b0316818301528252600190920191016110eb565b505050915250909150606690505f6111518282611b85565b50505f5b8251518110156111c257825180516066919083908110611177576111776123dd565b6020908102919091018101518254600181810185555f94855293839020825192909301516001600160601b0316600160a01b026001600160a01b039092169190911791015501611155565b507f23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e8183604051610e12929190612666565b6001600160a01b0381165f908152606d6020526040812081908190819061121a90610b8c565b6001600160a01b0386165f908152606e602052604090205490915060ff1661127f576112468184612693565b9250825f036112585750909392505050565b6001600160a01b0385165f908152606d602052604081206112789161153d565b50506112ca565b6112888561057d565b91506112948183612693565b9250825f036112a65750909392505050565b6001600160a01b0385165f908152606d602052604090206112c7908361153d565b50505b60408051828152602081018490526001600160a01b038716917f88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe594910160405180910390a250909392505050565b5f80611323606b610b8c565b91505f611330848461253c565b9150819050611340606b8261153d565b505060408051848152602081018490527f86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b910160405180910390a150915091565b5f61138f600284841861251d565b6104bd9084841661250a565b8082146113be576040516001621398b960e31b0319815260040160405180910390fd5b815f036104765760405163251f56a160e21b815260040160405180910390fd5b5f438263ffffffff16106114055760405163e64f180f60e01b815260040160405180910390fd5b6001600160a01b0383165f908152606a602052604090206104bd9063ffffffff808516906109d616565b806001600160a01b0316826001600160a01b0316106104765760405163ba50f91160e01b815260040160405180910390fd5b6114756001600160a01b0384168383611750565b61109d57604051638baa579f60e01b815260040160405180910390fd5b5f438263ffffffff16106114b95760405163e64f180f60e01b815260040160405180910390fd5b6001600160a01b0383165f908152606d602052604090206104bd9063ffffffff808516906109d616565b5f6114ed82611895565b90508083111561151057604051634b05a0f760e11b815260040160405180910390fd5b5f61151a836118d0565b9050838111156108d25760405163e121632f60e01b815260040160405180910390fd5b81545f9081908161154d86610b8c565b90505f8211801561158957504386611566600185612563565b81548110611576576115766123dd565b5f9182526020909120015463ffffffff16145b156115e6576115978561190b565b866115a3600185612563565b815481106115b3576115b36123dd565b905f5260205f20015f0160046101000a8154816001600160e01b0302191690836001600160e01b03160217905550611651565b855f0160405180604001604052806115fd43611977565b63ffffffff1681526020016116118861190b565b6001600160e01b0390811690915282546001810184555f93845260209384902083519490930151909116600160201b0263ffffffff909316929092179101555b9250839150505b9250929050565b5f54610100900460ff166116855760405162461bcd60e51b81526004016108589061261b565b6105006119db565b80515f90818080805b845181101561172f578481815181106116b1576116b16123dd565b60200260200101515f01519250826001600160a01b0316846001600160a01b0316106116f05760405163ba50f91160e01b815260040160405180910390fd5b829350848181518110611705576117056123dd565b6020026020010151602001516001600160601b031682611725919061250a565b9150600101611696565b50612710811461174457505f95945050505050565b50600195945050505050565b5f805f61175d8585611a0a565b90925090505f816004811115611775576117756126b9565b1480156117935750856001600160a01b0316826001600160a01b0316145b156117a3576001925050506104bd565b5f80876001600160a01b0316631626ba7e60e01b88886040516024016117ca9291906126cd565b60408051601f198184030181529181526020820180516001600160e01b03166001600160e01b031990941693909317909252905161180891906126e5565b5f60405180830381855afa9150503d805f8114611840576040519150601f19603f3d011682016040523d82523d5f602084013e611845565b606091505b5091509150818015611858575080516020145b801561188957508051630b135d3f60e11b9061187d90830160209081019084016126fb565b6001600160e01b031916145b98975050505050505050565b5f438263ffffffff16106118bc5760405163e64f180f60e01b815260040160405180910390fd5b610353606b63ffffffff808516906109d616565b5f438263ffffffff16106118f75760405163e64f180f60e01b815260040160405180910390fd5b610353606c63ffffffff808516906109d616565b5f6001600160e01b038211156119735760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20326044820152663234206269747360c81b6064820152608401610858565b5090565b5f63ffffffff8211156119735760405162461bcd60e51b815260206004820152602660248201527f53616665436173743a2076616c756520646f65736e27742066697420696e203360448201526532206269747360d01b6064820152608401610858565b5f54610100900460ff16611a015760405162461bcd60e51b81526004016108589061261b565b61050033610e1e565b5f808251604103611a3e576020830151604084015160608501515f1a611a3287828585611a72565b94509450505050611658565b8251604003611a675760208301516040840151611a5c868383611b4d565b935093505050611658565b505f90506002611658565b5f806fa2a8918ca85bafe22016d0b997e4df60600160ff1b03831115611a9d57505f90506003611b44565b8460ff16601b14158015611ab557508460ff16601c14155b15611ac557505f90506004611b44565b604080515f8082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015611b16573d5f803e3d5ffd5b5050604051601f1901519150506001600160a01b038116611b3e575f60019250925050611b44565b91505f90505b94509492505050565b5f806001600160ff1b03831681611b6960ff86901c601b61250a565b9050611b7787828885611a72565b935093505050935093915050565b5080545f8255905f5260205f209081019061033b91905b80821115611973575f8155600101611b9c565b634e487b7160e01b5f52604160045260245ffd5b604051602081016001600160401b0381118282101715611be557611be5611baf565b60405290565b604080519081016001600160401b0381118282101715611be557611be5611baf565b604051601f8201601f191681016001600160401b0381118282101715611c3557611c35611baf565b604052919050565b5f6001600160401b03821115611c5557611c55611baf565b5060051b60200190565b6001600160a01b038116811461033b575f80fd5b5f82601f830112611c82575f80fd5b8135611c95611c9082611c3d565b611c0d565b8082825260208201915060208360051b860101925085831115611cb6575f80fd5b602085015b83811015611cdc578035611cce81611c5f565b835260209283019201611cbb565b5095945050505050565b5f60208284031215611cf6575f80fd5b81356001600160401b03811115611d0b575f80fd5b611d1784828501611c73565b949350505050565b63ffffffff8116811461033b575f80fd5b5f60208284031215611d40575f80fd5b81356104bd81611d1f565b5f6001600160401b03821115611d6357611d63611baf565b50601f01601f191660200190565b5f82601f830112611d80575f80fd5b8135611d8e611c9082611d4b565b818152846020838601011115611da2575f80fd5b816020850160208301375f918101602001919091529392505050565b5f8060408385031215611dcf575f80fd5b8235915060208301356001600160401b03811115611deb575f80fd5b611df785828601611d71565b9150509250929050565b8051602080845281518482018190525f9290910190829060408601905b80831015611cdc57835180516001600160a01b031683526020908101516001600160601b0316818401529093019260019290920191604090910190611e1e565b602081525f6104bd6020830184611e01565b5f60208284031215611e80575f80fd5b81356104bd81611c5f565b5f8060408385031215611e9c575f80fd5b82356001600160401b03811115611eb1575f80fd5b830160608186031215611ec2575f80fd5b604051606081016001600160401b0381118282101715611ee457611ee4611baf565b60405281356001600160401b03811115611efc575f80fd5b611f0887828501611d71565b82525060208281013581830152604092830135928201929092529250830135611f3081611c5f565b809150509250929050565b5f8060408385031215611f4c575f80fd5b82356001600160401b03811115611f61575f80fd5b8301601f81018513611f71575f80fd5b8035611f7f611c9082611c3d565b8082825260208201915060208360051b850101925087831115611fa0575f80fd5b602084015b83811015611fe05780356001600160401b03811115611fc2575f80fd5b611fd18a602083890101611c73565b84525060209283019201611fa5565b50945050505060208301356001600160401b03811115611deb575f80fd5b5f806040838503121561200f575f80fd5b823561201a81611c5f565b946020939093013593505050565b6001600160a01b0391909116815260200190565b5f6020828403121561204c575f80fd5b5035919050565b5f8060408385031215612064575f80fd5b8235915060208301356001600160401b03811115612080575f80fd5b611df785828601611c73565b5f806040838503121561209d575f80fd5b82356120a881611c5f565b91506020830135611f3081611d1f565b5f602082840312156120c8575f80fd5b6120d0611bc3565b905081356001600160401b038111156120e7575f80fd5b8201601f810184136120f7575f80fd5b8035612105611c9082611c3d565b8082825260208201915060208360061b850101925086831115612126575f80fd5b6020840193505b8284101561218e5760408488031215612144575f80fd5b61214c611beb565b843561215781611c5f565b815260208501356001600160601b0381168114612172575f80fd5b806020830152508083525060208201915060408401935061212d565b84525091949350505050565b5f805f606084860312156121ac575f80fd5b83356121b781611c5f565b92506020840135915060408401356001600160401b038111156121d8575f80fd5b6121e4868287016120b8565b9150509250925092565b5f80604083850312156121ff575f80fd5b82356001600160401b03811115612214575f80fd5b612220858286016120b8565b92505060208301356001600160401b03811115612080575f80fd5b5f82601f83011261224a575f80fd5b8151612258611c9082611c3d565b8082825260208201915060208360051b860101925085831115612279575f80fd5b602085015b83811015611cdc5780516001600160401b0381111561229b575f80fd5b8601603f810188136122ab575f80fd5b60208101516122bc611c9082611d4b565b8181526040838301018a10156122d0575f80fd5b8160408401602083015e5f6020838301015280865250505060208301925060208101905061227e565b805161230481611d1f565b919050565b5f805f6060848603121561231b575f80fd5b83516001600160401b03811115612330575f80fd5b8401601f81018613612340575f80fd5b805161234e611c9082611c3d565b8082825260208201915060208360051b85010192508883111561236f575f80fd5b6020840193505b8284101561239a57835161238981611c5f565b825260209384019390910190612376565b6020880151909650925050506001600160401b038111156123b9575f80fd5b6123c58682870161223b565b9250506123d4604085016122f9565b90509250925092565b634e487b7160e01b5f52603260045260245ffd5b6001600160a01b03831681526040602080830182905283519183018290525f91908401906060840190835b818110156124435783516001600160a01b031683526020938401939092019160010161241c565b50909695505050505050565b5f6020828403121561245f575f80fd5b81516001600160401b03811115612474575f80fd5b8201601f81018413612484575f80fd5b8051612492611c9082611c3d565b8082825260208201915060208360051b8501019250868311156124b3575f80fd5b6020840193505b828410156124d55783518252602093840193909101906124ba565b9695505050505050565b634e487b7160e01b5f52601160045260245ffd5b8082028115828204841417610353576103536124df565b80820180821115610353576103536124df565b5f8261253757634e487b7160e01b5f52601260045260245ffd5b500490565b8082018281125f83128015821682158216171561255b5761255b6124df565b505092915050565b81810381811115610353576103536124df565b5f60018201612587576125876124df565b5060010190565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b60018060a01b0383168152604060208201525f8251606060408401526125e560a084018261258e565b90506020840151606084015260408401516080840152809150509392505050565b5f81612614576126146124df565b505f190190565b6020808252602b908201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960408201526a6e697469616c697a696e6760a81b606082015260800190565b604081525f6126786040830185611e01565b828103602084015261268a8185611e01565b95945050505050565b8181035f8312801583831316838312821617156126b2576126b26124df565b5092915050565b634e487b7160e01b5f52602160045260245ffd5b828152604060208201525f611d17604083018461258e565b5f82518060208501845e5f920191825250919050565b5f6020828403121561270b575f80fd5b81516001600160e01b0319811681146104bd575f80fdfea264697066735822122082e9a76615d2262622df7ac8410170614df2238dc5c61021d1b2d7afa2b2101f64736f6c634300081a0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW_\x80\xFD[P`\x046\x10a\x010W_5`\xE0\x1C\x80b\xCF*\xB5\x14a\x014W\x80c\r\xBA3\x94\x14a\x01IW\x80c\x16&\xBA~\x14a\x01oW\x80c\x17\x03\xA0\x18\x14a\x01\x9BW\x80c\x1EL\xD8^\x14a\x01\xB0W\x80c1O:I\x14a\x01\xC3W\x80c;$.J\x14a\x01\xCBW\x80c=V\x11\xF6\x14a\x01\xDEW\x80c@\xBF/\xB7\x14a\x01\xF1W\x80cQ@\xA5H\x14a\x01\xF9W\x80c^\x10B\xE8\x14a\x02\x0CW\x80c^\xF53)\x14a\x02,W\x80cibU\xBE\x14a\x02?W\x80cqP\x18\xA6\x14a\x02RW\x80ct<1\xF4\x14a\x02ZW\x80c\x85}\xC1\x90\x14a\x02mW\x80c\x8D\xA5\xCB[\x14a\x02uW\x80c\x95_-\x90\x14a\x02}W\x80c\x98\xEC\x1A\xC9\x14a\x02\x90W\x80c\xAB\x11\x89\x95\x14a\x02\xA3W\x80c\xB93\xFAt\x14a\x02\xB6W\x80c\xCD\xCD5\x81\x14a\x02\xBEW\x80c\xDE\xC5\xD1\xF6\x14a\x02\xD1W\x80c\xEC\x7F\xBB1\x14a\x02\xE4W\x80c\xF2\xFD\xE3\x8B\x14a\x03\x1FW[_\x80\xFD[a\x01Ga\x01B6`\x04a\x1C\xE6V[a\x032V[\0[a\x01\\a\x01W6`\x04a\x1D0V[a\x03>V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\x82a\x01}6`\x04a\x1D\xBEV[a\x03YV[`@Q`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16\x81R` \x01a\x01fV[a\x01\xA3a\x03\x95V[`@Qa\x01f\x91\x90a\x1E^V[a\x01\\a\x01\xBE6`\x04a\x1D0V[a\x04&V[a\x01\\a\x04;V[a\x01\\a\x01\xD96`\x04a\x1EpV[a\x04KV[a\x01Ga\x01\xEC6`\x04a\x1E\x8BV[a\x04kV[`gTa\x01\\V[a\x01Ga\x02\x076`\x04a\x1F;V[a\x04zV[a\x02\x1Fa\x02\x1A6`\x04a\x1F\xFEV[a\x04\x9CV[`@Qa\x01f\x91\x90a (V[a\x01Ga\x02:6`\x04a <V[a\x04\xC4V[a\x01Ga\x02M6`\x04a SV[a\x04\xD5V[a\x01Ga\x04\xEFV[a\x01Ga\x02h6`\x04a\x1EpV[a\x05\x02V[a\x01Ga\x05;V[a\x02\x1Fa\x05DV[a\x01\\a\x02\x8B6`\x04a \x8CV[a\x05SV[a\x01\\a\x02\x9E6`\x04a\x1EpV[a\x05}V[a\x01Ga\x02\xB16`\x04a!\x9AV[a\x07\xC2V[a\x01\\a\x08\xD8V[a\x02\x1Fa\x02\xCC6`\x04a\x1EpV[a\x08\xE3V[a\x01Ga\x02\xDF6`\x04a!\xEEV[a\t\x03V[a\x03\x0Fa\x02\xF26`\x04a\x1EpV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`n` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01fV[a\x01Ga\x03-6`\x04a\x1EpV[a\t\x14V[a\x03;\x81a\t\x8AV[PV[_a\x03S`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\t\xD6\x16V[\x92\x91PPV[_\x80_\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03q\x91\x90a#\tV[\x92P\x92P\x92Pa\x03\x83\x86\x84\x84\x84a\n\xDDV[Pc\x0B\x13]?`\xE1\x1B\x95\x94PPPPPV[`@\x80Q` \x81\x01\x90\x91R``\x81R`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R\x92\x93\x91\x92\x84\x92\x90\x91\x84\x91_\x90\x85\x01[\x82\x82\x10\x15a\x04\x19W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x03\xCBV[PPPP\x81RPP\x90P\x90V[_a\x03S`lc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\t\xD6\x16V[_a\x04F`ka\x0B\x8CV[\x90P\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`m` R`@\x81 a\x03S\x90a\x0B\x8CV[a\x04v3\x83\x83a\x0B\xE4V[PPV[a\x04v\x82_\x81Q\x81\x10a\x04\x8FWa\x04\x8Fa#\xDDV[` \x02` \x01\x01Qa\r\x13V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`j` R`@\x81 a\x04\xBD\x90\x83a\t\xD6V[\x93\x92PPPV[a\x04\xCCa\r6V[a\x03;\x81a\r\x95V[a\x04\xDDa\r6V[a\x04\xE6\x82a\r\xD8V[a\x04v\x81a\t\x8AV[a\x04\xF7a\r6V[a\x05\0_a\x0E\x1EV[V[3_\x90\x81R`n` R`@\x90 T`\xFF\x16a\x051W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03;3\x82a\x0EoV[a\x05\x003a\x0F%V[`3T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`m` R`@\x81 a\x04\xBD\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\t\xD6\x16V[_\x80`f_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x05\xF0W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x05\xA2V[PPPP\x90P_\x80\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x06\x11Wa\x06\x11a\x1B\xAFV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x06:W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x06\x97W\x83\x81\x81Q\x81\x10a\x06ZWa\x06Za#\xDDV[` \x02` \x01\x01Q_\x01Q\x82\x82\x81Q\x81\x10a\x06wWa\x06wa#\xDDV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x06?V[P`@Qc\x90\x04\x13G`\xE0\x1B\x81R_\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x90\x04\x13G\x90a\x06\xE8\x90\x89\x90\x86\x90`\x04\x01a#\xF1V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x02W=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07)\x91\x90\x81\x01\x90a$OV[\x90P_[\x84Q\x81\x10\x15a\x07\x95W\x84\x81\x81Q\x81\x10a\x07HWa\x07Ha#\xDDV[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82\x82\x81Q\x81\x10a\x07oWa\x07oa#\xDDV[` \x02` \x01\x01Qa\x07\x81\x91\x90a$\xF3V[a\x07\x8B\x90\x85a%\nV[\x93P`\x01\x01a\x07-V[Pa\x07\xA2a'\x10\x84a%\x1DV[\x92P`gT\x83\x10a\x07\xB7WP\x90\x94\x93PPPPV[P_\x95\x94PPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x07\xE0WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\x07\xF9WP0;\x15\x80\x15a\x07\xF9WP_T`\xFF\x16`\x01\x14[a\x08aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x08\x82W_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x08\x8D\x84\x84\x84a\x10BV[\x80\x15a\x08\xD2W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPV[_a\x04F`la\x0B\x8CV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`j` R`@\x81 a\x03S\x90a\x0B\x8CV[a\t\x0Ba\r6V[a\x04\xE6\x82a\x10\xA2V[a\t\x1Ca\r6V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\t\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x08XV[a\x03;\x81a\x0E\x1EV[_\x80[\x82Q\x81\x10\x15a\t\xCCWa\t\xB8\x83\x82\x81Q\x81\x10a\t\xABWa\t\xABa#\xDDV[` \x02` \x01\x01Qa\x11\xF4V[a\t\xC2\x90\x83a%<V[\x91P`\x01\x01a\t\x8DV[Pa\x08\xD2\x81a\x13\x17V[_C\x82\x10a\n&W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FCheckpoints: block not yet mined`D\x82\x01R`d\x01a\x08XV[\x82T_[\x81\x81\x10\x15a\n\x87W_a\n=\x82\x84a\x13\x81V[\x90P\x84\x86_\x01\x82\x81T\x81\x10a\nTWa\nTa#\xDDV[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\nsW\x80\x92Pa\n\x81V[a\n~\x81`\x01a%\nV[\x91P[Pa\n*V[\x81\x15a\n\xC9W\x84a\n\x99`\x01\x84a%cV[\x81T\x81\x10a\n\xA9Wa\n\xA9a#\xDDV[_\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\n\xCBV[_[`\x01`\x01`\xE0\x1B\x03\x16\x95\x94PPPPPV[_\x83Q\x90P_\x80_\x80a\n\xF1\x85\x88Qa\x13\x9BV[_[\x85\x81\x10\x15a\x0BvW\x88\x81\x81Q\x81\x10a\x0B\rWa\x0B\ra#\xDDV[` \x02` \x01\x01Q\x94Pa\x0B!\x85\x88a\x13\xDEV[\x92Pa\x0B-\x84\x86a\x14/V[a\x0BQ\x83\x8B\x8A\x84\x81Q\x81\x10a\x0BDWa\x0BDa#\xDDV[` \x02` \x01\x01Qa\x14aV[\x84\x93P_a\x0B_\x86\x89a\x14\x92V[\x90Pa\x0Bk\x81\x84a%\nV[\x92PP`\x01\x01a\n\xF3V[Pa\x0B\x81\x81\x87a\x14\xE3V[PPPPPPPPPV[\x80T_\x90\x80\x15a\x0B\xD2W\x82a\x0B\xA2`\x01\x83a%cV[\x81T\x81\x10a\x0B\xB2Wa\x0B\xB2a#\xDDV[_\x91\x82R` \x90\x91 \x01T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x0B\xD4V[_[`\x01`\x01`\xE0\x1B\x03\x16\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`n` R`@\x90 T`\xFF\x16\x15a\x0C\x1DW`@QcB\xEEh\xB5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90_a\x0C,\x83a%vV[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`n` R`@\x81 \x80T`\xFF\x19\x16`\x01\x17\x90Ua\x0C\\\x84a\x11\xF4V[\x90Pa\x0Cg\x81a\x13\x17V[PPa\x0Cs\x84\x83a\x0EoV[`hT`@Qc\x99&\xEE}`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x99&\xEE}\x90a\x0C\xA5\x90\x87\x90\x87\x90`\x04\x01a%\xBCV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\xBCW_\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\xCEW=_\x80>=_\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x87\x16\x91P\x7F\xA4S\xDBa*\xF5\x9EU!\xD6\xAB\x92\x84\xDC>-\x06\xAF(n\xB1\xB1\xB7\xB7q\xFC\xE4ql\x19\xF2\xC1\x90_\x90\xA3PPPPV[`eT\x81Q\x14a\x032W`@Qc\x16\x9E\xFB[`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3a\r?a\x05DV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x08XV[a\r\xA0`l\x82a\x15=V[PP`@Q\x81\x81R\x7F\x93$\xF7\xE5\xA7\xC0(\x88\x08\xA64\xCC\xDED\xB8\xE9ygdt\xB2.)\xEE\x9D\xD5i\xB5^y\x1AK\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`g\x80T\x90\x82\x90U`@\x80Q\x82\x81R` \x81\x01\x84\x90R\x7Fq<\xA5;\x88\xD6\xEBc\xF5\xB1\x85L\xB8\xCB\xDDsn\xC5\x1E\xDA\"^Fy\x1A\xA9)\x8B\x01`d\x8F\x91\x01[`@Q\x80\x91\x03\x90\xA1PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`j` R`@\x81 a\x0E\x8F\x90a\x0B\x8CV[\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x0E\xAFWPPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`j` R`@\x90 a\x0E\xD2\x91\x84\x16a\x15=V[PP\x81`\x01`\x01`\xA0\x1B\x03\x16C\x84`\x01`\x01`\xA0\x1B\x03\x16\x7F\xD0a\x16\x82R\xF4As6X\xF0\x9EM\x8F[-\x99\x8E\xD4\xEF$\xA2\xBB\xFDl\xEC\xA5.\xA11P\x02\x84`@Qa\x0F\x18\x91\x90a (V[`@Q\x80\x91\x03\x90\xA4PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`n` R`@\x90 T`\xFF\x16a\x0F]W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`e\x80T\x90_a\x0Fl\x83a&\x06V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`n` R`@\x81 \x80T`\xFF\x19\x16\x90Ua\x0F\x99\x82a\x11\xF4V[\x90Pa\x0F\xA4\x81a\x13\x17V[PP`hT`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xA3d\xF4\xDA\x90a\x0F\xD6\x90\x85\x90`\x04\x01a (V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\xEDW_\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\xFFW=_\x80>=_\xFD[PP`hT`@Q`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93P\x90\x85\x16\x91P\x7F1\xE0\xAD\xFE\xC7\x1B\xCC\xEE7\xB6\xE8:\x90\xC2\xFE\xDB\x17\xD8\xF1i?\xEE\x86<Gq\xE7\xBF\xE2\xAE\xD5\x80\x90_\x90\xA3PPV[_Ta\x01\0\x90\x04`\xFF\x16a\x10hW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08X\x90a&\x1BV[`h\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x85\x16\x17\x90Ua\x10\x8C\x82a\r\x95V[a\x10\x95\x81a\x10\xA2V[a\x10\x9Da\x16_V[PPPV[a\x10\xAB\x81a\x16\x8DV[a\x10\xC8W`@Qc\xD1sWy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`f\x80T` \x81\x81\x02\x84\x01\x85\x01\x85R\x83\x01\x81\x81R_\x94\x84\x92\x84\x91\x87\x90\x85\x01[\x82\x82\x10\x15a\x119W_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x10\xEBV[PPP\x91RP\x90\x91P`f\x90P_a\x11Q\x82\x82a\x1B\x85V[PP_[\x82QQ\x81\x10\x15a\x11\xC2W\x82Q\x80Q`f\x91\x90\x83\x90\x81\x10a\x11wWa\x11wa#\xDDV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x81\x01\x85U_\x94\x85R\x93\x83\x90 \x82Q\x92\x90\x93\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x91\x01U\x01a\x11UV[P\x7F#\xAA\xD4\xE6\x17D\xEC\xE1d\x13\n\xA4\x15\xC1an\x80\x13k\x0F\x07p\xE5e\x89C\x8B\x90\xB2i&^\x81\x83`@Qa\x0E\x12\x92\x91\x90a&fV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`m` R`@\x81 \x81\x90\x81\x90\x81\x90a\x12\x1A\x90a\x0B\x8CV[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`n` R`@\x90 T\x90\x91P`\xFF\x16a\x12\x7FWa\x12F\x81\x84a&\x93V[\x92P\x82_\x03a\x12XWP\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`m` R`@\x81 a\x12x\x91a\x15=V[PPa\x12\xCAV[a\x12\x88\x85a\x05}V[\x91Pa\x12\x94\x81\x83a&\x93V[\x92P\x82_\x03a\x12\xA6WP\x90\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`m` R`@\x90 a\x12\xC7\x90\x83a\x15=V[PP[`@\x80Q\x82\x81R` \x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x87\x16\x91\x7F\x88w\r\xC8b\xE4z~\xD5\x86\x90xW\xEB\x1Bu\xE4\xC5\xFF\xC8\xB7\x07\xC7\xEE\x10\xEBt\xD6\x88_\xE5\x94\x91\x01`@Q\x80\x91\x03\x90\xA2P\x90\x93\x92PPPV[_\x80a\x13#`ka\x0B\x8CV[\x91P_a\x130\x84\x84a%<V[\x91P\x81\x90Pa\x13@`k\x82a\x15=V[PP`@\x80Q\x84\x81R` \x81\x01\x84\x90R\x7F\x86\xDC\xF8k\x12\xDF\xEE\xDE\xA7J\xE90\r\xBD\xAA\x19;\xCC\xE5\x80\x93i\xC8\x17~\xA2\xF4\xEA\xAAer\x9B\x91\x01`@Q\x80\x91\x03\x90\xA1P\x91P\x91V[_a\x13\x8F`\x02\x84\x84\x18a%\x1DV[a\x04\xBD\x90\x84\x84\x16a%\nV[\x80\x82\x14a\x13\xBEW`@Q`\x01b\x13\x98\xB9`\xE3\x1B\x03\x19\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81_\x03a\x04vW`@Qc%\x1FV\xA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x14\x05W`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`j` R`@\x90 a\x04\xBD\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\t\xD6\x16V[\x80`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x10a\x04vW`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x14u`\x01`\x01`\xA0\x1B\x03\x84\x16\x83\x83a\x17PV[a\x10\x9DW`@Qc\x8B\xAAW\x9F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x14\xB9W`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16_\x90\x81R`m` R`@\x90 a\x04\xBD\x90c\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\t\xD6\x16V[_a\x14\xED\x82a\x18\x95V[\x90P\x80\x83\x11\x15a\x15\x10W`@QcK\x05\xA0\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x15\x1A\x83a\x18\xD0V[\x90P\x83\x81\x11\x15a\x08\xD2W`@Qc\xE1!c/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81T_\x90\x81\x90\x81a\x15M\x86a\x0B\x8CV[\x90P_\x82\x11\x80\x15a\x15\x89WPC\x86a\x15f`\x01\x85a%cV[\x81T\x81\x10a\x15vWa\x15va#\xDDV[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x14[\x15a\x15\xE6Wa\x15\x97\x85a\x19\x0BV[\x86a\x15\xA3`\x01\x85a%cV[\x81T\x81\x10a\x15\xB3Wa\x15\xB3a#\xDDV[\x90_R` _ \x01_\x01`\x04a\x01\0\n\x81T\x81`\x01`\x01`\xE0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xE0\x1B\x03\x16\x02\x17\x90UPa\x16QV[\x85_\x01`@Q\x80`@\x01`@R\x80a\x15\xFDCa\x19wV[c\xFF\xFF\xFF\xFF\x16\x81R` \x01a\x16\x11\x88a\x19\x0BV[`\x01`\x01`\xE0\x1B\x03\x90\x81\x16\x90\x91R\x82T`\x01\x81\x01\x84U_\x93\x84R` \x93\x84\x90 \x83Q\x94\x90\x93\x01Q\x90\x91\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x91\x01U[\x92P\x83\x91PP[\x92P\x92\x90PV[_Ta\x01\0\x90\x04`\xFF\x16a\x16\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08X\x90a&\x1BV[a\x05\0a\x19\xDBV[\x80Q_\x90\x81\x80\x80\x80[\x84Q\x81\x10\x15a\x17/W\x84\x81\x81Q\x81\x10a\x16\xB1Wa\x16\xB1a#\xDDV[` \x02` \x01\x01Q_\x01Q\x92P\x82`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x10a\x16\xF0W`@Qc\xBAP\xF9\x11`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x93P\x84\x81\x81Q\x81\x10a\x17\x05Wa\x17\x05a#\xDDV[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x82a\x17%\x91\x90a%\nV[\x91P`\x01\x01a\x16\x96V[Pa'\x10\x81\x14a\x17DWP_\x95\x94PPPPPV[P`\x01\x95\x94PPPPPV[_\x80_a\x17]\x85\x85a\x1A\nV[\x90\x92P\x90P_\x81`\x04\x81\x11\x15a\x17uWa\x17ua&\xB9V[\x14\x80\x15a\x17\x93WP\x85`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x14[\x15a\x17\xA3W`\x01\x92PPPa\x04\xBDV[_\x80\x87`\x01`\x01`\xA0\x1B\x03\x16c\x16&\xBA~`\xE0\x1B\x88\x88`@Q`$\x01a\x17\xCA\x92\x91\x90a&\xCDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x18\x08\x91\x90a&\xE5V[_`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80_\x81\x14a\x18@W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>a\x18EV[``\x91P[P\x91P\x91P\x81\x80\x15a\x18XWP\x80Q` \x14[\x80\x15a\x18\x89WP\x80Qc\x0B\x13]?`\xE1\x1B\x90a\x18}\x90\x83\x01` \x90\x81\x01\x90\x84\x01a&\xFBV[`\x01`\x01`\xE0\x1B\x03\x19\x16\x14[\x98\x97PPPPPPPPV[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x18\xBCW`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03S`kc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\t\xD6\x16V[_C\x82c\xFF\xFF\xFF\xFF\x16\x10a\x18\xF7W`@Qc\xE6O\x18\x0F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03S`lc\xFF\xFF\xFF\xFF\x80\x85\x16\x90a\t\xD6\x16V[_`\x01`\x01`\xE0\x1B\x03\x82\x11\x15a\x19sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 2`D\x82\x01Rf24 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\x08XV[P\x90V[_c\xFF\xFF\xFF\xFF\x82\x11\x15a\x19sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FSafeCast: value doesn't fit in 3`D\x82\x01Re2 bits`\xD0\x1B`d\x82\x01R`\x84\x01a\x08XV[_Ta\x01\0\x90\x04`\xFF\x16a\x1A\x01W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x08X\x90a&\x1BV[a\x05\x003a\x0E\x1EV[_\x80\x82Q`A\x03a\x1A>W` \x83\x01Q`@\x84\x01Q``\x85\x01Q_\x1Aa\x1A2\x87\x82\x85\x85a\x1ArV[\x94P\x94PPPPa\x16XV[\x82Q`@\x03a\x1AgW` \x83\x01Q`@\x84\x01Qa\x1A\\\x86\x83\x83a\x1BMV[\x93P\x93PPPa\x16XV[P_\x90P`\x02a\x16XV[_\x80o\xA2\xA8\x91\x8C\xA8[\xAF\xE2 \x16\xD0\xB9\x97\xE4\xDF``\x01`\xFF\x1B\x03\x83\x11\x15a\x1A\x9DWP_\x90P`\x03a\x1BDV[\x84`\xFF\x16`\x1B\x14\x15\x80\x15a\x1A\xB5WP\x84`\xFF\x16`\x1C\x14\x15[\x15a\x1A\xC5WP_\x90P`\x04a\x1BDV[`@\x80Q_\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x88\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x86\x90R`\x80\x81\x01\x85\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x1B\x16W=_\x80>=_\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1B>W_`\x01\x92P\x92PPa\x1BDV[\x91P_\x90P[\x94P\x94\x92PPPV[_\x80`\x01`\x01`\xFF\x1B\x03\x83\x16\x81a\x1Bi`\xFF\x86\x90\x1C`\x1Ba%\nV[\x90Pa\x1Bw\x87\x82\x88\x85a\x1ArV[\x93P\x93PPP\x93P\x93\x91PPV[P\x80T_\x82U\x90_R` _ \x90\x81\x01\x90a\x03;\x91\x90[\x80\x82\x11\x15a\x19sW_\x81U`\x01\x01a\x1B\x9CV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q` \x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1B\xE5Wa\x1B\xE5a\x1B\xAFV[`@R\x90V[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1B\xE5Wa\x1B\xE5a\x1B\xAFV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1C5Wa\x1C5a\x1B\xAFV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1CUWa\x1CUa\x1B\xAFV[P`\x05\x1B` \x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03;W_\x80\xFD[_\x82`\x1F\x83\x01\x12a\x1C\x82W_\x80\xFD[\x815a\x1C\x95a\x1C\x90\x82a\x1C=V[a\x1C\rV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\x1C\xB6W_\x80\xFD[` \x85\x01[\x83\x81\x10\x15a\x1C\xDCW\x805a\x1C\xCE\x81a\x1C_V[\x83R` \x92\x83\x01\x92\x01a\x1C\xBBV[P\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15a\x1C\xF6W_\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\x0BW_\x80\xFD[a\x1D\x17\x84\x82\x85\x01a\x1CsV[\x94\x93PPPPV[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x03;W_\x80\xFD[_` \x82\x84\x03\x12\x15a\x1D@W_\x80\xFD[\x815a\x04\xBD\x81a\x1D\x1FV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1DcWa\x1Dca\x1B\xAFV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_\x82`\x1F\x83\x01\x12a\x1D\x80W_\x80\xFD[\x815a\x1D\x8Ea\x1C\x90\x82a\x1DKV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1D\xA2W_\x80\xFD[\x81` \x85\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a\x1D\xCFW_\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xEBW_\x80\xFD[a\x1D\xF7\x85\x82\x86\x01a\x1DqV[\x91PP\x92P\x92\x90PV[\x80Q` \x80\x84R\x81Q\x84\x82\x01\x81\x90R_\x92\x90\x91\x01\x90\x82\x90`@\x86\x01\x90[\x80\x83\x10\x15a\x1C\xDCW\x83Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x90\x81\x01Q`\x01`\x01``\x1B\x03\x16\x81\x84\x01R\x90\x93\x01\x92`\x01\x92\x90\x92\x01\x91`@\x90\x91\x01\x90a\x1E\x1EV[` \x81R_a\x04\xBD` \x83\x01\x84a\x1E\x01V[_` \x82\x84\x03\x12\x15a\x1E\x80W_\x80\xFD[\x815a\x04\xBD\x81a\x1C_V[_\x80`@\x83\x85\x03\x12\x15a\x1E\x9CW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xB1W_\x80\xFD[\x83\x01``\x81\x86\x03\x12\x15a\x1E\xC2W_\x80\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E\xE4Wa\x1E\xE4a\x1B\xAFV[`@R\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1E\xFCW_\x80\xFD[a\x1F\x08\x87\x82\x85\x01a\x1DqV[\x82RP` \x82\x81\x015\x81\x83\x01R`@\x92\x83\x015\x92\x82\x01\x92\x90\x92R\x92P\x83\x015a\x1F0\x81a\x1C_V[\x80\x91PP\x92P\x92\x90PV[_\x80`@\x83\x85\x03\x12\x15a\x1FLW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1FaW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a\x1FqW_\x80\xFD[\x805a\x1F\x7Fa\x1C\x90\x82a\x1C=V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a\x1F\xA0W_\x80\xFD[` \x84\x01[\x83\x81\x10\x15a\x1F\xE0W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xC2W_\x80\xFD[a\x1F\xD1\x8A` \x83\x89\x01\x01a\x1CsV[\x84RP` \x92\x83\x01\x92\x01a\x1F\xA5V[P\x94PPPP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xEBW_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a \x0FW_\x80\xFD[\x825a \x1A\x81a\x1C_V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x81R` \x01\x90V[_` \x82\x84\x03\x12\x15a LW_\x80\xFD[P5\x91\x90PV[_\x80`@\x83\x85\x03\x12\x15a dW_\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \x80W_\x80\xFD[a\x1D\xF7\x85\x82\x86\x01a\x1CsV[_\x80`@\x83\x85\x03\x12\x15a \x9DW_\x80\xFD[\x825a \xA8\x81a\x1C_V[\x91P` \x83\x015a\x1F0\x81a\x1D\x1FV[_` \x82\x84\x03\x12\x15a \xC8W_\x80\xFD[a \xD0a\x1B\xC3V[\x90P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a \xE7W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a \xF7W_\x80\xFD[\x805a!\x05a\x1C\x90\x82a\x1C=V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a!&W_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a!\x8EW`@\x84\x88\x03\x12\x15a!DW_\x80\xFD[a!La\x1B\xEBV[\x845a!W\x81a\x1C_V[\x81R` \x85\x015`\x01`\x01``\x1B\x03\x81\x16\x81\x14a!rW_\x80\xFD[\x80` \x83\x01RP\x80\x83RP` \x82\x01\x91P`@\x84\x01\x93Pa!-V[\x84RP\x91\x94\x93PPPPV[_\x80_``\x84\x86\x03\x12\x15a!\xACW_\x80\xFD[\x835a!\xB7\x81a\x1C_V[\x92P` \x84\x015\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a!\xD8W_\x80\xFD[a!\xE4\x86\x82\x87\x01a \xB8V[\x91PP\x92P\x92P\x92V[_\x80`@\x83\x85\x03\x12\x15a!\xFFW_\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x14W_\x80\xFD[a\" \x85\x82\x86\x01a \xB8V[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a \x80W_\x80\xFD[_\x82`\x1F\x83\x01\x12a\"JW_\x80\xFD[\x81Qa\"Xa\x1C\x90\x82a\x1C=V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a\"yW_\x80\xFD[` \x85\x01[\x83\x81\x10\x15a\x1C\xDCW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\x9BW_\x80\xFD[\x86\x01`?\x81\x01\x88\x13a\"\xABW_\x80\xFD[` \x81\x01Qa\"\xBCa\x1C\x90\x82a\x1DKV[\x81\x81R`@\x83\x83\x01\x01\x8A\x10\x15a\"\xD0W_\x80\xFD[\x81`@\x84\x01` \x83\x01^_` \x83\x83\x01\x01R\x80\x86RPPP` \x83\x01\x92P` \x81\x01\x90Pa\"~V[\x80Qa#\x04\x81a\x1D\x1FV[\x91\x90PV[_\x80_``\x84\x86\x03\x12\x15a#\x1BW_\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a#0W_\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a#@W_\x80\xFD[\x80Qa#Na\x1C\x90\x82a\x1C=V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a#oW_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a#\x9AW\x83Qa#\x89\x81a\x1C_V[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a#vV[` \x88\x01Q\x90\x96P\x92PPP`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xB9W_\x80\xFD[a#\xC5\x86\x82\x87\x01a\";V[\x92PPa#\xD4`@\x85\x01a\"\xF9V[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x80\x83\x01\x82\x90R\x83Q\x91\x83\x01\x82\x90R_\x91\x90\x84\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a$CW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a$\x1CV[P\x90\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15a$_W_\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a$tW_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a$\x84W_\x80\xFD[\x80Qa$\x92a\x1C\x90\x82a\x1C=V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a$\xB3W_\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a$\xD5W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a$\xBAV[\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03SWa\x03Sa$\xDFV[\x80\x82\x01\x80\x82\x11\x15a\x03SWa\x03Sa$\xDFV[_\x82a%7WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a%[Wa%[a$\xDFV[PP\x92\x91PPV[\x81\x81\x03\x81\x81\x11\x15a\x03SWa\x03Sa$\xDFV[_`\x01\x82\x01a%\x87Wa%\x87a$\xDFV[P`\x01\x01\x90V[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_\x82Q```@\x84\x01Ra%\xE5`\xA0\x84\x01\x82a%\x8EV[\x90P` \x84\x01Q``\x84\x01R`@\x84\x01Q`\x80\x84\x01R\x80\x91PP\x93\x92PPPV[_\x81a&\x14Wa&\x14a$\xDFV[P_\x19\x01\x90V[` \x80\x82R`+\x90\x82\x01R\x7FInitializable: contract is not i`@\x82\x01Rjnitializing`\xA8\x1B``\x82\x01R`\x80\x01\x90V[`@\x81R_a&x`@\x83\x01\x85a\x1E\x01V[\x82\x81\x03` \x84\x01Ra&\x8A\x81\x85a\x1E\x01V[\x95\x94PPPPPV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&\xB2Wa&\xB2a$\xDFV[P\x92\x91PPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x82\x81R`@` \x82\x01R_a\x1D\x17`@\x83\x01\x84a%\x8EV[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a'\x0BW_\x80\xFD[\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x14a\x04\xBDW_\x80\xFD\xFE\xA2dipfsX\"\x12 \x82\xE9\xA7f\x15\xD2&&\"\xDFz\xC8A\x01paM\xF2#\x8D\xC5\xC6\x10!\xD1\xB2\xD7\xAF\xA2\xB2\x10\x1FdsolcC\0\x08\x1A\x003",
    );
    /**```solidity
struct Quorum { StrategyParams[] strategies; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Quorum {
        pub strategies: alloy::sol_types::private::Vec<
            <StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
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
            alloy::sol_types::sol_data::Array<StrategyParams>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <StrategyParams as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<Quorum> for UnderlyingRustTuple<'_> {
            fn from(value: Quorum) -> Self {
                (value.strategies,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Quorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { strategies: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Quorum {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Quorum {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
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
        impl alloy_sol_types::SolType for Quorum {
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
        impl alloy_sol_types::SolStruct for Quorum {
            const NAME: &'static str = "Quorum";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Quorum(StrategyParams[] strategies)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <StrategyParams as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <StrategyParams as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                <alloy::sol_types::sol_data::Array<
                    StrategyParams,
                > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                    .0
                    .to_vec()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Quorum {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyParams,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
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
                    StrategyParams,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
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
struct StrategyParams { address strategy; uint96 multiplier; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyParams {
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
        impl ::core::convert::From<StrategyParams> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyParams) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyParams {
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
        impl alloy_sol_types::SolType for StrategyParams {
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
        impl alloy_sol_types::SolStruct for StrategyParams {
            const NAME: &'static str = "StrategyParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyParams(address strategy,uint96 multiplier)",
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
        impl alloy_sol_types::EventTopic for StrategyParams {
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
    /**Custom error with signature `InsufficientSignedStake()` and selector `0xe121632f`.
```solidity
error InsufficientSignedStake();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientSignedStake {}
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
        impl ::core::convert::From<InsufficientSignedStake> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientSignedStake) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientSignedStake {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientSignedStake {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientSignedStake()";
            const SELECTOR: [u8; 4] = [225u8, 33u8, 99u8, 47u8];
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
    /**Custom error with signature `InsufficientWeight()` and selector `0xa8792fd1`.
```solidity
error InsufficientWeight();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientWeight {}
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
        impl ::core::convert::From<InsufficientWeight> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientWeight) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientWeight {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientWeight {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientWeight()";
            const SELECTOR: [u8; 4] = [168u8, 121u8, 47u8, 209u8];
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
    /**Custom error with signature `InvalidLength()` and selector `0x947d5a84`.
```solidity
error InvalidLength();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidLength {}
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
        impl ::core::convert::From<InvalidLength> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidLength()";
            const SELECTOR: [u8; 4] = [148u8, 125u8, 90u8, 132u8];
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
    /**Custom error with signature `InvalidQuorum()` and selector `0xd1735779`.
```solidity
error InvalidQuorum();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidQuorum {}
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
        impl ::core::convert::From<InvalidQuorum> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidQuorum) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidQuorum {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidQuorum {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidQuorum()";
            const SELECTOR: [u8; 4] = [209u8, 115u8, 87u8, 121u8];
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
    /**Custom error with signature `InvalidReferenceBlock()` and selector `0xe64f180f`.
```solidity
error InvalidReferenceBlock();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidReferenceBlock {}
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
        impl ::core::convert::From<InvalidReferenceBlock> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidReferenceBlock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidReferenceBlock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidReferenceBlock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidReferenceBlock()";
            const SELECTOR: [u8; 4] = [230u8, 79u8, 24u8, 15u8];
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
    /**Custom error with signature `InvalidSignedWeight()` and selector `0x960b41ee`.
```solidity
error InvalidSignedWeight();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignedWeight {}
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
        impl ::core::convert::From<InvalidSignedWeight> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignedWeight) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignedWeight {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignedWeight {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignedWeight()";
            const SELECTOR: [u8; 4] = [150u8, 11u8, 65u8, 238u8];
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
    /**Custom error with signature `InvalidThreshold()` and selector `0xaabd5a09`.
```solidity
error InvalidThreshold();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidThreshold {}
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
        impl ::core::convert::From<InvalidThreshold> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidThreshold) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidThreshold {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidThreshold {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidThreshold()";
            const SELECTOR: [u8; 4] = [170u8, 189u8, 90u8, 9u8];
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
    /**Custom error with signature `LengthMismatch()` and selector `0xff633a38`.
```solidity
error LengthMismatch();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct LengthMismatch {}
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
        impl ::core::convert::From<LengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: LengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for LengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for LengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "LengthMismatch()";
            const SELECTOR: [u8; 4] = [255u8, 99u8, 58u8, 56u8];
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
    /**Custom error with signature `MustUpdateAllOperators()` and selector `0x2d3df6b6`.
```solidity
error MustUpdateAllOperators();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MustUpdateAllOperators {}
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
        impl ::core::convert::From<MustUpdateAllOperators> for UnderlyingRustTuple<'_> {
            fn from(value: MustUpdateAllOperators) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MustUpdateAllOperators {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MustUpdateAllOperators {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MustUpdateAllOperators()";
            const SELECTOR: [u8; 4] = [45u8, 61u8, 246u8, 182u8];
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
    /**Custom error with signature `NotSorted()` and selector `0xba50f911`.
```solidity
error NotSorted();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotSorted {}
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
        impl ::core::convert::From<NotSorted> for UnderlyingRustTuple<'_> {
            fn from(value: NotSorted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotSorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotSorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotSorted()";
            const SELECTOR: [u8; 4] = [186u8, 80u8, 249u8, 17u8];
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
    /**Custom error with signature `OperatorAlreadyRegistered()` and selector `0x42ee68b5`.
```solidity
error OperatorAlreadyRegistered();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorAlreadyRegistered {}
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
        impl ::core::convert::From<OperatorAlreadyRegistered>
        for UnderlyingRustTuple<'_> {
            fn from(value: OperatorAlreadyRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OperatorAlreadyRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorAlreadyRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorAlreadyRegistered()";
            const SELECTOR: [u8; 4] = [66u8, 238u8, 104u8, 181u8];
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
    /**Custom error with signature `OperatorNotRegistered()` and selector `0x25ec6c1f`.
```solidity
error OperatorNotRegistered();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotRegistered {}
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
        impl ::core::convert::From<OperatorNotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorNotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotRegistered()";
            const SELECTOR: [u8; 4] = [37u8, 236u8, 108u8, 31u8];
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
    /**Event with signature `MinimumWeightUpdated(uint256,uint256)` and selector `0x713ca53b88d6eb63f5b1854cb8cbdd736ec51eda225e46791aa9298b0160648f`.
```solidity
event MinimumWeightUpdated(uint256 _old, uint256 _new);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MinimumWeightUpdated {
        #[allow(missing_docs)]
        pub _old: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _new: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for MinimumWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MinimumWeightUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                113u8,
                60u8,
                165u8,
                59u8,
                136u8,
                214u8,
                235u8,
                99u8,
                245u8,
                177u8,
                133u8,
                76u8,
                184u8,
                203u8,
                221u8,
                115u8,
                110u8,
                197u8,
                30u8,
                218u8,
                34u8,
                94u8,
                70u8,
                121u8,
                26u8,
                169u8,
                41u8,
                139u8,
                1u8,
                96u8,
                100u8,
                143u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _old: data.0, _new: data.1 }
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._old),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._new),
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
        impl alloy_sol_types::private::IntoLogData for MinimumWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MinimumWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MinimumWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorDeregistered(address,address)` and selector `0x31e0adfec71bccee37b6e83a90c2fedb17d8f1693fee863c4771e7bfe2aed580`.
```solidity
event OperatorDeregistered(address indexed _operator, address indexed _avs);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorDeregistered {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _avs: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OperatorDeregistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorDeregistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                49u8,
                224u8,
                173u8,
                254u8,
                199u8,
                27u8,
                204u8,
                238u8,
                55u8,
                182u8,
                232u8,
                58u8,
                144u8,
                194u8,
                254u8,
                219u8,
                23u8,
                216u8,
                241u8,
                105u8,
                63u8,
                238u8,
                134u8,
                60u8,
                71u8,
                113u8,
                231u8,
                191u8,
                226u8,
                174u8,
                213u8,
                128u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _operator: topics.1,
                    _avs: topics.2,
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
                (Self::SIGNATURE_HASH.into(), self._operator.clone(), self._avs.clone())
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
                    &self._operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorDeregistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorDeregistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorDeregistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRegistered(address,address)` and selector `0xa453db612af59e5521d6ab9284dc3e2d06af286eb1b1b7b771fce4716c19f2c1`.
```solidity
event OperatorRegistered(address indexed _operator, address indexed _avs);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorRegistered {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _avs: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for OperatorRegistered {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRegistered(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                164u8,
                83u8,
                219u8,
                97u8,
                42u8,
                245u8,
                158u8,
                85u8,
                33u8,
                214u8,
                171u8,
                146u8,
                132u8,
                220u8,
                62u8,
                45u8,
                6u8,
                175u8,
                40u8,
                110u8,
                177u8,
                177u8,
                183u8,
                183u8,
                113u8,
                252u8,
                228u8,
                113u8,
                108u8,
                25u8,
                242u8,
                193u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _operator: topics.1,
                    _avs: topics.2,
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
                (Self::SIGNATURE_HASH.into(), self._operator.clone(), self._avs.clone())
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
                    &self._operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self._avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorRegistered {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorRegistered> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorRegistered) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorWeightUpdated(address,uint256,uint256)` and selector `0x88770dc862e47a7ed586907857eb1b75e4c5ffc8b707c7ee10eb74d6885fe594`.
```solidity
event OperatorWeightUpdated(address indexed _operator, uint256 oldWeight, uint256 newWeight);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorWeightUpdated {
        #[allow(missing_docs)]
        pub _operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newWeight: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for OperatorWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorWeightUpdated(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                136u8,
                119u8,
                13u8,
                200u8,
                98u8,
                228u8,
                122u8,
                126u8,
                213u8,
                134u8,
                144u8,
                120u8,
                87u8,
                235u8,
                27u8,
                117u8,
                228u8,
                197u8,
                255u8,
                200u8,
                183u8,
                7u8,
                199u8,
                238u8,
                16u8,
                235u8,
                116u8,
                214u8,
                136u8,
                95u8,
                229u8,
                148u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    _operator: topics.1,
                    oldWeight: data.0,
                    newWeight: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldWeight),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newWeight),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self._operator.clone())
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
                    &self._operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorWeightUpdated) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `QuorumUpdated(((address,uint96)[]),((address,uint96)[]))` and selector `0x23aad4e61744ece164130aa415c1616e80136b0f0770e56589438b90b269265e`.
```solidity
event QuorumUpdated(Quorum _old, Quorum _new);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumUpdated {
        #[allow(missing_docs)]
        pub _old: <Quorum as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _new: <Quorum as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for QuorumUpdated {
            type DataTuple<'a> = (Quorum, Quorum);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "QuorumUpdated(((address,uint96)[]),((address,uint96)[]))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8,
                170u8,
                212u8,
                230u8,
                23u8,
                68u8,
                236u8,
                225u8,
                100u8,
                19u8,
                10u8,
                164u8,
                21u8,
                193u8,
                97u8,
                110u8,
                128u8,
                19u8,
                107u8,
                15u8,
                7u8,
                112u8,
                229u8,
                101u8,
                137u8,
                67u8,
                139u8,
                144u8,
                178u8,
                105u8,
                38u8,
                94u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _old: data.0, _new: data.1 }
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
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._old),
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._new),
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
        impl alloy_sol_types::private::IntoLogData for QuorumUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `SigningKeyUpdate(address,uint256,address,address)` and selector `0xd061168252f441733658f09e4d8f5b2d998ed4ef24a2bbfd6ceca52ea1315002`.
```solidity
event SigningKeyUpdate(address indexed operator, uint256 indexed updateBlock, address indexed newSigningKey, address oldSigningKey);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct SigningKeyUpdate {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub updateBlock: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newSigningKey: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldSigningKey: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for SigningKeyUpdate {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "SigningKeyUpdate(address,uint256,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                208u8,
                97u8,
                22u8,
                130u8,
                82u8,
                244u8,
                65u8,
                115u8,
                54u8,
                88u8,
                240u8,
                158u8,
                77u8,
                143u8,
                91u8,
                45u8,
                153u8,
                142u8,
                212u8,
                239u8,
                36u8,
                162u8,
                187u8,
                253u8,
                108u8,
                236u8,
                165u8,
                46u8,
                161u8,
                49u8,
                80u8,
                2u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operator: topics.1,
                    updateBlock: topics.2,
                    newSigningKey: topics.3,
                    oldSigningKey: data.0,
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
                        &self.oldSigningKey,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.operator.clone(),
                    self.updateBlock.clone(),
                    self.newSigningKey.clone(),
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
                    &self.operator,
                );
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.updateBlock);
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newSigningKey,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for SigningKeyUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&SigningKeyUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &SigningKeyUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ThresholdWeightUpdated(uint256)` and selector `0x9324f7e5a7c0288808a634ccde44b8e979676474b22e29ee9dd569b55e791a4b`.
```solidity
event ThresholdWeightUpdated(uint256 _thresholdWeight);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ThresholdWeightUpdated {
        #[allow(missing_docs)]
        pub _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for ThresholdWeightUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ThresholdWeightUpdated(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                147u8,
                36u8,
                247u8,
                229u8,
                167u8,
                192u8,
                40u8,
                136u8,
                8u8,
                166u8,
                52u8,
                204u8,
                222u8,
                68u8,
                184u8,
                233u8,
                121u8,
                103u8,
                100u8,
                116u8,
                178u8,
                46u8,
                41u8,
                238u8,
                157u8,
                213u8,
                105u8,
                181u8,
                94u8,
                121u8,
                26u8,
                75u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _thresholdWeight: data.0 }
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._thresholdWeight),
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
        impl alloy_sol_types::private::IntoLogData for ThresholdWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ThresholdWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ThresholdWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TotalWeightUpdated(uint256,uint256)` and selector `0x86dcf86b12dfeedea74ae9300dbdaa193bcce5809369c8177ea2f4eaaa65729b`.
```solidity
event TotalWeightUpdated(uint256 oldTotalWeight, uint256 newTotalWeight);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TotalWeightUpdated {
        #[allow(missing_docs)]
        pub oldTotalWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newTotalWeight: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for TotalWeightUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TotalWeightUpdated(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                134u8,
                220u8,
                248u8,
                107u8,
                18u8,
                223u8,
                238u8,
                222u8,
                167u8,
                74u8,
                233u8,
                48u8,
                13u8,
                189u8,
                170u8,
                25u8,
                59u8,
                204u8,
                229u8,
                128u8,
                147u8,
                105u8,
                200u8,
                23u8,
                126u8,
                162u8,
                244u8,
                234u8,
                170u8,
                101u8,
                114u8,
                155u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldTotalWeight: data.0,
                    newTotalWeight: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldTotalWeight),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newTotalWeight),
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
        impl alloy_sol_types::private::IntoLogData for TotalWeightUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TotalWeightUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TotalWeightUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `UpdateMinimumWeight(uint256,uint256)` and selector `0x1ea42186b305fa37310450d9fb87ea1e8f0c7f447e771479e3b27634bfe84dc1`.
```solidity
event UpdateMinimumWeight(uint256 oldMinimumWeight, uint256 newMinimumWeight);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct UpdateMinimumWeight {
        #[allow(missing_docs)]
        pub oldMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for UpdateMinimumWeight {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "UpdateMinimumWeight(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                30u8,
                164u8,
                33u8,
                134u8,
                179u8,
                5u8,
                250u8,
                55u8,
                49u8,
                4u8,
                80u8,
                217u8,
                251u8,
                135u8,
                234u8,
                30u8,
                143u8,
                12u8,
                127u8,
                68u8,
                126u8,
                119u8,
                20u8,
                121u8,
                227u8,
                178u8,
                118u8,
                52u8,
                191u8,
                232u8,
                77u8,
                193u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldMinimumWeight: data.0,
                    newMinimumWeight: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldMinimumWeight),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMinimumWeight),
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
        impl alloy_sol_types::private::IntoLogData for UpdateMinimumWeight {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&UpdateMinimumWeight> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &UpdateMinimumWeight) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _delegationManager);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _delegationManager: alloy::sol_types::private::Address,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._delegationManager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _delegationManager: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
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
                        &self._delegationManager,
                    ),
                )
            }
        }
    };
    /**Function with signature `deregisterOperator()` and selector `0x857dc190`.
```solidity
function deregisterOperator() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorCall {}
    ///Container type for the return parameters of the [`deregisterOperator()`](deregisterOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorReturn {}
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
            impl ::core::convert::From<deregisterOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorCall {
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
            impl ::core::convert::From<deregisterOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperator()";
            const SELECTOR: [u8; 4] = [133u8, 125u8, 193u8, 144u8];
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
    /**Function with signature `getLastCheckpointOperatorWeight(address)` and selector `0x3b242e4a`.
```solidity
function getLastCheckpointOperatorWeight(address _operator) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointOperatorWeightCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getLastCheckpointOperatorWeight(address)`](getLastCheckpointOperatorWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointOperatorWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getLastCheckpointOperatorWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointOperatorWeightCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointOperatorWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getLastCheckpointOperatorWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointOperatorWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointOperatorWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointOperatorWeightCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointOperatorWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointOperatorWeight(address)";
            const SELECTOR: [u8; 4] = [59u8, 36u8, 46u8, 74u8];
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
    /**Function with signature `getLastCheckpointThresholdWeight()` and selector `0xb933fa74`.
```solidity
function getLastCheckpointThresholdWeight() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightCall {}
    ///Container type for the return parameters of the [`getLastCheckpointThresholdWeight()`](getLastCheckpointThresholdWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getLastCheckpointThresholdWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointThresholdWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getLastCheckpointThresholdWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointThresholdWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointThresholdWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointThresholdWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointThresholdWeight()";
            const SELECTOR: [u8; 4] = [185u8, 51u8, 250u8, 116u8];
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
    /**Function with signature `getLastCheckpointThresholdWeightAtBlock(uint32)` and selector `0x1e4cd85e`.
```solidity
function getLastCheckpointThresholdWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightAtBlockCall {
        pub _blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getLastCheckpointThresholdWeightAtBlock(uint32)`](getLastCheckpointThresholdWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointThresholdWeightAtBlockReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<getLastCheckpointThresholdWeightAtBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightAtBlockCall) -> Self {
                    (value._blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointThresholdWeightAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _blockNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getLastCheckpointThresholdWeightAtBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointThresholdWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointThresholdWeightAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointThresholdWeightAtBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointThresholdWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointThresholdWeightAtBlock(uint32)";
            const SELECTOR: [u8; 4] = [30u8, 76u8, 216u8, 94u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._blockNumber),
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
    /**Function with signature `getLastCheckpointTotalWeight()` and selector `0x314f3a49`.
```solidity
function getLastCheckpointTotalWeight() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightCall {}
    ///Container type for the return parameters of the [`getLastCheckpointTotalWeight()`](getLastCheckpointTotalWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getLastCheckpointTotalWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointTotalWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getLastCheckpointTotalWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointTotalWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointTotalWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointTotalWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointTotalWeight()";
            const SELECTOR: [u8; 4] = [49u8, 79u8, 58u8, 73u8];
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
    /**Function with signature `getLastCheckpointTotalWeightAtBlock(uint32)` and selector `0x0dba3394`.
```solidity
function getLastCheckpointTotalWeightAtBlock(uint32 _blockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightAtBlockCall {
        pub _blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getLastCheckpointTotalWeightAtBlock(uint32)`](getLastCheckpointTotalWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastCheckpointTotalWeightAtBlockReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32,);
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
            impl ::core::convert::From<getLastCheckpointTotalWeightAtBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightAtBlockCall) -> Self {
                    (value._blockNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointTotalWeightAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _blockNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getLastCheckpointTotalWeightAtBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastCheckpointTotalWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastCheckpointTotalWeightAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastCheckpointTotalWeightAtBlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastCheckpointTotalWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastCheckpointTotalWeightAtBlock(uint32)";
            const SELECTOR: [u8; 4] = [13u8, 186u8, 51u8, 148u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._blockNumber),
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
    /**Function with signature `getLastestOperatorSigningKey(address)` and selector `0xcdcd3581`.
```solidity
function getLastestOperatorSigningKey(address _operator) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastestOperatorSigningKeyCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getLastestOperatorSigningKey(address)`](getLastestOperatorSigningKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLastestOperatorSigningKeyReturn {
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
            impl ::core::convert::From<getLastestOperatorSigningKeyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastestOperatorSigningKeyCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastestOperatorSigningKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
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
            impl ::core::convert::From<getLastestOperatorSigningKeyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLastestOperatorSigningKeyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLastestOperatorSigningKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLastestOperatorSigningKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLastestOperatorSigningKeyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLastestOperatorSigningKey(address)";
            const SELECTOR: [u8; 4] = [205u8, 205u8, 53u8, 129u8];
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
    /**Function with signature `getOperatorSigningKeyAtBlock(address,uint256)` and selector `0x5e1042e8`.
```solidity
function getOperatorSigningKeyAtBlock(address _operator, uint256 _blockNumber) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSigningKeyAtBlockCall {
        pub _operator: alloy::sol_types::private::Address,
        pub _blockNumber: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getOperatorSigningKeyAtBlock(address,uint256)`](getOperatorSigningKeyAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSigningKeyAtBlockReturn {
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
            impl ::core::convert::From<getOperatorSigningKeyAtBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSigningKeyAtBlockCall) -> Self {
                    (value._operator, value._blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSigningKeyAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _operator: tuple.0,
                        _blockNumber: tuple.1,
                    }
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
            impl ::core::convert::From<getOperatorSigningKeyAtBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSigningKeyAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSigningKeyAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSigningKeyAtBlockCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSigningKeyAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSigningKeyAtBlock(address,uint256)";
            const SELECTOR: [u8; 4] = [94u8, 16u8, 66u8, 232u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._blockNumber),
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
    /**Function with signature `getOperatorWeight(address)` and selector `0x98ec1ac9`.
```solidity
function getOperatorWeight(address _operator) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorWeight(address)`](getOperatorWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getOperatorWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getOperatorWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorWeightCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorWeight(address)";
            const SELECTOR: [u8; 4] = [152u8, 236u8, 26u8, 201u8];
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
    /**Function with signature `getOperatorWeightAtBlock(address,uint32)` and selector `0x955f2d90`.
```solidity
function getOperatorWeightAtBlock(address _operator, uint32 _blockNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightAtBlockCall {
        pub _operator: alloy::sol_types::private::Address,
        pub _blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getOperatorWeightAtBlock(address,uint32)`](getOperatorWeightAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorWeightAtBlockReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
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
            impl ::core::convert::From<getOperatorWeightAtBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightAtBlockCall) -> Self {
                    (value._operator, value._blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorWeightAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _operator: tuple.0,
                        _blockNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getOperatorWeightAtBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorWeightAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorWeightAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorWeightAtBlockCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorWeightAtBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorWeightAtBlock(address,uint32)";
            const SELECTOR: [u8; 4] = [149u8, 95u8, 45u8, 144u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._blockNumber),
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
    /**Function with signature `initialize(address,uint256,((address,uint96)[]))` and selector `0xab118995`.
```solidity
function initialize(address _serviceManager, uint256 _thresholdWeight, Quorum memory _quorum) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _serviceManager: alloy::sol_types::private::Address,
        pub _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
        pub _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`initialize(address,uint256,((address,uint96)[]))`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                Quorum,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                <Quorum as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._serviceManager, value._thresholdWeight, value._quorum)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _serviceManager: tuple.0,
                        _thresholdWeight: tuple.1,
                        _quorum: tuple.2,
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
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                Quorum,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,uint256,((address,uint96)[]))";
            const SELECTOR: [u8; 4] = [171u8, 17u8, 137u8, 149u8];
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
                        &self._serviceManager,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._thresholdWeight),
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._quorum),
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
    /**Function with signature `isValidSignature(bytes32,bytes)` and selector `0x1626ba7e`.
```solidity
function isValidSignature(bytes32 _dataHash, bytes memory _signatureData) external view returns (bytes4);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureCall {
        pub _dataHash: alloy::sol_types::private::FixedBytes<32>,
        pub _signatureData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`isValidSignature(bytes32,bytes)`](isValidSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isValidSignatureReturn {
        pub _0: alloy::sol_types::private::FixedBytes<4>,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<isValidSignatureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureCall) -> Self {
                    (value._dataHash, value._signatureData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _dataHash: tuple.0,
                        _signatureData: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
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
            impl ::core::convert::From<isValidSignatureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isValidSignatureReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isValidSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isValidSignatureCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isValidSignatureReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isValidSignature(bytes32,bytes)";
            const SELECTOR: [u8; 4] = [22u8, 38u8, 186u8, 126u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._dataHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._signatureData,
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
    /**Function with signature `minimumWeight()` and selector `0x40bf2fb7`.
```solidity
function minimumWeight() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumWeightCall {}
    ///Container type for the return parameters of the [`minimumWeight()`](minimumWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumWeightReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<minimumWeightCall> for UnderlyingRustTuple<'_> {
                fn from(value: minimumWeightCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<minimumWeightReturn> for UnderlyingRustTuple<'_> {
                fn from(value: minimumWeightReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minimumWeightCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = minimumWeightReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minimumWeight()";
            const SELECTOR: [u8; 4] = [64u8, 191u8, 47u8, 183u8];
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
    /**Function with signature `operatorRegistered(address)` and selector `0xec7fbb31`.
```solidity
function operatorRegistered(address _operator) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorRegisteredCall {
        pub _operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorRegistered(address)`](operatorRegisteredCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorRegisteredReturn {
        pub _0: bool,
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
            impl ::core::convert::From<operatorRegisteredCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorRegisteredCall) -> Self {
                    (value._operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorRegisteredCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
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
            impl ::core::convert::From<operatorRegisteredReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorRegisteredReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorRegisteredReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorRegisteredCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorRegisteredReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorRegistered(address)";
            const SELECTOR: [u8; 4] = [236u8, 127u8, 187u8, 49u8];
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
    /**Function with signature `quorum()` and selector `0x1703a018`.
```solidity
function quorum() external view returns (Quorum memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumCall {}
    ///Container type for the return parameters of the [`quorum()`](quorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct quorumReturn {
        pub _0: <Quorum as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<quorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: quorumCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Quorum,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Quorum as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<quorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: quorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for quorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for quorumCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = quorumReturn;
            type ReturnTuple<'a> = (Quorum,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "quorum()";
            const SELECTOR: [u8; 4] = [23u8, 3u8, 160u8, 24u8];
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
    /**Function with signature `registerOperatorWithSignature((bytes,bytes32,uint256),address)` and selector `0x3d5611f6`.
```solidity
function registerOperatorWithSignature(ISignatureUtils.SignatureWithSaltAndExpiry memory _operatorSignature, address _signingKey) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorWithSignatureCall {
        pub _operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        pub _signingKey: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`registerOperatorWithSignature((bytes,bytes32,uint256),address)`](registerOperatorWithSignatureCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorWithSignatureReturn {}
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
                ISignatureUtils::SignatureWithSaltAndExpiry,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerOperatorWithSignatureCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithSignatureCall) -> Self {
                    (value._operatorSignature, value._signingKey)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorWithSignatureCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _operatorSignature: tuple.0,
                        _signingKey: tuple.1,
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
            impl ::core::convert::From<registerOperatorWithSignatureReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorWithSignatureReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorWithSignatureReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorWithSignatureCall {
            type Parameters<'a> = (
                ISignatureUtils::SignatureWithSaltAndExpiry,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorWithSignatureReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperatorWithSignature((bytes,bytes32,uint256),address)";
            const SELECTOR: [u8; 4] = [61u8, 86u8, 17u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <ISignatureUtils::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self._operatorSignature,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._signingKey,
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
    /**Function with signature `updateMinimumWeight(uint256,address[])` and selector `0x696255be`.
```solidity
function updateMinimumWeight(uint256 _newMinimumWeight, address[] memory _operators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateMinimumWeightCall {
        pub _newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
        pub _operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`updateMinimumWeight(uint256,address[])`](updateMinimumWeightCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateMinimumWeightReturn {}
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<updateMinimumWeightCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateMinimumWeightCall) -> Self {
                    (value._newMinimumWeight, value._operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateMinimumWeightCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _newMinimumWeight: tuple.0,
                        _operators: tuple.1,
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
            impl ::core::convert::From<updateMinimumWeightReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateMinimumWeightReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateMinimumWeightReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateMinimumWeightCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateMinimumWeightReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateMinimumWeight(uint256,address[])";
            const SELECTOR: [u8; 4] = [105u8, 98u8, 85u8, 190u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._newMinimumWeight),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._operators),
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
    /**Function with signature `updateOperatorSigningKey(address)` and selector `0x743c31f4`.
```solidity
function updateOperatorSigningKey(address _newSigningKey) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorSigningKeyCall {
        pub _newSigningKey: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`updateOperatorSigningKey(address)`](updateOperatorSigningKeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorSigningKeyReturn {}
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
            impl ::core::convert::From<updateOperatorSigningKeyCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorSigningKeyCall) -> Self {
                    (value._newSigningKey,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorSigningKeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _newSigningKey: tuple.0 }
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
            impl ::core::convert::From<updateOperatorSigningKeyReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorSigningKeyReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorSigningKeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorSigningKeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorSigningKeyReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorSigningKey(address)";
            const SELECTOR: [u8; 4] = [116u8, 60u8, 49u8, 244u8];
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
                        &self._newSigningKey,
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
    /**Function with signature `updateOperators(address[])` and selector `0x00cf2ab5`.
```solidity
function updateOperators(address[] memory _operators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsCall {
        pub _operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`updateOperators(address[])`](updateOperatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsReturn {}
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
            impl ::core::convert::From<updateOperatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsCall) -> Self {
                    (value._operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _operators: tuple.0 }
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
            impl ::core::convert::From<updateOperatorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperators(address[])";
            const SELECTOR: [u8; 4] = [0u8, 207u8, 42u8, 181u8];
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
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._operators),
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
    /**Function with signature `updateOperatorsForQuorum(address[][],bytes)` and selector `0x5140a548`.
```solidity
function updateOperatorsForQuorum(address[][] memory operatorsPerQuorum, bytes memory) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumCall {
        pub operatorsPerQuorum: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        >,
        pub _1: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`updateOperatorsForQuorum(address[][],bytes)`](updateOperatorsForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorsForQuorumReturn {}
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
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                >,
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
            impl ::core::convert::From<updateOperatorsForQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumCall) -> Self {
                    (value.operatorsPerQuorum, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorsForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorsPerQuorum: tuple.0,
                        _1: tuple.1,
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
            impl ::core::convert::From<updateOperatorsForQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorsForQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorsForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorsForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    >,
                >,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorsForQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorsForQuorum(address[][],bytes)";
            const SELECTOR: [u8; 4] = [81u8, 64u8, 165u8, 72u8];
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
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Address,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorsPerQuorum),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
    /**Function with signature `updateQuorumConfig(((address,uint96)[]),address[])` and selector `0xdec5d1f6`.
```solidity
function updateQuorumConfig(Quorum memory _quorum, address[] memory _operators) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateQuorumConfigCall {
        pub _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
        pub _operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`updateQuorumConfig(((address,uint96)[]),address[])`](updateQuorumConfigCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateQuorumConfigReturn {}
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
                Quorum,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Quorum as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<updateQuorumConfigCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateQuorumConfigCall) -> Self {
                    (value._quorum, value._operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateQuorumConfigCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _quorum: tuple.0,
                        _operators: tuple.1,
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
            impl ::core::convert::From<updateQuorumConfigReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateQuorumConfigReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateQuorumConfigReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateQuorumConfigCall {
            type Parameters<'a> = (
                Quorum,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateQuorumConfigReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateQuorumConfig(((address,uint96)[]),address[])";
            const SELECTOR: [u8; 4] = [222u8, 197u8, 209u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Quorum as alloy_sol_types::SolType>::tokenize(&self._quorum),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self._operators),
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
    /**Function with signature `updateStakeThreshold(uint256)` and selector `0x5ef53329`.
```solidity
function updateStakeThreshold(uint256 _thresholdWeight) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateStakeThresholdCall {
        pub _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`updateStakeThreshold(uint256)`](updateStakeThresholdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateStakeThresholdReturn {}
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<updateStakeThresholdCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateStakeThresholdCall) -> Self {
                    (value._thresholdWeight,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateStakeThresholdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _thresholdWeight: tuple.0 }
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
            impl ::core::convert::From<updateStakeThresholdReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateStakeThresholdReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateStakeThresholdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateStakeThresholdCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateStakeThresholdReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateStakeThreshold(uint256)";
            const SELECTOR: [u8; 4] = [94u8, 245u8, 51u8, 41u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._thresholdWeight),
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
    ///Container for all the [`ECDSAStakeRegistry`](self) function calls.
    pub enum ECDSAStakeRegistryCalls {
        deregisterOperator(deregisterOperatorCall),
        getLastCheckpointOperatorWeight(getLastCheckpointOperatorWeightCall),
        getLastCheckpointThresholdWeight(getLastCheckpointThresholdWeightCall),
        getLastCheckpointThresholdWeightAtBlock(
            getLastCheckpointThresholdWeightAtBlockCall,
        ),
        getLastCheckpointTotalWeight(getLastCheckpointTotalWeightCall),
        getLastCheckpointTotalWeightAtBlock(getLastCheckpointTotalWeightAtBlockCall),
        getLastestOperatorSigningKey(getLastestOperatorSigningKeyCall),
        getOperatorSigningKeyAtBlock(getOperatorSigningKeyAtBlockCall),
        getOperatorWeight(getOperatorWeightCall),
        getOperatorWeightAtBlock(getOperatorWeightAtBlockCall),
        initialize(initializeCall),
        isValidSignature(isValidSignatureCall),
        minimumWeight(minimumWeightCall),
        operatorRegistered(operatorRegisteredCall),
        owner(ownerCall),
        quorum(quorumCall),
        registerOperatorWithSignature(registerOperatorWithSignatureCall),
        renounceOwnership(renounceOwnershipCall),
        transferOwnership(transferOwnershipCall),
        updateMinimumWeight(updateMinimumWeightCall),
        updateOperatorSigningKey(updateOperatorSigningKeyCall),
        updateOperators(updateOperatorsCall),
        updateOperatorsForQuorum(updateOperatorsForQuorumCall),
        updateQuorumConfig(updateQuorumConfigCall),
        updateStakeThreshold(updateStakeThresholdCall),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 207u8, 42u8, 181u8],
            [13u8, 186u8, 51u8, 148u8],
            [22u8, 38u8, 186u8, 126u8],
            [23u8, 3u8, 160u8, 24u8],
            [30u8, 76u8, 216u8, 94u8],
            [49u8, 79u8, 58u8, 73u8],
            [59u8, 36u8, 46u8, 74u8],
            [61u8, 86u8, 17u8, 246u8],
            [64u8, 191u8, 47u8, 183u8],
            [81u8, 64u8, 165u8, 72u8],
            [94u8, 16u8, 66u8, 232u8],
            [94u8, 245u8, 51u8, 41u8],
            [105u8, 98u8, 85u8, 190u8],
            [113u8, 80u8, 24u8, 166u8],
            [116u8, 60u8, 49u8, 244u8],
            [133u8, 125u8, 193u8, 144u8],
            [141u8, 165u8, 203u8, 91u8],
            [149u8, 95u8, 45u8, 144u8],
            [152u8, 236u8, 26u8, 201u8],
            [171u8, 17u8, 137u8, 149u8],
            [185u8, 51u8, 250u8, 116u8],
            [205u8, 205u8, 53u8, 129u8],
            [222u8, 197u8, 209u8, 246u8],
            [236u8, 127u8, 187u8, 49u8],
            [242u8, 253u8, 227u8, 139u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ECDSAStakeRegistryCalls {
        const NAME: &'static str = "ECDSAStakeRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::deregisterOperator(_) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointOperatorWeight(_) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointThresholdWeight(_) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointThresholdWeightAtBlock(_) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointTotalWeight(_) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastCheckpointTotalWeightAtBlock(_) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLastestOperatorSigningKey(_) => {
                    <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSigningKeyAtBlock(_) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorWeight(_) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorWeightAtBlock(_) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isValidSignature(_) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minimumWeight(_) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorRegistered(_) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::quorum(_) => <quorumCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::registerOperatorWithSignature(_) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateMinimumWeight(_) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorSigningKey(_) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperators(_) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorsForQuorum(_) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateQuorumConfig(_) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateStakeThreshold(_) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls>] = &[
                {
                    fn updateOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateOperators)
                    }
                    updateOperators
                },
                {
                    fn getLastCheckpointTotalWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointTotalWeightAtBlock,
                            )
                    }
                    getLastCheckpointTotalWeightAtBlock
                },
                {
                    fn isValidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <isValidSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::isValidSignature)
                    }
                    isValidSignature
                },
                {
                    fn quorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <quorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::quorum)
                    }
                    quorum
                },
                {
                    fn getLastCheckpointThresholdWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointThresholdWeightAtBlock,
                            )
                    }
                    getLastCheckpointThresholdWeightAtBlock
                },
                {
                    fn getLastCheckpointTotalWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getLastCheckpointTotalWeight)
                    }
                    getLastCheckpointTotalWeight
                },
                {
                    fn getLastCheckpointOperatorWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointOperatorWeight,
                            )
                    }
                    getLastCheckpointOperatorWeight
                },
                {
                    fn registerOperatorWithSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::registerOperatorWithSignature)
                    }
                    registerOperatorWithSignature
                },
                {
                    fn minimumWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <minimumWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::minimumWeight)
                    }
                    minimumWeight
                },
                {
                    fn updateOperatorsForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateOperatorsForQuorum)
                    }
                    updateOperatorsForQuorum
                },
                {
                    fn getOperatorSigningKeyAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getOperatorSigningKeyAtBlock)
                    }
                    getOperatorSigningKeyAtBlock
                },
                {
                    fn updateStakeThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateStakeThreshold)
                    }
                    updateStakeThreshold
                },
                {
                    fn updateMinimumWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateMinimumWeight)
                    }
                    updateMinimumWeight
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn updateOperatorSigningKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateOperatorSigningKey)
                    }
                    updateOperatorSigningKey
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::owner)
                    }
                    owner
                },
                {
                    fn getOperatorWeightAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getOperatorWeightAtBlock)
                    }
                    getOperatorWeightAtBlock
                },
                {
                    fn getOperatorWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getOperatorWeight)
                    }
                    getOperatorWeight
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getLastCheckpointThresholdWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                ECDSAStakeRegistryCalls::getLastCheckpointThresholdWeight,
                            )
                    }
                    getLastCheckpointThresholdWeight
                },
                {
                    fn getLastestOperatorSigningKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::getLastestOperatorSigningKey)
                    }
                    getLastestOperatorSigningKey
                },
                {
                    fn updateQuorumConfig(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::updateQuorumConfig)
                    }
                    updateQuorumConfig
                },
                {
                    fn operatorRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::operatorRegistered)
                    }
                    operatorRegistered
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryCalls::transferOwnership)
                    }
                    transferOwnership
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
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointOperatorWeight(inner) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointThresholdWeight(inner) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointThresholdWeightAtBlock(inner) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointTotalWeight(inner) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastCheckpointTotalWeightAtBlock(inner) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLastestOperatorSigningKey(inner) => {
                    <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorSigningKeyAtBlock(inner) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorWeight(inner) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorWeightAtBlock(inner) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::minimumWeight(inner) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorRegistered(inner) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::quorum(inner) => {
                    <quorumCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerOperatorWithSignature(inner) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateMinimumWeight(inner) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorSigningKey(inner) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateQuorumConfig(inner) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateStakeThreshold(inner) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointOperatorWeight(inner) => {
                    <getLastCheckpointOperatorWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointThresholdWeight(inner) => {
                    <getLastCheckpointThresholdWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointThresholdWeightAtBlock(inner) => {
                    <getLastCheckpointThresholdWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointTotalWeight(inner) => {
                    <getLastCheckpointTotalWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastCheckpointTotalWeightAtBlock(inner) => {
                    <getLastCheckpointTotalWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLastestOperatorSigningKey(inner) => {
                    <getLastestOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorSigningKeyAtBlock(inner) => {
                    <getOperatorSigningKeyAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorWeight(inner) => {
                    <getOperatorWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorWeightAtBlock(inner) => {
                    <getOperatorWeightAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isValidSignature(inner) => {
                    <isValidSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minimumWeight(inner) => {
                    <minimumWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorRegistered(inner) => {
                    <operatorRegisteredCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::quorum(inner) => {
                    <quorumCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registerOperatorWithSignature(inner) => {
                    <registerOperatorWithSignatureCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateMinimumWeight(inner) => {
                    <updateMinimumWeightCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorSigningKey(inner) => {
                    <updateOperatorSigningKeyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperators(inner) => {
                    <updateOperatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorsForQuorum(inner) => {
                    <updateOperatorsForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateQuorumConfig(inner) => {
                    <updateQuorumConfigCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateStakeThreshold(inner) => {
                    <updateStakeThresholdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ECDSAStakeRegistry`](self) custom errors.
    pub enum ECDSAStakeRegistryErrors {
        InsufficientSignedStake(InsufficientSignedStake),
        InsufficientWeight(InsufficientWeight),
        InvalidLength(InvalidLength),
        InvalidQuorum(InvalidQuorum),
        InvalidReferenceBlock(InvalidReferenceBlock),
        InvalidSignature(InvalidSignature),
        InvalidSignedWeight(InvalidSignedWeight),
        InvalidThreshold(InvalidThreshold),
        LengthMismatch(LengthMismatch),
        MustUpdateAllOperators(MustUpdateAllOperators),
        NotSorted(NotSorted),
        OperatorAlreadyRegistered(OperatorAlreadyRegistered),
        OperatorNotRegistered(OperatorNotRegistered),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [37u8, 236u8, 108u8, 31u8],
            [45u8, 61u8, 246u8, 182u8],
            [66u8, 238u8, 104u8, 181u8],
            [139u8, 170u8, 87u8, 159u8],
            [148u8, 125u8, 90u8, 132u8],
            [150u8, 11u8, 65u8, 238u8],
            [168u8, 121u8, 47u8, 209u8],
            [170u8, 189u8, 90u8, 9u8],
            [186u8, 80u8, 249u8, 17u8],
            [209u8, 115u8, 87u8, 121u8],
            [225u8, 33u8, 99u8, 47u8],
            [230u8, 79u8, 24u8, 15u8],
            [255u8, 99u8, 58u8, 56u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ECDSAStakeRegistryErrors {
        const NAME: &'static str = "ECDSAStakeRegistryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 13usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InsufficientSignedStake(_) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientWeight(_) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidLength(_) => {
                    <InvalidLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidQuorum(_) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidReferenceBlock(_) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignedWeight(_) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidThreshold(_) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::SELECTOR
                }
                Self::LengthMismatch(_) => {
                    <LengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MustUpdateAllOperators(_) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotSorted(_) => <NotSorted as alloy_sol_types::SolError>::SELECTOR,
                Self::OperatorAlreadyRegistered(_) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors>] = &[
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::OperatorNotRegistered)
                    }
                    OperatorNotRegistered
                },
                {
                    fn MustUpdateAllOperators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::MustUpdateAllOperators)
                    }
                    MustUpdateAllOperators
                },
                {
                    fn OperatorAlreadyRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::OperatorAlreadyRegistered)
                    }
                    OperatorAlreadyRegistered
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn InvalidLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidLength)
                    }
                    InvalidLength
                },
                {
                    fn InvalidSignedWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidSignedWeight as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidSignedWeight)
                    }
                    InvalidSignedWeight
                },
                {
                    fn InsufficientWeight(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InsufficientWeight as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InsufficientWeight)
                    }
                    InsufficientWeight
                },
                {
                    fn InvalidThreshold(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidThreshold as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidThreshold)
                    }
                    InvalidThreshold
                },
                {
                    fn NotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <NotSorted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::NotSorted)
                    }
                    NotSorted
                },
                {
                    fn InvalidQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidQuorum as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidQuorum)
                    }
                    InvalidQuorum
                },
                {
                    fn InsufficientSignedStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InsufficientSignedStake as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InsufficientSignedStake)
                    }
                    InsufficientSignedStake
                },
                {
                    fn InvalidReferenceBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::InvalidReferenceBlock)
                    }
                    InvalidReferenceBlock
                },
                {
                    fn LengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ECDSAStakeRegistryErrors> {
                        <LengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ECDSAStakeRegistryErrors::LengthMismatch)
                    }
                    LengthMismatch
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
                Self::InsufficientSignedStake(inner) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InsufficientWeight(inner) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidLength(inner) => {
                    <InvalidLength as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidQuorum(inner) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidReferenceBlock(inner) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignedWeight(inner) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidThreshold(inner) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::LengthMismatch(inner) => {
                    <LengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MustUpdateAllOperators(inner) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InsufficientSignedStake(inner) => {
                    <InsufficientSignedStake as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InsufficientWeight(inner) => {
                    <InsufficientWeight as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidLength(inner) => {
                    <InvalidLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidQuorum(inner) => {
                    <InvalidQuorum as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidReferenceBlock(inner) => {
                    <InvalidReferenceBlock as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSignedWeight(inner) => {
                    <InvalidSignedWeight as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidThreshold(inner) => {
                    <InvalidThreshold as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::LengthMismatch(inner) => {
                    <LengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MustUpdateAllOperators(inner) => {
                    <MustUpdateAllOperators as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NotSorted(inner) => {
                    <NotSorted as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OperatorAlreadyRegistered(inner) => {
                    <OperatorAlreadyRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`ECDSAStakeRegistry`](self) events.
    pub enum ECDSAStakeRegistryEvents {
        Initialized(Initialized),
        MinimumWeightUpdated(MinimumWeightUpdated),
        OperatorDeregistered(OperatorDeregistered),
        OperatorRegistered(OperatorRegistered),
        OperatorWeightUpdated(OperatorWeightUpdated),
        OwnershipTransferred(OwnershipTransferred),
        QuorumUpdated(QuorumUpdated),
        SigningKeyUpdate(SigningKeyUpdate),
        ThresholdWeightUpdated(ThresholdWeightUpdated),
        TotalWeightUpdated(TotalWeightUpdated),
        UpdateMinimumWeight(UpdateMinimumWeight),
    }
    #[automatically_derived]
    impl ECDSAStakeRegistryEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                30u8,
                164u8,
                33u8,
                134u8,
                179u8,
                5u8,
                250u8,
                55u8,
                49u8,
                4u8,
                80u8,
                217u8,
                251u8,
                135u8,
                234u8,
                30u8,
                143u8,
                12u8,
                127u8,
                68u8,
                126u8,
                119u8,
                20u8,
                121u8,
                227u8,
                178u8,
                118u8,
                52u8,
                191u8,
                232u8,
                77u8,
                193u8,
            ],
            [
                35u8,
                170u8,
                212u8,
                230u8,
                23u8,
                68u8,
                236u8,
                225u8,
                100u8,
                19u8,
                10u8,
                164u8,
                21u8,
                193u8,
                97u8,
                110u8,
                128u8,
                19u8,
                107u8,
                15u8,
                7u8,
                112u8,
                229u8,
                101u8,
                137u8,
                67u8,
                139u8,
                144u8,
                178u8,
                105u8,
                38u8,
                94u8,
            ],
            [
                49u8,
                224u8,
                173u8,
                254u8,
                199u8,
                27u8,
                204u8,
                238u8,
                55u8,
                182u8,
                232u8,
                58u8,
                144u8,
                194u8,
                254u8,
                219u8,
                23u8,
                216u8,
                241u8,
                105u8,
                63u8,
                238u8,
                134u8,
                60u8,
                71u8,
                113u8,
                231u8,
                191u8,
                226u8,
                174u8,
                213u8,
                128u8,
            ],
            [
                113u8,
                60u8,
                165u8,
                59u8,
                136u8,
                214u8,
                235u8,
                99u8,
                245u8,
                177u8,
                133u8,
                76u8,
                184u8,
                203u8,
                221u8,
                115u8,
                110u8,
                197u8,
                30u8,
                218u8,
                34u8,
                94u8,
                70u8,
                121u8,
                26u8,
                169u8,
                41u8,
                139u8,
                1u8,
                96u8,
                100u8,
                143u8,
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
                134u8,
                220u8,
                248u8,
                107u8,
                18u8,
                223u8,
                238u8,
                222u8,
                167u8,
                74u8,
                233u8,
                48u8,
                13u8,
                189u8,
                170u8,
                25u8,
                59u8,
                204u8,
                229u8,
                128u8,
                147u8,
                105u8,
                200u8,
                23u8,
                126u8,
                162u8,
                244u8,
                234u8,
                170u8,
                101u8,
                114u8,
                155u8,
            ],
            [
                136u8,
                119u8,
                13u8,
                200u8,
                98u8,
                228u8,
                122u8,
                126u8,
                213u8,
                134u8,
                144u8,
                120u8,
                87u8,
                235u8,
                27u8,
                117u8,
                228u8,
                197u8,
                255u8,
                200u8,
                183u8,
                7u8,
                199u8,
                238u8,
                16u8,
                235u8,
                116u8,
                214u8,
                136u8,
                95u8,
                229u8,
                148u8,
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
                147u8,
                36u8,
                247u8,
                229u8,
                167u8,
                192u8,
                40u8,
                136u8,
                8u8,
                166u8,
                52u8,
                204u8,
                222u8,
                68u8,
                184u8,
                233u8,
                121u8,
                103u8,
                100u8,
                116u8,
                178u8,
                46u8,
                41u8,
                238u8,
                157u8,
                213u8,
                105u8,
                181u8,
                94u8,
                121u8,
                26u8,
                75u8,
            ],
            [
                164u8,
                83u8,
                219u8,
                97u8,
                42u8,
                245u8,
                158u8,
                85u8,
                33u8,
                214u8,
                171u8,
                146u8,
                132u8,
                220u8,
                62u8,
                45u8,
                6u8,
                175u8,
                40u8,
                110u8,
                177u8,
                177u8,
                183u8,
                183u8,
                113u8,
                252u8,
                228u8,
                113u8,
                108u8,
                25u8,
                242u8,
                193u8,
            ],
            [
                208u8,
                97u8,
                22u8,
                130u8,
                82u8,
                244u8,
                65u8,
                115u8,
                54u8,
                88u8,
                240u8,
                158u8,
                77u8,
                143u8,
                91u8,
                45u8,
                153u8,
                142u8,
                212u8,
                239u8,
                36u8,
                162u8,
                187u8,
                253u8,
                108u8,
                236u8,
                165u8,
                46u8,
                161u8,
                49u8,
                80u8,
                2u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for ECDSAStakeRegistryEvents {
        const NAME: &'static str = "ECDSAStakeRegistryEvents";
        const COUNT: usize = 11usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
                }
                Some(
                    <MinimumWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MinimumWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::MinimumWeightUpdated)
                }
                Some(
                    <OperatorDeregistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorDeregistered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorDeregistered)
                }
                Some(
                    <OperatorRegistered as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorRegistered as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorRegistered)
                }
                Some(
                    <OperatorWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorWeightUpdated)
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
                Some(<QuorumUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::QuorumUpdated)
                }
                Some(<SigningKeyUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <SigningKeyUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::SigningKeyUpdate)
                }
                Some(
                    <ThresholdWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ThresholdWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ThresholdWeightUpdated)
                }
                Some(
                    <TotalWeightUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <TotalWeightUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::TotalWeightUpdated)
                }
                Some(
                    <UpdateMinimumWeight as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <UpdateMinimumWeight as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::UpdateMinimumWeight)
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
    impl alloy_sol_types::private::IntoLogData for ECDSAStakeRegistryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MinimumWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::SigningKeyUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ThresholdWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TotalWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::UpdateMinimumWeight(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MinimumWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorDeregistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRegistered(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::SigningKeyUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ThresholdWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TotalWeightUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::UpdateMinimumWeight(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`ECDSAStakeRegistry`](self) contract instance.

See the [wrapper's documentation](`ECDSAStakeRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ECDSAStakeRegistryInstance<T, P, N> {
        ECDSAStakeRegistryInstance::<T, P, N>::new(address, provider)
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
        _delegationManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<ECDSAStakeRegistryInstance<T, P, N>>,
    > {
        ECDSAStakeRegistryInstance::<T, P, N>::deploy(provider, _delegationManager)
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
        _delegationManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        ECDSAStakeRegistryInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _delegationManager)
    }
    /**A [`ECDSAStakeRegistry`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ECDSAStakeRegistry`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ECDSAStakeRegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ECDSAStakeRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ECDSAStakeRegistryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ECDSAStakeRegistryInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ECDSAStakeRegistry`](self) contract instance.

See the [wrapper's documentation](`ECDSAStakeRegistryInstance`) for more details.*/
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
            _delegationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<ECDSAStakeRegistryInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _delegationManager);
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
            _delegationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
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
    impl<T, P: ::core::clone::Clone, N> ECDSAStakeRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ECDSAStakeRegistryInstance<T, P, N> {
            ECDSAStakeRegistryInstance {
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
    > ECDSAStakeRegistryInstance<T, P, N> {
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
        ///Creates a new call builder for the [`deregisterOperator`] function.
        pub fn deregisterOperator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorCall, N> {
            self.call_builder(&deregisterOperatorCall {})
        }
        ///Creates a new call builder for the [`getLastCheckpointOperatorWeight`] function.
        pub fn getLastCheckpointOperatorWeight(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getLastCheckpointOperatorWeightCall,
            N,
        > {
            self.call_builder(
                &getLastCheckpointOperatorWeightCall {
                    _operator,
                },
            )
        }
        ///Creates a new call builder for the [`getLastCheckpointThresholdWeight`] function.
        pub fn getLastCheckpointThresholdWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getLastCheckpointThresholdWeightCall,
            N,
        > {
            self.call_builder(
                &getLastCheckpointThresholdWeightCall {
                },
            )
        }
        ///Creates a new call builder for the [`getLastCheckpointThresholdWeightAtBlock`] function.
        pub fn getLastCheckpointThresholdWeightAtBlock(
            &self,
            _blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getLastCheckpointThresholdWeightAtBlockCall,
            N,
        > {
            self.call_builder(
                &getLastCheckpointThresholdWeightAtBlockCall {
                    _blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getLastCheckpointTotalWeight`] function.
        pub fn getLastCheckpointTotalWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastCheckpointTotalWeightCall, N> {
            self.call_builder(
                &getLastCheckpointTotalWeightCall {
                },
            )
        }
        ///Creates a new call builder for the [`getLastCheckpointTotalWeightAtBlock`] function.
        pub fn getLastCheckpointTotalWeightAtBlock(
            &self,
            _blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getLastCheckpointTotalWeightAtBlockCall,
            N,
        > {
            self.call_builder(
                &getLastCheckpointTotalWeightAtBlockCall {
                    _blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getLastestOperatorSigningKey`] function.
        pub fn getLastestOperatorSigningKey(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLastestOperatorSigningKeyCall, N> {
            self.call_builder(
                &getLastestOperatorSigningKeyCall {
                    _operator,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorSigningKeyAtBlock`] function.
        pub fn getOperatorSigningKeyAtBlock(
            &self,
            _operator: alloy::sol_types::private::Address,
            _blockNumber: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSigningKeyAtBlockCall, N> {
            self.call_builder(
                &getOperatorSigningKeyAtBlockCall {
                    _operator,
                    _blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorWeight`] function.
        pub fn getOperatorWeight(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorWeightCall, N> {
            self.call_builder(&getOperatorWeightCall { _operator })
        }
        ///Creates a new call builder for the [`getOperatorWeightAtBlock`] function.
        pub fn getOperatorWeightAtBlock(
            &self,
            _operator: alloy::sol_types::private::Address,
            _blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorWeightAtBlockCall, N> {
            self.call_builder(
                &getOperatorWeightAtBlockCall {
                    _operator,
                    _blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _serviceManager: alloy::sol_types::private::Address,
            _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
            _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    _serviceManager,
                    _thresholdWeight,
                    _quorum,
                },
            )
        }
        ///Creates a new call builder for the [`isValidSignature`] function.
        pub fn isValidSignature(
            &self,
            _dataHash: alloy::sol_types::private::FixedBytes<32>,
            _signatureData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, isValidSignatureCall, N> {
            self.call_builder(
                &isValidSignatureCall {
                    _dataHash,
                    _signatureData,
                },
            )
        }
        ///Creates a new call builder for the [`minimumWeight`] function.
        pub fn minimumWeight(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, minimumWeightCall, N> {
            self.call_builder(&minimumWeightCall {})
        }
        ///Creates a new call builder for the [`operatorRegistered`] function.
        pub fn operatorRegistered(
            &self,
            _operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorRegisteredCall, N> {
            self.call_builder(
                &operatorRegisteredCall {
                    _operator,
                },
            )
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`quorum`] function.
        pub fn quorum(&self) -> alloy_contract::SolCallBuilder<T, &P, quorumCall, N> {
            self.call_builder(&quorumCall {})
        }
        ///Creates a new call builder for the [`registerOperatorWithSignature`] function.
        pub fn registerOperatorWithSignature(
            &self,
            _operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
            _signingKey: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            registerOperatorWithSignatureCall,
            N,
        > {
            self.call_builder(
                &registerOperatorWithSignatureCall {
                    _operatorSignature,
                    _signingKey,
                },
            )
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`updateMinimumWeight`] function.
        pub fn updateMinimumWeight(
            &self,
            _newMinimumWeight: alloy::sol_types::private::primitives::aliases::U256,
            _operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateMinimumWeightCall, N> {
            self.call_builder(
                &updateMinimumWeightCall {
                    _newMinimumWeight,
                    _operators,
                },
            )
        }
        ///Creates a new call builder for the [`updateOperatorSigningKey`] function.
        pub fn updateOperatorSigningKey(
            &self,
            _newSigningKey: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorSigningKeyCall, N> {
            self.call_builder(
                &updateOperatorSigningKeyCall {
                    _newSigningKey,
                },
            )
        }
        ///Creates a new call builder for the [`updateOperators`] function.
        pub fn updateOperators(
            &self,
            _operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsCall, N> {
            self.call_builder(&updateOperatorsCall { _operators })
        }
        ///Creates a new call builder for the [`updateOperatorsForQuorum`] function.
        pub fn updateOperatorsForQuorum(
            &self,
            operatorsPerQuorum: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            >,
            _1: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorsForQuorumCall, N> {
            self.call_builder(
                &updateOperatorsForQuorumCall {
                    operatorsPerQuorum,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`updateQuorumConfig`] function.
        pub fn updateQuorumConfig(
            &self,
            _quorum: <Quorum as alloy::sol_types::SolType>::RustType,
            _operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateQuorumConfigCall, N> {
            self.call_builder(
                &updateQuorumConfigCall {
                    _quorum,
                    _operators,
                },
            )
        }
        ///Creates a new call builder for the [`updateStakeThreshold`] function.
        pub fn updateStakeThreshold(
            &self,
            _thresholdWeight: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateStakeThresholdCall, N> {
            self.call_builder(
                &updateStakeThresholdCall {
                    _thresholdWeight,
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
    > ECDSAStakeRegistryInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`MinimumWeightUpdated`] event.
        pub fn MinimumWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MinimumWeightUpdated, N> {
            self.event_filter::<MinimumWeightUpdated>()
        }
        ///Creates a new event filter for the [`OperatorDeregistered`] event.
        pub fn OperatorDeregistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorDeregistered, N> {
            self.event_filter::<OperatorDeregistered>()
        }
        ///Creates a new event filter for the [`OperatorRegistered`] event.
        pub fn OperatorRegistered_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRegistered, N> {
            self.event_filter::<OperatorRegistered>()
        }
        ///Creates a new event filter for the [`OperatorWeightUpdated`] event.
        pub fn OperatorWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorWeightUpdated, N> {
            self.event_filter::<OperatorWeightUpdated>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`QuorumUpdated`] event.
        pub fn QuorumUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, QuorumUpdated, N> {
            self.event_filter::<QuorumUpdated>()
        }
        ///Creates a new event filter for the [`SigningKeyUpdate`] event.
        pub fn SigningKeyUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, SigningKeyUpdate, N> {
            self.event_filter::<SigningKeyUpdate>()
        }
        ///Creates a new event filter for the [`ThresholdWeightUpdated`] event.
        pub fn ThresholdWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ThresholdWeightUpdated, N> {
            self.event_filter::<ThresholdWeightUpdated>()
        }
        ///Creates a new event filter for the [`TotalWeightUpdated`] event.
        pub fn TotalWeightUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TotalWeightUpdated, N> {
            self.event_filter::<TotalWeightUpdated>()
        }
        ///Creates a new event filter for the [`UpdateMinimumWeight`] event.
        pub fn UpdateMinimumWeight_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, UpdateMinimumWeight, N> {
            self.event_filter::<UpdateMinimumWeight>()
        }
    }
}
