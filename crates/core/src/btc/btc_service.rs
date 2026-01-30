use crate::context::CoreContext;
use anyhow::{Result, anyhow, bail};
use bip32::secp256k1::sha2::{Digest, Sha256, Sha512};
use bitcoin::{
    Address as BTCAddress, Amount, CompressedPublicKey, EcdsaSighashType, Network, OutPoint,
    PrivateKey, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, Witness,
    absolute::LockTime,
    bip32::{DerivationPath, Xpriv, Xpub},
    consensus::serialize,
    key::Secp256k1,
    secp256k1::{self, Message},
    sighash::SighashCache,
};
use ethers::{
    providers::{Middleware, Provider},
    types::{Bytes as TypesBytes, U256},
    utils::{hex, parse_units},
};
use pbkdf2::pbkdf2_hmac;
use serde::Deserialize;
use serde_json::Value;
use std::str::FromStr;
use tracing::{error, info, warn};

pub fn epoch_start(height: u64) -> u64 {
    height - (height % 2016)
}

pub fn parse_human_btc(x: impl ToString) -> Result<U256> {
    let s = x.to_string();
    let v = parse_units(s, 8)?;
    Ok(v.into())
}

pub fn parse_btc_network(name: &str) -> Result<Network> {
    match name.to_lowercase().as_str() {
        "bitcoin" | "mainnet" => Ok(Network::Bitcoin),
        "testnet" => Ok(Network::Testnet),
        "signet" => Ok(Network::Signet),
        "regtest" => Ok(Network::Regtest),
        _ => bail!("Unknown BTC network: {}", name),
    }
}

pub fn from_0x(s: &str) -> &str {
    s.strip_prefix("0x").unwrap_or(s)
}

pub fn to_0x(s: &str) -> String {
    if s.starts_with("0x") {
        s.to_owned()
    } else {
        format!("0x{}", s)
    }
}

pub async fn header80_by_height(ctx: &CoreContext, height: u64) -> Result<(String, String)> {
    // 1. GET block hash by height (BE)
    let url_hash = format!("{}/block-height/{}", ctx.cfg.esplora_base, height);
    let block_hash_be = ctx
        .http
        .get(&url_hash)
        .send()
        .await?
        .text()
        .await?
        .trim()
        .to_string();

    // 2. GET block header hex
    let url_header = format!("{}/block/{}/header", ctx.cfg.esplora_base, block_hash_be);
    let header_hex = ctx
        .http
        .get(&url_header)
        .send()
        .await?
        .text()
        .await?
        .trim()
        .to_string();

    // 3. Validate length (80 bytes → 160 hex chars)
    if header_hex.len() != 160 {
        bail!("header80 hex has {} chars, expected 160", header_hex.len());
    }

    // 4. Ensure "0x" prefix (same as your TS code)
    let header80_prefixed = if header_hex.starts_with("0x") {
        header_hex
    } else {
        format!("0x{}", header_hex)
    };

    Ok((block_hash_be, header80_prefixed))
}

pub async fn btc_tip_height(ctx: &CoreContext) -> Result<u64> {
    let url = format!("{}/blocks/tip/height", ctx.cfg.esplora_base);

    // Fetch text from the endpoint
    let text = ctx.http.get(&url).send().await?.text().await?;
    let trimmed = text.trim();

    // Parse height as number and sub with 6 as high finality confirmed block
    let height: u64 = (trimmed.parse::<u64>()?).saturating_sub(0);

    Ok(height)
}

pub fn dsha256(buf: &[u8]) -> [u8; 32] {
    let first = Sha256::digest(buf);
    let second = Sha256::digest(first);
    let mut out = [0u8; 32];
    out.copy_from_slice(&second);
    out
}

pub fn header_hash_both_from_header80(header80_0x: &str) -> Result<(String, String)> {
    let hex = &header80_0x[2..];
    let raw = hex::decode(hex)?;
    if raw.len() != 80 {
        bail!("header not 80 bytes");
    }

    let h: [u8; 32] = dsha256(&raw);

    let mut rev = h;
    rev.reverse();

    let be_hex = format!("0x{}", hex::encode(rev));
    let le_hex = format!("0x{}", hex::encode(h));

    Ok((be_hex, le_hex))
}

pub fn pow_limit() -> U256 {
    U256::from(0xffffu128) << (8 * (0x1d - 3))
}

pub fn target_from_bits(bits: u32) -> U256 {
    let exp = (bits >> 24) & 0xff;
    let mant = bits & 0x007fffff;

    let mant = U256::from(mant);

    let target = if exp <= 3 {
        let shift: usize = (8 * (3 - exp)) as usize;
        mant >> shift
    } else {
        let shift: usize = (8 * (exp - 3)) as usize;
        mant << shift
    };

    let limit = pow_limit();
    if target > limit { limit } else { target }
}

