// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./ISupraSValueFeed.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

/**
 * @title iPoW Cross-Network Communication (V1-Proof of Concept)
 * @author kelwinshen - Protocol Engineer, Paradapp
 * @notice iPoW (Interoperable Proof of Work) enables architecture-agnostic, trustless 
 * communication between disparate programmable networks.
 * @dev By anchoring state to Bitcoin's Proof of Work, iPoW bypasses the limitations 
 * of specific blockchain architectures. Any network capable of state-machine 
 * programmability can integrate and synchronize via this decentralized relay.
 * * Protocol Architecture:
 * 1. Global Header Relay: A shared, canonical chain of Bitcoin block headers.
 * 2. Windowed SPV: Context-specific verification windows anchored to Bitcoin block heights.
 * 3. Dynamic Progression Logic:
 * - Idle State (Jumping): Efficiency-optimized non-contiguous relaying.
 * - Active State (Contiguous): Strict height continuity (h = tip + 1) during active conversions.
 * 4. Automated Finalization: Asynchronous SPV proof resolution and settlement.
 * * @custom:security Interoperable Proof of Work (iPoW) utilizes Bitcoin's cumulative 
 * difficulty as the root-of-trust for cross-network state transitions.
 */

contract iPoWParadappV1 is ReentrancyGuard {

// --- 1. Role check ---
/// @notice Thrown when the action performed by authorized party
error Unauthorized();

// --- 2. Configuration ---
/// @notice Thrown when the parameters provided during deployment are invalid.
error InvalidConstructor();
/// @notice Thrown when the fee configuration violates protocol constraints.
error InvalidFeeConfig();
/// @notice Thrown when the target network configuration is unsupported or malformed.
error InvalidNetworkConfig();
/// @notice Thrown when attempting to modify a configuration that is currently locked.
error NetworkChangeLocked();

// --- 3. Input Validation ---
/// @notice Thrown when the provided commit fee does not match the required amount.
error IncorrectCommitFee();
/// @notice Thrown when a transaction receives an unexpected value.
error UnexpectedValue();
/// @notice Thrown when a required input value is zero.
error ZeroValue();
/// @notice Thrown when a provided value is mathematically or logically incorrect.
error IncorrectValue();
/// @notice Thrown when an operation requires a defined duty window.
error NeedDutyWindow();
/// @notice Thrown when an operation requires a non-zero Bitcoin amount.
error NeedBitcoinAmount();
/// @notice Thrown when the provided Bitcoin script or program is malformed.
error BadBitcoinProgram();
/// @notice Thrown when the user's Bitcoin program type is not permitted.
error UserBitcoinProgramNotAllowed();
/// @notice Thrown when a destination address for the target network is missing.
error NeedDestAddress();

// --- 4. Network Routing ---
/// @notice Thrown when the specified network ID is incorrect.
error IncorrectNetwork();
/// @notice Thrown when the destination network address is incorrectly formatted.
error IncorrectNetworkAddress();
/// @notice Thrown when the target network is not allowed by the protocol.
error NetworkNotAllowed();
/// @notice Thrown when the specific network address is blacklisted or restricted.
error NetworkAddressNotAllowed();

// --- 5. Lifecycle or State ---
/// @notice Thrown when the provided Transaction ID is invalid.
error BadTxId();
/// @notice Thrown when the contract is in an invalid state for the requested action.
error BadState();
/// @notice Thrown when the conversion type does not match the function logic.
error WrongConversionType();
/// @notice Thrown when the approval window for the conversion has closed.
error ApproveWindowOver();
/// @notice Thrown when the assigned duty period has expired.
error DutyExpired();
/// @notice Thrown when attempting an action before the duty period has expired.
error DutyNotExpired();
/// @notice Thrown when the Bitcoin relay has no headers recorded.
error NoHeadersYet();
/// @notice Thrown when header streaming has already been initialized.
error HeaderStarted();
/// @notice Thrown when the requested header window is invalid.
error IncorrectWindow();
/// @notice Thrown when the proof or transaction has already been verified.
error AlreadyVerified();
/// @notice Thrown when the first header or anchor in a batch is invalid.
error InvalidFirstOrAnchor();
/// @notice Thrown when the block height for the anchor is incorrect.
error InvalidAnchorHeight();
/// @notice Thrown when the filter applied to the conversion type is invalid.
error InvalidTypeFilter();

// --- 6. Liquidity or Economics ---
/// @notice Thrown when the available liquidity is insufficient for the operation.
error LowLiquidity();
/// @notice Thrown when the protocol reserves are below the required threshold.
error LowReserve();
/// @notice Thrown when a withdrawal exceeds the allowed removable amount.
error ExceedsRemovable();
/// @notice Thrown when the price slippage exceeds the defined tolerance.
error BadSlippage();
/// @notice Thrown when slippage protection is not allowed for this operation.
error SlippageNotAllowed();

// --- 7. Bitcoin Headers or Consensus ---
/// @notice Thrown when a Bitcoin program has already been processed.
error ProgramAlreadyUsed();
/// @notice Thrown when a provided Bitcoin header is invalid.
error InvalidHeader();
/// @notice Thrown when the proof of work difficulty is below the required threshold.
error LowWork();
/// @notice Thrown when a header submission attempts to overwrite a finalized height.
error HeightRewrite();
/// @notice Thrown when attempting to jump headers while conversions are active.
error NoJumpWhenActive();
/// @notice Thrown when the previous block hash does not match the current tip.
error PrevAndTipUnmatch();
/// @notice Thrown when the first header of a Bitcoin epoch is missing.
error EpochFirstMissing();
/// @notice Thrown when the Bitcoin difficulty retarget logic is invalid.
error InvalidRetarget();
/// @notice Thrown when required epoch anchors are missing from the relay.
error EpochAnchorsMissing();
/// @notice Thrown when metadata for a Bitcoin epoch is missing.
error EpochMetaMissing();
/// @notice Thrown when the global first header is not set.
error GlobalFirstHeaderMissing();
/// @notice Thrown when the metadata first header is missing.
error MetaFirstHeaderMissing();
/// @notice Thrown when the global anchor is not found.
error GlobalAnchorMissing();
/// @notice Thrown when the metadata anchor header is missing.
error MetaAnchorHeaderMissing();
/// @notice Thrown when the provided anchor must be the current chain tip.
error AnchorMustBeTip();

// --- 8. Oracle ---
/// @notice Thrown when the oracle returns a zero price for an asset.
error OracleZeroPrice();
/// @notice Thrown when the oracle decimal precision is incorrect.
error OracleDecimalsIncorrect();

// --- 9. Transaction Parsing ---
/// @notice Thrown when the transaction data is too short to be valid.
error TransactionTooShort();
/// @notice Thrown when parsing exceeds the transaction buffer.
error TransactionOverflow();
/// @notice Thrown when a Vout index is out of the transaction bounds.
error VoutOutOfBounds();
/// @notice Thrown when a parsed value exceeds allowed limits.
error ValueOutOfBounds();
/// @notice Thrown when a parsed program exceeds allowed limits.
error ProgramOutOfBounds();
/// @notice Thrown when a Variable Integer parsing exceeds bounds.
error VarIntOutOfBounds();
/// @notice Thrown when a 16-bit variable parsing exceeds bounds.
error Var16OutOfBounds();
/// @notice Thrown when a 32-bit variable parsing exceeds bounds.
error Var32OutOfBounds();
/// @notice Thrown when a 64-bit variable parsing exceeds bounds.
error Var64OutOfBounds();
/// @notice Thrown when an 8-byte Little Endian parsing exceeds bounds.
error LE8OutOfBounds();

// --- 10. Transfers ---
/// @notice Thrown when an asset transfer fails.
error TransferFailed();


   
// === ROLES ===

/// @notice The address authorized to approve conversions, manage tunnel states, and stream Bitcoin headers.
address public operator;

/// @dev Access control modifier that ensures the caller is the designated operator.
/// @custom:error Unauthorized Thrown if the caller is not the `operator` address.
modifier onlyOperator() {
    if (msg.sender != operator) revert Unauthorized();
    _;
}


   // ========= CONSTANTS =========

/// @notice The denominator used for all basis point calculations (10,000 = 100%).
uint256 public constant BPS_DENOM = 10_000;

/// @notice The maximum time duration allowed for an operator to approve a conversion after a commit.
uint256 public constant APPROVAL_WINDOW_SEC = 15 minutes;

/// @notice The block-relative window for accepting native deposits in Native-to-Bitcoin conversions.
uint256 public constant DEPOSIT_BLOCKS_WINDOW = 10;

/// @notice The block-relative window within which a proven transaction must be included in the chain.
uint256 public constant PROOF_BLOCKS_WINDOW = 40;

/// @notice The required reserve margin for Bitcoin-to-Native payouts, expressed in basis points.
uint256 public constant RESERVE_MARGIN_BPS = 10_000;

/// @notice The minimum number of Bitcoin confirmations required before a proof is considered finalized.
uint256 public constant CONFIRMATIONS_REQUIRED = 1;

/// @notice The length of a Bitcoin difficulty epoch in blocks.
uint256 public constant DIFF_PERIOD = 2016;

/// @notice The target duration for a Bitcoin difficulty epoch.
uint256 public constant RETARGET_PERIOD_SEC = 14 days;

/// @notice The minimum measured timespan allowed for Bitcoin difficulty retargeting calculations.
uint256 public constant MIN_TIMESPAN_SEC = RETARGET_PERIOD_SEC / 4;

/// @notice The maximum measured timespan allowed for Bitcoin difficulty retargeting calculations.
uint256 public constant MAX_TIMESPAN_SEC = RETARGET_PERIOD_SEC * 4;


// === DECIMAL & NETWORK CONFIGURATION ===

/// @notice The decimal precision of the native token on this host network.
uint256 public immutable NATIVE_DECIMALS;

/// @notice The fixed decimal precision used by the Bitcoin protocol.
uint256 public constant BTC_DECIMALS = 8;

/// @dev A fixed-point scaling factor (10^18) used for high-precision internal price and ratio arithmetic.
uint256 private constant ONE_E18 = 1e18;

/// @notice The unique identifier assigned to this network within the Paradapp cross-chain protocol.
uint256 public immutable SELF_NETWORK_ID;



   // ========= ORACLE =========

/// @dev The interface for the Supra S-Value price feed used for cross-chain valuation.
ISupraSValueFeed internal oracle;

/// @notice The identifier for the Bitcoin/USD price feed.
uint256 public bitcoinUsdPriceId;

/// @notice The identifier for the Native/USD price feed on this network.
uint256 public nativeUsdPriceId;

// ========= FEES =========

/// @notice The fixed native asset fee required to initiate a conversion commitment.
uint256 public commitFeeNative;

/// @notice The protocol service fee applied to conversion quotes, expressed in basis points.
uint16 public serviceFeeBps;

// ========= LIQUIDITY POOLS =========

/// @notice The total volume of native liquidity currently managed by the contract for settlement.
uint256 public nativeLiquidity;

// ========= SAFETY BUCKETS =========

/// @notice The aggregate of native deposits locked awaiting Bitcoin-side proof (Native to Bitcoin).
uint256 public totalLockedDeposits;

/// @notice The native liquidity earmarked for fulfillment of Bitcoin-to-Native payouts.
uint256 public totalReservedNative;

/// @notice The total commit fees held in escrow until conversion finalization or refund.
uint256 public totalHeldCommitFees;

/// @notice The minimum Bitcoin block height required for a valid anchor point in this contract.
uint256 public minAnchorHeight;

   

   /**
 * @notice Represents the full state and configuration of a cross-chain conversion.
 * @dev This struct tracks the lifecycle from commitment to finalization or refund.
 */
struct Conversion {
    // --- Identity & Routing ---
    /// @notice The address of the user who initiated the conversion.
    address user;
    /// @notice Direction flag: true for Native-to-Bitcoin, false for Bitcoin-to-Native.
    bool isNativeToBitcoin;
    /// @notice The maximum allowed price slippage for the conversion quote.
    uint16 slippage;
    /// @notice The user's Bitcoin payout script (used in Native-to-Bitcoin).
    bytes userProgram;
    /// @notice The protocol's Bitcoin receiving script to monitor (used in Bitcoin-to-Native).
    bytes paradappReceiveProgram;
    /// @notice The encoded destination address on the target network.
    bytes networkAddress;
    /// @notice The unique identifier of the target network.
    uint256 networkId;

    // --- Accounting ---
    /// @notice The quoted amount of the native asset in its smallest unit (atomic).
    uint256 nativeAmount;
    /// @notice The quoted amount of Bitcoin in Satoshis (sats).
    uint256 bitcoinAmount;
    /// @notice The actual native fee paid by the user at the time of commitment.
    uint256 commitFee;
    /// @notice The native liquidity reserved for Bitcoin-to-Native payouts at approval.
    uint256 reservedNative;

    // --- Lifecycle Timestamps ---
    /// @notice The block timestamp when the conversion was first committed.
    uint256 createdAt;
    /// @notice The block timestamp when the operator approved the conversion.
    uint256 approvedAt;
    /// @notice The block timestamp when the user's native deposit was confirmed.
    uint256 depositedAt;
    /// @notice The absolute deadline by which the operator must fulfill their duties.
    uint256 operatorDutyExpiresAt;

    // --- Status Flags ---
    /// @notice Indicates if the operator has formally approved the conversion request.
    bool approved;
    /// @notice Indicates if the user's native deposit has been received and verified.
    bool deposited;
    /// @notice Terminal flag indicating the conversion was successfully finalized.
    bool completed;
    /// @notice Terminal flag indicating the conversion was canceled and funds were returned.
    bool refunded;
}

/**
 * @notice Represents the high-level lifecycle phases of a cross-chain conversion.
 * @dev These phases are derived from timestamps, flags, and protocol constants.
 */
enum Phase {
    /// @notice The conversion does not exist or has not been initialized.
    NONE,
    
    /// @notice Initial state: Awaiting operator review and quote validation.
    WAITING_OPERATOR_APPROVAL,
    
    /// @notice Terminal State: The operator failed to approve the request within the required window.
    OPERATOR_APPROVAL_EXPIRED,
    
    /// @notice Pending user action (e.g., Native deposit) following operator approval.
    WAITING_USER_ACTION,
    
    /// @notice Terminal State: The user failed to perform the required action within the allotted time.
    USER_ACTION_EXPIRED,
    
    /// @notice All deposits are locked; awaiting Bitcoin header streaming and proof finalization.
    ACTIVE_WAITING_PROOF,
    
    /// @notice Terminal State: The operator failed to finalize the proof within the mandated duty window.
    OPERATOR_DUTY_EXPIRED,
    
    /// @notice Terminal State: The conversion was successfully verified and finalized.
    COMPLETED,
    
    /// @notice Terminal State: The conversion was canceled and funds were returned to the user.
    REFUNDED
}

/**
 * @notice Categorization filters used for querying or processing specific conversion paths.
 */
enum TypeFilter {
    /// @notice No filter applied; matches all conversion types.
    ANY,
    /// @notice Unidirectional exchange from Bitcoin to the host network's native asset.
    BITCOIN_TO_NATIVE,
    /// @notice Unidirectional exchange from the host network's native asset to Bitcoin.
    NATIVE_TO_BITCOIN,
    /// @notice Cross-chain transfer of a native asset from a remote network into this network.
    NATIVE_TO_NATIVE_IN,
    /// @notice Cross-chain transfer of this network's native asset out to a remote network.
    NATIVE_TO_NATIVE_OUT
}

// ========= STORAGE =========

/// @notice The next unique transaction identifier to be assigned to a new conversion.
/// @dev Monotonically increasing counter ensuring unique lookup keys for the conversion mapping.
uint256 public nextTxId = 1;

/// @notice Primary storage mapping for all conversion data, keyed by a unique transaction ID.
mapping(uint256 => Conversion) public conversions;

  // ========= GLOBAL Bitcoin HEADERS =========

/**
 * @notice Metadata for a validated Bitcoin block header.
 * @dev Hashes are stored in Little-Endian (LE) to match native Bitcoin network encoding.
 */
struct GlobalHeaderMeta {
    /// @notice The Little-Endian hash of the previous block in the chain.
    bytes32 prevHashLE;
    /// @notice The Little-Endian Merkle root of all transactions in the block.
    bytes32 merkleRootLE;
    /// @notice The nBits field representing the target difficulty for the block.
    uint32 nBits;
    /// @notice The Bitcoin block timestamp (Unix).
    uint32 timestamp;
    /// @notice Internal flag indicating if the header metadata has been successfully initialized.
    bool set;
    /// @notice The local block timestamp when this header was recorded by the protocol.
    uint64 arrivalTime;
}

/// @notice Maps a Bitcoin block height to its corresponding Little-Endian block hash.
mapping(uint256 => bytes32) public globalHeightToHashLE;

/// @notice Maps a Little-Endian block hash to its full header metadata.
mapping(bytes32 => GlobalHeaderMeta) public globalHeaders;

/// @notice The highest Bitcoin block height currently indexed and validated by the protocol.
uint256 public globalTipHeight;



/**
 * @notice A temporary cache for Simplified Payment Verification (SPV) data.
 * @dev Stores Merkle paths and transaction details awaiting confirmation by the header relay.
 */
struct ProofCache {
    // --- Lifecycle Flags ---
    /// @notice Indicates if a proof has been submitted for this conversion.
    bool set;
    /// @notice Indicates if the Merkle proof is valid and the required confirmations are met.
    bool verified;
    /// @notice Sticky flag set if a proof fails verification in a way that cannot be retried.
    bool invalid;
    /// @notice Total number of submission attempts for monitoring and telemetry.
    uint8 attempts;

    // --- Transaction Data ---
    /// @notice The double-SHA256 Transaction ID in Little-Endian format.
    bytes32 txidLE;
    /// @notice The hash of the block containing the transaction (Little-Endian).
    bytes32 blockHashLE;
    /// @notice The Bitcoin block height where the transaction is recorded.
    uint256 blockHeight;

    // --- Merkle Path ---
    /// @notice The Merkle siblings (ordered leaf to root) in Little-Endian format.
    bytes32[] branchLE;
    /// @notice The index of the transaction within the block's Merkle tree.
    uint256 index;

    // --- Output Parsing ---
    /// @notice The parsed value of the specific Vout being claimed (in Satoshis).
    uint64 outValueSats;
    /// @notice The scriptPubKey (program) of the selected Vout.
    bytes outProgram;
    /// @notice Indicates if the output data has been successfully parsed and stored.
    bool outSet;
}


/**
 * @notice Defines the specific Bitcoin header range required to finalize a conversion.
 * @dev Anchors the conversion to a specific Bitcoin difficulty epoch (2016 blocks).
 */
struct HeaderWindow {
    /// @notice Indicates if the header streaming window has been initialized.
    bool started;
    /// @notice Indicates if the window has been finalized and closed for further streaming.
    bool closed;
    /// @notice The starting block height of the current Bitcoin difficulty epoch.
    uint256 epochStartHeight;
    /// @notice The specific anchor height from which the header stream begins.
    uint256 windowStartHeight;
    /// @notice The proof context associated with the conversion within this window.
    ProofCache proof;
}


// ========= PROOF ANTI-REUSE =========

/**
 * @notice Prevents the reuse of a specific (Transaction ID + Block Hash) pair across multiple conversions.
 * @dev Keyed by keccak256(txidLE || headerHashLE).
 */
mapping(bytes32 => bool) public usedProofs;

// ========= NETWORK CONFIGURATION =========

/**
 * @notice Configuration parameters for external networks supported by the protocol.
 * @dev Governs the operational state and address validation logic for cross-chain routing.
 */
struct NetworkConfig {
    /// @notice Indicates if the network is currently active and accepting conversions.
    bool enabled;
    /// @notice The minimum byte length allowed for a destination address on this network.
    uint16 minAddrLen;
    /// @notice The maximum byte length allowed for a destination address on this network.
    uint16 maxAddrLen;
}

/// @notice Maps a unique network identifier to its specific validation and status configuration.
mapping(uint256 => NetworkConfig) public networkConfigs;

// ========= STATE TRACKING & DEFERRED LOGIC =========

/**
 * @notice A look-up table for Bitcoin block heights that have proofs awaiting finalization.
 * @dev Maps a Bitcoin height to a list of transaction IDs (txIds) that must be verified 
 * once the relay reaches this height.
 */
mapping(uint256 => uint256[]) internal pendingProofsAtHeight;

/**
 * @notice Tracks specific Bitcoin scripts (Paradapp-owned) to ensure they are not reused.
 * @dev Guard against replay or collision for internal protocol monitoring scripts.
 */
mapping(bytes => bool) public usedParadappPrograms;

/**
 * @notice Accessor for a conversion's specific header window and internal proof state.
 * @dev Private mapping to ensure state is only modified via controlled internal logic.
 */
mapping(uint256 => HeaderWindow) private windows;

// ========= SYSTEM STATE =========

/// @notice The current count of active, non-terminal conversion processes managed by the contract.
/// @dev Used to determine if the Bitcoin header relay is allowed to "jump" or must stream contiguously.
uint256 public activeOpenConversions;

// ========= EVENTS =========

/// @notice Emitted when the administrative operator address is updated.
/// @param newOperator The address of the newly appointed operator.
event OperatorChanged(address newOperator);

/// @notice Emitted when the protocol's fee parameters are modified.
/// @param newCommitFee The updated flat fee for commitments.
/// @param newServiceFeeBps The updated service fee in basis points.
event FeesUpdated(uint256 newCommitFee, uint16 newServiceFeeBps);

/// @notice Emitted when the tracked native liquidity balance is adjusted.
/// @param nativeLiquidity The updated total of tracked native assets.
event LiquidityUpdated(uint256 nativeLiquidity);

/// @notice Emitted when a user initiates a new conversion request.
/// @param txId Unique identifier for the conversion.
/// @param user The address of the user initiating the request.
/// @param isNativetoBitcoin Direction flag for the exchange path.
event ConversionCommitted(uint256 indexed txId, address indexed user, bool isNativetoBitcoin);

/// @notice Emitted when the operator authorizes a conversion and defines the header window.
/// @param txId Unique identifier for the conversion.
/// @param dutyWindowSeconds The time duration assigned for operator duties.
/// @param firstHeight The Bitcoin block height serving as the window anchor.
/// @param firstHeaderHashLE The Little-Endian hash of the anchor block header.
event ConversionApproved(
    uint256 indexed txId,
    uint256 dutyWindowSeconds,
    uint256 firstHeight,
    bytes32 firstHeaderHashLE
);

/// @notice Emitted when a native asset deposit is confirmed for a conversion.
/// @param txId Unique identifier for the conversion.
/// @param nativeAmount The amount of native assets successfully deposited.
event ConversionDeposited(uint256 indexed txId, uint256 nativeAmount);

/// @notice Emitted when a conversion reaches its successful terminal state.
/// @param txId Unique identifier for the conversion.
event ConversionCompleted(uint256 indexed txId);

/// @notice Emitted when a conversion is canceled and funds are returned.
/// @param txId Unique identifier for the conversion.
/// @param refundNative The amount of native assets returned to the user.
/// @param commitFeeRefunded Indicates if the initial commit fee was included in the refund.
event ConversionRefunded(uint256 indexed txId, uint256 refundNative, bool commitFeeRefunded);

// --- Global Header Events ---

/// @notice Emitted when a new Bitcoin header is validated and added to the global relay.
/// @param height The Bitcoin block height of the appended header.
/// @param hashLE The Little-Endian hash of the new header.
/// @param prevHashLE The Little-Endian hash of the preceding block.
/// @param merkleRootLE The Little-Endian Merkle root of the block.
/// @param nBits The difficulty target encoding for the block.
/// @param timestamp The Bitcoin block timestamp (Unix).
event GlobalHeaderAppended(
    uint256 height,
    bytes32 hashLE,
    bytes32 prevHashLE,
    bytes32 merkleRootLE,
    uint32 nBits,
    uint32 timestamp
);



   // ========= MODIFIER =========

/**
 * @dev Validates that a transaction ID exists within the protocol's registry.
 * @notice Ensures the provided `txId` refers to an initialized conversion.
 * @param txId The unique identifier of the conversion to validate.
 * @custom:error BadTxId Thrown if the `txId` is zero or has not yet been assigned by the registry.
 */
modifier validTx(uint256 txId) {
    if (txId == 0 || txId >= nextTxId) revert BadTxId();
    _;
}

   /**
     * @notice Initializes the protocol with essential network, security, and pricing configurations.
     * @dev Sets immutable parameters and validates core addresses to ensure immediate operational integrity.
     * @param _nativeDecimals The decimal precision of the host network's native asset (e.g., 18 for ETH).
     * @param _selfNetworkId The unique protocol-level identifier assigned to this specific deployment.
     * @param _operator The address authorized to manage conversions, open tunnels, and stream headers.
     * @param _oracle The address of the Supra S-Value price feed contract.
     * @param _bitcoinUsdPriceId The feed identifier for the Bitcoin/USD price pair.
     * @param _nativeUsdPriceId The feed identifier for the Native/USD price pair.
     * @param _commitFeeNative The initial flat fee required for conversion commitments.
     * @param _serviceFeeBps The protocol service fee expressed in basis points (100 = 1.00%).
     * @custom:error InvalidConstructor Thrown if network ID is zero, addresses are null, or fees exceed the BPS denominator.
     */
    constructor(
        uint256 _nativeDecimals,
        uint256 _selfNetworkId,
        address _operator,
        address _oracle,
        uint256 _bitcoinUsdPriceId,
        uint256 _nativeUsdPriceId,
        uint256 _commitFeeNative,
        uint16 _serviceFeeBps
    ) {
        if (_selfNetworkId == 0 || _operator == address(0) || _oracle == address(0) || _serviceFeeBps > BPS_DENOM) revert InvalidConstructor();
        
        // Identity & Network
        NATIVE_DECIMALS = _nativeDecimals;
        SELF_NETWORK_ID = _selfNetworkId;
        
        // Roles & External Integrations
        operator = _operator;
        oracle = ISupraSValueFeed(_oracle);
        bitcoinUsdPriceId = _bitcoinUsdPriceId;
        nativeUsdPriceId = _nativeUsdPriceId;
        
        // Economic Parameters
        commitFeeNative = _commitFeeNative;
        serviceFeeBps = _serviceFeeBps;
    }

   // ========= ROLE / ADMIN =========

/**
 * @notice Transfers the operator role to a new address.
 * @dev Restricted to the current operator. Emits an `OperatorChanged` event upon success.
 * @param newOperator The address to be granted the operator role.
 */
function setOperator(address newOperator) external onlyOperator {
    operator = newOperator;
    emit OperatorChanged(newOperator);
}

/**
 * @notice Registers and enables a new target network for cross-chain operations.
 * @dev This function is protected by a safety lock: it cannot be executed while 
 * any conversions are active to prevent routing inconsistencies.
 * @param networkId The unique identifier of the network to add.
 * @param minAddrLen The minimum byte length allowed for addresses on this network.
 * @param maxAddrLen The maximum byte length allowed for addresses on this network.
 * @custom:error NetworkChangeLocked Thrown if there are active, non-terminal conversions.
 * @custom:error InvalidNetworkConfig Thrown if the networkId is invalid, already enabled, 
 * or if length constraints are logically inconsistent.
 */
function addNetwork(
    uint256 networkId,
    uint16 minAddrLen,
    uint16 maxAddrLen
) external onlyOperator {
    if (activeOpenConversions > 0) revert NetworkChangeLocked();
    if (
        networkId == 0 || 
        networkId == SELF_NETWORK_ID || 
        networkConfigs[networkId].enabled || 
        minAddrLen == 0 || 
        minAddrLen > maxAddrLen
    ) revert InvalidNetworkConfig();

    networkConfigs[networkId] = NetworkConfig({
        enabled: true,
        minAddrLen: minAddrLen,
        maxAddrLen: maxAddrLen
    });
}

/**
 * @notice Deactivates and removes a previously supported network.
 * @dev Like `addNetwork`, this requires the protocol state to be idle (no active conversions).
 * @param networkId The unique identifier of the network to remove.
 * @custom:error NetworkChangeLocked Thrown if there are active, non-terminal conversions.
 * @custom:error InvalidNetworkConfig Thrown if the networkId is zero or not currently enabled.
 */
function removeNetwork(uint256 networkId) external onlyOperator {
    if (activeOpenConversions > 0) revert NetworkChangeLocked();
    if (networkId == 0 || networkConfigs[networkId].enabled == false) revert InvalidNetworkConfig();
    delete networkConfigs[networkId];
}


   // ========= FEE MANAGEMENT =========

/**
 * @notice Updates the protocol fee structure.
 * @dev Restricted to the operator. Validates that the service fee is within basis point limits.
 * @param newCommitFee The new flat fee for conversion commitments.
 * @param newServiceFeeBps The new service fee in basis points (e.g., 50 = 0.5%).
 * @custom:error InvalidFeeConfig Thrown if the service fee exceeds `BPS_DENOM`.
 */
function setFees(uint256 newCommitFee, uint16 newServiceFeeBps) external onlyOperator {
    if (newServiceFeeBps > BPS_DENOM) revert InvalidFeeConfig();
    commitFeeNative = newCommitFee;
    serviceFeeBps = newServiceFeeBps;
    emit FeesUpdated(newCommitFee, newServiceFeeBps);
}

// ========= LIQUIDITY MANAGEMENT =========

/**
 * @notice Deposits native assets into the contract's liquidity pool.
 * @dev Increases the tracked `nativeLiquidity` balance.
 * @custom:error ZeroValue Thrown if the transaction value is zero.
 */
function addNativeLiquidity() external payable onlyOperator {
    if (msg.value == 0) revert ZeroValue();
    nativeLiquidity += msg.value;
    emit LiquidityUpdated(nativeLiquidity);
}

/**
 * @notice Calculates the maximum amount of native assets available for withdrawal.
 * @dev Subtracts all protocol liabilities (locked deposits, reserved payouts, and held fees) 
 * from the contract's total balance to determine safe liquidity.
 * @return The amount of native assets that can be removed without affecting protocol safety.
 */
function removableNative() public view returns (uint256) {
    uint256 bal = address(this).balance;
    // Sum of all liabilities and earmarked buckets
    uint256 unavailable = totalLockedDeposits + totalReservedNative + totalHeldCommitFees;
    
    if (bal <= unavailable) return 0;
    
    uint256 byBalance = bal - unavailable;
    // The removable amount is capped by both the surplus balance and tracked liquidity
    if (nativeLiquidity < byBalance) return nativeLiquidity;
    return byBalance;
}

/**
 * @notice Withdraws native liquidity from the contract to the operator.
 * @dev Utilizes `removableNative` to ensure protocol solvency is maintained.
 * @param amount The amount of native assets to withdraw.
 * @custom:error ExceedsRemovable Thrown if the requested amount exceeds the safe liquidity limit.
 * @custom:error TransferFailed Thrown if the native asset transfer to the operator fails.
 */
function removeNativeLiquidity(uint256 amount) external onlyOperator nonReentrant {
    uint256 removable = removableNative();
    if (amount > removable) revert ExceedsRemovable();
    
    nativeLiquidity -= amount;
    (bool ok, ) = payable(msg.sender).call{value: amount}("");
    if (!ok) revert TransferFailed();
    
    emit LiquidityUpdated(nativeLiquidity);
}

   
   /**
 * @notice Initiates the conversion lifecycle from the host network's native asset toward Bitcoin.
 * @dev This phase handles both direct Bitcoin settlement (networkId 0) and cross-chain routing 
 * to remote networks. It escrow's the commit fee and persists the initial conversion state.
 * @param nativeAmount The requested amount of native assets to be converted.
 * @param networkId The destination network identifier (0 for direct Bitcoin).
 * @param networkAddress The encoded destination address on the remote network (if applicable).
 * @param userProgram The Bitcoin scriptPubKey for direct settlement (if applicable).
 * @custom:error IncorrectCommitFee Thrown if the sent ETH/Native does not match `commitFeeNative`.
 * @custom:error ZeroValue Thrown if the requested conversion amount is zero.
 * @custom:error BadBitcoinProgram Thrown if the Bitcoin script length is invalid for direct settlement.
 * @custom:error NetworkAddressNotAllowed Thrown if a network address is provided for a direct Bitcoin conversion.
 * @custom:error UserBitcoinProgramNotAllowed Thrown if a Bitcoin program is provided for a routed cross-chain conversion.
 */
function commitNativeToBitcoin(
    uint256 nativeAmount,
    uint256 networkId,
    bytes calldata networkAddress,
    bytes calldata userProgram
) external payable {
    // 1. Fee & Input Validation
    if (msg.value != commitFeeNative) revert IncorrectCommitFee();
    if (nativeAmount == 0) revert ZeroValue();

    _validateNetwork(networkId);

    // 2. Routing Logic Differentiation
    if (networkId == 0) {
        // Direct Settlement: Validate Bitcoin script bounds
        if (userProgram.length == 0 || userProgram.length > 80) revert BadBitcoinProgram();
        if (networkAddress.length > 0) revert NetworkAddressNotAllowed();
    } else {
        // Cross-Chain Routing: Validate destination address format
        if (userProgram.length > 0) revert UserBitcoinProgramNotAllowed();
        _validateNetworkAddress(networkId, networkAddress);
    }

    // 3. State Persistence
    uint256 txId = nextTxId++;

    conversions[txId] = Conversion({
        user: msg.sender,
        isNativeToBitcoin: true,
        userProgram: networkId == 0 ? userProgram : bytes(""),
        paradappReceiveProgram: "",
        slippage: 0,
        nativeAmount: nativeAmount,
        bitcoinAmount: 0,
        createdAt: block.timestamp,
        approvedAt: 0,
        depositedAt: 0,
        commitFee: msg.value,
        approved: false,
        deposited: false,
        completed: false,
        refunded: false,
        reservedNative: 0,
        operatorDutyExpiresAt: 0,
        networkId: networkId,
        networkAddress: networkId == 0 ? bytes("") : networkAddress
    });

    // 4. Accounting Update
    totalHeldCommitFees += msg.value;

    emit ConversionCommitted(txId, msg.sender, true);
}


/**
 * @notice Initiates a Bitcoin-to-Native conversion, supporting both standard user commitments and immediate operator-managed "tunnels."
 * @dev Distinguishes between user-led requests (requiring subsequent operator approval) and operator-led setups. 
 * @param bitcoinAmount The quantity of Bitcoin (in Satoshis) to be converted.
 * @param networkId The target network identifier for the native asset payout.
 * @param userProgram The Bitcoin scriptPubKey associated with the user's side of the trade.
 * @param destAddress The recipient address of the native funds (Operator path only).
 * @param networkAddress The encoded destination address on a remote network (if applicable).
 * @param dutyWindowSeconds The time duration allotted for the operator to fulfill obligations (Operator path only).
 * @param paradappReceiveProgram The protocol's Bitcoin receiving script to monitor (Operator path only).
 * @param lockedAnchorHeight The Bitcoin block height serving as the consensus anchor (Operator path only).
 * @param slippage The slippage tolerance for the conversion quote in basis points.
 */
function commitBitcoinToNative(
    uint256 bitcoinAmount,
    uint256 networkId,
    bytes calldata userProgram,
    address destAddress,
    bytes calldata networkAddress, 
    uint256 dutyWindowSeconds,
    bytes calldata paradappReceiveProgram,
    uint256 lockedAnchorHeight,
     uint16 slippage
) external payable {
    if (bitcoinAmount == 0) revert NeedBitcoinAmount();

    bool isOperator = (msg.sender == operator);
    uint256 txId = nextTxId++;

    // ================= 1. USER PATH (Awaiting Approval) =================
    // Standard entry for users wishing to initiate a conversion manually.
    if (!isOperator) {
        if (msg.value != commitFeeNative) revert IncorrectCommitFee();
        if (networkId > 0) revert NetworkNotAllowed();
        if (networkAddress.length > 0) revert NetworkAddressNotAllowed();
        if (slippage > 0) revert SlippageNotAllowed();
        if (userProgram.length == 0 || userProgram.length > 80) revert BadBitcoinProgram();

        conversions[txId] = Conversion({
            user: msg.sender,
            isNativeToBitcoin: false,
            userProgram: userProgram,
            slippage: 0,
            paradappReceiveProgram: "",
            nativeAmount: 0,
            bitcoinAmount: bitcoinAmount,
            createdAt: block.timestamp,
            approvedAt: 0,
            depositedAt: 0,
            commitFee: msg.value,
            approved: false,
            deposited: false,
            completed: false,
            refunded: false,
            reservedNative: 0,
            operatorDutyExpiresAt: 0,
            networkId: 0,
            networkAddress: ""
        });

        totalHeldCommitFees += msg.value;
        emit ConversionCommitted(txId, msg.sender, false);
        return;
    }

    // ================= 2. OPERATOR TUNNEL PATH (Immediate Approval) =================
    // High-performance entry for pre-negotiated or automated liquidity tunnels.
    if (lockedAnchorHeight < minAnchorHeight || lockedAnchorHeight > globalTipHeight) revert InvalidAnchorHeight();
    if (destAddress == address(0)) revert NeedDestAddress();
    if (msg.value > 0) revert UnexpectedValue();
    if (networkId == 0) revert IncorrectNetwork();
    if (dutyWindowSeconds == 0) revert NeedDutyWindow();
    if (slippage > BPS_DENOM || slippage == 0) revert BadSlippage();
    if (paradappReceiveProgram.length == 0 || paradappReceiveProgram.length > 80) revert BadBitcoinProgram();
    if (userProgram.length > 0) revert UserBitcoinProgramNotAllowed();

    // Prevent reuse of protocol receiving scripts
    if (usedParadappPrograms[paradappReceiveProgram]) revert ProgramAlreadyUsed();
    usedParadappPrograms[paradappReceiveProgram] = true;

    _validateNetwork(networkId);
    _validateNetworkAddress(networkId, networkAddress);

    // Calculate and verify required liquidity reserve
    uint256 reserve = _estimateNativeFromBitcoin(bitcoinAmount) * (RESERVE_MARGIN_BPS - slippage) / BPS_DENOM;
    if (_availableForReserve() < reserve) revert LowReserve();

    conversions[txId] = Conversion({
        user: destAddress,
        isNativeToBitcoin: false,
        userProgram: "",
        slippage: slippage,
        paradappReceiveProgram: paradappReceiveProgram,
        nativeAmount: 0,
        bitcoinAmount: bitcoinAmount,
        createdAt: block.timestamp,
        approvedAt: block.timestamp,
        depositedAt: 0,
        commitFee: 0,
        approved: true,
        deposited: false,
        completed: false,
        refunded: false,
        reservedNative: reserve,
        operatorDutyExpiresAt: block.timestamp + dutyWindowSeconds,
        networkId: networkId,
        networkAddress: networkAddress
    });

    totalReservedNative += reserve;

    // Initialize the header streaming window anchored to the Bitcoin epoch
    uint256 firstHeight = lockedAnchorHeight - (lockedAnchorHeight % DIFF_PERIOD);
    _startHeaderWindow(txId, lockedAnchorHeight, firstHeight);

    // Emit both initiation and approval events as this path bypasses the waiting state
    emit ConversionCommitted(txId, msg.sender, false);
    emit ConversionApproved(
        txId,
        dutyWindowSeconds,
        firstHeight,
        globalHeightToHashLE[firstHeight]
    );
}

/**
 * @notice Authorizes a pending conversion, reserves liquidity, and anchors the Bitcoin header window.
 * @dev Transitions the conversion state to 'Approved'. This function performs critical safety checks 
 * regarding the approval window, liquidity availability, and Bitcoin script uniqueness.
 * @param txId The unique identifier of the conversion to be approved.
 * @param dutyWindowSeconds The duration (in seconds) from now during which the operator must fulfill duties.
 * @param paradappReceiveProgram The protocol's Bitcoin receiving script (scriptPubKey) for this conversion.
 * @param slippage The slippage tolerance for the conversion quote, expressed in basis points.
 * @custom:error BadState Thrown if the conversion is already approved, refunded, or completed.
 * @custom:error NeedDutyWindow Thrown if the provided duty window is zero.
 * @custom:error ApproveWindowOver Thrown if the `APPROVAL_WINDOW_SEC` has elapsed since commitment.
 * @custom:error BadSlippage Thrown if slippage is zero or exceeds `BPS_DENOM`.
 * @custom:error LowReserve Thrown if the contract lacks sufficient native liquidity to cover the quote.
 * @custom:error BadBitcoinProgram Thrown if the provided Bitcoin script exceeds length constraints.
 * @custom:error ProgramAlreadyUsed Thrown if the `paradappReceiveProgram` has already been utilized.
 */
function approveAndStartWithAnchorAndFirst(
    uint256 txId,
    uint256 dutyWindowSeconds,
    bytes calldata paradappReceiveProgram,
    uint16 slippage
) external onlyOperator validTx(txId) {
    
    Conversion storage c = conversions[txId];

    // 1. Validation & State Checks
    if (c.networkId > 0) {
        _validateNetwork(c.networkId);
        _validateNetworkAddress(c.networkId, c.networkAddress);
    }
    if (c.approved || c.refunded || c.completed) revert BadState();
    if (dutyWindowSeconds == 0) revert NeedDutyWindow();
    if (block.timestamp > c.createdAt + APPROVAL_WINDOW_SEC) revert ApproveWindowOver();
    if (slippage > BPS_DENOM || slippage == 0) revert BadSlippage();

    // 2. State Transition
    c.approved = true;
    c.approvedAt = block.timestamp;
    c.operatorDutyExpiresAt = block.timestamp + dutyWindowSeconds;
    c.slippage = slippage;

    // 3. Liquidity Reservation & Program Assignment
    // Handle Bitcoin-to-Native (Direct or Routed)
    if (!c.isNativeToBitcoin) {
        uint256 reserve = _estimateNativeFromBitcoin(c.bitcoinAmount) * (RESERVE_MARGIN_BPS - slippage) / BPS_DENOM;

        if (_availableForReserve() < reserve) revert LowReserve();

        c.reservedNative = reserve;
        totalReservedNative += reserve;
        
        if (paradappReceiveProgram.length == 0 || paradappReceiveProgram.length > 80) revert BadBitcoinProgram();
        if (usedParadappPrograms[paradappReceiveProgram]) revert ProgramAlreadyUsed();
        
        usedParadappPrograms[paradappReceiveProgram] = true;
        c.paradappReceiveProgram = paradappReceiveProgram;
    } 
    // Handle Native-to-Bitcoin (Cross-chain hop)
    else if (c.isNativeToBitcoin && c.userProgram.length == 0) {
        if (paradappReceiveProgram.length == 0 || paradappReceiveProgram.length > 80) revert BadBitcoinProgram();
        if (usedParadappPrograms[paradappReceiveProgram]) revert ProgramAlreadyUsed();
        
        usedParadappPrograms[paradappReceiveProgram] = true;
        c.userProgram = paradappReceiveProgram;
    }

    // 4. Bitcoin Consensus Anchoring
    // Calculate the start of the current 2016-block difficulty epoch
    uint256 firstHeight = globalTipHeight - (globalTipHeight % DIFF_PERIOD);

    _startHeaderWindow(txId, globalTipHeight, firstHeight); 
    
    emit ConversionApproved(
        txId, 
        dutyWindowSeconds, 
        firstHeight, 
        globalHeightToHashLE[firstHeight]
    );
}
/**
 * @notice Appends a validated Bitcoin block header to the global relay chain.
 * @dev Enforces Bitcoin consensus rules, including Proof-of-Work validation and 
 * difficulty retargeting, while managing state-dependent chain continuity.
 * @param header80 The raw 80-byte Bitcoin block header.
 * @param height The claimed Bitcoin block height for this header.
 * @custom:error InvalidHeader Thrown if the header length is not exactly 80 bytes.
 * @custom:error LowWork Thrown if the header hash does not meet the difficulty target.
 * @custom:error HeightRewrite Thrown if an attempt is made to overwrite an existing height with a different hash.
 * @custom:error NoJumpWhenActive Thrown if a non-contiguous header is submitted while conversions are active.
 * @custom:error PrevAndTipUnmatch Thrown if the header's `prevHash` does not link to the current relay tip.
 * @custom:error EpochFirstMissing Thrown if a jump is attempted without the first header of the target epoch.
 * @custom:error InvalidRetarget Thrown if the difficulty bits do not match the calculated Bitcoin retargeting logic.
 */
function commitGlobalBitcoinHeader80(
    bytes calldata header80,
    uint256 height
) external onlyOperator {
    // 1. Parsing & PoW Validation
    if (header80.length != 80) revert InvalidHeader();

    bytes32 hHashLE = _hashHeaderLE(header80);
    bytes32 prevLE = _extractPrevLE(header80);
    bytes32 mRootLE = _extractMerkleLE(header80);
    uint256 target = _extractTarget(header80);
    uint32 bits = _readCompact(header80);
    uint32 ts = _extractTimestamp(header80);

    if (!_validateWorkLE(hHashLE, target)) revert LowWork();

    // 2. Conflict & Continuity Logic
    bytes32 existing = globalHeightToHashLE[height];
    if (existing != bytes32(0)) {
        // Enforce immutability of the established chain
        if (existing != hHashLE) revert HeightRewrite();
    } else {
        if (globalTipHeight != 0) {
            if (activeOpenConversions > 0) {
                // STRICT CONTINUITY: Required to protect active cross-chain state
                if (height != globalTipHeight + 1) revert NoJumpWhenActive();
                if (prevLE != globalHeightToHashLE[globalTipHeight]) revert PrevAndTipUnmatch();
            } else {
                // OPTIMIZED JUMPING: Allowed when no active conversions rely on contiguous proofs
                if (height == globalTipHeight + 1) {
                    if (prevLE != globalHeightToHashLE[globalTipHeight]) revert PrevAndTipUnmatch();
                } else if (height > globalTipHeight + 1) {
                    // Ensure the anchor for the new epoch exists to validate future retargets
                    uint256 epochStart = height - (height % DIFF_PERIOD);
                    if (epochStart > 0 && epochStart < height) {
                        if (globalHeightToHashLE[epochStart] == bytes32(0)) revert EpochFirstMissing();
                    }
                    if (height > minAnchorHeight) minAnchorHeight = height;
                }
            }
        }

        // 3. State Persistence
        globalHeightToHashLE[height] = hHashLE;
        globalHeaders[hHashLE] = GlobalHeaderMeta({
            prevHashLE: prevLE,
            merkleRootLE: mRootLE,
            nBits: bits,
            timestamp: ts,
            set: true,
            arrivalTime: uint64(block.timestamp)
        });

        if (height > globalTipHeight) globalTipHeight = height;

        // 4. Consensus: Difficulty Retargeting Check
        // Verified every 2016 blocks if sufficient anchors are present
        if (height % DIFF_PERIOD == 0 && height >= DIFF_PERIOD) {
            uint256 prevEpochStart = height - DIFF_PERIOD;
            uint256 prevEpochEnd = height - 1;
            if (
                globalHeightToHashLE[prevEpochStart] != bytes32(0) &&
                globalHeightToHashLE[prevEpochEnd] != bytes32(0)
            ) {
                if (bits != _expectedRetargetBits(height)) revert InvalidRetarget();
            }
        }
    }

    // 5. Automated SPV Finalization
    // Trigger verification for any proofs waiting on this specific block height
    uint256[] memory autoIds = pendingProofsAtHeight[height];
    if (autoIds.length > 0) {
        for (uint256 i = 0; i < autoIds.length; i++) {
            _tryFinalizeProof(autoIds[i]);
        }
    }
}

    // ========= DEPOSIT / PROOF API =========
    function depositApprovedConversion(uint256 txId) external payable validTx(txId) nonReentrant {
        Conversion storage c = conversions[txId];

         if (!c.isNativeToBitcoin) revert WrongConversionType();
          if (c.user != msg.sender) revert Unauthorized(); 
       
      
        if (msg.value != c.nativeAmount) revert IncorrectValue();
        if (!c.approved || c.deposited || c.refunded || c.completed) revert BadState();
      

        HeaderWindow storage hw = windows[txId];
        if (hw.started == false) revert NoHeadersYet();


        // Deposit must occur before the deposit window is considered passed.

        if (globalTipHeight > hw.windowStartHeight + (DEPOSIT_BLOCKS_WINDOW - 1)) revert IncorrectWindow();


        c.bitcoinAmount = _estimateBitcoinFromNative(msg.value) * (BPS_DENOM-c.slippage) / BPS_DENOM;

        c.deposited = true;
        c.depositedAt = block.timestamp;
        totalLockedDeposits += msg.value;

        emit ConversionDeposited(txId, msg.value);
    }


  /**
 * @notice Submits a Bitcoin Merkle proof and transaction data for verification against the header relay.
 * @dev Replaces any existing pending proof for the conversion. The function validates the role of the 
 * caller based on the conversion direction and ensures the proof falls within the allowed block window.
 * @param txId The unique identifier of the conversion associated with this proof.
 * @param txRaw The raw hex-encoded Bitcoin transaction data.
 * @param voutIndex The index of the output (Vout) within the transaction to be verified.
 * @param blockHashLE The Little-Endian hash of the Bitcoin block containing the transaction.
 * @param blockHeight The Bitcoin block height where the transaction is recorded.
 * @param branchLE The Merkle siblings (ordered leaf to root) in Little-Endian format.
 * @param index The index of the transaction within the block's Merkle tree.
 * @custom:error NoHeadersYet Thrown if the header window for this conversion has not been started.
 * @custom:error IncorrectWindow Thrown if the claimed block height is outside the valid proof window.
 * @custom:error Unauthorized Thrown if the caller is not the authorized party for the conversion direction.
 * @custom:error AlreadyVerified Thrown if the conversion has already reached a finalized state.
 */
function submitBitcoinMerkleProofWithTx(
    uint256 txId,
    bytes calldata txRaw,
    uint256 voutIndex,
    bytes32 blockHashLE,
    uint256 blockHeight,
    bytes32[] calldata branchLE,
    uint256 index
) external validTx(txId) {
    HeaderWindow storage hw = windows[txId];
    
    // 1. Window & Access Validation
    if (!hw.started) revert NoHeadersYet();
    if (blockHeight < hw.windowStartHeight || blockHeight > hw.windowStartHeight + (PROOF_BLOCKS_WINDOW - 1)) {
        revert IncorrectWindow();
    }

    Conversion storage c = conversions[txId];

    // Role-gating logic:
    // Native → Bitcoin: Operator proves the payout was sent.
    // Bitcoin → Native: User proves the deposit was made.
    if (c.isNativeToBitcoin) {
        if (msg.sender != operator) revert Unauthorized();
    } else {
        if (msg.sender != c.user) revert Unauthorized();
    }

    ProofCache storage p = hw.proof;
    if (p.verified) revert AlreadyVerified();

    // 2. Transaction Identification
    // Compute the double-SHA256 Transaction ID (Little-Endian)
    bytes32 txidLE = sha256(abi.encodePacked(sha256(txRaw)));
    p.txidLE = txidLE;

    // 3. Output Parsing & Proof Setup
    (uint64 valueSats, bytes memory program) = _parseOutputAt(txRaw, voutIndex);

    // Track attempts and reset state for the new proof submission
    if (p.set && !p.verified) {
        delete p.branchLE;
        p.attempts += 1;
    } else {
        p.attempts = 1;
    }

    p.set = true;
    p.verified = false;
    p.invalid = false;
    p.blockHashLE = blockHashLE; 
    p.blockHeight = blockHeight;
    p.index = index;

    // Persist the Merkle path
    for (uint256 i = 0; i < branchLE.length; i++) {
        p.branchLE.push(branchLE[i]);
    }
    
    p.outValueSats = valueSats;
    p.outProgram = program;
    p.outSet = true;

    // 4. Queue for Finalization
    // Link the conversion to the height for automated verification upon header arrival
    pendingProofsAtHeight[blockHeight].push(txId);
    
    // Immediate attempt to finalize if the header is already present
    _tryFinalizeProof(txId);
}


// ========= TIMEOUTS / REFUNDS =========

/**
 * @notice Cancels a Native-to-Bitcoin conversion if the user fails to deposit funds within the window.
 * @dev Reverts if the operator has already expired or if the deposit window has not yet passed.
 * @param txId The unique identifier of the conversion to timeout.
 * @custom:error WrongConversionType Thrown if the conversion is not Native-to-Bitcoin.
 * @custom:error BadState Thrown if the conversion is already in a terminal or deposited state.
 * @custom:error DutyExpired Thrown if the operator's duty window has already passed.
 */
function timeoutNoDeposit_NativetoBitcoin(uint256 txId) external validTx(txId) onlyOperator nonReentrant {
    Conversion storage c = conversions[txId];

    if (!c.isNativeToBitcoin) revert WrongConversionType();
    if (!c.approved || c.deposited || c.completed || c.refunded) revert BadState();
    if (c.operatorDutyExpiresAt < block.timestamp) revert DutyExpired();

    HeaderWindow storage hw = windows[txId];
    if (!hw.started) revert NoHeadersYet();
    
    // Ensure the specific Bitcoin block window for the deposit has elapsed
    if (globalTipHeight <= hw.windowStartHeight + (DEPOSIT_BLOCKS_WINDOW - 1)) revert IncorrectWindow();

    _payOperatorCommitFee(c);
    c.refunded = true;
    _closeActive(txId);
    
    emit ConversionRefunded(txId, 0, false);
}

/**
 * @notice Allows the user or operator to refund a Native-to-Bitcoin conversion if proof finalization fails.
 * @dev Handles two scenarios: 1. Full refund if native assets were deposited. 2. Fee-only refund if no deposit was made.
 * @param txId The unique identifier of the conversion to refund.
 */
function refundAfterNoProof_NativeToBitcoin(uint256 txId) external validTx(txId) nonReentrant {
    Conversion storage c = conversions[txId];
    if (!c.isNativeToBitcoin) revert WrongConversionType();
    if (msg.sender != c.user && msg.sender != operator) revert Unauthorized();
    if (!c.approved || c.completed || c.refunded) revert BadState();

    HeaderWindow storage hw = windows[txId];
    if (!hw.started) revert NoHeadersYet();

    uint256 endHeight = hw.windowStartHeight + (PROOF_BLOCKS_WINDOW - 1) + (CONFIRMATIONS_REQUIRED - 1);
    bool confsOver = (globalTipHeight > endHeight);
    bool operatorExpired = (c.operatorDutyExpiresAt != 0 && block.timestamp > c.operatorDutyExpiresAt);

    if (c.deposited) {
        // Requires either the Bitcoin window to close or the operator's time-lock to expire
        if (!confsOver && !operatorExpired) revert IncorrectWindow();

        c.refunded = true;
        totalLockedDeposits -= c.nativeAmount;
        totalHeldCommitFees -= c.commitFee;
        _closeActive(txId);

        (bool ok, ) = payable(c.user).call{value: c.nativeAmount + c.commitFee}("");
        if (!ok) revert TransferFailed();
        emit ConversionRefunded(txId, c.nativeAmount, true);
    } else {
        // Simple fee refund if the operator fails to process the request
        if (!operatorExpired) revert IncorrectWindow();

        c.refunded = true;
        uint256 fee = c.commitFee;
        totalHeldCommitFees -= fee;
        c.commitFee = 0;
        _closeActive(txId);

        (bool ok2, ) = payable(c.user).call{value: fee}("");
        if (!ok2) revert TransferFailed();
        emit ConversionRefunded(txId, 0, true);
    }
}

/**
 * @notice Allows the operator to close a Bitcoin-to-Native conversion if the user fails to provide proof.
 * @dev High-integrity check: The operator can only close if they fulfilled their header-streaming duties.
 * @param txId The unique identifier of the conversion to close.
 */
function closeNoBitcoin_BitcoinToNative(uint256 txId) external validTx(txId) onlyOperator nonReentrant {
    Conversion storage c = conversions[txId];
    if (c.isNativeToBitcoin) revert WrongConversionType();
    if (!c.approved || c.completed || c.refunded) revert BadState();

    HeaderWindow storage hw = windows[txId];
    if (!hw.started) revert NoHeadersYet();

    uint256 endHeight = hw.windowStartHeight + (PROOF_BLOCKS_WINDOW - 1) + (CONFIRMATIONS_REQUIRED - 1);
    if (globalTipHeight <= endHeight) revert IncorrectWindow();

    // SECURITY: Prevents operator from malicious closing if they stalled the relay
    if (!_isOperatorDutyFulfilled(txId)) revert DutyExpired();

    c.refunded = true;
    totalReservedNative -= c.reservedNative;
    c.reservedNative = 0;

    _payOperatorCommitFee(c);
    _closeActive(txId);
    emit ConversionRefunded(txId, 0, false);
}

/**
 * @notice Sovereign recovery function for users if the operator fails to fulfill their duties.
 * @dev Transfers reserved liquidity and the commit fee back to the user as a penalty for operator failure.
 * @param txId The unique identifier of the conversion to claim.
 */
function claimNative_AfterOperatorExpired(uint256 txId) external validTx(txId) nonReentrant {
    Conversion storage c = conversions[txId];
    if (c.isNativeToBitcoin) revert WrongConversionType();  
    if (msg.sender != c.user && msg.sender != operator) revert Unauthorized();
    if (!c.approved || c.completed || c.refunded) revert BadState();

    if (c.operatorDutyExpiresAt == 0 || block.timestamp <= c.operatorDutyExpiresAt || _isOperatorDutyFulfilled(txId)) {
        revert DutyNotExpired();
    }

    uint256 nativeOut = c.reservedNative;
    c.nativeAmount = c.reservedNative;
    totalReservedNative -= c.reservedNative;
    nativeLiquidity -= nativeOut;
    c.reservedNative = 0;
    totalHeldCommitFees -= c.commitFee;
    
    c.completed = true;
    _closeActive(txId);

    (bool ok, ) = payable(c.user).call{value: nativeOut + c.commitFee}("");
    if (!ok) revert TransferFailed();

    emit ConversionRefunded(txId, nativeOut + c.commitFee, true);
    emit ConversionCompleted(txId);
}

/**
 * @notice Allows a user to reclaim their commit fee if the operator ghosts the approval phase.
 * @param txId The unique identifier of the unapproved conversion.
 */
function refundIfNotApproved(uint256 txId) external validTx(txId) nonReentrant {
    Conversion storage c = conversions[txId];
    if (c.user != msg.sender) revert Unauthorized();
    if (c.approved || c.completed || c.refunded) revert BadState();
    if (block.timestamp < c.createdAt + APPROVAL_WINDOW_SEC) revert IncorrectWindow();

    c.refunded = true;
    totalHeldCommitFees -= c.commitFee;

    (bool ok, ) = payable(c.user).call{value: c.commitFee}("");
    if (!ok) revert TransferFailed();
    emit ConversionRefunded(txId, 0, true);
}

// ========= INTERNAL HELPERS =========

/**
 * @dev Determines the logical conversion type based on direction and destination network.
 * @param c The conversion storage reference.
 * @return The corresponding `TypeFilter` enum value.
 */
function _filterTypeOf(Conversion storage c) internal view returns (TypeFilter) {
    if (!c.isNativeToBitcoin && c.networkId == 0) return TypeFilter.BITCOIN_TO_NATIVE;
    if (c.isNativeToBitcoin && c.networkId == 0) return TypeFilter.NATIVE_TO_BITCOIN;
    if (c.isNativeToBitcoin && c.networkId != 0) return TypeFilter.NATIVE_TO_NATIVE_OUT;
    if (!c.isNativeToBitcoin && c.networkId != 0) return TypeFilter.NATIVE_TO_NATIVE_IN;

    revert InvalidTypeFilter();
}

// ========= INTERNAL VALIDATION =========

/**
 * @dev Validates that a network identifier is eligible for cross-chain operations.
 * @notice Ensures the target network is enabled and is not the current host network.
 * @param networkId The unique identifier of the destination network (0 for Bitcoin).
 * @custom:error IncorrectNetwork Thrown if the ID is the `SELF_NETWORK_ID` or is not enabled.
 */
function _validateNetwork(uint256 networkId) internal view {
    if (networkId == 0) {
        // Bitcoin (default) is always an eligible target
        return;
    }
    
    // Prevent self-routing and ensure the target network is active
    if (networkId == SELF_NETWORK_ID || !networkConfigs[networkId].enabled) {
        revert IncorrectNetwork();
    }
}

/**
 * @dev Validates the byte-length of a destination network address.
 * @notice Enforces network-specific constraints to prevent malformed routing data.
 * @param networkId The identifier of the network to which the address belongs.
 * @param networkAddress The raw byte data of the destination address.
 * @custom:error IncorrectNetworkAddress Thrown if the address length falls outside the configured bounds.
 */
function _validateNetworkAddress(
    uint256 networkId,
    bytes memory networkAddress
) internal view {
    // Skip length checks for direct Bitcoin settlement (handled via scriptPubKey logic)
    if (networkId == 0) return;

    NetworkConfig memory cfg = networkConfigs[networkId];
    uint256 len = networkAddress.length;

    // Ensure the address length matches the target network's specifications
    if (len < cfg.minAddrLen || len > cfg.maxAddrLen) {
        revert IncorrectNetworkAddress();
    }
}



// ========= INTERNAL WINDOW & DUTY LOGIC =========

/**
 * @dev Initializes the Bitcoin header window for a specific conversion.
 * @notice Anchors the conversion to a Bitcoin block height and its corresponding difficulty epoch.
 * @param txId The unique identifier of the conversion.
 * @param anchorHeight The specific Bitcoin block height serving as the window anchor.
 * @param firstHeight The starting block height of the current Bitcoin difficulty epoch.
 * @custom:error InvalidAnchorHeight Thrown if the anchor is outside the allowed global range.
 * @custom:error HeaderStarted Thrown if the window for this conversion is already initialized.
 * @custom:error GlobalFirstHeaderMissing Thrown if the epoch-first header hash is not found.
 * @custom:error MetaFirstHeaderMissing Thrown if the epoch-first header metadata is not initialized.
 * @custom:error GlobalAnchorMissing Thrown if the anchor block header hash is not found.
 * @custom:error MetaAnchorHeaderMissing Thrown if the anchor block metadata is not initialized.
 */
function _startHeaderWindow(
    uint256 txId,
    uint256 anchorHeight,
    uint256 firstHeight
) internal {
    // 1. Security & Range Validation
    if (anchorHeight < minAnchorHeight || anchorHeight > globalTipHeight) revert InvalidAnchorHeight();
    
    HeaderWindow storage hw = windows[txId];
    if (hw.started) revert HeaderStarted();
   
    // 2. Epoch Integrity Check: Ensure the epoch-first header exists to support retarget math
    bytes32 fHashLE = globalHeightToHashLE[firstHeight];
    if (fHashLE == bytes32(0)) revert GlobalFirstHeaderMissing();
    GlobalHeaderMeta storage fMeta = globalHeaders[fHashLE];
    if (!fMeta.set) revert MetaFirstHeaderMissing();

    // 3. Anchor Integrity Check: Ensure the anchor is a validated part of the chain tip
    bytes32 aHashLE = globalHeightToHashLE[anchorHeight];
    if (aHashLE == bytes32(0)) revert GlobalAnchorMissing();
    GlobalHeaderMeta storage aMeta = globalHeaders[aHashLE];
    if (!aMeta.set) revert MetaAnchorHeaderMissing();

    // 4. Window Initialization
    hw.started = true;
    hw.epochStartHeight = firstHeight;
    hw.windowStartHeight = anchorHeight;

    // Increment global activity to enforce contiguous header streaming
    activeOpenConversions += 1;
}

/**
 * @dev Evaluates whether the operator has fulfilled their data availability and verification duties.
 * @notice Performs a multi-factor "Uptime Check" including block availability, arrival timestamps, 
 * and proof finalization status.
 * @param txId The unique identifier of the conversion to check.
 * @return True if the operator has met all protocol obligations for the current conversion state.
 */
function _isOperatorDutyFulfilled(uint256 txId) internal view returns (bool) {
    HeaderWindow storage hw = windows[txId];
    Conversion storage c = conversions[txId];

    if (!hw.started) return false;

    // 1. Determine the required block depth (10 blocks for deposits, 40 blocks for proofs)
    uint256 requiredWindow = PROOF_BLOCKS_WINDOW;
    if (c.isNativeToBitcoin && !c.deposited) {
        requiredWindow = DEPOSIT_BLOCKS_WINDOW;
    }

    // 2. Data Availability: Verify the target header exists in the Global Relay
    uint256 targetHeight = hw.windowStartHeight + (requiredWindow - 1);
    bytes32 targetHash = globalHeightToHashLE[targetHeight];
    if (targetHash == bytes32(0)) return false; 

    // 3. Performance Check: Ensure headers arrived before the mandated deadline
    GlobalHeaderMeta storage meta = globalHeaders[targetHash];
    bool headersOnTime = (meta.arrivalTime <= c.operatorDutyExpiresAt);
    
    if (!headersOnTime) return false;

    // 4. Finalization Logic: 
    // For Native -> Bitcoin (after deposit), duty is only met once the proof is verified.
    if (c.isNativeToBitcoin && c.deposited) {
        return hw.proof.verified;
    }

    return true;
}



/**
 * @dev High-level state machine logic that derives the current lifecycle phase of a conversion.
 * @notice Resolves the conversion phase by analyzing timestamps, Bitcoin header availability, 
 * and the fulfillment of operator-specific duties.
 * @param c The conversion storage reference.
 * @param hw The header window storage reference.
 * @param txId The unique identifier of the conversion.
 * @return The current `Phase` enum value representing the operational state.
 */
function _computePhase(
    Conversion storage c,
    HeaderWindow storage hw,
    uint256 txId
) internal view returns (Phase) {
    // 1. Terminal & Initial Guardrails
    if (c.user == address(0)) return Phase.NONE;
    if (c.completed) return Phase.COMPLETED;
    if (c.refunded)  return Phase.REFUNDED;

    // 2. Approval Lifecycle
    if (!c.approved) {
        if (block.timestamp <= c.createdAt + APPROVAL_WINDOW_SEC) {
            return Phase.WAITING_OPERATOR_APPROVAL;
        } else {
            return Phase.OPERATOR_APPROVAL_EXPIRED;
        }
    }

    // 3. Operational Performance Metrics
    bool isTimeExpired = (c.operatorDutyExpiresAt != 0 && block.timestamp > c.operatorDutyExpiresAt);
    
    // Evaluates if the operator has provided required headers/proofs within the SLA
    bool isDutyFulfilled = _isOperatorDutyFulfilled(txId); 

    // 4. Directional State Resolution
    if (c.isNativeToBitcoin) {
        // --- Path A: Native → Bitcoin ---
        
        if (!c.deposited) {
            uint256 depositEnd = hw.windowStartHeight + (DEPOSIT_BLOCKS_WINDOW - 1);
            
            // User Timeout: Chain moved past the deposit window or absolute time expired
            if (globalTipHeight > depositEnd || isTimeExpired) {
                return Phase.USER_ACTION_EXPIRED;
            }
            return Phase.WAITING_USER_ACTION;
        }

        // Operator Timeout: Assets are locked (deposited), but operator failed to verify
        if (isTimeExpired && !isDutyFulfilled) {
            return Phase.OPERATOR_DUTY_EXPIRED;
        }

        return Phase.ACTIVE_WAITING_PROOF;

    } else {
        // --- Path B: Bitcoin → Native ---
        ProofCache storage p = hw.proof;

        // User Timeout: Operator provided the required headers, but user failed to provide SPV proof
        if (isDutyFulfilled) {
            return Phase.USER_ACTION_EXPIRED;
        }
            
        // Final Expiry Checks
        if (isTimeExpired) {
            // Operator did their job on time (Permanent Record), blame resides with User
            if (isDutyFulfilled) {
                return Phase.USER_ACTION_EXPIRED;
            }

            // Operator failed to stream headers or verify on time
            return Phase.OPERATOR_DUTY_EXPIRED;
        }

        // Active State Transitions
        if (!p.set) return Phase.WAITING_USER_ACTION;
        return Phase.ACTIVE_WAITING_PROOF;
    }
}



// ========= INTERNAL CONSENSUS & STATE =========

/**
 * @dev Computes the expected nBits (difficulty target) for a new Bitcoin epoch.
 * @notice Implements the standard Bitcoin difficulty retargeting algorithm, 
 * enforcing a maximum adjustment factor of 4x (up or down) per 2016-block epoch.
 * @param newEpochHeight The block height representing the start of the new difficulty epoch.
 * @return The 32-bit compact representation (nBits) of the newly calculated difficulty target.
 * @custom:error EpochAnchorsMissing Thrown if the start or end header hashes of the previous epoch are missing.
 * @custom:error EpochMetaMissing Thrown if the metadata for the previous epoch's boundaries is uninitialized.
 */
function _expectedRetargetBits(uint256 newEpochHeight) internal view returns (uint32) {
    uint256 prevEpochStart = newEpochHeight - DIFF_PERIOD;
    uint256 prevEpochEnd = newEpochHeight - 1;

    bytes32 startHash = globalHeightToHashLE[prevEpochStart];
    bytes32 endHash = globalHeightToHashLE[prevEpochEnd];

    // Ensure boundary headers are present for the calculation
    if (startHash == bytes32(0) || endHash == bytes32(0)) revert EpochAnchorsMissing();

    GlobalHeaderMeta storage startMeta = globalHeaders[startHash];
    GlobalHeaderMeta storage endMeta = globalHeaders[endHash];
    if (!startMeta.set || !endMeta.set) revert EpochMetaMissing();

    // Calculate the actual timespan of the epoch
    uint256 actual = endMeta.timestamp - startMeta.timestamp;

    // Apply the "4x rule" clamp to prevent extreme volatility in difficulty
    if (actual < MIN_TIMESPAN_SEC) actual = MIN_TIMESPAN_SEC;
    if (actual > MAX_TIMESPAN_SEC) actual = MAX_TIMESPAN_SEC;

    // Standard Bitcoin target adjustment formula
    uint256 prevTarget = _targetFromBits(startMeta.nBits);
    uint256 newTarget = (prevTarget * actual) / RETARGET_PERIOD_SEC;

    // Ensure the new target does not exceed the Proof-of-Work limit
    uint256 limit = _powLimit();
    if (newTarget > limit) newTarget = limit;

    return _bitsFromTarget(newTarget);
}

/**
 * @dev High-level accessor for available protocol liquidity.
 * @return The current amount of native assets eligible for reservation or withdrawal.
 */
function _availableForReserve() internal view returns (uint256) {
    return removableNative();
}

/**
 * @dev Transitions a conversion window to a closed state and updates global activity counters.
 * @notice Once closed, the conversion no longer influences the header relay's "jump" logic.
 * @param txId The unique identifier of the conversion to close.
 */
function _closeActive(uint256 txId) internal {
    HeaderWindow storage hw = windows[txId];
    // Guard against redundant state updates
    if (!hw.started || hw.closed) return;
    
    hw.closed = true;
    activeOpenConversions -= 1;
}


   // ========= INTERNAL SETTLEMENT & VERIFICATION =========
/**
 * @notice Transfers the escrowed commit fee to the protocol operator.
 * @dev Typically invoked upon successful conversion finalization or as compensation 
 * for operator duties when a user fails to perform required actions.
 * Follows the Checks-Effects-Interactions pattern to prevent reentrancy.
 * @param c The conversion storage reference containing the fee data.
 * @custom:error TransferFailed Thrown if the native asset transfer to the operator fails.
 */
function _payOperatorCommitFee(Conversion storage c) internal {
    if (c.commitFee > 0) {
        uint256 fee = c.commitFee;
        
        // EFFECT: Clear state before interaction
        c.commitFee = 0;
        totalHeldCommitFees -= fee;

        // INTERACTION: External call to the operator
        (bool ok, ) = payable(operator).call{value: fee}("");
        if (!ok) revert TransferFailed();
    }
}

/**
 * @dev Automatically evaluates and finalizes a stored SPV proof against the current header relay.
 * @notice This function acts as a 'soft' verifier; it performs cryptographic checks but 
 * does not revert on logical failures, allowing for proof resubmission or retry.
 * @param txId The unique identifier of the conversion to be finalized.
 */
function _tryFinalizeProof(uint256 txId) internal {
    Conversion storage c = conversions[txId];
    HeaderWindow storage hw = windows[txId];
    ProofCache storage p = hw.proof;

    // 1. Guardrails
    if (!p.set || p.verified || c.completed) return;

    // 2. Header & Confirmation Validation
    bytes32 bh = globalHeightToHashLE[p.blockHeight];
    if (bh == bytes32(0)) return; // Awaiting header arrival

    // Enforce Bitcoin security depth (confirmations)
    if (globalTipHeight < p.blockHeight + (CONFIRMATIONS_REQUIRED - 1)) return;

    GlobalHeaderMeta storage hm = globalHeaders[bh];
    if (!hm.set || !p.outSet) {
        p.invalid = true;
        return;
    }

    // 3. Anti-Replay Guard
    // Prevents the same Bitcoin transaction from being claimed across different conversions
    bytes32 reuseKey = keccak256(abi.encodePacked(p.txidLE, bh));
    if (usedProofs[reuseKey]) {
        p.invalid = true;
        return;
    }

    // 4. Cryptographic Merkle Verification
    // Reconstructs the Merkle root from the provided branch and compares it to the block header
    bool ok = _proveMerkleLE(
        p.txidLE,
        hm.merkleRootLE,
        _packBranchStorage(p.branchLE),
        p.index
    );
    if (!ok) {
        p.invalid = true;
        return;
    }

    // 5. Logical Finalization (Value & Program Checks)
    // Invokes the final settlement logic. If this returns false, the proof remains 'unverified'
    // allowing for correction without disrupting the header relay.
    bool finalized = _finalizeAfterProof(txId, p.txidLE, bh);
    if (!finalized) return;

    // 6. Terminal Verification Marking
    usedProofs[reuseKey] = true;
    p.verified = true;
}


function _readCompact(bytes calldata header80) internal pure returns (uint32 nBits) {
        nBits = uint32(uint8(header80[72])) |
            (uint32(uint8(header80[73])) << 8) |
            (uint32(uint8(header80[74])) << 16) |
            (uint32(uint8(header80[75])) << 24);
    }

 function _packBranchStorage(bytes32[] storage branch) internal view returns (bytes memory out) {
        out = new bytes(branch.length * 32);
        for (uint256 i = 0; i < branch.length; i++) {
            bytes32 v = branch[i];
            assembly ("memory-safe") {
                mstore(add(add(out, 32), mul(i, 32)), v)
            }
        }
    }


/**
 * @notice Finalizes a conversion by executing the settlement logic after a valid SPV proof.
 * @dev Reconciles the proved Bitcoin transaction against the conversion's parameters. 
 * This function performs the final accounting updates and asset transfers.
 * param txId The unique identifier of the conversion being finalized.
 * return True if the settlement was successful, false if internal logic or value checks failed.
 */
function _finalizeAfterProof(
    uint256 txId,
    bytes32 /*txidLE*/,
    bytes32 /*headerHashLE*/
) internal returns (bool) {
    Conversion storage c = conversions[txId];
    HeaderWindow storage hw = windows[txId];
    ProofCache storage p = hw.proof;

    // 1. Pre-condition Validation
    if (!p.outSet) return false;

    if (c.isNativeToBitcoin) {
        // --- Path A: Native → Bitcoin (Payout Verification) ---

        // Ensure the host assets were actually deposited before finalization
        if (!c.deposited) return false;

        // Verify the Bitcoin output value matches the expected quoted amount
        if (uint256(p.outValueSats) != c.bitcoinAmount) return false;

        // Verify the Bitcoin destination script matches the user's program
        if (keccak256(p.outProgram) != keccak256(c.userProgram)) return false;

        // Accounting: Release the locked native assets and pay operator
        totalLockedDeposits -= c.nativeAmount;
        _payOperatorCommitFee(c);
        
        c.completed = true;
        emit ConversionCompleted(txId);
    } else {
        // --- Path B: Bitcoin → Native (Deposit Verification & Payout) ---

        uint256 provedBitcoin = uint256(p.outValueSats);

        // Verify the Bitcoin transaction arrived at the protocol's monitored address
        if (keccak256(p.outProgram) != keccak256(c.paradappReceiveProgram)) {
            return false;
        }

        /**
         * Calculate slippage-adjusted native payout based on the proved Bitcoin amount.
         * The payout is capped by the initial reserved liquidity to prevent protocol over-exposure.
         */
        uint256 nativeOutNow = _estimateNativeFromBitcoin(provedBitcoin) * (BPS_DENOM - c.slippage) / BPS_DENOM;

        if (nativeOutNow > c.reservedNative) {
            nativeOutNow = c.reservedNative;
        }

        // Accounting: Liquidate reserves and adjust tracked liquidity
        c.nativeAmount = nativeOutNow;
        uint256 reserve = c.reservedNative;
        totalReservedNative -= reserve;
        c.reservedNative = 0;
        nativeLiquidity -= nativeOutNow;

        // Execute Native Asset Payout
        (bool ok, ) = payable(c.user).call{value: nativeOutNow}("");
        if (!ok) return false;

        _payOperatorCommitFee(c);
        c.completed = true;
        emit ConversionCompleted(txId);
    }

    // 2. Global State Cleanup
    _closeActive(txId);
    return true;
}


   // ========= ORACLE HELPERS =========

/**
 * @dev Fetches and normalizes price data from the Supra S-Value oracle.
 * @notice Retrieves current USD prices for Bitcoin and the native asset, 
 * scaling both values to a standard 18-decimal fixed-point format.
 * @return bitcoinPrice1e18 The BTC/USD price scaled to 1e18.
 * @return nativePrice1e18 The Native/USD price scaled to 1e18.
 * @custom:error OracleZeroPrice Thrown if the oracle returns a stale or zero price.
 * @custom:error OracleDecimalsIncorrect Thrown if the oracle feed precision exceeds 18 decimals.
 */
function _fetchOracle() internal view returns (uint256 bitcoinPrice1e18, uint256 nativePrice1e18) {
    ISupraSValueFeed.priceFeed memory bitcoin = oracle.getSvalue(bitcoinUsdPriceId);
    ISupraSValueFeed.priceFeed memory native = oracle.getSvalue(nativeUsdPriceId);

    // Sanity check: Ensure the oracle is providing valid data
    if (bitcoin.price == 0 || native.price == 0) revert OracleZeroPrice();

    // Protocol constraint: Only support precision up to 18 decimals
    if (bitcoin.decimals > 18 || native.decimals > 18) revert OracleDecimalsIncorrect();

    // Normalize both feeds to 1e18 for consistent cross-asset math
    bitcoinPrice1e18 = _scaleTo1e18(bitcoin.price, bitcoin.decimals);
    nativePrice1e18 = _scaleTo1e18(native.price, native.decimals);
}

/**
 * @dev General-purpose decimal scaler for adjusting token amounts between different precisions.
 * @param amount The original value to be scaled.
 * @param fromDec The current decimal precision of the amount.
 * @param toDec The target decimal precision.
 * @return The amount rescaled to the `toDec` precision.
 */
function _adjustDecimals(uint256 amount, uint256 fromDec, uint256 toDec) internal pure returns (uint256) {
    if (fromDec == toDec) return amount;
    if (toDec > fromDec) return amount * (10**(toDec - fromDec));
    return amount / (10**(fromDec - toDec));
}

/**
 * @dev Internal utility to normalize raw oracle values to a standard 18-decimal base.
 * @notice This is critical for fixed-point arithmetic, ensuring $1.00$ is always represented as $1 \times 10^{18}$.
 * @param value The raw price value from the oracle.
 * @param srcDec The source decimal precision reported by the oracle.
 * @return The value scaled to $10^{18}$ precision.
 */
function _scaleTo1e18(uint256 value, uint256 srcDec) internal pure returns (uint256) {
    if (srcDec == 18) return value;
    if (srcDec < 18) return value * (10 ** (18 - srcDec));
    return value / (10 ** (srcDec - 18));
}

  /**
 * @dev Estimates the amount of Bitcoin (in Satoshis) receivable for a given amount of native assets.
 * @notice Calculates a real-time quote by fetching USD prices, normalizing decimals, and deducting protocol fees.
 * @param nativeAmount The amount of native assets (in native decimals) to be converted.
 * @return netBitcoin The estimated Bitcoin amount in Satoshis (8 decimals).
 */
function _estimateBitcoinFromNative(uint256 nativeAmount) internal view returns (uint256) {
    (uint256 bitcoinP, uint256 nativeP) = _fetchOracle();
    
    // 1. Calculate Exchange Rate (Native/BTC) with 1e18 fixed-point precision
    uint256 bitcoinPerNative1e18 = (nativeP * ONE_E18) / bitcoinP;
    
    // 2. Derive Gross Bitcoin (inherits nativeAmount's decimal precision)
    uint256 grossBitcoinRaw = (nativeAmount * bitcoinPerNative1e18) / ONE_E18;

    // 3. Normalize: Convert from Native precision (e.g., 18) to Bitcoin precision (8)
    uint256 grossBitcoin = _adjustDecimals(grossBitcoinRaw, NATIVE_DECIMALS, BTC_DECIMALS);

    // 4. Fee Deduction: Subtract service fee in basis points
    uint256 netBitcoin = (grossBitcoin * (BPS_DENOM - serviceFeeBps)) / BPS_DENOM;
    return netBitcoin;
}

/**
 * @dev Estimates the amount of native assets receivable for a given amount of Bitcoin (Satoshis).
 * @notice Performs the inverse calculation of `_estimateBitcoinFromNative`, scaling results to native precision.
 * @param bitcoinAmount The amount of Bitcoin (in Satoshis) to be converted.
 * @return netNative The estimated native asset amount (in native decimals).
 */
function _estimateNativeFromBitcoin(uint256 bitcoinAmount) internal view returns (uint256) {
    (uint256 bitcoinP, uint256 nativeP) = _fetchOracle();
    
    // 1. Calculate Exchange Rate (BTC/Native) with 1e18 fixed-point precision
    uint256 nativePerBitcoin1e18 = (bitcoinP * ONE_E18) / nativeP;
    
    // 2. Derive Gross Native (inherits bitcoinAmount's decimal precision)
    uint256 grossNativeRaw = (bitcoinAmount * nativePerBitcoin1e18) / ONE_E18;

    // 3. Normalize: Convert from Bitcoin precision (8) to Native precision (e.g., 18)
    uint256 grossNative = _adjustDecimals(grossNativeRaw, BTC_DECIMALS, NATIVE_DECIMALS);

    // 4. Fee Deduction: Subtract service fee in basis points
    uint256 netNative = (grossNative * (BPS_DENOM - serviceFeeBps)) / BPS_DENOM;
    return netNative;
}

   /**
 * @dev Computes the double-SHA256 hash of two concatenated 32-byte values.
 * @notice Standard Bitcoin internal hash function for building Merkle trees.
 */
function _hash256Pair(bytes32 a, bytes32 b) internal pure returns (bytes32) {
    return sha256(abi.encodePacked(sha256(abi.encodePacked(a, b))));
}

/**
 * @notice Verifies a Merkle inclusion proof for a Bitcoin transaction.
 * @dev Reconstructs the Merkle root from a leaf and its siblings, accounting for node ordering.
 * @param leafLE The Little-Endian transaction ID.
 * @param rootLE The expected Merkle root from the block header.
 * @param siblingsLE Concatenated 32-byte sibling hashes.
 * @param index The position of the transaction in the block's Merkle tree.
 * @return True if the reconstructed hash matches the provided root.
 */
function _proveMerkleLE(
    bytes32 leafLE,
    bytes32 rootLE,
    bytes memory siblingsLE,
    uint256 index
) internal pure returns (bool) {
    if (siblingsLE.length % 32 != 0) return false;
    bytes32 h = leafLE;
    unchecked {
        for (uint256 off = 0; off < siblingsLE.length; off += 32) {
            bytes32 sib;
            // Gas-efficient sibling loading via assembly
            assembly ("memory-safe") { sib := mload(add(add(siblingsLE, 32), off)) }
            
            // Reconstruct the parent hash; index determines left vs right child
            h = (index & 1 == 1)
                ? _hash256Pair(sib, h)
                : _hash256Pair(h, sib);
            index >>= 1;
        }
    }
    return h == rootLE;
}


/**
 * @dev Converts a Little-Endian bytes32 to a Big-Endian uint256.
 * @notice Essential for comparing Bitcoin block hashes against difficulty targets.
 */
function _uintFromLE(bytes32 le) internal pure returns (uint256 v) {
    v = uint256(le);
    // Perform efficient bit-swapping to convert LE -> BE
    v = ((v >> 8)  & 0x00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF)
      | ((v & 0x00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF00FF) << 8);
    v = ((v >> 16) & 0x0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF)
      | ((v & 0x0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF0000FFFF) << 16);
    v = ((v >> 32) & 0x00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF)
      | ((v & 0x00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF00000000FFFFFFFF) << 32);
    v = ((v >> 64) & 0x0000000000000000FFFFFFFFFFFFFFFF0000000000000000FFFFFFFFFFFFFFFF)
      | ((v & 0x0000000000000000FFFFFFFFFFFFFFFF0000000000000000FFFFFFFFFFFFFFFF) << 64);
    v = (v >> 128) | (v << 128);
}

/**
 * @dev Validates that a block hash satisfies the required difficulty target.
 * @param hashLE The Little-Endian block hash.
 * @param target The full 256-bit difficulty target.
 */
function _validateWorkLE(bytes32 hashLE, uint256 target) internal pure returns (bool) {
    if (hashLE == bytes32(0)) return false;
    return _uintFromLE(hashLE) <= target;
}



/**
 * @notice Extracts the block timestamp from the header.
 */
function _extractTimestamp(bytes calldata header80) internal pure returns (uint32 ts) {
    ts = uint32(uint8(header80[68])) |
        (uint32(uint8(header80[69])) << 8) |
        (uint32(uint8(header80[70])) << 16) |
        (uint32(uint8(header80[71])) << 24);
}



/**
 * @dev Converts Bitcoin's compact nBits format to a full 256-bit difficulty target.
 */
function _targetFromBits(uint32 bits) internal pure returns (uint256 target) {
    uint256 exp = bits >> 24;
    uint256 mant = bits & 0x007FFFFF;
    if (exp <= 3) {
        target = mant >> (8 * (3 - exp));
    } else {
        target = mant << (8 * (exp - 3));
    }
    // Cap at Proof-of-Work limit
    uint256 limit = _powLimit();
    if (target > limit) target = limit;
}

/**
 * @dev The maximum possible target on the Bitcoin network.
 */
function _powLimit() internal pure returns (uint256) {
    return uint256(0xFFFF) << (8 * (0x1D - 3));
}


  /**
 * @dev A gas-optimized, minimal parser to extract a specific Vout's value and scriptPubKey.
 * @notice Iterates through the raw transaction hex to locate the requested output index.
 * @dev Handles both Legacy and SegWit transaction formats by correctly detecting the SegWit marker/flag.
 * @param txRaw The raw hex-encoded Bitcoin transaction.
 * @param voutIndex The index of the output to extract.
 * @return valueSats The value of the output in Satoshis (8-byte Little-Endian).
 * @return program The scriptPubKey (locking script) of the output.
 * @custom:error TransactionTooShort Thrown if the byte array is smaller than the minimum header.
 * @custom:error TransactionOverflow Thrown if the internal offsets exceed the transaction length.
 * @custom:error VoutOutOfBounds Thrown if the requested voutIndex does not exist in the transaction.
 */
function _parseOutputAt(bytes memory txRaw, uint256 voutIndex) internal pure returns (uint64 valueSats, bytes memory program) {
    uint256 o = 0;
    if(txRaw.length < 4) revert TransactionTooShort();
    
    // 1. Skip Version (4 bytes)
    o += 4; 

    // 2. Handle SegWit Marker & Flag
    // If bytes at offset are 0x00 0x01, it's a SegWit transaction.
    bool isSegwit = false;
    if (txRaw.length >= o + 2 && txRaw[o] == 0x00 && txRaw[o + 1] == 0x01) {
        isSegwit = true;
        o += 2;
    }

    // 3. Skip Inputs
    (uint256 inCount, uint256 s1) = _readVarInt(txRaw, o);
    o = s1;
    for (uint256 i = 0; i < inCount; i++) {
        // Each input: Outpoint (32-byte TXID + 4-byte Index) = 36 bytes
        o += 36;
        // Script length (VarInt) + Script + Sequence (4 bytes)
        (uint256 slen, uint256 s2) = _readVarInt(txRaw, o);
        o = s2 + slen + 4;
        if (o > txRaw.length) revert TransactionOverflow();
    }

    // 4. Locate Outputs
    (uint256 outCount, uint256 s3) = _readVarInt(txRaw, o);
    o = s3;
    if (voutIndex >= outCount) revert VoutOutOfBounds();

    for (uint256 j = 0; j < outCount; j++) {
        if (o + 8 > txRaw.length) revert ValueOutOfBounds();
        
        // Extract 8-byte Little-Endian Value
        uint64 val = _readLE8(txRaw, o);
        o += 8;

        // Extract ScriptPubKey
        (uint256 pkLen, uint256 s4) = _readVarInt(txRaw, o);
        o = s4;
        if(o + pkLen > txRaw.length) revert ProgramOutOfBounds();
        
        // Only allocate memory for the specific program we need
        if (j == voutIndex) {
            bytes memory spk = new bytes(pkLen);
            for (uint256 k = 0; k < pkLen; k++) {
                spk[k] = txRaw[o + k];
            }
            valueSats = val;
            program = spk;
            break; // Targeted Vout found; terminate loop
        }
        
        o += pkLen;
    }

    // Note: Segwit witnesses are at the end of the TX and are ignored by this parser
    // as we only care about the outputs located before the witness stack.
    isSegwit; 
}

/**
 * @dev Decodes a Bitcoin Variable Length Integer (VarInt / CompactSize) from a byte array.
 * @notice Bitcoin uses VarInts to store integers (0 to 2^64-1) using 1, 3, 5, or 9 bytes.
 * @param b The raw byte array containing the transaction or header data.
 * @param o The current memory offset (cursor) to start reading from.
 * @return v The decoded 256-bit integer value.
 * @return next The updated memory offset pointing to the byte immediately following the VarInt.
 * @custom:error VarIntOutOfBounds Thrown if the cursor is at the end of the byte array.
 * @custom:error Var16OutOfBounds Thrown if the prefix implies a 16-bit int but data is missing.
 * @custom:error Var32OutOfBounds Thrown if the prefix implies a 32-bit int but data is missing.
 * @custom:error Var64OutOfBounds Thrown if the prefix implies a 64-bit int but data is missing.
 */
function _readVarInt(bytes memory b, uint256 o) internal pure returns (uint256 v, uint256 next) {
    if (o >= b.length) revert VarIntOutOfBounds();
  
    // The first byte determines the length of the integer
    uint8 p = uint8(b[o]);
    
    if (p < 0xFD) {
        // 1-byte value: 0 to 252
        v = p;
        next = o + 1;
    } else if (p == 0xFD) {
        // 3-byte value: 0xFD followed by a 16-bit Little-Endian integer
        if (o + 3 > b.length) revert Var16OutOfBounds();
        v = uint16(uint8(b[o + 1])) | (uint16(uint8(b[o + 2])) << 8);
        next = o + 3;
    } else if (p == 0xFE) {
        // 5-byte value: 0xFE followed by a 32-bit Little-Endian integer
        if (o + 5 > b.length) revert Var32OutOfBounds();
        v = uint32(uint8(b[o + 1])) |
            (uint32(uint8(b[o + 2])) << 8) |
            (uint32(uint8(b[o + 3])) << 16) |
            (uint32(uint8(b[o + 4])) << 24);
        next = o + 5;
    } else {
        // 9-byte value: 0xFF followed by a 64-bit Little-Endian integer
        if (o + 9 > b.length) revert Var64OutOfBounds();
        v = _readLE8(b, o + 1);
        next = o + 9;
    }
}

/**
 * @dev Reads an 8-byte (64-bit) Little-Endian integer from a byte array.
 * @notice Used primarily for reading Bitcoin output values (Satoshis) and 64-bit VarInts.
 * @param b The raw byte array.
 * @param o The memory offset to start reading from.
 * @return v The decoded 64-bit integer.
 * @custom:error LE8OutOfBounds Thrown if there are fewer than 8 bytes remaining from the offset.
 */
function _readLE8(bytes memory b, uint256 o) internal pure returns (uint64 v) {
    if (o + 8 > b.length) revert LE8OutOfBounds();
    
    // Manual bit-shifting to assemble the 64-bit value from Little-Endian bytes
    v = uint64(uint8(b[o])) |
        (uint64(uint8(b[o + 1])) << 8) |
        (uint64(uint8(b[o + 2])) << 16) |
        (uint64(uint8(b[o + 3])) << 24) |
        (uint64(uint8(b[o + 4])) << 32) |
        (uint64(uint8(b[o + 5])) << 40) |
        (uint64(uint8(b[o + 6])) << 48) |
        (uint64(uint8(b[o + 7])) << 56);
}



/**
 * @notice Computes the canonical double-SHA256 hash of an 80-byte Bitcoin header.
 * @dev Implements the standard Bitcoin hashing algorithm: $$Hash = SHA256(SHA256(Header))$$.
 * Uses assembly `calldatacopy` for optimized memory management.
 * @param header80 The raw 80-byte Bitcoin block header.
 * @return outer The resulting 32-byte block hash in Little-Endian format.
 */
function _hashHeaderLE(bytes calldata header80) internal pure returns (bytes32 outer) {
    if(header80.length != 80) revert InvalidHeader();
    bytes memory tmp = new bytes(80);
    assembly ("memory-safe") {
        calldatacopy(add(tmp, 32), header80.offset, 80)
    }
    bytes32 inner = sha256(tmp);
    outer = sha256(abi.encodePacked(inner));
}


/**
 * @notice Converts a full 256-bit difficulty target into Bitcoin's 32-bit compact 'nBits' format.
 * @dev Reverses the target expansion logic. The resulting uint32 consists of an 8-bit exponent 
 * (base 256) and a 24-bit mantissa. Logic includes a safeguard for the high-bit overflow 
 * characteristic of the Bitcoin compact format.
 * @param target The 256-bit difficulty target to be compacted.
 * @return bits The resulting 32-bit compact representation (nBits).
 */
function _bitsFromTarget(uint256 target) internal pure returns (uint32 bits) {
    if (target == 0) return 0;
    
    // 1. Determine the exponent (byte length)
    uint256 exp = 0;
    uint256 t = target;
    while (t > 0) {
        t >>= 8;
        exp++;
    }

    // 2. Extract the 24-bit mantissa
    uint256 mant;
    if (exp <= 3) {
        mant = target << (8 * (3 - exp));
        exp = 3;
    } else {
        uint256 shift = 8 * (exp - 3);
        mant = target >> shift;
        
        // Handle 'sign bit' edge case: 
        // If the 24th bit is set, the value is treated as negative in Bitcoin's 
        // internal floating point. We shift right and increment exponent to fix this.
        if (mant & 0x00800000 != 0) {
            mant >>= 8;
            exp += 1;
        }
    }
    
    // 3. Pack the Exponent (MSB) and Mantissa
    mant &= 0x00FFFFFF;
    bits = uint32((exp << 24) | mant);
}
/**
 * @notice Extracts the 'previous block hash' field from a raw Bitcoin header.
 * @dev Performs a direct 32-byte `calldataload` at offset 4 of the 80-byte header.
 * @param header80 The raw 80-byte Bitcoin block header.
 * @return raw The 32-byte hash of the previous block in Little-Endian.
 */
function _extractPrevLE(bytes calldata header80) internal pure returns (bytes32 raw) {
    assembly ("memory-safe") {
        raw := calldataload(add(header80.offset, 4))
    }
    return raw;
}

/**
 * @notice Extracts the 'Merkle root' field from a raw Bitcoin header.
 * @dev Performs a direct 32-byte `calldataload` at offset 36 of the 80-byte header.
 * @param header80 The raw 80-byte Bitcoin block header.
 * @return raw The 32-byte Merkle root in Little-Endian.
 */
function _extractMerkleLE(bytes calldata header80) internal pure returns (bytes32 raw) {
    assembly ("memory-safe") {
        raw := calldataload(add(header80.offset, 36))
    }
    return raw;
}

 function _extractTarget(bytes calldata header80) internal pure returns (uint256) {
        uint32 bits = _readCompact(header80);
        return _targetFromBits(bits);
    }

   // ========= PUBLIC VIEW HELPERS =========

/**
 * @notice Retrieves the current status and metadata of a submitted SPV proof.
 * @dev Provides high-fidelity diagnostic data for off-chain monitors and user interfaces.
 * @param txId The unique identifier of the conversion to query.
 * @return set True if a proof has been submitted for this conversion.
 * @return verified True if the proof has successfully passed Merkle and confirmation checks.
 * @return invalid True if the proof was marked as permanently invalid (requires resubmission).
 * @return attempts Total number of proof submission attempts recorded.
 * @return txidLE The Double-SHA256 Transaction ID in Little-Endian format.
 * @return blockHashLE The Little-Endian hash of the Bitcoin block containing the transaction.
 * @return blockHeight The Bitcoin block height where the transaction is recorded.
 * @return outValueSats The specific output value (in Satoshis) extracted from the proof.
 * @return outProgram The scriptPubKey (locking script) extracted from the proof.
 */
function proofInfo(uint256 txId)
    external
    view
    validTx(txId)
    returns (
        bool set,
        bool verified,
        bool invalid,
        uint8 attempts,
        bytes32 txidLE,
        bytes32 blockHashLE,
        uint256 blockHeight,
        uint64 outValueSats,
        bytes memory outProgram
    )
{
    HeaderWindow storage hw = windows[txId];
    ProofCache storage p = hw.proof;

    set = p.set;
    verified = p.verified;
    invalid = p.invalid;
    attempts = p.attempts;
    txidLE = p.txidLE;
    blockHashLE = p.blockHashLE;
    blockHeight = p.blockHeight;
    outValueSats = p.outValueSats;
    outProgram = p.outProgram;
}

/**
 * @notice Retrieves the Bitcoin block heights used to anchor a conversion's header window.
 * @dev Essential for operators to know which range of headers must be streamed contiguously.
 * @param txId The unique identifier of the conversion to query.
 * @return anchorHeight The specific block height from which the verification window begins.
 * @return epochFirstHeight The height of the first block in the current Bitcoin difficulty epoch.
 * @custom:error NoHeadersYet Thrown if the conversion has not yet been approved or the window started.
 */
function anchorInfo(uint256 txId)
    external
    view
    validTx(txId)
    returns (
        uint256 anchorHeight,
        uint256 epochFirstHeight
    )
{
    HeaderWindow storage hw = windows[txId];
    if (hw.started == false) revert NoHeadersYet();

    anchorHeight = hw.windowStartHeight;   
    epochFirstHeight = hw.epochStartHeight; 
}



/**
 * @notice Scans the conversion registry and returns a list of transaction IDs matching specific criteria.
 * @dev Optimized for off-chain `eth_call` usage. Employs a "cheap-to-expensive" filter ordering to minimize execution overhead.
 * @param typeFilter Categorization of the conversion path (e.g., Bitcoin to Native).
 * @param phaseFilter The current operational state (e.g., Waiting for User, Completed).
 * @param userFilter The address of the user who initiated the conversion (zero address to skip).
 * @param bitcoinProgramFilter The raw Bitcoin scriptPubKey to search for.
 * @param searchUserProgram Toggle to search `userProgram` (true) or `paradappReceiveProgram` (false).
 * @param networkIdFilter The target network identifier to filter by.
 * @param useNetworkIdFilter Flag to enable/disable the network ID filter.
 * @param fromTxId The starting ID for the search range (inclusive).
 * @param toTxId The ending ID for the search range (inclusive).
 * @param maxResults The maximum number of IDs to return (pagination limit).
 * @return txIds An array of transaction identifiers that satisfy all active filters.
 */
function getTxIdsByFilter(
    TypeFilter typeFilter,
    Phase phaseFilter,
    address userFilter,
    bytes calldata bitcoinProgramFilter,
    bool searchUserProgram,
    uint256 networkIdFilter, 
    bool useNetworkIdFilter, 
    uint256 fromTxId,
    uint256 toTxId,
    uint256 maxResults
) external view returns (uint256[] memory txIds) {

    // 1. Pagination & Range Normalization
    if (fromTxId < 1) fromTxId = 1;
    if (toTxId >= nextTxId) toTxId = nextTxId - 1;
    if (fromTxId > toTxId || maxResults == 0) {
        return new uint256[](0);
    }

    uint256[] memory tmp = new uint256[](maxResults);
    uint256 count = 0;

    // 2. Gas Optimization: Pre-calculate hash for script comparisons
    bytes32 filterHash;
    bool hasProgramFilter = bitcoinProgramFilter.length > 0;
    if (hasProgramFilter) {
        filterHash = keccak256(bitcoinProgramFilter);
    }

    // 3. Iterative Filter Pipeline
    for (uint256 txId = fromTxId; txId <= toTxId; txId++) {
        Conversion storage c = conversions[txId];
        if (c.user == address(0)) continue;

        // Filter A: Identity (Cheap)
        if (userFilter != address(0) && c.user != userFilter) continue;

        // Filter B: Destination Network
        if (useNetworkIdFilter && c.networkId != networkIdFilter) continue;

        // Filter C: Bitcoin Script (Hashed Comparison)
        if (hasProgramFilter) {
            bytes memory targetProgram = searchUserProgram ? c.userProgram : c.paradappReceiveProgram;
            if (keccak256(targetProgram) != filterHash) continue;
        }

        // Filter D: Directional Type
        if (typeFilter != TypeFilter.ANY && _filterTypeOf(c) != typeFilter) continue;

        /** * Filter E: Phase (Expensive)
         * Kept last because `_computePhase` involves timestamp checks, global relay state, 
         * and duty fulfillment calculations.
         */
        if (phaseFilter != Phase.NONE) {
            HeaderWindow storage hw = windows[txId];
            if (_computePhase(c, hw, txId) != phaseFilter) continue;
        }

        tmp[count++] = txId;
        if (count == maxResults) break;
    }

    // 4. Memory Down-sizing
    txIds = new uint256[](count);
    for (uint256 i = 0; i < count; i++) {
        txIds[i] = tmp[i];
    }
}

      /**
 * @notice Retrieves the full data of a conversion along with its dynamically calculated phase.
 * @dev Combines storage access with the logic-heavy `_computePhase` to provide a complete status update.
 * @param txId The unique identifier of the conversion to query.
 * @return c The full `Conversion` data structure from storage.
 * @return phase The current operational stage (Phase) of the conversion.
 */
function getConversionWithPhase(uint256 txId)
    external
    view
    validTx(txId)
    returns (Conversion memory c, Phase phase)
{
    Conversion storage cs = conversions[txId];
    HeaderWindow storage hw = windows[txId];

    // Compute phase dynamically to reflect timeouts or duty expirations
    phase = _computePhase(cs, hw, txId);
    c = cs;
}

/**
 * @notice Public wrapper to estimate the Bitcoin payout for a native asset input.
 * @dev Utilizes the internal exchange rate engine and normalizes for Bitcoin's 8-decimal precision.
 * @param nativeAmount The amount of native assets to be converted.
 * @return The estimated amount of Satoshis (8 decimals) receivable.
 */
function estimateBitcoinFromNative(uint256 nativeAmount) external view returns (uint256) {
    return _estimateBitcoinFromNative(nativeAmount);
}

/**
 * @notice Public wrapper to estimate the native asset payout for a Bitcoin (Satoshi) input.
 * @dev Normalizes the 8-decimal Bitcoin input to the native host network's decimal precision.
 * @param bitcoinAmount The amount of Bitcoin (in Satoshis) to be converted.
 * @return The estimated amount of native assets (e.g., 18 decimals) receivable.
 */
function estimateNativeFromBitcoin(uint256 bitcoinAmount) external view returns (uint256) {
    return _estimateNativeFromBitcoin(bitcoinAmount);
}


   /**
 * @notice Provides the specific height and parent hash required for the next header submission.
 * @dev Primarily used by the off-chain 'Pusher' bot to synchronize the relay with the contract's tip.
 * @param txId The unique identifier of the conversion to check for window activation.
 * @return headersStarted True if the header window for the specified conversion is active.
 * @return nextHeight The block height expected for the next `commitGlobalBitcoinHeader80` call.
 * @return expectedPrevHashLE The Little-Endian hash of the current tip, required as the `prevHash` in the next header.
 */
function expectedNext(uint256 txId) external view validTx(txId) returns (
    bool headersStarted,
    uint256 nextHeight,
    bytes32 expectedPrevHashLE
) {
    HeaderWindow storage hw = windows[txId];
    headersStarted = hw.started;
    
    if (!headersStarted) {
        return (false, 0, bytes32(0));
    }

    nextHeight = globalTipHeight + 1;
    expectedPrevHashLE = bytes32(0);
    
    // Continuity Check: Fetch the parent hash to ensure the pusher has the correct local state
    if (globalTipHeight > 0) {
        expectedPrevHashLE = globalHeightToHashLE[globalTipHeight];
    }
}

/**
 * @notice Exposes the block-height and temporal windows governing a specific conversion.
 * @dev Combines protocol constants with live state to provide a 'progress report' for UIs and monitors.
 * @param txId The unique identifier of the conversion.
 * @return headersStarted Indicates if the verification window is initialized.
 * @return startHeight The Bitcoin block height where the window was anchored.
 * @return lastHeight The current highest Bitcoin block height indexed by the global relay.
 * @return depositWindowEndHeight The Bitcoin block height at which the deposit phase expires.
 * @return proofWindowEndHeight The Bitcoin block height at which the SPV proof phase expires.
 * @return operatorDutyExpiresAt The Unix timestamp representing the absolute deadline for operator performance.
 */
function windowsFor(uint256 txId) external view validTx(txId) returns (
    bool headersStarted,
    uint256 startHeight,
    uint256 lastHeight,
    uint256 depositWindowEndHeight,
    uint256 proofWindowEndHeight,
    uint256 operatorDutyExpiresAt
) {
    HeaderWindow storage hw = windows[txId];
    headersStarted = hw.started;
    startHeight = hw.windowStartHeight;
    lastHeight = globalTipHeight;

    // Derived block-height boundaries based on protocol constraints
    depositWindowEndHeight = hw.started ? (hw.windowStartHeight + (DEPOSIT_BLOCKS_WINDOW - 1)) : 0;
    proofWindowEndHeight = hw.started ? (hw.windowStartHeight + (PROOF_BLOCKS_WINDOW - 1)) : 0;
    
    operatorDutyExpiresAt = conversions[txId].operatorDutyExpiresAt;
}
   /**
 * @notice Decodes an 80-byte Bitcoin header into its constituent parts for diagnostic purposes.
 * @dev Primarily used during integration testing to verify that the off-chain pusher is 
 * providing correctly formatted Bitcoin data.
 * @param header80 The raw 80-byte Bitcoin block header.
 * @return hashLE The calculated double-SHA256 block hash (Little-Endian).
 * @return prevLE The Little-Endian hash of the previous block.
 * @return merkleLE The Little-Endian Merkle root of the block's transactions.
 * @return nBits The 32-bit compact difficulty target.
 * @return timestamp The Unix timestamp recorded in the block.
 * @custom:error InvalidHeader Thrown if the provided byte array is not exactly 80 bytes.
 */
function debugDecodeHeader(bytes calldata header80) external pure returns (
    bytes32 hashLE,
    bytes32 prevLE,
    bytes32 merkleLE,
    uint32 nBits,
    uint32 timestamp
) {
    if(header80.length != 80) revert InvalidHeader();
    
    // Leverage the internal library primitives to decompose the header
    hashLE = _hashHeaderLE(header80);
    prevLE = _extractPrevLE(header80);
    merkleLE = _extractMerkleLE(header80);
    nBits = _readCompact(header80);
    timestamp = _extractTimestamp(header80);
}

/**
 * @notice Fallback function to allow the contract to receive native assets.
 * @dev Essential for depositing protocol liquidity, receiving commit fees, and handling 
 * user refunds/deposits. Ensure that any native asset transfers to this contract 
 * are properly tracked via the protocol's internal accounting logic.
 */
receive() external payable {}


}
