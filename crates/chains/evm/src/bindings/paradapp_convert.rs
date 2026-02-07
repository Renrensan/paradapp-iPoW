// @generated — DO NOT EDIT
#![allow(clippy::module_inception)]

pub use paradapp_convert::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod paradapp_convert {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_nativeDecimals"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_selfNetworkId"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_operator"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_oracle"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_bitcoinUsdPriceId"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_nativeUsdPriceId"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_commitFeeNative"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_serviceFeeBps"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint16"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("APPROVAL_WINDOW_SEC"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("APPROVAL_WINDOW_SEC",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BPS_DENOM"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("BPS_DENOM"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BTC_DECIMALS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("BTC_DECIMALS"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("CONFIRMATIONS_REQUIRED"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("CONFIRMATIONS_REQUIRED",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DEPOSIT_BLOCKS_WINDOW"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DEPOSIT_BLOCKS_WINDOW",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DIFF_PERIOD"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DIFF_PERIOD"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_TIMESPAN_SEC"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MAX_TIMESPAN_SEC"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MIN_TIMESPAN_SEC"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MIN_TIMESPAN_SEC"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NATIVE_DECIMALS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("NATIVE_DECIMALS"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PROOF_BLOCKS_WINDOW"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("PROOF_BLOCKS_WINDOW",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RESERVE_MARGIN_BPS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("RESERVE_MARGIN_BPS"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("RETARGET_PERIOD_SEC"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("RETARGET_PERIOD_SEC",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SELF_NETWORK_ID"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("SELF_NETWORK_ID"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("activeOpenConversions"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("activeOpenConversions",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addNativeLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addNativeLiquidity"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("addNetwork"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("addNetwork"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("networkId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("minAddrLen"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("maxAddrLen"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("anchorInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("anchorInfo"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("anchorHeight"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("epochFirstHeight"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approveAndStartWithAnchorAndFirst"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("approveAndStartWithAnchorAndFirst",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("txId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("dutyWindowSeconds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("paradappReceiveProgram",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("slippage"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("bitcoinUsdPriceId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("bitcoinUsdPriceId"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("claimNative_AfterOperatorExpired"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("claimNative_AfterOperatorExpired",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("closeNoBitcoin_BitcoinToNative"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("closeNoBitcoin_BitcoinToNative",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("commitBitcoinToNative"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("commitBitcoinToNative",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("bitcoinAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("networkId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("userProgram"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("destAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("networkAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("dutyWindowSeconds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("paradappReceiveProgram",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lockedAnchorHeight",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("slippage"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("commitFeeNative"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("commitFeeNative"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("commitGlobalBitcoinHeader80"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("commitGlobalBitcoinHeader80",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("header80"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("height"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("commitNativeToBitcoin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("commitNativeToBitcoin",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nativeAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("networkId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("networkAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("userProgram"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("conversions"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("conversions"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("isNativeToBitcoin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("slippage"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("userProgram"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("paradappReceiveProgram",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("networkAddress"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("networkId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nativeAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("bitcoinAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("createdAt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("approvedAt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("depositedAt"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("commitFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("approved"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deposited"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("completed"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("refunded"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reservedNative"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorDutyExpiresAt",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("debugDecodeHeader"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("debugDecodeHeader"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("header80"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("hashLE"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("prevLE"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("merkleLE"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nBits"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("depositApprovedConversion"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("depositApprovedConversion",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("estimateBitcoinFromNative"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("estimateBitcoinFromNative",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("nativeAmount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("estimateNativeFromBitcoin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("estimateNativeFromBitcoin",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("bitcoinAmount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("expectedNext"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("expectedNext"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("headersStarted"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nextHeight"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expectedPrevHashLE",),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getConversionWithPhase"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getConversionWithPhase",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("c"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ::ethers::core::abi::ethabi::ParamType::Bool,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct ParadappConvert.Conversion",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("phase"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("enum ParadappConvert.Phase",),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getTxIdsByFilter"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getTxIdsByFilter"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("typeFilter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "enum ParadappConvert.TypeFilter",
                                    ),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("phaseFilter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("enum ParadappConvert.Phase",),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("userFilter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("bitcoinProgramFilter",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("searchUserProgram"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("networkIdFilter"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("useNetworkIdFilter",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("fromTxId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("toTxId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("maxResults"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txIds"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("globalHeaders"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("globalHeaders"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("prevHashLE"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("merkleRootLE"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nBits"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("set"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("arrivalTime"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("globalHeightToHashLE"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("globalHeightToHashLE",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("globalTipHeight"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("globalTipHeight"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("minAnchorHeight"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("minAnchorHeight"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nativeLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nativeLiquidity"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nativeUsdPriceId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nativeUsdPriceId"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("networkConfigs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("networkConfigs"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("enabled"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("minAddrLen"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("maxAddrLen"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("nextTxId"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nextTxId"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("operator"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proofInfo"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("proofInfo"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("set"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("verified"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("invalid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("attempts"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("txidLE"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockHashLE"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockHeight"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("outValueSats"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("outProgram"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("refundAfterNoProof_NativeToBitcoin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "refundAfterNoProof_NativeToBitcoin",
                        ),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("refundIfNotApproved"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("refundIfNotApproved",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removableNative"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removableNative"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeNativeLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeNativeLiquidity",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("removeNetwork"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("removeNetwork"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("networkId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("serviceFeeBps"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("serviceFeeBps"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint16"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setFees"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newCommitFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("newServiceFeeBps"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint16"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setOperator"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setOperator"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newOperator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("submitBitcoinMerkleProofWithTx"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("submitBitcoinMerkleProofWithTx",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("txId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("txRaw"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("voutIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockHashLE"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockHeight"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("branchLE"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("index"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("timeoutNoDeposit_NativetoBitcoin"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("timeoutNoDeposit_NativetoBitcoin",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalHeldCommitFees"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalHeldCommitFees",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalLockedDeposits"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalLockedDeposits",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("totalReservedNative"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalReservedNative",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("usedParadappPrograms"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("usedParadappPrograms",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("usedProofs"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("usedProofs"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("windowsFor"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("windowsFor"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("txId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("headersStarted"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("startHeight"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lastHeight"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("depositWindowEndHeight",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proofWindowEndHeight",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operatorDutyExpiresAt",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ConversionApproved"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ConversionApproved"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("txId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("dutyWindowSeconds"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("firstHeight"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("firstHeaderHashLE"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ConversionCommitted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ConversionCommitted",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("txId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("user"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("isNativetoBitcoin"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ConversionCompleted"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ConversionCompleted",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("txId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: true,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ConversionDeposited"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ConversionDeposited",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("txId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nativeAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ConversionRefunded"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ConversionRefunded"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("txId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("refundNative"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("commitFeeRefunded"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("FeesUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("FeesUpdated"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newCommitFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("newServiceFeeBps"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(16usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GlobalHeaderAppended"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("GlobalHeaderAppended",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("height"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("hashLE"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("prevHashLE"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("merkleRootLE"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("nBits"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LiquidityUpdated"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("LiquidityUpdated"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("nativeLiquidity"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorChanged"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("OperatorChanged"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("newOperator"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AlreadyVerified"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AlreadyVerified"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("AnchorMustBeTip"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AnchorMustBeTip"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApproveWindowOver"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ApproveWindowOver"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BadBitcoinProgram"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BadBitcoinProgram"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BadSlippage"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BadSlippage"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BadState"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BadState"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BadTxId"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BadTxId"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DutyExpired"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DutyExpired"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DutyNotExpired"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DutyNotExpired"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EpochAnchorsMissing"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("EpochAnchorsMissing",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EpochFirstMissing"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("EpochFirstMissing"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EpochMetaMissing"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("EpochMetaMissing"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExceedsRemovable"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ExceedsRemovable"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GlobalAnchorMissing"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("GlobalAnchorMissing",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GlobalFirstHeaderMissing"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("GlobalFirstHeaderMissing",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HeaderStarted"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("HeaderStarted"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("HeightRewrite"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("HeightRewrite"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncorrectCommitFee"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("IncorrectCommitFee"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncorrectNetwork"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("IncorrectNetwork"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncorrectNetworkAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("IncorrectNetworkAddress",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncorrectValue"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("IncorrectValue"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("IncorrectWindow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("IncorrectWindow"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidAnchorHeight"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidAnchorHeight",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidConstructor"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidConstructor"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFeeConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidFeeConfig"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidFirstOrAnchor"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidFirstOrAnchor",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidHeader"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidHeader"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidNetworkConfig"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidNetworkConfig",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidRetarget"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidRetarget"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTypeFilter"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidTypeFilter"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LE8OutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("LE8OutOfBounds"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LowLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("LowLiquidity"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LowReserve"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("LowReserve"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LowWork"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("LowWork"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MetaAnchorHeaderMissing"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MetaAnchorHeaderMissing",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MetaFirstHeaderMissing"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("MetaFirstHeaderMissing",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NeedBitcoinAmount"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NeedBitcoinAmount"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NeedDestAddress"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NeedDestAddress"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NeedDutyWindow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NeedDutyWindow"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NetworkAddressNotAllowed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NetworkAddressNotAllowed",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NetworkChangeLocked"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NetworkChangeLocked",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NetworkNotAllowed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NetworkNotAllowed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoHeadersYet"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NoHeadersYet"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoJumpWhenActive"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NoJumpWhenActive"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OracleDecimalsIncorrect"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OracleDecimalsIncorrect",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OracleZeroPrice"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OracleZeroPrice"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PrevAndTipUnmatch"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("PrevAndTipUnmatch"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProgramAlreadyUsed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ProgramAlreadyUsed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ProgramOutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ProgramOutOfBounds"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SlippageNotAllowed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SlippageNotAllowed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransactionOverflow"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TransactionOverflow",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransactionTooShort"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TransactionTooShort",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TransferFailed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("TransferFailed"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unauthorized"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Unauthorized"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnexpectedValue"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UnexpectedValue"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UserBitcoinProgramNotAllowed"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UserBitcoinProgramNotAllowed",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ValueOutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ValueOutOfBounds"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Var16OutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Var16OutOfBounds"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Var32OutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Var32OutOfBounds"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Var64OutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Var64OutOfBounds"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VarIntOutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("VarIntOutOfBounds"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("VoutOutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("VoutOutOfBounds"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("WrongConversionType"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("WrongConversionType",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ZeroValue"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ZeroValue"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static PARADAPPCONVERT_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    pub struct ParadappConvert<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ParadappConvert<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ParadappConvert<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ParadappConvert<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ParadappConvert<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ParadappConvert))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ParadappConvert<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                PARADAPPCONVERT_ABI.clone(),
                client,
            ))
        }
        ///Calls the contract's `APPROVAL_WINDOW_SEC` (0xdcd5d309) function
        pub fn approval_window_sec(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([220, 213, 211, 9], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BPS_DENOM` (0x6637e38c) function
        pub fn bps_denom(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([102, 55, 227, 140], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `BTC_DECIMALS` (0x8e32388e) function
        pub fn btc_decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([142, 50, 56, 142], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `CONFIRMATIONS_REQUIRED` (0xede42057) function
        pub fn confirmations_required(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([237, 228, 32, 87], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DEPOSIT_BLOCKS_WINDOW` (0x64f367f3) function
        pub fn deposit_blocks_window(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([100, 243, 103, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `DIFF_PERIOD` (0x969695a9) function
        pub fn diff_period(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([150, 150, 149, 169], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MAX_TIMESPAN_SEC` (0x6ebd4a13) function
        pub fn max_timespan_sec(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([110, 189, 74, 19], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `MIN_TIMESPAN_SEC` (0xd9d96f03) function
        pub fn min_timespan_sec(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([217, 217, 111, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `NATIVE_DECIMALS` (0xe66bf2d7) function
        pub fn native_decimals(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([230, 107, 242, 215], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `PROOF_BLOCKS_WINDOW` (0x4f29043b) function
        pub fn proof_blocks_window(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 41, 4, 59], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RESERVE_MARGIN_BPS` (0xf842f91e) function
        pub fn reserve_margin_bps(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([248, 66, 249, 30], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `RETARGET_PERIOD_SEC` (0x6b59dbba) function
        pub fn retarget_period_sec(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([107, 89, 219, 186], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `SELF_NETWORK_ID` (0xede4754a) function
        pub fn self_network_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([237, 228, 117, 74], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `activeOpenConversions` (0x9aa076a3) function
        pub fn active_open_conversions(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([154, 160, 118, 163], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addNativeLiquidity` (0xb238b533) function
        pub fn add_native_liquidity(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([178, 56, 181, 51], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addNetwork` (0x1e169eb7) function
        pub fn add_network(
            &self,
            network_id: ::ethers::core::types::U256,
            min_addr_len: u16,
            max_addr_len: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 22, 158, 183], (network_id, min_addr_len, max_addr_len))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `anchorInfo` (0xdcedfea3) function
        pub fn anchor_info(
            &self,
            tx_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([220, 237, 254, 163], tx_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveAndStartWithAnchorAndFirst` (0xefea725e) function
        pub fn approve_and_start_with_anchor_and_first(
            &self,
            tx_id: ::ethers::core::types::U256,
            duty_window_seconds: ::ethers::core::types::U256,
            paradapp_receive_program: ::ethers::core::types::Bytes,
            slippage: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [239, 234, 114, 94],
                    (
                        tx_id,
                        duty_window_seconds,
                        paradapp_receive_program,
                        slippage,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bitcoinUsdPriceId` (0x4f7c5ae3) function
        pub fn bitcoin_usd_price_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 124, 90, 227], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `claimNative_AfterOperatorExpired` (0x4e41fc76) function
        pub fn claim_native_after_operator_expired(
            &self,
            tx_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 65, 252, 118], tx_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `closeNoBitcoin_BitcoinToNative` (0xe3c088e9) function
        pub fn close_no_bitcoin_bitcoin_to_native(
            &self,
            tx_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 192, 136, 233], tx_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commitBitcoinToNative` (0x96b5f34c) function
        pub fn commit_bitcoin_to_native(
            &self,
            bitcoin_amount: ::ethers::core::types::U256,
            network_id: ::ethers::core::types::U256,
            user_program: ::ethers::core::types::Bytes,
            dest_address: ::ethers::core::types::Address,
            network_address: ::ethers::core::types::Bytes,
            duty_window_seconds: ::ethers::core::types::U256,
            paradapp_receive_program: ::ethers::core::types::Bytes,
            locked_anchor_height: ::ethers::core::types::U256,
            slippage: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [150, 181, 243, 76],
                    (
                        bitcoin_amount,
                        network_id,
                        user_program,
                        dest_address,
                        network_address,
                        duty_window_seconds,
                        paradapp_receive_program,
                        locked_anchor_height,
                        slippage,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commitFeeNative` (0x54213310) function
        pub fn commit_fee_native(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([84, 33, 51, 16], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commitGlobalBitcoinHeader80` (0x16118bee) function
        pub fn commit_global_bitcoin_header_80(
            &self,
            header_80: ::ethers::core::types::Bytes,
            height: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 17, 139, 238], (header_80, height))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `commitNativeToBitcoin` (0xa4cc4f8e) function
        pub fn commit_native_to_bitcoin(
            &self,
            native_amount: ::ethers::core::types::U256,
            network_id: ::ethers::core::types::U256,
            network_address: ::ethers::core::types::Bytes,
            user_program: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [164, 204, 79, 142],
                    (native_amount, network_id, network_address, user_program),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `conversions` (0x1c989390) function
        pub fn conversions(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                bool,
                u16,
                ::ethers::core::types::Bytes,
                ::ethers::core::types::Bytes,
                ::ethers::core::types::Bytes,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                bool,
                bool,
                bool,
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([28, 152, 147, 144], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `debugDecodeHeader` (0xc25b5e55) function
        pub fn debug_decode_header(
            &self,
            header_80: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32], [u8; 32], u32, u32)>
        {
            self.0
                .method_hash([194, 91, 94, 85], header_80)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `depositApprovedConversion` (0x8f242fa1) function
        pub fn deposit_approved_conversion(
            &self,
            tx_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 36, 47, 161], tx_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `estimateBitcoinFromNative` (0x70103a5b) function
        pub fn estimate_bitcoin_from_native(
            &self,
            native_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 16, 58, 91], native_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `estimateNativeFromBitcoin` (0x472b7c41) function
        pub fn estimate_native_from_bitcoin(
            &self,
            bitcoin_amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([71, 43, 124, 65], bitcoin_amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectedNext` (0x8a66b56d) function
        pub fn expected_next(
            &self,
            tx_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, ::ethers::core::types::U256, [u8; 32]),
        > {
            self.0
                .method_hash([138, 102, 181, 109], tx_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getConversionWithPhase` (0x7593a73a) function
        pub fn get_conversion_with_phase(
            &self,
            tx_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (Conversion, u8)> {
            self.0
                .method_hash([117, 147, 167, 58], tx_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getTxIdsByFilter` (0xab7ff8b0) function
        pub fn get_tx_ids_by_filter(
            &self,
            type_filter: u8,
            phase_filter: u8,
            user_filter: ::ethers::core::types::Address,
            bitcoin_program_filter: ::ethers::core::types::Bytes,
            search_user_program: bool,
            network_id_filter: ::ethers::core::types::U256,
            use_network_id_filter: bool,
            from_tx_id: ::ethers::core::types::U256,
            to_tx_id: ::ethers::core::types::U256,
            max_results: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash(
                    [171, 127, 248, 176],
                    (
                        type_filter,
                        phase_filter,
                        user_filter,
                        bitcoin_program_filter,
                        search_user_program,
                        network_id_filter,
                        use_network_id_filter,
                        from_tx_id,
                        to_tx_id,
                        max_results,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `globalHeaders` (0xd9a19d35) function
        pub fn global_headers(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32], u32, u32, bool, u64)>
        {
            self.0
                .method_hash([217, 161, 157, 53], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `globalHeightToHashLE` (0x4b271f5d) function
        pub fn global_height_to_hash_le(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([75, 39, 31, 93], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `globalTipHeight` (0xf7b21a16) function
        pub fn global_tip_height(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([247, 178, 26, 22], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `minAnchorHeight` (0x71c9502c) function
        pub fn min_anchor_height(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([113, 201, 80, 44], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nativeLiquidity` (0x06c67372) function
        pub fn native_liquidity(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([6, 198, 115, 114], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nativeUsdPriceId` (0xcb1a7d62) function
        pub fn native_usd_price_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([203, 26, 125, 98], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `networkConfigs` (0x7b99b603) function
        pub fn network_configs(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, u16, u16)> {
            self.0
                .method_hash([123, 153, 182, 3], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nextTxId` (0x8aff87b2) function
        pub fn next_tx_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([138, 255, 135, 178], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operator` (0x570ca735) function
        pub fn operator(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([87, 12, 167, 53], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proofInfo` (0x3cc5bdd9) function
        pub fn proof_info(
            &self,
            tx_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                bool,
                bool,
                u8,
                [u8; 32],
                [u8; 32],
                ::ethers::core::types::U256,
                u64,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([60, 197, 189, 217], tx_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `refundAfterNoProof_NativeToBitcoin` (0xc3f634ab) function
        pub fn refund_after_no_proof_native_to_bitcoin(
            &self,
            tx_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 246, 52, 171], tx_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `refundIfNotApproved` (0xbf1680c9) function
        pub fn refund_if_not_approved(
            &self,
            tx_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 22, 128, 201], tx_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removableNative` (0x8dd9e124) function
        pub fn removable_native(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([141, 217, 225, 36], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeNativeLiquidity` (0xf72780ef) function
        pub fn remove_native_liquidity(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 39, 128, 239], amount)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeNetwork` (0xb74d4c05) function
        pub fn remove_network(
            &self,
            network_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 77, 76, 5], network_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serviceFeeBps` (0x529c5514) function
        pub fn service_fee_bps(&self) -> ::ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([82, 156, 85, 20], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setFees` (0x26f73351) function
        pub fn set_fees(
            &self,
            new_commit_fee: ::ethers::core::types::U256,
            new_service_fee_bps: u16,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 247, 51, 81], (new_commit_fee, new_service_fee_bps))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setOperator` (0xb3ab15fb) function
        pub fn set_operator(
            &self,
            new_operator: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 171, 21, 251], new_operator)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `submitBitcoinMerkleProofWithTx` (0x9286cb25) function
        pub fn submit_bitcoin_merkle_proof_with_tx(
            &self,
            tx_id: ::ethers::core::types::U256,
            tx_raw: ::ethers::core::types::Bytes,
            vout_index: ::ethers::core::types::U256,
            block_hash_le: [u8; 32],
            block_height: ::ethers::core::types::U256,
            branch_le: ::std::vec::Vec<[u8; 32]>,
            index: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [146, 134, 203, 37],
                    (
                        tx_id,
                        tx_raw,
                        vout_index,
                        block_hash_le,
                        block_height,
                        branch_le,
                        index,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `timeoutNoDeposit_NativetoBitcoin` (0x040dc703) function
        pub fn timeout_no_deposit_nativeto_bitcoin(
            &self,
            tx_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 13, 199, 3], tx_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalHeldCommitFees` (0x1942f33e) function
        pub fn total_held_commit_fees(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([25, 66, 243, 62], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalLockedDeposits` (0xda7abe3d) function
        pub fn total_locked_deposits(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([218, 122, 190, 61], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `totalReservedNative` (0x7d6f91b6) function
        pub fn total_reserved_native(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([125, 111, 145, 182], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `usedParadappPrograms` (0x5c0510ea) function
        pub fn used_paradapp_programs(
            &self,
            p0: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([92, 5, 16, 234], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `usedProofs` (0xc30a0f25) function
        pub fn used_proofs(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([195, 10, 15, 37], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `windowsFor` (0x4fe0200b) function
        pub fn windows_for(
            &self,
            tx_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([79, 224, 32, 11], tx_id)
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ConversionApproved` event
        pub fn conversion_approved_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ConversionApprovedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ConversionCommitted` event
        pub fn conversion_committed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ConversionCommittedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ConversionCompleted` event
        pub fn conversion_completed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ConversionCompletedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ConversionDeposited` event
        pub fn conversion_deposited_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ConversionDepositedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `ConversionRefunded` event
        pub fn conversion_refunded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ConversionRefundedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `FeesUpdated` event
        pub fn fees_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, FeesUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `GlobalHeaderAppended` event
        pub fn global_header_appended_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GlobalHeaderAppendedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `LiquidityUpdated` event
        pub fn liquidity_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LiquidityUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OperatorChanged` event
        pub fn operator_changed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OperatorChangedFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ParadappConvertEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for ParadappConvert<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `AlreadyVerified` with signature `AlreadyVerified()` and selector `0x118fd7b8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "AlreadyVerified", abi = "AlreadyVerified()")]
    pub struct AlreadyVerified;
    ///Custom Error type `AnchorMustBeTip` with signature `AnchorMustBeTip()` and selector `0xe4c5fe08`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "AnchorMustBeTip", abi = "AnchorMustBeTip()")]
    pub struct AnchorMustBeTip;
    ///Custom Error type `ApproveWindowOver` with signature `ApproveWindowOver()` and selector `0xb6184819`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ApproveWindowOver", abi = "ApproveWindowOver()")]
    pub struct ApproveWindowOver;
    ///Custom Error type `BadBitcoinProgram` with signature `BadBitcoinProgram()` and selector `0x9e7cee0b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BadBitcoinProgram", abi = "BadBitcoinProgram()")]
    pub struct BadBitcoinProgram;
    ///Custom Error type `BadSlippage` with signature `BadSlippage()` and selector `0x0f4ec00b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BadSlippage", abi = "BadSlippage()")]
    pub struct BadSlippage;
    ///Custom Error type `BadState` with signature `BadState()` and selector `0x8523b62a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BadState", abi = "BadState()")]
    pub struct BadState;
    ///Custom Error type `BadTxId` with signature `BadTxId()` and selector `0x628c1f98`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "BadTxId", abi = "BadTxId()")]
    pub struct BadTxId;
    ///Custom Error type `DutyExpired` with signature `DutyExpired()` and selector `0x2784080e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "DutyExpired", abi = "DutyExpired()")]
    pub struct DutyExpired;
    ///Custom Error type `DutyNotExpired` with signature `DutyNotExpired()` and selector `0x318aac80`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "DutyNotExpired", abi = "DutyNotExpired()")]
    pub struct DutyNotExpired;
    ///Custom Error type `EpochAnchorsMissing` with signature `EpochAnchorsMissing()` and selector `0x55339168`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "EpochAnchorsMissing", abi = "EpochAnchorsMissing()")]
    pub struct EpochAnchorsMissing;
    ///Custom Error type `EpochFirstMissing` with signature `EpochFirstMissing()` and selector `0x37a17cf9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "EpochFirstMissing", abi = "EpochFirstMissing()")]
    pub struct EpochFirstMissing;
    ///Custom Error type `EpochMetaMissing` with signature `EpochMetaMissing()` and selector `0xd121194e`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "EpochMetaMissing", abi = "EpochMetaMissing()")]
    pub struct EpochMetaMissing;
    ///Custom Error type `ExceedsRemovable` with signature `ExceedsRemovable()` and selector `0xe486a93f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ExceedsRemovable", abi = "ExceedsRemovable()")]
    pub struct ExceedsRemovable;
    ///Custom Error type `GlobalAnchorMissing` with signature `GlobalAnchorMissing()` and selector `0x0880a3a2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "GlobalAnchorMissing", abi = "GlobalAnchorMissing()")]
    pub struct GlobalAnchorMissing;
    ///Custom Error type `GlobalFirstHeaderMissing` with signature `GlobalFirstHeaderMissing()` and selector `0xf17520fb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "GlobalFirstHeaderMissing", abi = "GlobalFirstHeaderMissing()")]
    pub struct GlobalFirstHeaderMissing;
    ///Custom Error type `HeaderStarted` with signature `HeaderStarted()` and selector `0x560d420d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "HeaderStarted", abi = "HeaderStarted()")]
    pub struct HeaderStarted;
    ///Custom Error type `HeightRewrite` with signature `HeightRewrite()` and selector `0x168ab990`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "HeightRewrite", abi = "HeightRewrite()")]
    pub struct HeightRewrite;
    ///Custom Error type `IncorrectCommitFee` with signature `IncorrectCommitFee()` and selector `0x41fe5560`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "IncorrectCommitFee", abi = "IncorrectCommitFee()")]
    pub struct IncorrectCommitFee;
    ///Custom Error type `IncorrectNetwork` with signature `IncorrectNetwork()` and selector `0x2b818947`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "IncorrectNetwork", abi = "IncorrectNetwork()")]
    pub struct IncorrectNetwork;
    ///Custom Error type `IncorrectNetworkAddress` with signature `IncorrectNetworkAddress()` and selector `0xfc29e7db`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "IncorrectNetworkAddress", abi = "IncorrectNetworkAddress()")]
    pub struct IncorrectNetworkAddress;
    ///Custom Error type `IncorrectValue` with signature `IncorrectValue()` and selector `0xd2ade556`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "IncorrectValue", abi = "IncorrectValue()")]
    pub struct IncorrectValue;
    ///Custom Error type `IncorrectWindow` with signature `IncorrectWindow()` and selector `0x1073b42f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "IncorrectWindow", abi = "IncorrectWindow()")]
    pub struct IncorrectWindow;
    ///Custom Error type `InvalidAnchorHeight` with signature `InvalidAnchorHeight()` and selector `0xcf720669`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidAnchorHeight", abi = "InvalidAnchorHeight()")]
    pub struct InvalidAnchorHeight;
    ///Custom Error type `InvalidConstructor` with signature `InvalidConstructor()` and selector `0x12c1701b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidConstructor", abi = "InvalidConstructor()")]
    pub struct InvalidConstructor;
    ///Custom Error type `InvalidFeeConfig` with signature `InvalidFeeConfig()` and selector `0x5601467a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidFeeConfig", abi = "InvalidFeeConfig()")]
    pub struct InvalidFeeConfig;
    ///Custom Error type `InvalidFirstOrAnchor` with signature `InvalidFirstOrAnchor()` and selector `0x8d54dc22`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidFirstOrAnchor", abi = "InvalidFirstOrAnchor()")]
    pub struct InvalidFirstOrAnchor;
    ///Custom Error type `InvalidHeader` with signature `InvalidHeader()` and selector `0xbabb01dd`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidHeader", abi = "InvalidHeader()")]
    pub struct InvalidHeader;
    ///Custom Error type `InvalidNetworkConfig` with signature `InvalidNetworkConfig()` and selector `0xcddd174b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidNetworkConfig", abi = "InvalidNetworkConfig()")]
    pub struct InvalidNetworkConfig;
    ///Custom Error type `InvalidRetarget` with signature `InvalidRetarget()` and selector `0x19effc57`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidRetarget", abi = "InvalidRetarget()")]
    pub struct InvalidRetarget;
    ///Custom Error type `InvalidTypeFilter` with signature `InvalidTypeFilter()` and selector `0xda68d9c0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidTypeFilter", abi = "InvalidTypeFilter()")]
    pub struct InvalidTypeFilter;
    ///Custom Error type `LE8OutOfBounds` with signature `LE8OutOfBounds()` and selector `0xfd5520b5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "LE8OutOfBounds", abi = "LE8OutOfBounds()")]
    pub struct LE8OutOfBounds;
    ///Custom Error type `LowLiquidity` with signature `LowLiquidity()` and selector `0x878f5b02`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "LowLiquidity", abi = "LowLiquidity()")]
    pub struct LowLiquidity;
    ///Custom Error type `LowReserve` with signature `LowReserve()` and selector `0x1d516e47`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "LowReserve", abi = "LowReserve()")]
    pub struct LowReserve;
    ///Custom Error type `LowWork` with signature `LowWork()` and selector `0x0ca34bf8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "LowWork", abi = "LowWork()")]
    pub struct LowWork;
    ///Custom Error type `MetaAnchorHeaderMissing` with signature `MetaAnchorHeaderMissing()` and selector `0x0845d9a1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "MetaAnchorHeaderMissing", abi = "MetaAnchorHeaderMissing()")]
    pub struct MetaAnchorHeaderMissing;
    ///Custom Error type `MetaFirstHeaderMissing` with signature `MetaFirstHeaderMissing()` and selector `0x4020e29b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "MetaFirstHeaderMissing", abi = "MetaFirstHeaderMissing()")]
    pub struct MetaFirstHeaderMissing;
    ///Custom Error type `NeedBitcoinAmount` with signature `NeedBitcoinAmount()` and selector `0x98ce0dfc`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NeedBitcoinAmount", abi = "NeedBitcoinAmount()")]
    pub struct NeedBitcoinAmount;
    ///Custom Error type `NeedDestAddress` with signature `NeedDestAddress()` and selector `0x479e460f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NeedDestAddress", abi = "NeedDestAddress()")]
    pub struct NeedDestAddress;
    ///Custom Error type `NeedDutyWindow` with signature `NeedDutyWindow()` and selector `0xf3791326`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NeedDutyWindow", abi = "NeedDutyWindow()")]
    pub struct NeedDutyWindow;
    ///Custom Error type `NetworkAddressNotAllowed` with signature `NetworkAddressNotAllowed()` and selector `0xcf4210a1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NetworkAddressNotAllowed", abi = "NetworkAddressNotAllowed()")]
    pub struct NetworkAddressNotAllowed;
    ///Custom Error type `NetworkChangeLocked` with signature `NetworkChangeLocked()` and selector `0xa93c9b64`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NetworkChangeLocked", abi = "NetworkChangeLocked()")]
    pub struct NetworkChangeLocked;
    ///Custom Error type `NetworkNotAllowed` with signature `NetworkNotAllowed()` and selector `0x2b821981`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NetworkNotAllowed", abi = "NetworkNotAllowed()")]
    pub struct NetworkNotAllowed;
    ///Custom Error type `NoHeadersYet` with signature `NoHeadersYet()` and selector `0x2cb2e46f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NoHeadersYet", abi = "NoHeadersYet()")]
    pub struct NoHeadersYet;
    ///Custom Error type `NoJumpWhenActive` with signature `NoJumpWhenActive()` and selector `0x486d1dde`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NoJumpWhenActive", abi = "NoJumpWhenActive()")]
    pub struct NoJumpWhenActive;
    ///Custom Error type `OracleDecimalsIncorrect` with signature `OracleDecimalsIncorrect()` and selector `0x6efa3b03`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "OracleDecimalsIncorrect", abi = "OracleDecimalsIncorrect()")]
    pub struct OracleDecimalsIncorrect;
    ///Custom Error type `OracleZeroPrice` with signature `OracleZeroPrice()` and selector `0x4b77fd3c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "OracleZeroPrice", abi = "OracleZeroPrice()")]
    pub struct OracleZeroPrice;
    ///Custom Error type `PrevAndTipUnmatch` with signature `PrevAndTipUnmatch()` and selector `0x3f3c533b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "PrevAndTipUnmatch", abi = "PrevAndTipUnmatch()")]
    pub struct PrevAndTipUnmatch;
    ///Custom Error type `ProgramAlreadyUsed` with signature `ProgramAlreadyUsed()` and selector `0xfaefc6fe`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ProgramAlreadyUsed", abi = "ProgramAlreadyUsed()")]
    pub struct ProgramAlreadyUsed;
    ///Custom Error type `ProgramOutOfBounds` with signature `ProgramOutOfBounds()` and selector `0x565ba85f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ProgramOutOfBounds", abi = "ProgramOutOfBounds()")]
    pub struct ProgramOutOfBounds;
    ///Custom Error type `SlippageNotAllowed` with signature `SlippageNotAllowed()` and selector `0x55286ed2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "SlippageNotAllowed", abi = "SlippageNotAllowed()")]
    pub struct SlippageNotAllowed;
    ///Custom Error type `TransactionOverflow` with signature `TransactionOverflow()` and selector `0xdff7dbbf`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TransactionOverflow", abi = "TransactionOverflow()")]
    pub struct TransactionOverflow;
    ///Custom Error type `TransactionTooShort` with signature `TransactionTooShort()` and selector `0x309173ca`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TransactionTooShort", abi = "TransactionTooShort()")]
    pub struct TransactionTooShort;
    ///Custom Error type `TransferFailed` with signature `TransferFailed()` and selector `0x90b8ec18`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "TransferFailed", abi = "TransferFailed()")]
    pub struct TransferFailed;
    ///Custom Error type `Unauthorized` with signature `Unauthorized()` and selector `0x82b42900`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Unauthorized", abi = "Unauthorized()")]
    pub struct Unauthorized;
    ///Custom Error type `UnexpectedValue` with signature `UnexpectedValue()` and selector `0x123146a6`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "UnexpectedValue", abi = "UnexpectedValue()")]
    pub struct UnexpectedValue;
    ///Custom Error type `UserBitcoinProgramNotAllowed` with signature `UserBitcoinProgramNotAllowed()` and selector `0x672ebceb`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "UserBitcoinProgramNotAllowed",
        abi = "UserBitcoinProgramNotAllowed()"
    )]
    pub struct UserBitcoinProgramNotAllowed;
    ///Custom Error type `ValueOutOfBounds` with signature `ValueOutOfBounds()` and selector `0xda9a94ce`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ValueOutOfBounds", abi = "ValueOutOfBounds()")]
    pub struct ValueOutOfBounds;
    ///Custom Error type `Var16OutOfBounds` with signature `Var16OutOfBounds()` and selector `0x457105c3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Var16OutOfBounds", abi = "Var16OutOfBounds()")]
    pub struct Var16OutOfBounds;
    ///Custom Error type `Var32OutOfBounds` with signature `Var32OutOfBounds()` and selector `0x1edd678d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Var32OutOfBounds", abi = "Var32OutOfBounds()")]
    pub struct Var32OutOfBounds;
    ///Custom Error type `Var64OutOfBounds` with signature `Var64OutOfBounds()` and selector `0xb4d5f8b3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Var64OutOfBounds", abi = "Var64OutOfBounds()")]
    pub struct Var64OutOfBounds;
    ///Custom Error type `VarIntOutOfBounds` with signature `VarIntOutOfBounds()` and selector `0xc6c11d57`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "VarIntOutOfBounds", abi = "VarIntOutOfBounds()")]
    pub struct VarIntOutOfBounds;
    ///Custom Error type `VoutOutOfBounds` with signature `VoutOutOfBounds()` and selector `0x234c32ef`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "VoutOutOfBounds", abi = "VoutOutOfBounds()")]
    pub struct VoutOutOfBounds;
    ///Custom Error type `WrongConversionType` with signature `WrongConversionType()` and selector `0x174eb6d9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "WrongConversionType", abi = "WrongConversionType()")]
    pub struct WrongConversionType;
    ///Custom Error type `ZeroValue` with signature `ZeroValue()` and selector `0x7c946ed7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ZeroValue", abi = "ZeroValue()")]
    pub struct ZeroValue;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ParadappConvertErrors {
        AlreadyVerified(AlreadyVerified),
        AnchorMustBeTip(AnchorMustBeTip),
        ApproveWindowOver(ApproveWindowOver),
        BadBitcoinProgram(BadBitcoinProgram),
        BadSlippage(BadSlippage),
        BadState(BadState),
        BadTxId(BadTxId),
        DutyExpired(DutyExpired),
        DutyNotExpired(DutyNotExpired),
        EpochAnchorsMissing(EpochAnchorsMissing),
        EpochFirstMissing(EpochFirstMissing),
        EpochMetaMissing(EpochMetaMissing),
        ExceedsRemovable(ExceedsRemovable),
        GlobalAnchorMissing(GlobalAnchorMissing),
        GlobalFirstHeaderMissing(GlobalFirstHeaderMissing),
        HeaderStarted(HeaderStarted),
        HeightRewrite(HeightRewrite),
        IncorrectCommitFee(IncorrectCommitFee),
        IncorrectNetwork(IncorrectNetwork),
        IncorrectNetworkAddress(IncorrectNetworkAddress),
        IncorrectValue(IncorrectValue),
        IncorrectWindow(IncorrectWindow),
        InvalidAnchorHeight(InvalidAnchorHeight),
        InvalidConstructor(InvalidConstructor),
        InvalidFeeConfig(InvalidFeeConfig),
        InvalidFirstOrAnchor(InvalidFirstOrAnchor),
        InvalidHeader(InvalidHeader),
        InvalidNetworkConfig(InvalidNetworkConfig),
        InvalidRetarget(InvalidRetarget),
        InvalidTypeFilter(InvalidTypeFilter),
        LE8OutOfBounds(LE8OutOfBounds),
        LowLiquidity(LowLiquidity),
        LowReserve(LowReserve),
        LowWork(LowWork),
        MetaAnchorHeaderMissing(MetaAnchorHeaderMissing),
        MetaFirstHeaderMissing(MetaFirstHeaderMissing),
        NeedBitcoinAmount(NeedBitcoinAmount),
        NeedDestAddress(NeedDestAddress),
        NeedDutyWindow(NeedDutyWindow),
        NetworkAddressNotAllowed(NetworkAddressNotAllowed),
        NetworkChangeLocked(NetworkChangeLocked),
        NetworkNotAllowed(NetworkNotAllowed),
        NoHeadersYet(NoHeadersYet),
        NoJumpWhenActive(NoJumpWhenActive),
        OracleDecimalsIncorrect(OracleDecimalsIncorrect),
        OracleZeroPrice(OracleZeroPrice),
        PrevAndTipUnmatch(PrevAndTipUnmatch),
        ProgramAlreadyUsed(ProgramAlreadyUsed),
        ProgramOutOfBounds(ProgramOutOfBounds),
        SlippageNotAllowed(SlippageNotAllowed),
        TransactionOverflow(TransactionOverflow),
        TransactionTooShort(TransactionTooShort),
        TransferFailed(TransferFailed),
        Unauthorized(Unauthorized),
        UnexpectedValue(UnexpectedValue),
        UserBitcoinProgramNotAllowed(UserBitcoinProgramNotAllowed),
        ValueOutOfBounds(ValueOutOfBounds),
        Var16OutOfBounds(Var16OutOfBounds),
        Var32OutOfBounds(Var32OutOfBounds),
        Var64OutOfBounds(Var64OutOfBounds),
        VarIntOutOfBounds(VarIntOutOfBounds),
        VoutOutOfBounds(VoutOutOfBounds),
        WrongConversionType(WrongConversionType),
        ZeroValue(ZeroValue),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ParadappConvertErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AlreadyVerified as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AlreadyVerified(decoded));
            }
            if let Ok(decoded) = <AnchorMustBeTip as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AnchorMustBeTip(decoded));
            }
            if let Ok(decoded) = <ApproveWindowOver as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ApproveWindowOver(decoded));
            }
            if let Ok(decoded) = <BadBitcoinProgram as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BadBitcoinProgram(decoded));
            }
            if let Ok(decoded) = <BadSlippage as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BadSlippage(decoded));
            }
            if let Ok(decoded) = <BadState as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BadState(decoded));
            }
            if let Ok(decoded) = <BadTxId as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BadTxId(decoded));
            }
            if let Ok(decoded) = <DutyExpired as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DutyExpired(decoded));
            }
            if let Ok(decoded) = <DutyNotExpired as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DutyNotExpired(decoded));
            }
            if let Ok(decoded) =
                <EpochAnchorsMissing as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EpochAnchorsMissing(decoded));
            }
            if let Ok(decoded) = <EpochFirstMissing as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EpochFirstMissing(decoded));
            }
            if let Ok(decoded) = <EpochMetaMissing as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EpochMetaMissing(decoded));
            }
            if let Ok(decoded) = <ExceedsRemovable as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExceedsRemovable(decoded));
            }
            if let Ok(decoded) =
                <GlobalAnchorMissing as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GlobalAnchorMissing(decoded));
            }
            if let Ok(decoded) =
                <GlobalFirstHeaderMissing as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GlobalFirstHeaderMissing(decoded));
            }
            if let Ok(decoded) = <HeaderStarted as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HeaderStarted(decoded));
            }
            if let Ok(decoded) = <HeightRewrite as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::HeightRewrite(decoded));
            }
            if let Ok(decoded) =
                <IncorrectCommitFee as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncorrectCommitFee(decoded));
            }
            if let Ok(decoded) = <IncorrectNetwork as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncorrectNetwork(decoded));
            }
            if let Ok(decoded) =
                <IncorrectNetworkAddress as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IncorrectNetworkAddress(decoded));
            }
            if let Ok(decoded) = <IncorrectValue as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IncorrectValue(decoded));
            }
            if let Ok(decoded) = <IncorrectWindow as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IncorrectWindow(decoded));
            }
            if let Ok(decoded) =
                <InvalidAnchorHeight as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidAnchorHeight(decoded));
            }
            if let Ok(decoded) =
                <InvalidConstructor as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidConstructor(decoded));
            }
            if let Ok(decoded) = <InvalidFeeConfig as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidFeeConfig(decoded));
            }
            if let Ok(decoded) =
                <InvalidFirstOrAnchor as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidFirstOrAnchor(decoded));
            }
            if let Ok(decoded) = <InvalidHeader as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidHeader(decoded));
            }
            if let Ok(decoded) =
                <InvalidNetworkConfig as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidNetworkConfig(decoded));
            }
            if let Ok(decoded) = <InvalidRetarget as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidRetarget(decoded));
            }
            if let Ok(decoded) = <InvalidTypeFilter as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidTypeFilter(decoded));
            }
            if let Ok(decoded) = <LE8OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LE8OutOfBounds(decoded));
            }
            if let Ok(decoded) = <LowLiquidity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LowLiquidity(decoded));
            }
            if let Ok(decoded) = <LowReserve as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LowReserve(decoded));
            }
            if let Ok(decoded) = <LowWork as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LowWork(decoded));
            }
            if let Ok(decoded) =
                <MetaAnchorHeaderMissing as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MetaAnchorHeaderMissing(decoded));
            }
            if let Ok(decoded) =
                <MetaFirstHeaderMissing as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MetaFirstHeaderMissing(decoded));
            }
            if let Ok(decoded) = <NeedBitcoinAmount as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NeedBitcoinAmount(decoded));
            }
            if let Ok(decoded) = <NeedDestAddress as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NeedDestAddress(decoded));
            }
            if let Ok(decoded) = <NeedDutyWindow as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NeedDutyWindow(decoded));
            }
            if let Ok(decoded) =
                <NetworkAddressNotAllowed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NetworkAddressNotAllowed(decoded));
            }
            if let Ok(decoded) =
                <NetworkChangeLocked as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NetworkChangeLocked(decoded));
            }
            if let Ok(decoded) = <NetworkNotAllowed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NetworkNotAllowed(decoded));
            }
            if let Ok(decoded) = <NoHeadersYet as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoHeadersYet(decoded));
            }
            if let Ok(decoded) = <NoJumpWhenActive as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NoJumpWhenActive(decoded));
            }
            if let Ok(decoded) =
                <OracleDecimalsIncorrect as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OracleDecimalsIncorrect(decoded));
            }
            if let Ok(decoded) = <OracleZeroPrice as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OracleZeroPrice(decoded));
            }
            if let Ok(decoded) = <PrevAndTipUnmatch as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrevAndTipUnmatch(decoded));
            }
            if let Ok(decoded) =
                <ProgramAlreadyUsed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProgramAlreadyUsed(decoded));
            }
            if let Ok(decoded) =
                <ProgramOutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProgramOutOfBounds(decoded));
            }
            if let Ok(decoded) =
                <SlippageNotAllowed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SlippageNotAllowed(decoded));
            }
            if let Ok(decoded) =
                <TransactionOverflow as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransactionOverflow(decoded));
            }
            if let Ok(decoded) =
                <TransactionTooShort as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransactionTooShort(decoded));
            }
            if let Ok(decoded) = <TransferFailed as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TransferFailed(decoded));
            }
            if let Ok(decoded) = <Unauthorized as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unauthorized(decoded));
            }
            if let Ok(decoded) = <UnexpectedValue as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UnexpectedValue(decoded));
            }
            if let Ok(decoded) =
                <UserBitcoinProgramNotAllowed as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UserBitcoinProgramNotAllowed(decoded));
            }
            if let Ok(decoded) = <ValueOutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValueOutOfBounds(decoded));
            }
            if let Ok(decoded) = <Var16OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Var16OutOfBounds(decoded));
            }
            if let Ok(decoded) = <Var32OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Var32OutOfBounds(decoded));
            }
            if let Ok(decoded) = <Var64OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::Var64OutOfBounds(decoded));
            }
            if let Ok(decoded) = <VarIntOutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VarIntOutOfBounds(decoded));
            }
            if let Ok(decoded) = <VoutOutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::VoutOutOfBounds(decoded));
            }
            if let Ok(decoded) =
                <WrongConversionType as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::WrongConversionType(decoded));
            }
            if let Ok(decoded) = <ZeroValue as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ZeroValue(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ParadappConvertErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AlreadyVerified(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AnchorMustBeTip(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ApproveWindowOver(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BadBitcoinProgram(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BadSlippage(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BadState(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BadTxId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DutyExpired(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DutyNotExpired(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EpochAnchorsMissing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EpochFirstMissing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::EpochMetaMissing(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExceedsRemovable(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GlobalAnchorMissing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GlobalFirstHeaderMissing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HeaderStarted(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::HeightRewrite(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncorrectCommitFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncorrectNetwork(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncorrectNetworkAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncorrectValue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IncorrectWindow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidAnchorHeight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidConstructor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidFeeConfig(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidFirstOrAnchor(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidHeader(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidNetworkConfig(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidRetarget(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidTypeFilter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LE8OutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LowLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LowReserve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LowWork(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MetaAnchorHeaderMissing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MetaFirstHeaderMissing(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NeedBitcoinAmount(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NeedDestAddress(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NeedDutyWindow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NetworkAddressNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NetworkChangeLocked(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NetworkNotAllowed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NoHeadersYet(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NoJumpWhenActive(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OracleDecimalsIncorrect(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OracleZeroPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PrevAndTipUnmatch(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProgramAlreadyUsed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProgramOutOfBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SlippageNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransactionOverflow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransactionTooShort(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferFailed(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Unauthorized(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UnexpectedValue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::UserBitcoinProgramNotAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValueOutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Var16OutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Var32OutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Var64OutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VarIntOutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VoutOutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WrongConversionType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ZeroValue(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ParadappConvertErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AlreadyVerified as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <AnchorMustBeTip as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ApproveWindowOver as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <BadBitcoinProgram as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <BadSlippage as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <BadState as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <BadTxId as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <DutyExpired as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <DutyNotExpired as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EpochAnchorsMissing as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <EpochFirstMissing as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <EpochMetaMissing as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <ExceedsRemovable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <GlobalAnchorMissing as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <GlobalFirstHeaderMissing as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <HeaderStarted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <HeightRewrite as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <IncorrectCommitFee as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <IncorrectNetwork as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <IncorrectNetworkAddress as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <IncorrectValue as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <IncorrectWindow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidAnchorHeight as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InvalidConstructor as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidFeeConfig as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidFirstOrAnchor as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidHeader as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidNetworkConfig as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidRetarget as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTypeFilter as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <LE8OutOfBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <LowLiquidity as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <LowReserve as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <LowWork as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <MetaAnchorHeaderMissing as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <MetaFirstHeaderMissing as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <NeedBitcoinAmount as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <NeedDestAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NeedDutyWindow as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NetworkAddressNotAllowed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <NetworkChangeLocked as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <NetworkNotAllowed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <NoHeadersYet as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NoJumpWhenActive as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OracleDecimalsIncorrect as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <OracleZeroPrice as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PrevAndTipUnmatch as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ProgramAlreadyUsed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <ProgramOutOfBounds as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <SlippageNotAllowed as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <TransactionOverflow as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <TransactionTooShort as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <TransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Unauthorized as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <UnexpectedValue as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UserBitcoinProgramNotAllowed as ::ethers::contract::EthError>::selector(
                    ) =>
                {
                    true
                }
                _ if selector == <ValueOutOfBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Var16OutOfBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Var32OutOfBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Var64OutOfBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <VarIntOutOfBounds as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <VoutOutOfBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <WrongConversionType as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <ZeroValue as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ParadappConvertErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AlreadyVerified(element) => ::core::fmt::Display::fmt(element, f),
                Self::AnchorMustBeTip(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveWindowOver(element) => ::core::fmt::Display::fmt(element, f),
                Self::BadBitcoinProgram(element) => ::core::fmt::Display::fmt(element, f),
                Self::BadSlippage(element) => ::core::fmt::Display::fmt(element, f),
                Self::BadState(element) => ::core::fmt::Display::fmt(element, f),
                Self::BadTxId(element) => ::core::fmt::Display::fmt(element, f),
                Self::DutyExpired(element) => ::core::fmt::Display::fmt(element, f),
                Self::DutyNotExpired(element) => ::core::fmt::Display::fmt(element, f),
                Self::EpochAnchorsMissing(element) => ::core::fmt::Display::fmt(element, f),
                Self::EpochFirstMissing(element) => ::core::fmt::Display::fmt(element, f),
                Self::EpochMetaMissing(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExceedsRemovable(element) => ::core::fmt::Display::fmt(element, f),
                Self::GlobalAnchorMissing(element) => ::core::fmt::Display::fmt(element, f),
                Self::GlobalFirstHeaderMissing(element) => ::core::fmt::Display::fmt(element, f),
                Self::HeaderStarted(element) => ::core::fmt::Display::fmt(element, f),
                Self::HeightRewrite(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncorrectCommitFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncorrectNetwork(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncorrectNetworkAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncorrectValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncorrectWindow(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidAnchorHeight(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidConstructor(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidFeeConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidFirstOrAnchor(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidHeader(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidNetworkConfig(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidRetarget(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidTypeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LE8OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::LowLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::LowReserve(element) => ::core::fmt::Display::fmt(element, f),
                Self::LowWork(element) => ::core::fmt::Display::fmt(element, f),
                Self::MetaAnchorHeaderMissing(element) => ::core::fmt::Display::fmt(element, f),
                Self::MetaFirstHeaderMissing(element) => ::core::fmt::Display::fmt(element, f),
                Self::NeedBitcoinAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::NeedDestAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::NeedDutyWindow(element) => ::core::fmt::Display::fmt(element, f),
                Self::NetworkAddressNotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::NetworkChangeLocked(element) => ::core::fmt::Display::fmt(element, f),
                Self::NetworkNotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoHeadersYet(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoJumpWhenActive(element) => ::core::fmt::Display::fmt(element, f),
                Self::OracleDecimalsIncorrect(element) => ::core::fmt::Display::fmt(element, f),
                Self::OracleZeroPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrevAndTipUnmatch(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProgramAlreadyUsed(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProgramOutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::SlippageNotAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransactionOverflow(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransactionTooShort(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::Unauthorized(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnexpectedValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::UserBitcoinProgramNotAllowed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValueOutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::Var16OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::Var32OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::Var64OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::VarIntOutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::VoutOutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::WrongConversionType(element) => ::core::fmt::Display::fmt(element, f),
                Self::ZeroValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ParadappConvertErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AlreadyVerified> for ParadappConvertErrors {
        fn from(value: AlreadyVerified) -> Self {
            Self::AlreadyVerified(value)
        }
    }
    impl ::core::convert::From<AnchorMustBeTip> for ParadappConvertErrors {
        fn from(value: AnchorMustBeTip) -> Self {
            Self::AnchorMustBeTip(value)
        }
    }
    impl ::core::convert::From<ApproveWindowOver> for ParadappConvertErrors {
        fn from(value: ApproveWindowOver) -> Self {
            Self::ApproveWindowOver(value)
        }
    }
    impl ::core::convert::From<BadBitcoinProgram> for ParadappConvertErrors {
        fn from(value: BadBitcoinProgram) -> Self {
            Self::BadBitcoinProgram(value)
        }
    }
    impl ::core::convert::From<BadSlippage> for ParadappConvertErrors {
        fn from(value: BadSlippage) -> Self {
            Self::BadSlippage(value)
        }
    }
    impl ::core::convert::From<BadState> for ParadappConvertErrors {
        fn from(value: BadState) -> Self {
            Self::BadState(value)
        }
    }
    impl ::core::convert::From<BadTxId> for ParadappConvertErrors {
        fn from(value: BadTxId) -> Self {
            Self::BadTxId(value)
        }
    }
    impl ::core::convert::From<DutyExpired> for ParadappConvertErrors {
        fn from(value: DutyExpired) -> Self {
            Self::DutyExpired(value)
        }
    }
    impl ::core::convert::From<DutyNotExpired> for ParadappConvertErrors {
        fn from(value: DutyNotExpired) -> Self {
            Self::DutyNotExpired(value)
        }
    }
    impl ::core::convert::From<EpochAnchorsMissing> for ParadappConvertErrors {
        fn from(value: EpochAnchorsMissing) -> Self {
            Self::EpochAnchorsMissing(value)
        }
    }
    impl ::core::convert::From<EpochFirstMissing> for ParadappConvertErrors {
        fn from(value: EpochFirstMissing) -> Self {
            Self::EpochFirstMissing(value)
        }
    }
    impl ::core::convert::From<EpochMetaMissing> for ParadappConvertErrors {
        fn from(value: EpochMetaMissing) -> Self {
            Self::EpochMetaMissing(value)
        }
    }
    impl ::core::convert::From<ExceedsRemovable> for ParadappConvertErrors {
        fn from(value: ExceedsRemovable) -> Self {
            Self::ExceedsRemovable(value)
        }
    }
    impl ::core::convert::From<GlobalAnchorMissing> for ParadappConvertErrors {
        fn from(value: GlobalAnchorMissing) -> Self {
            Self::GlobalAnchorMissing(value)
        }
    }
    impl ::core::convert::From<GlobalFirstHeaderMissing> for ParadappConvertErrors {
        fn from(value: GlobalFirstHeaderMissing) -> Self {
            Self::GlobalFirstHeaderMissing(value)
        }
    }
    impl ::core::convert::From<HeaderStarted> for ParadappConvertErrors {
        fn from(value: HeaderStarted) -> Self {
            Self::HeaderStarted(value)
        }
    }
    impl ::core::convert::From<HeightRewrite> for ParadappConvertErrors {
        fn from(value: HeightRewrite) -> Self {
            Self::HeightRewrite(value)
        }
    }
    impl ::core::convert::From<IncorrectCommitFee> for ParadappConvertErrors {
        fn from(value: IncorrectCommitFee) -> Self {
            Self::IncorrectCommitFee(value)
        }
    }
    impl ::core::convert::From<IncorrectNetwork> for ParadappConvertErrors {
        fn from(value: IncorrectNetwork) -> Self {
            Self::IncorrectNetwork(value)
        }
    }
    impl ::core::convert::From<IncorrectNetworkAddress> for ParadappConvertErrors {
        fn from(value: IncorrectNetworkAddress) -> Self {
            Self::IncorrectNetworkAddress(value)
        }
    }
    impl ::core::convert::From<IncorrectValue> for ParadappConvertErrors {
        fn from(value: IncorrectValue) -> Self {
            Self::IncorrectValue(value)
        }
    }
    impl ::core::convert::From<IncorrectWindow> for ParadappConvertErrors {
        fn from(value: IncorrectWindow) -> Self {
            Self::IncorrectWindow(value)
        }
    }
    impl ::core::convert::From<InvalidAnchorHeight> for ParadappConvertErrors {
        fn from(value: InvalidAnchorHeight) -> Self {
            Self::InvalidAnchorHeight(value)
        }
    }
    impl ::core::convert::From<InvalidConstructor> for ParadappConvertErrors {
        fn from(value: InvalidConstructor) -> Self {
            Self::InvalidConstructor(value)
        }
    }
    impl ::core::convert::From<InvalidFeeConfig> for ParadappConvertErrors {
        fn from(value: InvalidFeeConfig) -> Self {
            Self::InvalidFeeConfig(value)
        }
    }
    impl ::core::convert::From<InvalidFirstOrAnchor> for ParadappConvertErrors {
        fn from(value: InvalidFirstOrAnchor) -> Self {
            Self::InvalidFirstOrAnchor(value)
        }
    }
    impl ::core::convert::From<InvalidHeader> for ParadappConvertErrors {
        fn from(value: InvalidHeader) -> Self {
            Self::InvalidHeader(value)
        }
    }
    impl ::core::convert::From<InvalidNetworkConfig> for ParadappConvertErrors {
        fn from(value: InvalidNetworkConfig) -> Self {
            Self::InvalidNetworkConfig(value)
        }
    }
    impl ::core::convert::From<InvalidRetarget> for ParadappConvertErrors {
        fn from(value: InvalidRetarget) -> Self {
            Self::InvalidRetarget(value)
        }
    }
    impl ::core::convert::From<InvalidTypeFilter> for ParadappConvertErrors {
        fn from(value: InvalidTypeFilter) -> Self {
            Self::InvalidTypeFilter(value)
        }
    }
    impl ::core::convert::From<LE8OutOfBounds> for ParadappConvertErrors {
        fn from(value: LE8OutOfBounds) -> Self {
            Self::LE8OutOfBounds(value)
        }
    }
    impl ::core::convert::From<LowLiquidity> for ParadappConvertErrors {
        fn from(value: LowLiquidity) -> Self {
            Self::LowLiquidity(value)
        }
    }
    impl ::core::convert::From<LowReserve> for ParadappConvertErrors {
        fn from(value: LowReserve) -> Self {
            Self::LowReserve(value)
        }
    }
    impl ::core::convert::From<LowWork> for ParadappConvertErrors {
        fn from(value: LowWork) -> Self {
            Self::LowWork(value)
        }
    }
    impl ::core::convert::From<MetaAnchorHeaderMissing> for ParadappConvertErrors {
        fn from(value: MetaAnchorHeaderMissing) -> Self {
            Self::MetaAnchorHeaderMissing(value)
        }
    }
    impl ::core::convert::From<MetaFirstHeaderMissing> for ParadappConvertErrors {
        fn from(value: MetaFirstHeaderMissing) -> Self {
            Self::MetaFirstHeaderMissing(value)
        }
    }
    impl ::core::convert::From<NeedBitcoinAmount> for ParadappConvertErrors {
        fn from(value: NeedBitcoinAmount) -> Self {
            Self::NeedBitcoinAmount(value)
        }
    }
    impl ::core::convert::From<NeedDestAddress> for ParadappConvertErrors {
        fn from(value: NeedDestAddress) -> Self {
            Self::NeedDestAddress(value)
        }
    }
    impl ::core::convert::From<NeedDutyWindow> for ParadappConvertErrors {
        fn from(value: NeedDutyWindow) -> Self {
            Self::NeedDutyWindow(value)
        }
    }
    impl ::core::convert::From<NetworkAddressNotAllowed> for ParadappConvertErrors {
        fn from(value: NetworkAddressNotAllowed) -> Self {
            Self::NetworkAddressNotAllowed(value)
        }
    }
    impl ::core::convert::From<NetworkChangeLocked> for ParadappConvertErrors {
        fn from(value: NetworkChangeLocked) -> Self {
            Self::NetworkChangeLocked(value)
        }
    }
    impl ::core::convert::From<NetworkNotAllowed> for ParadappConvertErrors {
        fn from(value: NetworkNotAllowed) -> Self {
            Self::NetworkNotAllowed(value)
        }
    }
    impl ::core::convert::From<NoHeadersYet> for ParadappConvertErrors {
        fn from(value: NoHeadersYet) -> Self {
            Self::NoHeadersYet(value)
        }
    }
    impl ::core::convert::From<NoJumpWhenActive> for ParadappConvertErrors {
        fn from(value: NoJumpWhenActive) -> Self {
            Self::NoJumpWhenActive(value)
        }
    }
    impl ::core::convert::From<OracleDecimalsIncorrect> for ParadappConvertErrors {
        fn from(value: OracleDecimalsIncorrect) -> Self {
            Self::OracleDecimalsIncorrect(value)
        }
    }
    impl ::core::convert::From<OracleZeroPrice> for ParadappConvertErrors {
        fn from(value: OracleZeroPrice) -> Self {
            Self::OracleZeroPrice(value)
        }
    }
    impl ::core::convert::From<PrevAndTipUnmatch> for ParadappConvertErrors {
        fn from(value: PrevAndTipUnmatch) -> Self {
            Self::PrevAndTipUnmatch(value)
        }
    }
    impl ::core::convert::From<ProgramAlreadyUsed> for ParadappConvertErrors {
        fn from(value: ProgramAlreadyUsed) -> Self {
            Self::ProgramAlreadyUsed(value)
        }
    }
    impl ::core::convert::From<ProgramOutOfBounds> for ParadappConvertErrors {
        fn from(value: ProgramOutOfBounds) -> Self {
            Self::ProgramOutOfBounds(value)
        }
    }
    impl ::core::convert::From<SlippageNotAllowed> for ParadappConvertErrors {
        fn from(value: SlippageNotAllowed) -> Self {
            Self::SlippageNotAllowed(value)
        }
    }
    impl ::core::convert::From<TransactionOverflow> for ParadappConvertErrors {
        fn from(value: TransactionOverflow) -> Self {
            Self::TransactionOverflow(value)
        }
    }
    impl ::core::convert::From<TransactionTooShort> for ParadappConvertErrors {
        fn from(value: TransactionTooShort) -> Self {
            Self::TransactionTooShort(value)
        }
    }
    impl ::core::convert::From<TransferFailed> for ParadappConvertErrors {
        fn from(value: TransferFailed) -> Self {
            Self::TransferFailed(value)
        }
    }
    impl ::core::convert::From<Unauthorized> for ParadappConvertErrors {
        fn from(value: Unauthorized) -> Self {
            Self::Unauthorized(value)
        }
    }
    impl ::core::convert::From<UnexpectedValue> for ParadappConvertErrors {
        fn from(value: UnexpectedValue) -> Self {
            Self::UnexpectedValue(value)
        }
    }
    impl ::core::convert::From<UserBitcoinProgramNotAllowed> for ParadappConvertErrors {
        fn from(value: UserBitcoinProgramNotAllowed) -> Self {
            Self::UserBitcoinProgramNotAllowed(value)
        }
    }
    impl ::core::convert::From<ValueOutOfBounds> for ParadappConvertErrors {
        fn from(value: ValueOutOfBounds) -> Self {
            Self::ValueOutOfBounds(value)
        }
    }
    impl ::core::convert::From<Var16OutOfBounds> for ParadappConvertErrors {
        fn from(value: Var16OutOfBounds) -> Self {
            Self::Var16OutOfBounds(value)
        }
    }
    impl ::core::convert::From<Var32OutOfBounds> for ParadappConvertErrors {
        fn from(value: Var32OutOfBounds) -> Self {
            Self::Var32OutOfBounds(value)
        }
    }
    impl ::core::convert::From<Var64OutOfBounds> for ParadappConvertErrors {
        fn from(value: Var64OutOfBounds) -> Self {
            Self::Var64OutOfBounds(value)
        }
    }
    impl ::core::convert::From<VarIntOutOfBounds> for ParadappConvertErrors {
        fn from(value: VarIntOutOfBounds) -> Self {
            Self::VarIntOutOfBounds(value)
        }
    }
    impl ::core::convert::From<VoutOutOfBounds> for ParadappConvertErrors {
        fn from(value: VoutOutOfBounds) -> Self {
            Self::VoutOutOfBounds(value)
        }
    }
    impl ::core::convert::From<WrongConversionType> for ParadappConvertErrors {
        fn from(value: WrongConversionType) -> Self {
            Self::WrongConversionType(value)
        }
    }
    impl ::core::convert::From<ZeroValue> for ParadappConvertErrors {
        fn from(value: ZeroValue) -> Self {
            Self::ZeroValue(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "ConversionApproved",
        abi = "ConversionApproved(uint256,uint256,uint256,bytes32)"
    )]
    pub struct ConversionApprovedFilter {
        #[ethevent(indexed)]
        pub tx_id: ::ethers::core::types::U256,
        pub duty_window_seconds: ::ethers::core::types::U256,
        pub first_height: ::ethers::core::types::U256,
        pub first_header_hash_le: [u8; 32],
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "ConversionCommitted",
        abi = "ConversionCommitted(uint256,address,bool)"
    )]
    pub struct ConversionCommittedFilter {
        #[ethevent(indexed)]
        pub tx_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub user: ::ethers::core::types::Address,
        pub is_nativeto_bitcoin: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "ConversionCompleted", abi = "ConversionCompleted(uint256)")]
    pub struct ConversionCompletedFilter {
        #[ethevent(indexed)]
        pub tx_id: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "ConversionDeposited",
        abi = "ConversionDeposited(uint256,uint256)"
    )]
    pub struct ConversionDepositedFilter {
        #[ethevent(indexed)]
        pub tx_id: ::ethers::core::types::U256,
        pub native_amount: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "ConversionRefunded",
        abi = "ConversionRefunded(uint256,uint256,bool)"
    )]
    pub struct ConversionRefundedFilter {
        #[ethevent(indexed)]
        pub tx_id: ::ethers::core::types::U256,
        pub refund_native: ::ethers::core::types::U256,
        pub commit_fee_refunded: bool,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "FeesUpdated", abi = "FeesUpdated(uint256,uint16)")]
    pub struct FeesUpdatedFilter {
        pub new_commit_fee: ::ethers::core::types::U256,
        pub new_service_fee_bps: u16,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "GlobalHeaderAppended",
        abi = "GlobalHeaderAppended(uint256,bytes32,bytes32,bytes32,uint32,uint32)"
    )]
    pub struct GlobalHeaderAppendedFilter {
        pub height: ::ethers::core::types::U256,
        pub hash_le: [u8; 32],
        pub prev_hash_le: [u8; 32],
        pub merkle_root_le: [u8; 32],
        pub n_bits: u32,
        pub timestamp: u32,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "LiquidityUpdated", abi = "LiquidityUpdated(uint256)")]
    pub struct LiquidityUpdatedFilter {
        pub native_liquidity: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "OperatorChanged", abi = "OperatorChanged(address)")]
    pub struct OperatorChangedFilter {
        pub new_operator: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ParadappConvertEvents {
        ConversionApprovedFilter(ConversionApprovedFilter),
        ConversionCommittedFilter(ConversionCommittedFilter),
        ConversionCompletedFilter(ConversionCompletedFilter),
        ConversionDepositedFilter(ConversionDepositedFilter),
        ConversionRefundedFilter(ConversionRefundedFilter),
        FeesUpdatedFilter(FeesUpdatedFilter),
        GlobalHeaderAppendedFilter(GlobalHeaderAppendedFilter),
        LiquidityUpdatedFilter(LiquidityUpdatedFilter),
        OperatorChangedFilter(OperatorChangedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ParadappConvertEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ConversionApprovedFilter::decode_log(log) {
                return Ok(ParadappConvertEvents::ConversionApprovedFilter(decoded));
            }
            if let Ok(decoded) = ConversionCommittedFilter::decode_log(log) {
                return Ok(ParadappConvertEvents::ConversionCommittedFilter(decoded));
            }
            if let Ok(decoded) = ConversionCompletedFilter::decode_log(log) {
                return Ok(ParadappConvertEvents::ConversionCompletedFilter(decoded));
            }
            if let Ok(decoded) = ConversionDepositedFilter::decode_log(log) {
                return Ok(ParadappConvertEvents::ConversionDepositedFilter(decoded));
            }
            if let Ok(decoded) = ConversionRefundedFilter::decode_log(log) {
                return Ok(ParadappConvertEvents::ConversionRefundedFilter(decoded));
            }
            if let Ok(decoded) = FeesUpdatedFilter::decode_log(log) {
                return Ok(ParadappConvertEvents::FeesUpdatedFilter(decoded));
            }
            if let Ok(decoded) = GlobalHeaderAppendedFilter::decode_log(log) {
                return Ok(ParadappConvertEvents::GlobalHeaderAppendedFilter(decoded));
            }
            if let Ok(decoded) = LiquidityUpdatedFilter::decode_log(log) {
                return Ok(ParadappConvertEvents::LiquidityUpdatedFilter(decoded));
            }
            if let Ok(decoded) = OperatorChangedFilter::decode_log(log) {
                return Ok(ParadappConvertEvents::OperatorChangedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ParadappConvertEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConversionApprovedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConversionCommittedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConversionCompletedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConversionDepositedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConversionRefundedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeesUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GlobalHeaderAppendedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidityUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorChangedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ConversionApprovedFilter> for ParadappConvertEvents {
        fn from(value: ConversionApprovedFilter) -> Self {
            Self::ConversionApprovedFilter(value)
        }
    }
    impl ::core::convert::From<ConversionCommittedFilter> for ParadappConvertEvents {
        fn from(value: ConversionCommittedFilter) -> Self {
            Self::ConversionCommittedFilter(value)
        }
    }
    impl ::core::convert::From<ConversionCompletedFilter> for ParadappConvertEvents {
        fn from(value: ConversionCompletedFilter) -> Self {
            Self::ConversionCompletedFilter(value)
        }
    }
    impl ::core::convert::From<ConversionDepositedFilter> for ParadappConvertEvents {
        fn from(value: ConversionDepositedFilter) -> Self {
            Self::ConversionDepositedFilter(value)
        }
    }
    impl ::core::convert::From<ConversionRefundedFilter> for ParadappConvertEvents {
        fn from(value: ConversionRefundedFilter) -> Self {
            Self::ConversionRefundedFilter(value)
        }
    }
    impl ::core::convert::From<FeesUpdatedFilter> for ParadappConvertEvents {
        fn from(value: FeesUpdatedFilter) -> Self {
            Self::FeesUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<GlobalHeaderAppendedFilter> for ParadappConvertEvents {
        fn from(value: GlobalHeaderAppendedFilter) -> Self {
            Self::GlobalHeaderAppendedFilter(value)
        }
    }
    impl ::core::convert::From<LiquidityUpdatedFilter> for ParadappConvertEvents {
        fn from(value: LiquidityUpdatedFilter) -> Self {
            Self::LiquidityUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<OperatorChangedFilter> for ParadappConvertEvents {
        fn from(value: OperatorChangedFilter) -> Self {
            Self::OperatorChangedFilter(value)
        }
    }
    ///Container type for all input parameters for the `APPROVAL_WINDOW_SEC` function with signature `APPROVAL_WINDOW_SEC()` and selector `0xdcd5d309`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "APPROVAL_WINDOW_SEC", abi = "APPROVAL_WINDOW_SEC()")]
    pub struct ApprovalWindowSecCall;
    ///Container type for all input parameters for the `BPS_DENOM` function with signature `BPS_DENOM()` and selector `0x6637e38c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "BPS_DENOM", abi = "BPS_DENOM()")]
    pub struct BpsDenomCall;
    ///Container type for all input parameters for the `BTC_DECIMALS` function with signature `BTC_DECIMALS()` and selector `0x8e32388e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "BTC_DECIMALS", abi = "BTC_DECIMALS()")]
    pub struct BtcDecimalsCall;
    ///Container type for all input parameters for the `CONFIRMATIONS_REQUIRED` function with signature `CONFIRMATIONS_REQUIRED()` and selector `0xede42057`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "CONFIRMATIONS_REQUIRED", abi = "CONFIRMATIONS_REQUIRED()")]
    pub struct ConfirmationsRequiredCall;
    ///Container type for all input parameters for the `DEPOSIT_BLOCKS_WINDOW` function with signature `DEPOSIT_BLOCKS_WINDOW()` and selector `0x64f367f3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "DEPOSIT_BLOCKS_WINDOW", abi = "DEPOSIT_BLOCKS_WINDOW()")]
    pub struct DepositBlocksWindowCall;
    ///Container type for all input parameters for the `DIFF_PERIOD` function with signature `DIFF_PERIOD()` and selector `0x969695a9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "DIFF_PERIOD", abi = "DIFF_PERIOD()")]
    pub struct DiffPeriodCall;
    ///Container type for all input parameters for the `MAX_TIMESPAN_SEC` function with signature `MAX_TIMESPAN_SEC()` and selector `0x6ebd4a13`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "MAX_TIMESPAN_SEC", abi = "MAX_TIMESPAN_SEC()")]
    pub struct MaxTimespanSecCall;
    ///Container type for all input parameters for the `MIN_TIMESPAN_SEC` function with signature `MIN_TIMESPAN_SEC()` and selector `0xd9d96f03`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "MIN_TIMESPAN_SEC", abi = "MIN_TIMESPAN_SEC()")]
    pub struct MinTimespanSecCall;
    ///Container type for all input parameters for the `NATIVE_DECIMALS` function with signature `NATIVE_DECIMALS()` and selector `0xe66bf2d7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "NATIVE_DECIMALS", abi = "NATIVE_DECIMALS()")]
    pub struct NativeDecimalsCall;
    ///Container type for all input parameters for the `PROOF_BLOCKS_WINDOW` function with signature `PROOF_BLOCKS_WINDOW()` and selector `0x4f29043b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "PROOF_BLOCKS_WINDOW", abi = "PROOF_BLOCKS_WINDOW()")]
    pub struct ProofBlocksWindowCall;
    ///Container type for all input parameters for the `RESERVE_MARGIN_BPS` function with signature `RESERVE_MARGIN_BPS()` and selector `0xf842f91e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "RESERVE_MARGIN_BPS", abi = "RESERVE_MARGIN_BPS()")]
    pub struct ReserveMarginBpsCall;
    ///Container type for all input parameters for the `RETARGET_PERIOD_SEC` function with signature `RETARGET_PERIOD_SEC()` and selector `0x6b59dbba`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "RETARGET_PERIOD_SEC", abi = "RETARGET_PERIOD_SEC()")]
    pub struct RetargetPeriodSecCall;
    ///Container type for all input parameters for the `SELF_NETWORK_ID` function with signature `SELF_NETWORK_ID()` and selector `0xede4754a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "SELF_NETWORK_ID", abi = "SELF_NETWORK_ID()")]
    pub struct SelfNetworkIdCall;
    ///Container type for all input parameters for the `activeOpenConversions` function with signature `activeOpenConversions()` and selector `0x9aa076a3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "activeOpenConversions", abi = "activeOpenConversions()")]
    pub struct ActiveOpenConversionsCall;
    ///Container type for all input parameters for the `addNativeLiquidity` function with signature `addNativeLiquidity()` and selector `0xb238b533`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "addNativeLiquidity", abi = "addNativeLiquidity()")]
    pub struct AddNativeLiquidityCall;
    ///Container type for all input parameters for the `addNetwork` function with signature `addNetwork(uint256,uint16,uint16)` and selector `0x1e169eb7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "addNetwork", abi = "addNetwork(uint256,uint16,uint16)")]
    pub struct AddNetworkCall {
        pub network_id: ::ethers::core::types::U256,
        pub min_addr_len: u16,
        pub max_addr_len: u16,
    }
    ///Container type for all input parameters for the `anchorInfo` function with signature `anchorInfo(uint256)` and selector `0xdcedfea3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "anchorInfo", abi = "anchorInfo(uint256)")]
    pub struct AnchorInfoCall {
        pub tx_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `approveAndStartWithAnchorAndFirst` function with signature `approveAndStartWithAnchorAndFirst(uint256,uint256,bytes,uint16)` and selector `0xefea725e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "approveAndStartWithAnchorAndFirst",
        abi = "approveAndStartWithAnchorAndFirst(uint256,uint256,bytes,uint16)"
    )]
    pub struct ApproveAndStartWithAnchorAndFirstCall {
        pub tx_id: ::ethers::core::types::U256,
        pub duty_window_seconds: ::ethers::core::types::U256,
        pub paradapp_receive_program: ::ethers::core::types::Bytes,
        pub slippage: u16,
    }
    ///Container type for all input parameters for the `bitcoinUsdPriceId` function with signature `bitcoinUsdPriceId()` and selector `0x4f7c5ae3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "bitcoinUsdPriceId", abi = "bitcoinUsdPriceId()")]
    pub struct BitcoinUsdPriceIdCall;
    ///Container type for all input parameters for the `claimNative_AfterOperatorExpired` function with signature `claimNative_AfterOperatorExpired(uint256)` and selector `0x4e41fc76`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "claimNative_AfterOperatorExpired",
        abi = "claimNative_AfterOperatorExpired(uint256)"
    )]
    pub struct ClaimNativeAfterOperatorExpiredCall {
        pub tx_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `closeNoBitcoin_BitcoinToNative` function with signature `closeNoBitcoin_BitcoinToNative(uint256)` and selector `0xe3c088e9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "closeNoBitcoin_BitcoinToNative",
        abi = "closeNoBitcoin_BitcoinToNative(uint256)"
    )]
    pub struct CloseNoBitcoinBitcoinToNativeCall {
        pub tx_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `commitBitcoinToNative` function with signature `commitBitcoinToNative(uint256,uint256,bytes,address,bytes,uint256,bytes,uint256,uint16)` and selector `0x96b5f34c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "commitBitcoinToNative",
        abi = "commitBitcoinToNative(uint256,uint256,bytes,address,bytes,uint256,bytes,uint256,uint16)"
    )]
    pub struct CommitBitcoinToNativeCall {
        pub bitcoin_amount: ::ethers::core::types::U256,
        pub network_id: ::ethers::core::types::U256,
        pub user_program: ::ethers::core::types::Bytes,
        pub dest_address: ::ethers::core::types::Address,
        pub network_address: ::ethers::core::types::Bytes,
        pub duty_window_seconds: ::ethers::core::types::U256,
        pub paradapp_receive_program: ::ethers::core::types::Bytes,
        pub locked_anchor_height: ::ethers::core::types::U256,
        pub slippage: u16,
    }
    ///Container type for all input parameters for the `commitFeeNative` function with signature `commitFeeNative()` and selector `0x54213310`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "commitFeeNative", abi = "commitFeeNative()")]
    pub struct CommitFeeNativeCall;
    ///Container type for all input parameters for the `commitGlobalBitcoinHeader80` function with signature `commitGlobalBitcoinHeader80(bytes,uint256)` and selector `0x16118bee`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "commitGlobalBitcoinHeader80",
        abi = "commitGlobalBitcoinHeader80(bytes,uint256)"
    )]
    pub struct CommitGlobalBitcoinHeader80Call {
        pub header_80: ::ethers::core::types::Bytes,
        pub height: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `commitNativeToBitcoin` function with signature `commitNativeToBitcoin(uint256,uint256,bytes,bytes)` and selector `0xa4cc4f8e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "commitNativeToBitcoin",
        abi = "commitNativeToBitcoin(uint256,uint256,bytes,bytes)"
    )]
    pub struct CommitNativeToBitcoinCall {
        pub native_amount: ::ethers::core::types::U256,
        pub network_id: ::ethers::core::types::U256,
        pub network_address: ::ethers::core::types::Bytes,
        pub user_program: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `conversions` function with signature `conversions(uint256)` and selector `0x1c989390`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "conversions", abi = "conversions(uint256)")]
    pub struct ConversionsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `debugDecodeHeader` function with signature `debugDecodeHeader(bytes)` and selector `0xc25b5e55`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "debugDecodeHeader", abi = "debugDecodeHeader(bytes)")]
    pub struct DebugDecodeHeaderCall {
        pub header_80: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `depositApprovedConversion` function with signature `depositApprovedConversion(uint256)` and selector `0x8f242fa1`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "depositApprovedConversion",
        abi = "depositApprovedConversion(uint256)"
    )]
    pub struct DepositApprovedConversionCall {
        pub tx_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `estimateBitcoinFromNative` function with signature `estimateBitcoinFromNative(uint256)` and selector `0x70103a5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "estimateBitcoinFromNative",
        abi = "estimateBitcoinFromNative(uint256)"
    )]
    pub struct EstimateBitcoinFromNativeCall {
        pub native_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `estimateNativeFromBitcoin` function with signature `estimateNativeFromBitcoin(uint256)` and selector `0x472b7c41`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "estimateNativeFromBitcoin",
        abi = "estimateNativeFromBitcoin(uint256)"
    )]
    pub struct EstimateNativeFromBitcoinCall {
        pub bitcoin_amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `expectedNext` function with signature `expectedNext(uint256)` and selector `0x8a66b56d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "expectedNext", abi = "expectedNext(uint256)")]
    pub struct ExpectedNextCall {
        pub tx_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getConversionWithPhase` function with signature `getConversionWithPhase(uint256)` and selector `0x7593a73a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getConversionWithPhase",
        abi = "getConversionWithPhase(uint256)"
    )]
    pub struct GetConversionWithPhaseCall {
        pub tx_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getTxIdsByFilter` function with signature `getTxIdsByFilter(uint8,uint8,address,bytes,bool,uint256,bool,uint256,uint256,uint256)` and selector `0xab7ff8b0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "getTxIdsByFilter",
        abi = "getTxIdsByFilter(uint8,uint8,address,bytes,bool,uint256,bool,uint256,uint256,uint256)"
    )]
    pub struct GetTxIdsByFilterCall {
        pub type_filter: u8,
        pub phase_filter: u8,
        pub user_filter: ::ethers::core::types::Address,
        pub bitcoin_program_filter: ::ethers::core::types::Bytes,
        pub search_user_program: bool,
        pub network_id_filter: ::ethers::core::types::U256,
        pub use_network_id_filter: bool,
        pub from_tx_id: ::ethers::core::types::U256,
        pub to_tx_id: ::ethers::core::types::U256,
        pub max_results: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `globalHeaders` function with signature `globalHeaders(bytes32)` and selector `0xd9a19d35`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "globalHeaders", abi = "globalHeaders(bytes32)")]
    pub struct GlobalHeadersCall(pub [u8; 32]);
    ///Container type for all input parameters for the `globalHeightToHashLE` function with signature `globalHeightToHashLE(uint256)` and selector `0x4b271f5d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "globalHeightToHashLE", abi = "globalHeightToHashLE(uint256)")]
    pub struct GlobalHeightToHashLECall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `globalTipHeight` function with signature `globalTipHeight()` and selector `0xf7b21a16`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "globalTipHeight", abi = "globalTipHeight()")]
    pub struct GlobalTipHeightCall;
    ///Container type for all input parameters for the `minAnchorHeight` function with signature `minAnchorHeight()` and selector `0x71c9502c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "minAnchorHeight", abi = "minAnchorHeight()")]
    pub struct MinAnchorHeightCall;
    ///Container type for all input parameters for the `nativeLiquidity` function with signature `nativeLiquidity()` and selector `0x06c67372`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "nativeLiquidity", abi = "nativeLiquidity()")]
    pub struct NativeLiquidityCall;
    ///Container type for all input parameters for the `nativeUsdPriceId` function with signature `nativeUsdPriceId()` and selector `0xcb1a7d62`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "nativeUsdPriceId", abi = "nativeUsdPriceId()")]
    pub struct NativeUsdPriceIdCall;
    ///Container type for all input parameters for the `networkConfigs` function with signature `networkConfigs(uint256)` and selector `0x7b99b603`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "networkConfigs", abi = "networkConfigs(uint256)")]
    pub struct NetworkConfigsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `nextTxId` function with signature `nextTxId()` and selector `0x8aff87b2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "nextTxId", abi = "nextTxId()")]
    pub struct NextTxIdCall;
    ///Container type for all input parameters for the `operator` function with signature `operator()` and selector `0x570ca735`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "operator", abi = "operator()")]
    pub struct OperatorCall;
    ///Container type for all input parameters for the `proofInfo` function with signature `proofInfo(uint256)` and selector `0x3cc5bdd9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "proofInfo", abi = "proofInfo(uint256)")]
    pub struct ProofInfoCall {
        pub tx_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `refundAfterNoProof_NativeToBitcoin` function with signature `refundAfterNoProof_NativeToBitcoin(uint256)` and selector `0xc3f634ab`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "refundAfterNoProof_NativeToBitcoin",
        abi = "refundAfterNoProof_NativeToBitcoin(uint256)"
    )]
    pub struct RefundAfterNoProofNativeToBitcoinCall {
        pub tx_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `refundIfNotApproved` function with signature `refundIfNotApproved(uint256)` and selector `0xbf1680c9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "refundIfNotApproved", abi = "refundIfNotApproved(uint256)")]
    pub struct RefundIfNotApprovedCall {
        pub tx_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removableNative` function with signature `removableNative()` and selector `0x8dd9e124`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "removableNative", abi = "removableNative()")]
    pub struct RemovableNativeCall;
    ///Container type for all input parameters for the `removeNativeLiquidity` function with signature `removeNativeLiquidity(uint256)` and selector `0xf72780ef`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "removeNativeLiquidity", abi = "removeNativeLiquidity(uint256)")]
    pub struct RemoveNativeLiquidityCall {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `removeNetwork` function with signature `removeNetwork(uint256)` and selector `0xb74d4c05`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "removeNetwork", abi = "removeNetwork(uint256)")]
    pub struct RemoveNetworkCall {
        pub network_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `serviceFeeBps` function with signature `serviceFeeBps()` and selector `0x529c5514`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "serviceFeeBps", abi = "serviceFeeBps()")]
    pub struct ServiceFeeBpsCall;
    ///Container type for all input parameters for the `setFees` function with signature `setFees(uint256,uint16)` and selector `0x26f73351`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setFees", abi = "setFees(uint256,uint16)")]
    pub struct SetFeesCall {
        pub new_commit_fee: ::ethers::core::types::U256,
        pub new_service_fee_bps: u16,
    }
    ///Container type for all input parameters for the `setOperator` function with signature `setOperator(address)` and selector `0xb3ab15fb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "setOperator", abi = "setOperator(address)")]
    pub struct SetOperatorCall {
        pub new_operator: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `submitBitcoinMerkleProofWithTx` function with signature `submitBitcoinMerkleProofWithTx(uint256,bytes,uint256,bytes32,uint256,bytes32[],uint256)` and selector `0x9286cb25`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "submitBitcoinMerkleProofWithTx",
        abi = "submitBitcoinMerkleProofWithTx(uint256,bytes,uint256,bytes32,uint256,bytes32[],uint256)"
    )]
    pub struct SubmitBitcoinMerkleProofWithTxCall {
        pub tx_id: ::ethers::core::types::U256,
        pub tx_raw: ::ethers::core::types::Bytes,
        pub vout_index: ::ethers::core::types::U256,
        pub block_hash_le: [u8; 32],
        pub block_height: ::ethers::core::types::U256,
        pub branch_le: ::std::vec::Vec<[u8; 32]>,
        pub index: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `timeoutNoDeposit_NativetoBitcoin` function with signature `timeoutNoDeposit_NativetoBitcoin(uint256)` and selector `0x040dc703`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "timeoutNoDeposit_NativetoBitcoin",
        abi = "timeoutNoDeposit_NativetoBitcoin(uint256)"
    )]
    pub struct TimeoutNoDepositNativetoBitcoinCall {
        pub tx_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `totalHeldCommitFees` function with signature `totalHeldCommitFees()` and selector `0x1942f33e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "totalHeldCommitFees", abi = "totalHeldCommitFees()")]
    pub struct TotalHeldCommitFeesCall;
    ///Container type for all input parameters for the `totalLockedDeposits` function with signature `totalLockedDeposits()` and selector `0xda7abe3d`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "totalLockedDeposits", abi = "totalLockedDeposits()")]
    pub struct TotalLockedDepositsCall;
    ///Container type for all input parameters for the `totalReservedNative` function with signature `totalReservedNative()` and selector `0x7d6f91b6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "totalReservedNative", abi = "totalReservedNative()")]
    pub struct TotalReservedNativeCall;
    ///Container type for all input parameters for the `usedParadappPrograms` function with signature `usedParadappPrograms(bytes)` and selector `0x5c0510ea`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "usedParadappPrograms", abi = "usedParadappPrograms(bytes)")]
    pub struct UsedParadappProgramsCall(pub ::ethers::core::types::Bytes);
    ///Container type for all input parameters for the `usedProofs` function with signature `usedProofs(bytes32)` and selector `0xc30a0f25`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "usedProofs", abi = "usedProofs(bytes32)")]
    pub struct UsedProofsCall(pub [u8; 32]);
    ///Container type for all input parameters for the `windowsFor` function with signature `windowsFor(uint256)` and selector `0x4fe0200b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "windowsFor", abi = "windowsFor(uint256)")]
    pub struct WindowsForCall {
        pub tx_id: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ParadappConvertCalls {
        ApprovalWindowSec(ApprovalWindowSecCall),
        BpsDenom(BpsDenomCall),
        BtcDecimals(BtcDecimalsCall),
        ConfirmationsRequired(ConfirmationsRequiredCall),
        DepositBlocksWindow(DepositBlocksWindowCall),
        DiffPeriod(DiffPeriodCall),
        MaxTimespanSec(MaxTimespanSecCall),
        MinTimespanSec(MinTimespanSecCall),
        NativeDecimals(NativeDecimalsCall),
        ProofBlocksWindow(ProofBlocksWindowCall),
        ReserveMarginBps(ReserveMarginBpsCall),
        RetargetPeriodSec(RetargetPeriodSecCall),
        SelfNetworkId(SelfNetworkIdCall),
        ActiveOpenConversions(ActiveOpenConversionsCall),
        AddNativeLiquidity(AddNativeLiquidityCall),
        AddNetwork(AddNetworkCall),
        AnchorInfo(AnchorInfoCall),
        ApproveAndStartWithAnchorAndFirst(ApproveAndStartWithAnchorAndFirstCall),
        BitcoinUsdPriceId(BitcoinUsdPriceIdCall),
        ClaimNativeAfterOperatorExpired(ClaimNativeAfterOperatorExpiredCall),
        CloseNoBitcoinBitcoinToNative(CloseNoBitcoinBitcoinToNativeCall),
        CommitBitcoinToNative(CommitBitcoinToNativeCall),
        CommitFeeNative(CommitFeeNativeCall),
        CommitGlobalBitcoinHeader80(CommitGlobalBitcoinHeader80Call),
        CommitNativeToBitcoin(CommitNativeToBitcoinCall),
        Conversions(ConversionsCall),
        DebugDecodeHeader(DebugDecodeHeaderCall),
        DepositApprovedConversion(DepositApprovedConversionCall),
        EstimateBitcoinFromNative(EstimateBitcoinFromNativeCall),
        EstimateNativeFromBitcoin(EstimateNativeFromBitcoinCall),
        ExpectedNext(ExpectedNextCall),
        GetConversionWithPhase(GetConversionWithPhaseCall),
        GetTxIdsByFilter(GetTxIdsByFilterCall),
        GlobalHeaders(GlobalHeadersCall),
        GlobalHeightToHashLE(GlobalHeightToHashLECall),
        GlobalTipHeight(GlobalTipHeightCall),
        MinAnchorHeight(MinAnchorHeightCall),
        NativeLiquidity(NativeLiquidityCall),
        NativeUsdPriceId(NativeUsdPriceIdCall),
        NetworkConfigs(NetworkConfigsCall),
        NextTxId(NextTxIdCall),
        Operator(OperatorCall),
        ProofInfo(ProofInfoCall),
        RefundAfterNoProofNativeToBitcoin(RefundAfterNoProofNativeToBitcoinCall),
        RefundIfNotApproved(RefundIfNotApprovedCall),
        RemovableNative(RemovableNativeCall),
        RemoveNativeLiquidity(RemoveNativeLiquidityCall),
        RemoveNetwork(RemoveNetworkCall),
        ServiceFeeBps(ServiceFeeBpsCall),
        SetFees(SetFeesCall),
        SetOperator(SetOperatorCall),
        SubmitBitcoinMerkleProofWithTx(SubmitBitcoinMerkleProofWithTxCall),
        TimeoutNoDepositNativetoBitcoin(TimeoutNoDepositNativetoBitcoinCall),
        TotalHeldCommitFees(TotalHeldCommitFeesCall),
        TotalLockedDeposits(TotalLockedDepositsCall),
        TotalReservedNative(TotalReservedNativeCall),
        UsedParadappPrograms(UsedParadappProgramsCall),
        UsedProofs(UsedProofsCall),
        WindowsFor(WindowsForCall),
    }
    impl ::ethers::core::abi::AbiDecode for ParadappConvertCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ApprovalWindowSecCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ApprovalWindowSec(decoded));
            }
            if let Ok(decoded) = <BpsDenomCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BpsDenom(decoded));
            }
            if let Ok(decoded) = <BtcDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BtcDecimals(decoded));
            }
            if let Ok(decoded) =
                <ConfirmationsRequiredCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ConfirmationsRequired(decoded));
            }
            if let Ok(decoded) =
                <DepositBlocksWindowCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DepositBlocksWindow(decoded));
            }
            if let Ok(decoded) = <DiffPeriodCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DiffPeriod(decoded));
            }
            if let Ok(decoded) =
                <MaxTimespanSecCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxTimespanSec(decoded));
            }
            if let Ok(decoded) =
                <MinTimespanSecCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MinTimespanSec(decoded));
            }
            if let Ok(decoded) =
                <NativeDecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NativeDecimals(decoded));
            }
            if let Ok(decoded) =
                <ProofBlocksWindowCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProofBlocksWindow(decoded));
            }
            if let Ok(decoded) =
                <ReserveMarginBpsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ReserveMarginBps(decoded));
            }
            if let Ok(decoded) =
                <RetargetPeriodSecCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RetargetPeriodSec(decoded));
            }
            if let Ok(decoded) = <SelfNetworkIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SelfNetworkId(decoded));
            }
            if let Ok(decoded) =
                <ActiveOpenConversionsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ActiveOpenConversions(decoded));
            }
            if let Ok(decoded) =
                <AddNativeLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AddNativeLiquidity(decoded));
            }
            if let Ok(decoded) = <AddNetworkCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AddNetwork(decoded));
            }
            if let Ok(decoded) = <AnchorInfoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AnchorInfo(decoded));
            }
            if let Ok(decoded) =
                <ApproveAndStartWithAnchorAndFirstCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ApproveAndStartWithAnchorAndFirst(decoded));
            }
            if let Ok(decoded) =
                <BitcoinUsdPriceIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BitcoinUsdPriceId(decoded));
            }
            if let Ok(decoded) =
                <ClaimNativeAfterOperatorExpiredCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::ClaimNativeAfterOperatorExpired(decoded));
            }
            if let Ok(decoded) =
                <CloseNoBitcoinBitcoinToNativeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CloseNoBitcoinBitcoinToNative(decoded));
            }
            if let Ok(decoded) =
                <CommitBitcoinToNativeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CommitBitcoinToNative(decoded));
            }
            if let Ok(decoded) =
                <CommitFeeNativeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CommitFeeNative(decoded));
            }
            if let Ok(decoded) =
                <CommitGlobalBitcoinHeader80Call as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CommitGlobalBitcoinHeader80(decoded));
            }
            if let Ok(decoded) =
                <CommitNativeToBitcoinCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CommitNativeToBitcoin(decoded));
            }
            if let Ok(decoded) = <ConversionsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Conversions(decoded));
            }
            if let Ok(decoded) =
                <DebugDecodeHeaderCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DebugDecodeHeader(decoded));
            }
            if let Ok(decoded) =
                <DepositApprovedConversionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DepositApprovedConversion(decoded));
            }
            if let Ok(decoded) =
                <EstimateBitcoinFromNativeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EstimateBitcoinFromNative(decoded));
            }
            if let Ok(decoded) =
                <EstimateNativeFromBitcoinCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::EstimateNativeFromBitcoin(decoded));
            }
            if let Ok(decoded) = <ExpectedNextCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ExpectedNext(decoded));
            }
            if let Ok(decoded) =
                <GetConversionWithPhaseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetConversionWithPhase(decoded));
            }
            if let Ok(decoded) =
                <GetTxIdsByFilterCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetTxIdsByFilter(decoded));
            }
            if let Ok(decoded) = <GlobalHeadersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GlobalHeaders(decoded));
            }
            if let Ok(decoded) =
                <GlobalHeightToHashLECall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GlobalHeightToHashLE(decoded));
            }
            if let Ok(decoded) =
                <GlobalTipHeightCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GlobalTipHeight(decoded));
            }
            if let Ok(decoded) =
                <MinAnchorHeightCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MinAnchorHeight(decoded));
            }
            if let Ok(decoded) =
                <NativeLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NativeLiquidity(decoded));
            }
            if let Ok(decoded) =
                <NativeUsdPriceIdCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NativeUsdPriceId(decoded));
            }
            if let Ok(decoded) =
                <NetworkConfigsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NetworkConfigs(decoded));
            }
            if let Ok(decoded) = <NextTxIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NextTxId(decoded));
            }
            if let Ok(decoded) = <OperatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Operator(decoded));
            }
            if let Ok(decoded) = <ProofInfoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ProofInfo(decoded));
            }
            if let Ok(decoded) =
                <RefundAfterNoProofNativeToBitcoinCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::RefundAfterNoProofNativeToBitcoin(decoded));
            }
            if let Ok(decoded) =
                <RefundIfNotApprovedCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RefundIfNotApproved(decoded));
            }
            if let Ok(decoded) =
                <RemovableNativeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemovableNative(decoded));
            }
            if let Ok(decoded) =
                <RemoveNativeLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveNativeLiquidity(decoded));
            }
            if let Ok(decoded) = <RemoveNetworkCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RemoveNetwork(decoded));
            }
            if let Ok(decoded) = <ServiceFeeBpsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ServiceFeeBps(decoded));
            }
            if let Ok(decoded) = <SetFeesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetFees(decoded));
            }
            if let Ok(decoded) = <SetOperatorCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetOperator(decoded));
            }
            if let Ok(decoded) =
                <SubmitBitcoinMerkleProofWithTxCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SubmitBitcoinMerkleProofWithTx(decoded));
            }
            if let Ok(decoded) =
                <TimeoutNoDepositNativetoBitcoinCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::TimeoutNoDepositNativetoBitcoin(decoded));
            }
            if let Ok(decoded) =
                <TotalHeldCommitFeesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalHeldCommitFees(decoded));
            }
            if let Ok(decoded) =
                <TotalLockedDepositsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalLockedDeposits(decoded));
            }
            if let Ok(decoded) =
                <TotalReservedNativeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TotalReservedNative(decoded));
            }
            if let Ok(decoded) =
                <UsedParadappProgramsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UsedParadappPrograms(decoded));
            }
            if let Ok(decoded) = <UsedProofsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::UsedProofs(decoded));
            }
            if let Ok(decoded) = <WindowsForCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WindowsFor(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ParadappConvertCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ApprovalWindowSec(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BpsDenom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BtcDecimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ConfirmationsRequired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DepositBlocksWindow(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DiffPeriod(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxTimespanSec(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MinTimespanSec(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NativeDecimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProofBlocksWindow(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ReserveMarginBps(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RetargetPeriodSec(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SelfNetworkId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ActiveOpenConversions(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddNativeLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AddNetwork(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AnchorInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ApproveAndStartWithAnchorAndFirst(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BitcoinUsdPriceId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ClaimNativeAfterOperatorExpired(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CloseNoBitcoinBitcoinToNative(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CommitBitcoinToNative(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CommitFeeNative(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CommitGlobalBitcoinHeader80(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CommitNativeToBitcoin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Conversions(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DebugDecodeHeader(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DepositApprovedConversion(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EstimateBitcoinFromNative(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EstimateNativeFromBitcoin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExpectedNext(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetConversionWithPhase(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetTxIdsByFilter(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GlobalHeaders(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GlobalHeightToHashLE(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GlobalTipHeight(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MinAnchorHeight(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NativeLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NativeUsdPriceId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NetworkConfigs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NextTxId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Operator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProofInfo(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RefundAfterNoProofNativeToBitcoin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RefundIfNotApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemovableNative(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RemoveNativeLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RemoveNetwork(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ServiceFeeBps(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetFees(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetOperator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SubmitBitcoinMerkleProofWithTx(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TimeoutNoDepositNativetoBitcoin(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalHeldCommitFees(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalLockedDeposits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TotalReservedNative(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UsedParadappPrograms(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UsedProofs(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::WindowsFor(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ParadappConvertCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalWindowSec(element) => ::core::fmt::Display::fmt(element, f),
                Self::BpsDenom(element) => ::core::fmt::Display::fmt(element, f),
                Self::BtcDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConfirmationsRequired(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositBlocksWindow(element) => ::core::fmt::Display::fmt(element, f),
                Self::DiffPeriod(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxTimespanSec(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinTimespanSec(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofBlocksWindow(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReserveMarginBps(element) => ::core::fmt::Display::fmt(element, f),
                Self::RetargetPeriodSec(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelfNetworkId(element) => ::core::fmt::Display::fmt(element, f),
                Self::ActiveOpenConversions(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddNativeLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::AddNetwork(element) => ::core::fmt::Display::fmt(element, f),
                Self::AnchorInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveAndStartWithAnchorAndFirst(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BitcoinUsdPriceId(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClaimNativeAfterOperatorExpired(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CloseNoBitcoinBitcoinToNative(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CommitBitcoinToNative(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitFeeNative(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitGlobalBitcoinHeader80(element) => ::core::fmt::Display::fmt(element, f),
                Self::CommitNativeToBitcoin(element) => ::core::fmt::Display::fmt(element, f),
                Self::Conversions(element) => ::core::fmt::Display::fmt(element, f),
                Self::DebugDecodeHeader(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositApprovedConversion(element) => ::core::fmt::Display::fmt(element, f),
                Self::EstimateBitcoinFromNative(element) => ::core::fmt::Display::fmt(element, f),
                Self::EstimateNativeFromBitcoin(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectedNext(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetConversionWithPhase(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetTxIdsByFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GlobalHeaders(element) => ::core::fmt::Display::fmt(element, f),
                Self::GlobalHeightToHashLE(element) => ::core::fmt::Display::fmt(element, f),
                Self::GlobalTipHeight(element) => ::core::fmt::Display::fmt(element, f),
                Self::MinAnchorHeight(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::NativeUsdPriceId(element) => ::core::fmt::Display::fmt(element, f),
                Self::NetworkConfigs(element) => ::core::fmt::Display::fmt(element, f),
                Self::NextTxId(element) => ::core::fmt::Display::fmt(element, f),
                Self::Operator(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProofInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::RefundAfterNoProofNativeToBitcoin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RefundIfNotApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemovableNative(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveNativeLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveNetwork(element) => ::core::fmt::Display::fmt(element, f),
                Self::ServiceFeeBps(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetOperator(element) => ::core::fmt::Display::fmt(element, f),
                Self::SubmitBitcoinMerkleProofWithTx(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TimeoutNoDepositNativetoBitcoin(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TotalHeldCommitFees(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalLockedDeposits(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalReservedNative(element) => ::core::fmt::Display::fmt(element, f),
                Self::UsedParadappPrograms(element) => ::core::fmt::Display::fmt(element, f),
                Self::UsedProofs(element) => ::core::fmt::Display::fmt(element, f),
                Self::WindowsFor(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalWindowSecCall> for ParadappConvertCalls {
        fn from(value: ApprovalWindowSecCall) -> Self {
            Self::ApprovalWindowSec(value)
        }
    }
    impl ::core::convert::From<BpsDenomCall> for ParadappConvertCalls {
        fn from(value: BpsDenomCall) -> Self {
            Self::BpsDenom(value)
        }
    }
    impl ::core::convert::From<BtcDecimalsCall> for ParadappConvertCalls {
        fn from(value: BtcDecimalsCall) -> Self {
            Self::BtcDecimals(value)
        }
    }
    impl ::core::convert::From<ConfirmationsRequiredCall> for ParadappConvertCalls {
        fn from(value: ConfirmationsRequiredCall) -> Self {
            Self::ConfirmationsRequired(value)
        }
    }
    impl ::core::convert::From<DepositBlocksWindowCall> for ParadappConvertCalls {
        fn from(value: DepositBlocksWindowCall) -> Self {
            Self::DepositBlocksWindow(value)
        }
    }
    impl ::core::convert::From<DiffPeriodCall> for ParadappConvertCalls {
        fn from(value: DiffPeriodCall) -> Self {
            Self::DiffPeriod(value)
        }
    }
    impl ::core::convert::From<MaxTimespanSecCall> for ParadappConvertCalls {
        fn from(value: MaxTimespanSecCall) -> Self {
            Self::MaxTimespanSec(value)
        }
    }
    impl ::core::convert::From<MinTimespanSecCall> for ParadappConvertCalls {
        fn from(value: MinTimespanSecCall) -> Self {
            Self::MinTimespanSec(value)
        }
    }
    impl ::core::convert::From<NativeDecimalsCall> for ParadappConvertCalls {
        fn from(value: NativeDecimalsCall) -> Self {
            Self::NativeDecimals(value)
        }
    }
    impl ::core::convert::From<ProofBlocksWindowCall> for ParadappConvertCalls {
        fn from(value: ProofBlocksWindowCall) -> Self {
            Self::ProofBlocksWindow(value)
        }
    }
    impl ::core::convert::From<ReserveMarginBpsCall> for ParadappConvertCalls {
        fn from(value: ReserveMarginBpsCall) -> Self {
            Self::ReserveMarginBps(value)
        }
    }
    impl ::core::convert::From<RetargetPeriodSecCall> for ParadappConvertCalls {
        fn from(value: RetargetPeriodSecCall) -> Self {
            Self::RetargetPeriodSec(value)
        }
    }
    impl ::core::convert::From<SelfNetworkIdCall> for ParadappConvertCalls {
        fn from(value: SelfNetworkIdCall) -> Self {
            Self::SelfNetworkId(value)
        }
    }
    impl ::core::convert::From<ActiveOpenConversionsCall> for ParadappConvertCalls {
        fn from(value: ActiveOpenConversionsCall) -> Self {
            Self::ActiveOpenConversions(value)
        }
    }
    impl ::core::convert::From<AddNativeLiquidityCall> for ParadappConvertCalls {
        fn from(value: AddNativeLiquidityCall) -> Self {
            Self::AddNativeLiquidity(value)
        }
    }
    impl ::core::convert::From<AddNetworkCall> for ParadappConvertCalls {
        fn from(value: AddNetworkCall) -> Self {
            Self::AddNetwork(value)
        }
    }
    impl ::core::convert::From<AnchorInfoCall> for ParadappConvertCalls {
        fn from(value: AnchorInfoCall) -> Self {
            Self::AnchorInfo(value)
        }
    }
    impl ::core::convert::From<ApproveAndStartWithAnchorAndFirstCall> for ParadappConvertCalls {
        fn from(value: ApproveAndStartWithAnchorAndFirstCall) -> Self {
            Self::ApproveAndStartWithAnchorAndFirst(value)
        }
    }
    impl ::core::convert::From<BitcoinUsdPriceIdCall> for ParadappConvertCalls {
        fn from(value: BitcoinUsdPriceIdCall) -> Self {
            Self::BitcoinUsdPriceId(value)
        }
    }
    impl ::core::convert::From<ClaimNativeAfterOperatorExpiredCall> for ParadappConvertCalls {
        fn from(value: ClaimNativeAfterOperatorExpiredCall) -> Self {
            Self::ClaimNativeAfterOperatorExpired(value)
        }
    }
    impl ::core::convert::From<CloseNoBitcoinBitcoinToNativeCall> for ParadappConvertCalls {
        fn from(value: CloseNoBitcoinBitcoinToNativeCall) -> Self {
            Self::CloseNoBitcoinBitcoinToNative(value)
        }
    }
    impl ::core::convert::From<CommitBitcoinToNativeCall> for ParadappConvertCalls {
        fn from(value: CommitBitcoinToNativeCall) -> Self {
            Self::CommitBitcoinToNative(value)
        }
    }
    impl ::core::convert::From<CommitFeeNativeCall> for ParadappConvertCalls {
        fn from(value: CommitFeeNativeCall) -> Self {
            Self::CommitFeeNative(value)
        }
    }
    impl ::core::convert::From<CommitGlobalBitcoinHeader80Call> for ParadappConvertCalls {
        fn from(value: CommitGlobalBitcoinHeader80Call) -> Self {
            Self::CommitGlobalBitcoinHeader80(value)
        }
    }
    impl ::core::convert::From<CommitNativeToBitcoinCall> for ParadappConvertCalls {
        fn from(value: CommitNativeToBitcoinCall) -> Self {
            Self::CommitNativeToBitcoin(value)
        }
    }
    impl ::core::convert::From<ConversionsCall> for ParadappConvertCalls {
        fn from(value: ConversionsCall) -> Self {
            Self::Conversions(value)
        }
    }
    impl ::core::convert::From<DebugDecodeHeaderCall> for ParadappConvertCalls {
        fn from(value: DebugDecodeHeaderCall) -> Self {
            Self::DebugDecodeHeader(value)
        }
    }
    impl ::core::convert::From<DepositApprovedConversionCall> for ParadappConvertCalls {
        fn from(value: DepositApprovedConversionCall) -> Self {
            Self::DepositApprovedConversion(value)
        }
    }
    impl ::core::convert::From<EstimateBitcoinFromNativeCall> for ParadappConvertCalls {
        fn from(value: EstimateBitcoinFromNativeCall) -> Self {
            Self::EstimateBitcoinFromNative(value)
        }
    }
    impl ::core::convert::From<EstimateNativeFromBitcoinCall> for ParadappConvertCalls {
        fn from(value: EstimateNativeFromBitcoinCall) -> Self {
            Self::EstimateNativeFromBitcoin(value)
        }
    }
    impl ::core::convert::From<ExpectedNextCall> for ParadappConvertCalls {
        fn from(value: ExpectedNextCall) -> Self {
            Self::ExpectedNext(value)
        }
    }
    impl ::core::convert::From<GetConversionWithPhaseCall> for ParadappConvertCalls {
        fn from(value: GetConversionWithPhaseCall) -> Self {
            Self::GetConversionWithPhase(value)
        }
    }
    impl ::core::convert::From<GetTxIdsByFilterCall> for ParadappConvertCalls {
        fn from(value: GetTxIdsByFilterCall) -> Self {
            Self::GetTxIdsByFilter(value)
        }
    }
    impl ::core::convert::From<GlobalHeadersCall> for ParadappConvertCalls {
        fn from(value: GlobalHeadersCall) -> Self {
            Self::GlobalHeaders(value)
        }
    }
    impl ::core::convert::From<GlobalHeightToHashLECall> for ParadappConvertCalls {
        fn from(value: GlobalHeightToHashLECall) -> Self {
            Self::GlobalHeightToHashLE(value)
        }
    }
    impl ::core::convert::From<GlobalTipHeightCall> for ParadappConvertCalls {
        fn from(value: GlobalTipHeightCall) -> Self {
            Self::GlobalTipHeight(value)
        }
    }
    impl ::core::convert::From<MinAnchorHeightCall> for ParadappConvertCalls {
        fn from(value: MinAnchorHeightCall) -> Self {
            Self::MinAnchorHeight(value)
        }
    }
    impl ::core::convert::From<NativeLiquidityCall> for ParadappConvertCalls {
        fn from(value: NativeLiquidityCall) -> Self {
            Self::NativeLiquidity(value)
        }
    }
    impl ::core::convert::From<NativeUsdPriceIdCall> for ParadappConvertCalls {
        fn from(value: NativeUsdPriceIdCall) -> Self {
            Self::NativeUsdPriceId(value)
        }
    }
    impl ::core::convert::From<NetworkConfigsCall> for ParadappConvertCalls {
        fn from(value: NetworkConfigsCall) -> Self {
            Self::NetworkConfigs(value)
        }
    }
    impl ::core::convert::From<NextTxIdCall> for ParadappConvertCalls {
        fn from(value: NextTxIdCall) -> Self {
            Self::NextTxId(value)
        }
    }
    impl ::core::convert::From<OperatorCall> for ParadappConvertCalls {
        fn from(value: OperatorCall) -> Self {
            Self::Operator(value)
        }
    }
    impl ::core::convert::From<ProofInfoCall> for ParadappConvertCalls {
        fn from(value: ProofInfoCall) -> Self {
            Self::ProofInfo(value)
        }
    }
    impl ::core::convert::From<RefundAfterNoProofNativeToBitcoinCall> for ParadappConvertCalls {
        fn from(value: RefundAfterNoProofNativeToBitcoinCall) -> Self {
            Self::RefundAfterNoProofNativeToBitcoin(value)
        }
    }
    impl ::core::convert::From<RefundIfNotApprovedCall> for ParadappConvertCalls {
        fn from(value: RefundIfNotApprovedCall) -> Self {
            Self::RefundIfNotApproved(value)
        }
    }
    impl ::core::convert::From<RemovableNativeCall> for ParadappConvertCalls {
        fn from(value: RemovableNativeCall) -> Self {
            Self::RemovableNative(value)
        }
    }
    impl ::core::convert::From<RemoveNativeLiquidityCall> for ParadappConvertCalls {
        fn from(value: RemoveNativeLiquidityCall) -> Self {
            Self::RemoveNativeLiquidity(value)
        }
    }
    impl ::core::convert::From<RemoveNetworkCall> for ParadappConvertCalls {
        fn from(value: RemoveNetworkCall) -> Self {
            Self::RemoveNetwork(value)
        }
    }
    impl ::core::convert::From<ServiceFeeBpsCall> for ParadappConvertCalls {
        fn from(value: ServiceFeeBpsCall) -> Self {
            Self::ServiceFeeBps(value)
        }
    }
    impl ::core::convert::From<SetFeesCall> for ParadappConvertCalls {
        fn from(value: SetFeesCall) -> Self {
            Self::SetFees(value)
        }
    }
    impl ::core::convert::From<SetOperatorCall> for ParadappConvertCalls {
        fn from(value: SetOperatorCall) -> Self {
            Self::SetOperator(value)
        }
    }
    impl ::core::convert::From<SubmitBitcoinMerkleProofWithTxCall> for ParadappConvertCalls {
        fn from(value: SubmitBitcoinMerkleProofWithTxCall) -> Self {
            Self::SubmitBitcoinMerkleProofWithTx(value)
        }
    }
    impl ::core::convert::From<TimeoutNoDepositNativetoBitcoinCall> for ParadappConvertCalls {
        fn from(value: TimeoutNoDepositNativetoBitcoinCall) -> Self {
            Self::TimeoutNoDepositNativetoBitcoin(value)
        }
    }
    impl ::core::convert::From<TotalHeldCommitFeesCall> for ParadappConvertCalls {
        fn from(value: TotalHeldCommitFeesCall) -> Self {
            Self::TotalHeldCommitFees(value)
        }
    }
    impl ::core::convert::From<TotalLockedDepositsCall> for ParadappConvertCalls {
        fn from(value: TotalLockedDepositsCall) -> Self {
            Self::TotalLockedDeposits(value)
        }
    }
    impl ::core::convert::From<TotalReservedNativeCall> for ParadappConvertCalls {
        fn from(value: TotalReservedNativeCall) -> Self {
            Self::TotalReservedNative(value)
        }
    }
    impl ::core::convert::From<UsedParadappProgramsCall> for ParadappConvertCalls {
        fn from(value: UsedParadappProgramsCall) -> Self {
            Self::UsedParadappPrograms(value)
        }
    }
    impl ::core::convert::From<UsedProofsCall> for ParadappConvertCalls {
        fn from(value: UsedProofsCall) -> Self {
            Self::UsedProofs(value)
        }
    }
    impl ::core::convert::From<WindowsForCall> for ParadappConvertCalls {
        fn from(value: WindowsForCall) -> Self {
            Self::WindowsFor(value)
        }
    }
    ///Container type for all return fields from the `APPROVAL_WINDOW_SEC` function with signature `APPROVAL_WINDOW_SEC()` and selector `0xdcd5d309`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ApprovalWindowSecReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `BPS_DENOM` function with signature `BPS_DENOM()` and selector `0x6637e38c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BpsDenomReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `BTC_DECIMALS` function with signature `BTC_DECIMALS()` and selector `0x8e32388e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BtcDecimalsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `CONFIRMATIONS_REQUIRED` function with signature `CONFIRMATIONS_REQUIRED()` and selector `0xede42057`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ConfirmationsRequiredReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `DEPOSIT_BLOCKS_WINDOW` function with signature `DEPOSIT_BLOCKS_WINDOW()` and selector `0x64f367f3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DepositBlocksWindowReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `DIFF_PERIOD` function with signature `DIFF_PERIOD()` and selector `0x969695a9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DiffPeriodReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MAX_TIMESPAN_SEC` function with signature `MAX_TIMESPAN_SEC()` and selector `0x6ebd4a13`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MaxTimespanSecReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `MIN_TIMESPAN_SEC` function with signature `MIN_TIMESPAN_SEC()` and selector `0xd9d96f03`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MinTimespanSecReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `NATIVE_DECIMALS` function with signature `NATIVE_DECIMALS()` and selector `0xe66bf2d7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NativeDecimalsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `PROOF_BLOCKS_WINDOW` function with signature `PROOF_BLOCKS_WINDOW()` and selector `0x4f29043b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProofBlocksWindowReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `RESERVE_MARGIN_BPS` function with signature `RESERVE_MARGIN_BPS()` and selector `0xf842f91e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ReserveMarginBpsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `RETARGET_PERIOD_SEC` function with signature `RETARGET_PERIOD_SEC()` and selector `0x6b59dbba`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RetargetPeriodSecReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `SELF_NETWORK_ID` function with signature `SELF_NETWORK_ID()` and selector `0xede4754a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct SelfNetworkIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `activeOpenConversions` function with signature `activeOpenConversions()` and selector `0x9aa076a3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ActiveOpenConversionsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `anchorInfo` function with signature `anchorInfo(uint256)` and selector `0xdcedfea3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct AnchorInfoReturn {
        pub anchor_height: ::ethers::core::types::U256,
        pub epoch_first_height: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `bitcoinUsdPriceId` function with signature `bitcoinUsdPriceId()` and selector `0x4f7c5ae3`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BitcoinUsdPriceIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `commitFeeNative` function with signature `commitFeeNative()` and selector `0x54213310`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct CommitFeeNativeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `conversions` function with signature `conversions(uint256)` and selector `0x1c989390`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ConversionsReturn {
        pub user: ::ethers::core::types::Address,
        pub is_native_to_bitcoin: bool,
        pub slippage: u16,
        pub user_program: ::ethers::core::types::Bytes,
        pub paradapp_receive_program: ::ethers::core::types::Bytes,
        pub network_address: ::ethers::core::types::Bytes,
        pub network_id: ::ethers::core::types::U256,
        pub native_amount: ::ethers::core::types::U256,
        pub bitcoin_amount: ::ethers::core::types::U256,
        pub created_at: ::ethers::core::types::U256,
        pub approved_at: ::ethers::core::types::U256,
        pub deposited_at: ::ethers::core::types::U256,
        pub commit_fee: ::ethers::core::types::U256,
        pub approved: bool,
        pub deposited: bool,
        pub completed: bool,
        pub refunded: bool,
        pub reserved_native: ::ethers::core::types::U256,
        pub operator_duty_expires_at: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `debugDecodeHeader` function with signature `debugDecodeHeader(bytes)` and selector `0xc25b5e55`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct DebugDecodeHeaderReturn {
        pub hash_le: [u8; 32],
        pub prev_le: [u8; 32],
        pub merkle_le: [u8; 32],
        pub n_bits: u32,
        pub timestamp: u32,
    }
    ///Container type for all return fields from the `estimateBitcoinFromNative` function with signature `estimateBitcoinFromNative(uint256)` and selector `0x70103a5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EstimateBitcoinFromNativeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `estimateNativeFromBitcoin` function with signature `estimateNativeFromBitcoin(uint256)` and selector `0x472b7c41`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct EstimateNativeFromBitcoinReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `expectedNext` function with signature `expectedNext(uint256)` and selector `0x8a66b56d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ExpectedNextReturn {
        pub headers_started: bool,
        pub next_height: ::ethers::core::types::U256,
        pub expected_prev_hash_le: [u8; 32],
    }
    ///Container type for all return fields from the `getConversionWithPhase` function with signature `getConversionWithPhase(uint256)` and selector `0x7593a73a`
    #[derive(Clone, ::ethers::contract::EthAbiType, ::ethers::contract::EthAbiCodec)]
    pub struct GetConversionWithPhaseReturn {
        pub c: Conversion,
        pub phase: u8,
    }
    ///Container type for all return fields from the `getTxIdsByFilter` function with signature `getTxIdsByFilter(uint8,uint8,address,bytes,bool,uint256,bool,uint256,uint256,uint256)` and selector `0xab7ff8b0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GetTxIdsByFilterReturn {
        pub tx_ids: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all return fields from the `globalHeaders` function with signature `globalHeaders(bytes32)` and selector `0xd9a19d35`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GlobalHeadersReturn {
        pub prev_hash_le: [u8; 32],
        pub merkle_root_le: [u8; 32],
        pub n_bits: u32,
        pub timestamp: u32,
        pub set: bool,
        pub arrival_time: u64,
    }
    ///Container type for all return fields from the `globalHeightToHashLE` function with signature `globalHeightToHashLE(uint256)` and selector `0x4b271f5d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GlobalHeightToHashLEReturn(pub [u8; 32]);
    ///Container type for all return fields from the `globalTipHeight` function with signature `globalTipHeight()` and selector `0xf7b21a16`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct GlobalTipHeightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `minAnchorHeight` function with signature `minAnchorHeight()` and selector `0x71c9502c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct MinAnchorHeightReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `nativeLiquidity` function with signature `nativeLiquidity()` and selector `0x06c67372`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NativeLiquidityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `nativeUsdPriceId` function with signature `nativeUsdPriceId()` and selector `0xcb1a7d62`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NativeUsdPriceIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `networkConfigs` function with signature `networkConfigs(uint256)` and selector `0x7b99b603`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NetworkConfigsReturn {
        pub enabled: bool,
        pub min_addr_len: u16,
        pub max_addr_len: u16,
    }
    ///Container type for all return fields from the `nextTxId` function with signature `nextTxId()` and selector `0x8aff87b2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NextTxIdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `operator` function with signature `operator()` and selector `0x570ca735`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct OperatorReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `proofInfo` function with signature `proofInfo(uint256)` and selector `0x3cc5bdd9`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ProofInfoReturn {
        pub set: bool,
        pub verified: bool,
        pub invalid: bool,
        pub attempts: u8,
        pub txid_le: [u8; 32],
        pub block_hash_le: [u8; 32],
        pub block_height: ::ethers::core::types::U256,
        pub out_value_sats: u64,
        pub out_program: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `removableNative` function with signature `removableNative()` and selector `0x8dd9e124`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct RemovableNativeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `serviceFeeBps` function with signature `serviceFeeBps()` and selector `0x529c5514`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct ServiceFeeBpsReturn(pub u16);
    ///Container type for all return fields from the `totalHeldCommitFees` function with signature `totalHeldCommitFees()` and selector `0x1942f33e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalHeldCommitFeesReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalLockedDeposits` function with signature `totalLockedDeposits()` and selector `0xda7abe3d`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalLockedDepositsReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `totalReservedNative` function with signature `totalReservedNative()` and selector `0x7d6f91b6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct TotalReservedNativeReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `usedParadappPrograms` function with signature `usedParadappPrograms(bytes)` and selector `0x5c0510ea`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UsedParadappProgramsReturn(pub bool);
    ///Container type for all return fields from the `usedProofs` function with signature `usedProofs(bytes32)` and selector `0xc30a0f25`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct UsedProofsReturn(pub bool);
    ///Container type for all return fields from the `windowsFor` function with signature `windowsFor(uint256)` and selector `0x4fe0200b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct WindowsForReturn {
        pub headers_started: bool,
        pub start_height: ::ethers::core::types::U256,
        pub last_height: ::ethers::core::types::U256,
        pub deposit_window_end_height: ::ethers::core::types::U256,
        pub proof_window_end_height: ::ethers::core::types::U256,
        pub operator_duty_expires_at: ::ethers::core::types::U256,
    }
    ///`Conversion(address,bool,uint16,bytes,bytes,bytes,uint256,uint256,uint256,uint256,uint256,uint256,uint256,bool,bool,bool,bool,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct Conversion {
        pub user: ::ethers::core::types::Address,
        pub is_native_to_bitcoin: bool,
        pub slippage: u16,
        pub user_program: ::ethers::core::types::Bytes,
        pub paradapp_receive_program: ::ethers::core::types::Bytes,
        pub network_address: ::ethers::core::types::Bytes,
        pub network_id: ::ethers::core::types::U256,
        pub native_amount: ::ethers::core::types::U256,
        pub bitcoin_amount: ::ethers::core::types::U256,
        pub created_at: ::ethers::core::types::U256,
        pub approved_at: ::ethers::core::types::U256,
        pub deposited_at: ::ethers::core::types::U256,
        pub commit_fee: ::ethers::core::types::U256,
        pub approved: bool,
        pub deposited: bool,
        pub completed: bool,
        pub refunded: bool,
        pub reserved_native: ::ethers::core::types::U256,
        pub operator_duty_expires_at: ::ethers::core::types::U256,
    }
}