pub fn read_compact_bits_from_header80(header80_0x: &str) -> Result<u32> {
    let s = &header80_0x[2..];

    let b72 = u32::from_str_radix(&s[144..146], 16)?;
    let b73 = u32::from_str_radix(&s[146..148], 16)?;
    let b74 = u32::from_str_radix(&s[148..150], 16)?;
    let b75 = u32::from_str_radix(&s[150..152], 16)?;

    Ok(b72 | (b73 << 8) | (b74 << 16) | (b75 << 24))
}

pub fn check_work_le(header80_0x: &str) -> Result<(bool, u32, U256, U256)> {
    let (_be, le_hex) = header_hash_both_from_header80(header80_0x)?;
    let bits = read_compact_bits_from_header80(header80_0x)?;
    let target = target_from_bits(bits); // now U256

    // strip 0x
    let hex = &le_hex[2..];
    let bytes = hex::decode(hex)?;

    // Parse 32-byte little-endian hash into U256
    let mut h_val = U256::zero();
    for (i, byte) in bytes.iter().take(32).enumerate() {
        let b = U256::from(*byte);
        h_val |= b << (8 * i);
    }

    Ok((h_val <= target, bits, target, h_val))
}

pub async fn check_hedera_alive(provider: &Provider<ethers::providers::Http>) -> bool {
    match provider.get_block_number().await {
        Ok(bn) => {
            info!(block_number = %bn, "Hedera RPC alive");
            true
        }
        Err(e) => {
            error!(error = %e, "Hedera RPC health check failed");
            false
        }
    }
}

pub async fn check_bitcoin_alive(ctx: &CoreContext, esplora_base: &str) -> bool {
    let url = format!("{esplora_base}/blocks/tip/height");

    match ctx.http.get(url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => match text.parse::<u32>() {
                Ok(tip) => {
                    info!(tip_height = tip, "Bitcoin Esplora alive");
                    true
                }
                Err(_) => false,
            },
            Err(_) => false,
        },
        Err(e) => {
            error!(error = %e, "Bitcoin Esplora health failed");
            false
        }
    }
}

pub fn decode_header80(header80_hex: &str) -> Result<TypesBytes, String> {
    let clean = header80_hex.trim_start_matches("0x");

    let decoded = match hex::decode(clean) {
        Ok(bytes) => bytes,
        Err(_) => return Err(format!("invalid hex in header80: {header80_hex}")),
    };

    if decoded.len() != 80 {
        return Err(format!(
            "header80 must be 80 bytes, got {}: {header80_hex}",
            decoded.len()
        ));
    }

    Ok(TypesBytes::from(decoded))
}

pub fn hex_le_from_be32(hex: &str) -> Result<String> {
    let s = from_0x(hex);
    if s.len() != 64 {
        return Err(anyhow!("need 32B hex"));
    }
    let rev = s
        .as_bytes()
        .chunks(2)
        .map(|c| std::str::from_utf8(c).unwrap())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<&str>>()
        .join("");
    Ok(format!("0x{}", rev))
}

pub async fn tx_merkle_proof(ctx: &CoreContext, txid_be: &str) -> Result<Value> {
    let url = format!("{}/tx/{}/merkle-proof", ctx.cfg.esplora_base, txid_be);
    let json = ctx.http.get(url).send().await?.json::<Value>().await?;
    Ok(json)
}

pub async fn tx_hex(ctx: &CoreContext, txid_be: &str) -> Result<String> {
    let url = format!("{}/tx/{}/hex", ctx.cfg.esplora_base, txid_be);
    let hex = ctx.http.get(url).send().await?.text().await?;
    Ok(format!("0x{}", hex.trim()))
}

/// Convert BigInt-like counts to Bitcoin varint bytes
pub fn write_varint(n: u64) -> Vec<u8> {
    if n < 0xFD {
        vec![n as u8]
    } else if n <= 0xFFFF {
        let mut b = vec![0xFD, 0, 0];
        b[1] = (n & 0xFF) as u8;
        b[2] = ((n >> 8) & 0xFF) as u8;
        b
    } else if n <= 0xFFFF_FFFF {
        let mut b = vec![0xFE, 0, 0, 0, 0];
        for i in 0..4 {
            b[1 + i] = ((n >> (8 * i)) & 0xFF) as u8;
        }
        b
    } else {
        let mut b = vec![0xFF, 0, 0, 0, 0, 0, 0, 0, 0];
        for i in 0..8 {
            b[1 + i] = ((n >> (8 * i)) & 0xFF) as u8;
        }
        b
    }
}

/// Read Bitcoin varint
pub fn read_varint(buf: &[u8], o: usize) -> Result<(u64, usize)> {
    let p = *buf.get(o).ok_or(anyhow!("varint: out of bounds"))?;

    Ok(match p {
        x if x < 0xFD => (x as u64, o + 1),

        0xFD => {
            let lo = *buf.get(o + 1).ok_or(anyhow!("varint: 0xfd lo"))? as u64;
            let hi = *buf.get(o + 2).ok_or(anyhow!("varint: 0xfd hi"))? as u64;
            ((hi << 8) | lo, o + 3)
        }

        0xFE => {
            let mut v = 0u64;
            for i in 0..4 {
                v |= (*buf.get(o + 1 + i).ok_or(anyhow!("varint: 0xfe"))? as u64) << (8 * i);
            }
            (v, o + 5)
        }

        0xFF => {
            let mut v = 0u64;
            for i in 0..8 {
                v |= (*buf.get(o + 1 + i).ok_or(anyhow!("varint: 0xff"))? as u64) << (8 * i);
            }
            (v, o + 9)
        }

        _ => unreachable!(),
    })
}

pub fn to_legacy_serialization_strict(tx_hex_0x: &str) -> Result<Vec<u8>> {
    let hex = from_0x(tx_hex_0x);
    let buf = hex::decode(hex)?;

    if buf.len() < 4 {
        return Err(anyhow!("tx too short"));
    }

    let mut o = 0usize;

    // ---- version ----
    let version = buf[o..o + 4].to_vec();
    o += 4;

    // ---- skip marker+flag if present (0x00 0x01) ----
    if buf.get(o) == Some(&0x00) && buf.get(o + 1) == Some(&0x01) {
        o += 2; // exactly like JS
    }

    // ---- vin ----
    let (in_count, o2) = read_varint(&buf, o)?;
    o = o2;

    let mut vin_chunks: Vec<Vec<u8>> = Vec::new();

    for _ in 0..in_count {
        let start = o;

        // prevout hash (32) + index (4)
        o += 32 + 4;

        // script varint
        let (script_len, next) = read_varint(&buf, o)?;
        o = next;

        o += script_len as usize;

        // sequence
        o += 4;

        vin_chunks.push(buf[start..o].to_vec());
    }

    // ---- vout ----
    let (out_count, next) = read_varint(&buf, o)?;
    o = next;

    let mut vout_chunks = Vec::new();

    for _ in 0..out_count {
        let start = o;

        // value (8 bytes)
        o += 8;

        let (pk_len, next2) = read_varint(&buf, o)?;
        o = next2;

        o += pk_len as usize;

        vout_chunks.push(buf[start..o].to_vec());
    }

    // ---- locktime (last 4 bytes) ----
    let locktime = buf[buf.len() - 4..].to_vec();

    // ---- reassemble
    let mut out = Vec::new();
    out.extend_from_slice(&version);
    out.extend_from_slice(&write_varint(in_count));
    for chunk in vin_chunks {
        out.extend_from_slice(&chunk);
    }
    out.extend_from_slice(&write_varint(out_count));
    for chunk in vout_chunks {
        out.extend_from_slice(&chunk);
    }
    out.extend_from_slice(&locktime);

    Ok(out)
}

pub struct ConfirmedReceive {
    pub txid_be: String,
    pub vout_index: usize,
    pub value_sats: u64,
    pub script_pubkey_hex: String,
    pub block_height: u64,
    pub block_hash_be: String,
}
pub async fn get_confirmed_receive_by_txid(
    ctx: &CoreContext,
    address: &str,
    target_txid: &str,
) -> Result<ConfirmedReceive> {
    let mut last_seen_txid: Option<String> = None;

    loop {
        let url = if let Some(ref last) = last_seen_txid {
            format!(
                "{}/address/{}/txs/chain/{}",
                ctx.cfg.esplora_base, address, last
            )
        } else {
            format!("{}/address/{}/txs", ctx.cfg.esplora_base, address)
        };

        let client = ctx.http.clone();
        let txs: Value = client.get(&url).send().await?.json().await?;

        let arr = txs.as_array().ok_or(anyhow!("tx not found"))?;
        if arr.is_empty() {
            return Err(anyhow!("tx not found"));
        }

        for tx in arr {
            let txid = tx["txid"].as_str().unwrap_or("");

            if txid != target_txid {
                continue;
            }

            let confirmed = tx["status"]["confirmed"].as_bool().unwrap_or(false);
            if !confirmed {
                return Err(anyhow!("tx not confirmed"));
            }

            let vouts = tx["vout"].as_array().ok_or(anyhow!("malformed vout"))?;
            for (i, vout) in vouts.iter().enumerate() {
                let spk_addr = vout["scriptpubkey_address"].as_str().unwrap_or("");
                let value_sats = vout["value"].as_u64().unwrap_or(0);

                if spk_addr == address && value_sats > 0 {
                    let script_hex = vout["scriptpubkey"].as_str().unwrap_or("");

                    return Ok(ConfirmedReceive {
                        txid_be: txid.to_string(),
                        vout_index: i,
                        value_sats,
                        script_pubkey_hex: format!("0x{}", script_hex),
                        block_height: tx["status"]["block_height"].as_u64().unwrap_or(0),
                        block_hash_be: tx["status"]["block_hash"]
                            .as_str()
                            .unwrap_or("")
                            .to_string(),
                    });
                }
            }

            return Err(anyhow!("no output to address"));
        }

        let last = arr
            .last()
            .and_then(|x| x["txid"].as_str())
            .ok_or(anyhow!("missing txid in pagination"))?;

        last_seen_txid = Some(last.to_string());
    }
}

#[derive(Debug)]
pub struct ProofBundle {
    pub legacy_0x: String,
    pub vout_index: usize,
    pub block_hash_le: String,
    pub block_height: u64,
    pub branch: Vec<String>,
    pub index: u64,
}

pub async fn build_proof_bundle(
    ctx: &CoreContext,
    txid_be_expected: &str,
    vout_index: usize,
    block_hash_be: &str,
    block_height: u64,
) -> Result<ProofBundle> {
    // 1. Merkle proof (JSON)
    let proof = tx_merkle_proof(ctx, txid_be_expected).await?;

    let merkle = proof["merkle"]
        .as_array()
        .ok_or_else(|| anyhow!("missing field `merkle`"))?;

    let pos = proof["pos"]
        .as_u64()
        .ok_or_else(|| anyhow!("missing field `pos`"))?;

    // 2. Header
    let (_hash_be_check, header80) = header80_by_height(ctx, block_height).await?;
    let header80_raw = from_0x(&header80);
    let header_root_le = format!("0x{}", &header80_raw[72..136]);

    // 3. Get raw TX
    let tx_raw_original = tx_hex(ctx, txid_be_expected).await?;
    let legacy = to_legacy_serialization_strict(&tx_raw_original)?;

    // txidBE = reverse(dsha256)
    let txid_be = {
        let h = dsha256(&legacy);
        hex::encode(h.iter().rev().copied().collect::<Vec<u8>>())
    };
    let txid_le = hex_le_from_be32(&txid_be)?;

    // 4. Convert merkle branch BE → LE
    let branch_le_from_api: Vec<String> = merkle
        .iter()
        .map(|v| {
            let h = v
                .as_str()
                .ok_or_else(|| anyhow!("merkle element is not string"))?;
            hex_le_from_be32(h)
        })
        .collect::<Result<Vec<_>, _>>()?;

    // 5. Compute Merkle root
    fn compute_root(txid_le: &str, pos: u64, branch: &[String]) -> String {
        let mut h = hex::decode(from_0x(txid_le)).unwrap();
        let mut idx = pos;

        for sib in branch {
            let s = hex::decode(from_0x(sib)).unwrap();
            let concat = if idx & 1 == 1 {
                [s.as_slice(), h.as_slice()].concat()
            } else {
                [h.as_slice(), s.as_slice()].concat()
            };
            h = dsha256(&concat).to_vec();
            idx >>= 1;
        }

        format!("0x{}", hex::encode(h))
    }

    // 6. Correct branch direction if needed
    let mut branch_to_use = branch_le_from_api.clone();
    let check_root = compute_root(&txid_le, pos, &branch_le_from_api);

    if check_root != header_root_le {
        branch_to_use.reverse();
    }

    let block_hash_le = hex_le_from_be32(block_hash_be)?;

    // Return
    Ok(ProofBundle {
        legacy_0x: format!("0x{}", hex::encode(legacy)),
        vout_index,
        block_hash_le,
        block_height,
        branch: branch_to_use,
        index: pos,
    })
}

#[derive(Debug, serde:: Deserialize)]
pub struct TxStatus {
    pub confirmed: bool,

    pub block_height: Option<u64>,
    pub block_hash: Option<String>,
}

#[derive(Debug)]
pub struct BitcoinMerkleProofPayload {
    pub tx_id: U256,
    pub legacy_tx: ethers::types::Bytes,
    pub vout_index: U256,
    pub block_hash_le: [u8; 32],
    pub block_height: U256,
    pub branch: Vec<[u8; 32]>,
    pub index: U256,
}
pub async fn check_confirmation_and_build_proof(
    ctx: &CoreContext,
    tx_id: U256,
    btc_txid: &str,
) -> Result<Option<BitcoinMerkleProofPayload>> {
    // ---- 1) Check BTC confirmation ----
    let status_url = format!("{}/tx/{}/status", ctx.cfg.esplora_base, btc_txid);
    let res = ctx.http.get(&status_url).send().await?;

    if !res.status().is_success() {
        info!("Mempool API error for txid {}, will retry later", btc_txid);
        return Ok(None);
    }

    let status: TxStatus = res.json().await?;

    if !status.confirmed {
        info!("BTC tx {} not confirmed yet", btc_txid);
        return Ok(None);
    }

    info!("BTC tx {} confirmed! Building merkle proof…", btc_txid);

    // ---- 2) Extract block info ----
    let block_height = status
        .block_height
        .ok_or_else(|| anyhow!("missing block_height"))?;

    let block_hash_be = status
        .block_hash
        .as_ref()
        .ok_or_else(|| anyhow!("missing block_hash"))?;

    // ---- 3) Build proof bundle ----
    // vout_index is fixed to 0 in your tx structure
    let vout_index_usize: usize = 0;

    let proof =
        build_proof_bundle(ctx, btc_txid, vout_index_usize, block_hash_be, block_height).await?;

    // ---- 4) Convert fields to contract-ready types ----
    let legacy_tx =
        ethers::types::Bytes::from(hex::decode(proof.legacy_0x.trim_start_matches("0x"))?);

    let mut block_hash_le = [0u8; 32];
    block_hash_le.copy_from_slice(&hex::decode(proof.block_hash_le.trim_start_matches("0x"))?);

    let branch = proof
        .branch
        .iter()
        .map(|h| {
            let mut node = [0u8; 32];
            let decoded = hex::decode(h.trim_start_matches("0x"))?;
            node.copy_from_slice(&decoded);
            Ok::<_, anyhow::Error>(node)
        })
        .collect::<Result<Vec<_>>>()?;

    // ---- 5) Return payload (no side effects) ----
    Ok(Some(BitcoinMerkleProofPayload {
        tx_id,
        legacy_tx,
        vout_index: U256::from(proof.vout_index),
        block_hash_le,
        block_height: U256::from(proof.block_height),
        branch,
        index: U256::from(proof.index),
    }))
}

pub fn derive_address_from_script_bytes(
    ctx: &CoreContext,
    script_bytes: &[u8],
) -> Result<BTCAddress> {
    let script = ScriptBuf::from_bytes(script_bytes.to_vec());

    let addr = BTCAddress::from_script(&script, ctx.btc_network)
        .map_err(|e| anyhow!("cannot derive address: {}", e))?;

    Ok(addr)
}

/// Derive a P2WPKH receive address from an XPUB at m/0/<index>
pub fn derive_p2wpkh_address(
    xpub: &str,
    index: u32,
    network: Network,
) -> Result<(u32, String, Vec<u8>)> {
    // 1️⃣ Parse XPUB
    let root_xpub: Xpub = xpub.parse().map_err(|e| anyhow!("Invalid XPUB: {e}"))?;

    // 2️⃣ Derive child pubkey: m/0/<index>
    let secp = Secp256k1::verification_only();
    let path = DerivationPath::from_str(&format!("m/0/{index}"))?;

    let child_xpub = root_xpub
        .derive_pub(&secp, &path)
        .map_err(|e| anyhow!("XPUB derivation failed: {e}"))?;

    let compressed = CompressedPublicKey(child_xpub.public_key);

    // 3️⃣ Native SegWit (P2WPKH) address
    let address = BTCAddress::p2wpkh(&compressed, network);

    // 4️⃣ ScriptPubKey
    let script: ScriptBuf = address.script_pubkey();

    Ok((index, address.to_string(), script.to_bytes()))
}

#[derive(Deserialize, Debug)]
pub struct Stats {
    pub funded_txo_sum: Option<u64>,
    pub spent_txo_sum: Option<u64>,
}
#[derive(Deserialize, Debug)]
pub struct AddressInfo {
    pub chain_stats: Option<Stats>,
    pub mempool_stats: Option<Stats>,
}
pub async fn get_address_balance_sats(ctx: &CoreContext, address: &str) -> Result<u128> {
    if address.is_empty() {
        return Err(anyhow!("address empty"));
    }

    let url = format!("{}/address/{}", ctx.cfg.esplora_base, address);
    let client = &ctx.http;

    // HTTP GET
    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| anyhow!("HTTP error for {}: {}", address, e))?;

    // JSON parse
    let json: AddressInfo = resp
        .json()
        .await
        .map_err(|e| anyhow!("JSON decode error for {}: {}", address, e))?;

    let chain = json.chain_stats.unwrap_or(Stats {
        funded_txo_sum: Some(0),
        spent_txo_sum: Some(0),
    });

    let mem = json.mempool_stats.unwrap_or(Stats {
        funded_txo_sum: Some(0),
        spent_txo_sum: Some(0),
    });

    let funded = chain.funded_txo_sum.unwrap_or(0) as u128;
    let spent = chain.spent_txo_sum.unwrap_or(0) as u128;
    let mem_funded = mem.funded_txo_sum.unwrap_or(0) as u128;
    let mem_spent = mem.spent_txo_sum.unwrap_or(0) as u128;

    let total = funded - spent + (mem_funded - mem_spent);

    Ok(total)
}

pub async fn maybe_rebalance_btc_wallets(ctx: &CoreContext) -> anyhow::Result<()> {
    if ctx.cfg.btc_hot_address.is_none() || ctx.cfg.btc_main_address.is_none() {
        info!("BTC_HOT_ADDRESS / BTC_MAIN_ADDRESS not set; skipping wallet rebalance simulation.");
        return Ok(());
    }

    let hot = ctx.cfg.btc_hot_address.as_ref().unwrap().clone();
    let main = ctx.cfg.btc_main_address.as_ref().unwrap().clone();

    // Fetch balance (in sats)
    let hot_bal_sats = match get_address_balance_sats(ctx, &hot).await {
        Ok(v) => v,
        Err(e) => {
            error!(error=%e, "Failed to fetch hot wallet balance");
            return Ok(());
        }
    };

    // Convert to U256
    let hot_bal = U256::from(hot_bal_sats);

    // threshold
    let low_hot = parse_human_btc("0.005")?; // 0.005 BTC
    let high_hot = parse_human_btc("0.05")?; // 0.05 BTC

    info!(
        wallet = %hot,
        balance_btc = %ethers::utils::format_units(hot_bal, 8).unwrap(),
        "💼 Hot BTC wallet balance"
    );

    if hot_bal < low_hot {
        info!(
            low_threshold = %ethers::utils::format_units(low_hot, 8).unwrap(),
            "⚠️ Hot wallet below low threshold. Refill from exchange or main wallet → {}",
            main
        );
    } else if hot_bal > high_hot {
        info!(
            high_threshold = %ethers::utils::format_units(high_hot, 8).unwrap(),
            "⚠️ Hot wallet ABOVE high threshold. Consider sweeping excess back → {}",
            main
        );
    } else {
        info!("💼 Hot BTC wallet within desired range.");
    }

    Ok(())
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRecommended {
    pub economy_fee: f64,
}
#[derive(serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub value: u64,
}
pub async fn send_to_user_program(
    ctx: &CoreContext,
    user_program: &[u8],
    amount_sats: u64,
) -> Result<String> {
    let mempool_api = &ctx.cfg.mempool_api;
    let dev_address = &ctx.cfg.operator_btc_wallet_address;
    let network = ctx.btc_network;

    // Parse operator private key
    let wif = &ctx.cfg.operator_btc_wallet_private_key;
    let private_key = bitcoin::PrivateKey::from_wif(wif)
        .map_err(|_| anyhow::anyhow!("Invalid WIF private key"))?;
    let secret_key = private_key.inner;
    let secp = Secp256k1::new();

    // Get fee rate
    let fee_res: FeeRecommended = ctx
        .http
        .get(format!("{}/v1/fees/recommended", mempool_api))
        .send()
        .await?
        .json()
        .await?;
    let dynamic_rate = fee_res.economy_fee.ceil() as u64;
    let fee_rate = 1u64.max(dynamic_rate.min(2));

    // Get operator UTXOs
    let utxos: Vec<Utxo> = ctx
        .http
        .get(format!("{}/address/{}/utxo", mempool_api, dev_address))
        .send()
        .await?
        .json()
        .await?;
    if utxos.is_empty() {
        return Err(anyhow::anyhow!("No UTXOs available"));
    }

    // Select UTXOs and calculate fee
    let mut selected = vec![];
    let mut input_sum: u64 = 0;
    let mut final_fee: u64 = 0;

    for utxo in &utxos {
        selected.push(utxo.clone());
        input_sum += utxo.value;

        let est_vbytes = (selected.len() as u64 * 59) + (2 * 31) + 10;
        final_fee = est_vbytes * fee_rate;
        final_fee = 300.max(final_fee.min(800));

        if input_sum >= amount_sats + final_fee {
            break;
        }
    }

    if input_sum < amount_sats + final_fee {
        return Err(anyhow::anyhow!("Not enough funds"));
    }

    let change = input_sum - amount_sats - final_fee;

    // Build transaction
    let mut tx = Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![],
        output: vec![],
    };

    // Output to user program (raw script)
    tx.output.push(TxOut {
        value: Amount::from_sat(amount_sats),
        script_pubkey: ScriptBuf::from(user_program.to_vec()),
    });

    // Change output back to operator wallet
    if change > 0 {
        let change_script = BTCAddress::from_str(dev_address)?
            .require_network(network)?
            .script_pubkey();
        tx.output.push(TxOut {
            value: Amount::from_sat(change),
            script_pubkey: change_script,
        });
    }

    // Inputs
    for utxo in &selected {
        let txid = Txid::from_str(&utxo.txid)?;
        tx.input.push(TxIn {
            previous_output: OutPoint {
                txid,
                vout: utxo.vout,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
            witness: Witness::new(),
        });
    }

    // Sign inputs
    for (i, utxo) in selected.iter().enumerate() {
        let mut cache = SighashCache::new(&tx);
        let script_pubkey = BTCAddress::from_str(dev_address)?
            .require_network(network)?
            .script_pubkey();
        let sighash = cache.p2wpkh_signature_hash(
            i,
            &script_pubkey,
            Amount::from_sat(utxo.value),
            EcdsaSighashType::All,
        )?;
        let msg = Message::from_digest_slice(&sighash[..])?;
        let sig = secp.sign_ecdsa(&msg, &secret_key);

        let mut sig_ser = sig.serialize_der().to_vec();
        sig_ser.push(EcdsaSighashType::All as u8);

        tx.input[i].witness.push(sig_ser);
        tx.input[i]
            .witness
            .push(private_key.public_key(&secp).to_bytes());
    }

    // Serialize and broadcast
    let tx_hex = hex::encode(serialize(&tx));
    let response = ctx
        .http
        .post(format!("{}/tx", mempool_api))
        .header("Content-Type", "text/plain")
        .body(tx_hex)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

#[derive(Debug)]
pub struct DerivedAddressInfo {
    pub index: u32,
    pub path: String,
    pub address: String,
    pub private_key_hex: String,
    pub wif: String,
}
fn mnemonic_to_seed_unchecked(mnemonic: &str, passphrase: &str) -> [u8; 64] {
    let mut seed = [0u8; 64];
    let salt = format!("mnemonic{}", passphrase);

    pbkdf2_hmac::<Sha512>(mnemonic.as_bytes(), salt.as_bytes(), 2048, &mut seed);

    seed
}
pub async fn derive_address_from_mnemonic(
    ctx: &CoreContext,
    indices: Vec<u32>,
) -> Result<Vec<DerivedAddressInfo>> {
    let network: Network = ctx.btc_network;

    let mnemonic_str = ctx
        .cfg
        .btc_mnemonic
        .as_deref()
        .ok_or_else(|| anyhow!("btc_mnemonic not set"))?
        .trim();

    // let mnemonic = Mnemonic::parse(mnemonic_str).map_err(|e| {
    //     error!("Failed to parse mnemonic: {}", e);
    //     anyhow!("failed to parse mnemonic: {}", e)
    // })?;
    // let seed = mnemonic.to_seed("");

    // Temporarily dont check mnemonic validity
    let seed = mnemonic_to_seed_unchecked(mnemonic_str, "");
    warn!(
        "⚠️ UNSAFE MNEMONIC MODE ENABLED: BIP-39 checksum validation is DISABLED. \
     This mnemonic is NOT standards-compliant. Use only for recovery / legacy compatibility."
    );

    let secp = secp256k1::Secp256k1::new();

    let xprv = Xpriv::new_master(network, &seed).map_err(|e| {
        error!("Failed to create master xprv: {}", e);
        anyhow!("failed to create master xprv: {}", e)
    })?;

    // Coin type: 0 = mainnet, 1 = testnet
    let coin_type = match network {
        Network::Bitcoin => 0u32,
        _ => 1u32,
    };

    let mut out: Vec<DerivedAddressInfo> = Vec::with_capacity(indices.len());

    for idx in indices.into_iter() {
        let path_str = format!("m/84'/{}'/0'/0/{}", coin_type, idx);

        let derivation_path: DerivationPath = path_str.parse().map_err(|e| {
            error!("Invalid derivation path {}: {}", path_str, e);
            anyhow!("invalid derivation path {}: {}", path_str, e)
        })?;

        let child_xprv = xprv.derive_priv(&secp, &derivation_path).map_err(|e| {
            error!("derive_priv failed for {}: {}", path_str, e);
            anyhow!("derive_priv failed for {}: {}", path_str, e)
        })?;

        let secret_key = child_xprv.private_key;

        let privkey = PrivateKey {
            inner: secret_key,
            network: network.into(),
            compressed: true,
        };

        let wif = privkey.to_wif();
        let priv_hex = hex::encode(secret_key.secret_bytes());

        let pubkey = CompressedPublicKey::from_private_key(&secp, &privkey).map_err(|e| {
            error!("Failed to derive compressed pubkey for {}: {}", path_str, e);
            anyhow!("failed to derive compressed pubkey for {}: {}", path_str, e)
        })?;

        let address = BTCAddress::p2wpkh(&pubkey, network);

        out.push(DerivedAddressInfo {
            index: idx,
            path: path_str,
            address: address.to_string(),
            private_key_hex: priv_hex,
            wif,
        });
    }

    Ok(out)
}

pub async fn send_all_btc_from_account_to_dev(
    ctx: &CoreContext,
    from_address_str: &str,
    from_wif: &str,
) -> Result<String> {
    let network = ctx.btc_network;

    let from_address_unchecked = BTCAddress::from_str(from_address_str)?;
    let from_address = from_address_unchecked.require_network(network)?;

    let mempool_api = &ctx.cfg.mempool_api;
    let dev_address = &ctx.cfg.operator_btc_wallet_address;

    let private_key = bitcoin::PrivateKey::from_wif(from_wif)
        .map_err(|_| anyhow::anyhow!("Invalid WIF private key"))?;
    let secret_key = private_key.inner;
    let secp = Secp256k1::new();

    let url = format!("{}/v1/fees/recommended", mempool_api);
    let fee_res: FeeRecommended = ctx.http.get(&url).send().await?.json().await?;
    let dynamic_rate = fee_res.economy_fee.ceil() as u64;
    let fee_rate = 1u64.max(dynamic_rate.min(2));

    let url = format!("{}/address/{}/utxo", mempool_api, from_address);
    let utxos: Vec<Utxo> = ctx.http.get(&url).send().await?.json().await?;
    if utxos.is_empty() {
        return Ok(String::new());
    }

    let mut input_sum: u64 = 0;
    let mut selected: Vec<Utxo> = vec![];
    let mut final_fee: u64 = 0;

    for utxo in &utxos {
        selected.push(utxo.clone());
        input_sum += utxo.value;
        let est_vbytes = (selected.len() as u64 * 59) + 31 + 10;
        final_fee = est_vbytes * fee_rate;
        final_fee = 300.max(final_fee.min(800));
    }

    if input_sum <= final_fee {
        warn!(
            "Skipping send from {} — balance {} sats below fee {} sats",
            from_address, input_sum, final_fee
        );
        return Ok(String::new());
    }

    let send_amount = input_sum - final_fee;

    let mut tx = Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![],
        output: vec![],
    };

    let to_script = BTCAddress::from_str(dev_address)?
        .require_network(network)?
        .script_pubkey();
    tx.output.push(TxOut {
        value: Amount::from_sat(send_amount),
        script_pubkey: to_script,
    });

    for utxo in &selected {
        let txid = Txid::from_str(&utxo.txid)?;
        tx.input.push(TxIn {
            previous_output: OutPoint {
                txid,
                vout: utxo.vout,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
            witness: Witness::new(),
        });
    }

    for (i, utxo) in selected.iter().enumerate() {
        let mut cache = SighashCache::new(&tx);
        let script_pubkey = from_address.script_pubkey();
        let sighash = cache.p2wpkh_signature_hash(
            i,
            &script_pubkey,
            Amount::from_sat(utxo.value),
            EcdsaSighashType::All,
        )?;
        let msg = Message::from_digest_slice(&sighash[..])?;
        let sig = secp.sign_ecdsa(&msg, &secret_key);
        let mut sig_ser = sig.serialize_der().to_vec();
        sig_ser.push(EcdsaSighashType::All as u8);
        tx.input[i].witness.push(sig_ser);
        tx.input[i]
            .witness
            .push(private_key.public_key(&secp).to_bytes());
    }

    let tx_bytes = serialize(&tx);
    let tx_hex = hex::encode(tx_bytes);
    let url = format!("{}/tx", mempool_api);
    let response = ctx
        .http
        .post(&url)
        .header("Content-Type", "text/plain")
        .body(tx_hex)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}
