use anyhow::{Result, anyhow, bail};
use bip32::secp256k1::sha2::{Digest, Sha256, Sha512};
use bitcoin::{
    Address as BTCAddress, Amount, CompressedPublicKey, EcdsaSighashType,
    Network, OutPoint, PrivateKey, ScriptBuf, Sequence, Transaction, TxIn,
    TxOut, Txid, Witness,
    absolute::LockTime,
    bip32::{DerivationPath, Xpriv, Xpub},
    consensus::serialize,
    key::Secp256k1,
    secp256k1::{self, Message},
    sighash::SighashCache,
};
use ethers::{
    types::{Bytes as TypesBytes, U256},
    utils::{hex, parse_units},
};
use pbkdf2::pbkdf2_hmac;
use serde::Deserialize;
use serde_json::Value;
use std::str::FromStr;
use tracing::{error, info, warn};

use crate::dependencies::context::CoreContext;

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
    if s.starts_with("0x") { s.to_owned() } else { format!("0x{}", s) }
}

pub async fn header80_by_height(
    ctx: &CoreContext,
    height: u64,
) -> Result<(String, String)> {
    // 1. Acquire the permit from the global rate limiter
    let _permit = ctx.rpc_limiter.acquire().await.map_err(|e| {
        anyhow::anyhow!("Failed to acquire RPC permit for header fetch: {}", e)
    })?;

    // Define the available endpoints for rotation
    let endpoints = [&ctx.cfg.mempool_api, &ctx.cfg.esplora_base];
    let mut last_error = None;

    for base_url in endpoints {
        // 2. GET block hash by height (BE)
        let url_hash = format!("{}/block-height/{}", base_url, height);
        let block_hash_be_res = ctx.http.get(&url_hash).send().await;

        let block_hash_be = match block_hash_be_res {
            Ok(resp) => match resp.text().await {
                Ok(t) => t.trim().to_string(),
                Err(e) => {
                    last_error = Some(e.into());
                    continue;
                },
            },
            Err(e) => {
                last_error = Some(e.into());
                continue;
            },
        };

        // 3. GET block header hex
        let url_header = format!("{}/block/{}/header", base_url, block_hash_be);
        let header_hex_res = ctx.http.get(&url_header).send().await;

        let header_hex = match header_hex_res {
            Ok(resp) => match resp.text().await {
                Ok(t) => t.trim().to_string(),
                Err(e) => {
                    last_error = Some(e.into());
                    continue;
                },
            },
            Err(e) => {
                last_error = Some(e.into());
                continue;
            },
        };

        // 4. Validate length (80 bytes → 160 hex chars)
        if header_hex.len() != 160 {
            last_error = Some(anyhow!(
                "header80 hex has {} chars, expected 160",
                header_hex.len()
            ));
            continue;
        }

        // 5. Ensure "0x" prefix
        let header80_prefixed = if header_hex.starts_with("0x") {
            header_hex
        } else {
            format!("0x{}", header_hex)
        };

        // 6. Mandatory 250ms gap enforcement before releasing the permit.
        // This protects the Esplora API from rapid-fire sequential calls
        // from different worker tasks.
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;

        return Ok((block_hash_be, header80_prefixed));
    }

    // If we exhausted all endpoints, return the last error encountered
    Err(last_error.unwrap_or_else(|| anyhow!("All RPC endpoints failed")))
}

pub async fn btc_tip_height(ctx: &CoreContext) -> Result<u64> {
    // Define the available endpoints for rotation
    let endpoints = [&ctx.cfg.mempool_api, &ctx.cfg.esplora_base];
    let mut last_error = None;

    for base_url in endpoints {
        let url = format!("{}/blocks/tip/height", base_url);

        // Fetch text from the endpoint
        let response = match ctx.http.get(&url).send().await {
            Ok(res) => res,
            Err(e) => {
                last_error = Some(anyhow::anyhow!(e));
                continue;
            },
        };

        let text = match response.text().await {
            Ok(t) => t,
            Err(e) => {
                last_error = Some(anyhow::anyhow!(e));
                continue;
            },
        };

        let trimmed = text.trim();

        // Parse height as number and sub with 6 as high finality confirmed block
        let height_res = trimmed.parse::<u64>();

        match height_res {
            Ok(h) => {
                let height = h.saturating_sub(
                    ctx.cfg.high_finality_confirmed_block as u64,
                );
                return Ok(height);
            },
            Err(e) => {
                last_error = Some(anyhow::anyhow!(
                    "Failed to parse height '{}': {}",
                    trimmed,
                    e
                ));
                continue;
            },
        }
    }

    // If we exhausted all endpoints, return the last error encountered
    Err(last_error.unwrap_or_else(|| {
        anyhow::anyhow!("All RPC endpoints failed to fetch tip height")
    }))
}

pub fn dsha256(buf: &[u8]) -> [u8; 32] {
    let first = Sha256::digest(buf);
    let second = Sha256::digest(first);
    let mut out = [0u8; 32];
    out.copy_from_slice(&second);
    out
}

pub fn header_hash_both_from_header80(
    header80_0x: &str,
) -> Result<(String, String)> {
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

pub fn decode_header80(header80_hex: &str) -> Result<TypesBytes, String> {
    let clean = header80_hex.trim_start_matches("0x");

    let decoded = match hex::decode(clean) {
        Ok(bytes) => bytes,
        Err(_) => {
            return Err(format!("invalid hex in header80: {header80_hex}"));
        },
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

pub async fn tx_merkle_proof(
    ctx: &CoreContext,
    txid_be: &str,
) -> Result<Value> {
    // 1. Acquire permit from the global rate limiter
    let _permit = ctx.rpc_limiter.acquire().await.map_err(|e| {
        anyhow::anyhow!("Failed to acquire RPC permit for merkle proof: {}", e)
    })?;

    // Define the available endpoints, prioritizing mempool_api
    let endpoints = [&ctx.cfg.mempool_api, &ctx.cfg.esplora_base];
    let mut last_error = None;

    for base_url in endpoints {
        let url = format!("{}/tx/{}/merkle-proof", base_url, txid_be);

        // Fetch JSON from the endpoint
        let response = match ctx.http.get(url).send().await {
            Ok(res) => res,
            Err(e) => {
                last_error = Some(anyhow::anyhow!(e));
                continue;
            },
        };

        let json_res = response.json::<Value>().await;

        match json_res {
            Ok(json) => {
                // 2. Mandatory 250ms gap before releasing the permit
                tokio::time::sleep(tokio::time::Duration::from_millis(250))
                    .await;
                return Ok(json);
            },
            Err(e) => {
                last_error = Some(anyhow::anyhow!(e));
                continue;
            },
        }
    }

    // If we exhausted all endpoints, return the last error encountered
    Err(last_error.unwrap_or_else(|| {
        anyhow::anyhow!("All RPC endpoints failed to fetch merkle proof")
    }))
}

pub async fn tx_hex(ctx: &CoreContext, txid_be: &str) -> Result<String> {
    // 1. Acquire permit from the global rate limiter
    let _permit = ctx.rpc_limiter.acquire().await.map_err(|e| {
        anyhow::anyhow!("Failed to acquire RPC permit for tx hex: {}", e)
    })?;

    // Define the available endpoints, prioritizing mempool_api
    let endpoints = [&ctx.cfg.mempool_api, &ctx.cfg.esplora_base];
    let mut last_error = None;

    for base_url in endpoints {
        let url = format!("{}/tx/{}/hex", base_url, txid_be);

        // Fetch text from the endpoint
        let response = match ctx.http.get(url).send().await {
            Ok(res) => res,
            Err(e) => {
                last_error = Some(anyhow::anyhow!(e));
                continue;
            },
        };

        let hex_res = response.text().await;

        match hex_res {
            Ok(hex) => {
                // 2. Mandatory 250ms gap before releasing the permit
                tokio::time::sleep(tokio::time::Duration::from_millis(250))
                    .await;
                return Ok(format!("0x{}", hex.trim()));
            },
            Err(e) => {
                last_error = Some(anyhow::anyhow!(e));
                continue;
            },
        }
    }

    // If we exhausted all endpoints, return the last error encountered
    Err(last_error.unwrap_or_else(|| {
        anyhow::anyhow!("All RPC endpoints failed to fetch tx hex")
    }))
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
        },

        0xFE => {
            let mut v = 0u64;
            for i in 0..4 {
                v |= (*buf.get(o + 1 + i).ok_or(anyhow!("varint: 0xfe"))?
                    as u64)
                    << (8 * i);
            }
            (v, o + 5)
        },

        0xFF => {
            let mut v = 0u64;
            for i in 0..8 {
                v |= (*buf.get(o + 1 + i).ok_or(anyhow!("varint: 0xff"))?
                    as u64)
                    << (8 * i);
            }
            (v, o + 9)
        },

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
        // Acquire permit and enforce gap per pagination request
        let _permit = ctx
            .rpc_limiter
            .acquire()
            .await
            .map_err(|e| anyhow!("Failed to acquire RPC permit: {}", e))?;

        // Define the available endpoints, prioritizing mempool_api
        let endpoints = [&ctx.cfg.mempool_api, &ctx.cfg.esplora_base];
        let mut txs: Option<Value> = None;
        let mut last_error = None;

        for base_url in endpoints {
            let url = if let Some(ref last) = last_seen_txid {
                format!("{}/address/{}/txs/chain/{}", base_url, address, last)
            } else {
                format!("{}/address/{}/txs", base_url, address)
            };

            let client = ctx.http.clone();
            match client.get(&url).send().await {
                Ok(resp) => {
                    match resp.json::<Value>().await {
                        Ok(json) => {
                            txs = Some(json);
                            break; // Success! Break out of the endpoint rotation loop
                        },
                        Err(e) => last_error = Some(anyhow!(e)),
                    }
                },
                Err(e) => last_error = Some(anyhow!(e)),
            }
        }

        // If no endpoint worked for this page, return the last error
        let txs_val = txs.ok_or_else(|| {
            last_error.unwrap_or_else(|| {
                anyhow!(
                    "All RPC endpoints failed to fetch address transactions"
                )
            })
        })?;

        let arr = txs_val.as_array().ok_or(anyhow!("tx not found"))?;
        if arr.is_empty() {
            return Err(anyhow!("tx not found"));
        }

        for tx in arr {
            let txid = tx["txid"].as_str().unwrap_or("");

            if txid != target_txid {
                continue;
            }

            let confirmed =
                tx["status"]["confirmed"].as_bool().unwrap_or(false);
            if !confirmed {
                return Err(anyhow!("tx not confirmed"));
            }

            let vouts =
                tx["vout"].as_array().ok_or(anyhow!("malformed vout"))?;
            for (i, vout) in vouts.iter().enumerate() {
                let spk_addr =
                    vout["scriptpubkey_address"].as_str().unwrap_or("");
                let value_sats = vout["value"].as_u64().unwrap_or(0);

                if spk_addr == address && value_sats > 0 {
                    let script_hex =
                        vout["scriptpubkey"].as_str().unwrap_or("");

                    return Ok(ConfirmedReceive {
                        txid_be: txid.to_string(),
                        vout_index: i,
                        value_sats,
                        script_pubkey_hex: format!("0x{}", script_hex),
                        block_height: tx["status"]["block_height"]
                            .as_u64()
                            .unwrap_or(0),
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

        // Hold permit for 250ms before looping for the next page
        tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;
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

    let pos =
        proof["pos"].as_u64().ok_or_else(|| anyhow!("missing field `pos`"))?;

    // 2. Header
    let (_hash_be_check, header80) =
        header80_by_height(ctx, block_height).await?;
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
    pub mempool_height: Option<u64>,
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
    pub mempool_height: Option<u64>,
}

pub async fn check_confirmation_and_build_proof(
    ctx: &CoreContext,
    tx_id: U256,
    btc_txid: &str,
) -> Result<Option<BitcoinMerkleProofPayload>> {
    // ---- 1) Check BTC confirmation ----
    let status = {
        let _permit = ctx.rpc_limiter.acquire().await?;
        let endpoints = [&ctx.cfg.mempool_api, &ctx.cfg.esplora_base];
        let mut result = None;

        for base_url in endpoints {
            let status_url = format!("{}/tx/{}/status", base_url, btc_txid);
            if let Ok(res) = ctx.http.get(&status_url).send().await {
                if res.status().is_success() {
                    if let Ok(s) = res.json::<TxStatus>().await {
                        result = Some(s);
                        break;
                    }
                }
            }
        }

        match result {
            Some(s) => {
                tokio::time::sleep(tokio::time::Duration::from_millis(250))
                    .await;
                s
            },
            None => {
                info!(
                    "All RPC APIs error for txid:{} btc_txid {}, will retry later",
                    tx_id, btc_txid
                );
                tokio::time::sleep(tokio::time::Duration::from_millis(250))
                    .await;
                return Ok(None);
            },
        }
    };

    if !status.confirmed {
        let endpoints = [&ctx.cfg.mempool_api, &ctx.cfg.esplora_base];

        // --- Fetch TX Data for Fee/Weight ---
        let mut tx_res_val = None;
        for base_url in endpoints {
            let tx_url = format!("{}/tx/{}", base_url, btc_txid);
            if let Ok(res) = ctx.http.get(&tx_url).send().await {
                if let Ok(json) = res.json::<serde_json::Value>().await {
                    tx_res_val = Some(json);
                    break;
                }
            }
        }
        let tx_res = tx_res_val.ok_or_else(|| {
            anyhow!("Failed to fetch tx data from all endpoints")
        })?;

        let fee = tx_res["fee"].as_f64().unwrap_or(0.0);
        let weight = tx_res["weight"].as_f64().unwrap_or(1.0);
        let fee_rate = fee / (weight / 4.0);

        // --- Fetch Mempool Fee Estimations ---
        let mut mempool_data_val = None;
        for base_url in endpoints {
            let blocks_url = format!("{}/v1/fees/mempool-blocks", base_url);
            if let Ok(res) = ctx.http.get(&blocks_url).send().await {
                if let Ok(json) = res.json::<Vec<serde_json::Value>>().await {
                    mempool_data_val = Some(json);
                    break;
                }
            }
        }
        let mempool_data = mempool_data_val.ok_or_else(|| {
            anyhow!("Failed to fetch mempool blocks from all endpoints")
        })?;

        let mut eta_blocks = 0;
        for block in mempool_data {
            eta_blocks += 1;
            if block["feeRange"]
                .as_array()
                .and_then(|f| f.first()?.as_f64())
                .is_some_and(|min_fee| fee_rate >= min_fee)
            {
                break;
            }
        }

        let tip_height = btc_tip_height(ctx).await?;
        let eta_block_confirmation_height = tip_height + eta_blocks;

        return Ok(Some(BitcoinMerkleProofPayload {
            tx_id,
            legacy_tx: ethers::types::Bytes::new(),
            vout_index: U256::zero(),
            block_hash_le: [0u8; 32],
            block_height: U256::zero(),
            branch: vec![],
            index: U256::zero(),
            mempool_height: Some(eta_block_confirmation_height),
        }));
    }

    info!("BTC tx {} confirmed! Building merkle proof…", btc_txid);

    // ---- 2) Extract block info ----
    let block_height =
        status.block_height.ok_or_else(|| anyhow!("missing block_height"))?;
    let block_hash_be = status
        .block_hash
        .as_ref()
        .ok_or_else(|| anyhow!("missing block_hash"))?;

    // ---- 3) Build proof bundle ----
    let vout_index_usize: usize = 0;
    let proof = build_proof_bundle(
        ctx,
        btc_txid,
        vout_index_usize,
        block_hash_be,
        block_height,
    )
    .await?;

    // ---- 4) Convert fields to contract-ready types ----
    let legacy_tx = ethers::types::Bytes::from(hex::decode(
        proof.legacy_0x.trim_start_matches("0x"),
    )?);
    let mut block_hash_le = [0u8; 32];
    block_hash_le.copy_from_slice(&hex::decode(
        proof.block_hash_le.trim_start_matches("0x"),
    )?);

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

    Ok(Some(BitcoinMerkleProofPayload {
        tx_id,
        legacy_tx,
        vout_index: U256::from(proof.vout_index),
        block_hash_le,
        block_height: U256::from(proof.block_height),
        branch,
        index: U256::from(proof.index),
        mempool_height: status.mempool_height,
    }))
}

/// Derive a P2WPKH receive address from an XPUB at m/0/<index>
pub fn derive_p2wpkh_address(
    xpub: &str,
    index: u32,
    network: Network,
) -> Result<(u32, String, Vec<u8>)> {
    // 1️⃣ Parse XPUB
    let root_xpub: Xpub =
        xpub.parse().map_err(|e| anyhow!("Invalid XPUB: {e}"))?;

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

pub async fn get_address_balance_sats(
    ctx: &CoreContext,
    address: &str,
) -> Result<u128> {
    if address.is_empty() {
        return Err(anyhow!("address empty"));
    }

    // Define endpoints with Mempool first
    let endpoints = [&ctx.cfg.mempool_api, &ctx.cfg.esplora_base];
    let mut last_error = None;
    let mut address_info: Option<AddressInfo> = None;

    for base_url in endpoints {
        let url = format!("{}/address/{}", base_url, address);

        // Attempt HTTP GET
        let resp = match ctx.http.get(&url).send().await {
            Ok(r) => r,
            Err(e) => {
                last_error = Some(anyhow!("HTTP error for {}: {}", address, e));
                continue;
            },
        };

        // Attempt JSON parse
        match resp.json::<AddressInfo>().await {
            Ok(json) => {
                address_info = Some(json);
                break;
            },
            Err(e) => {
                last_error =
                    Some(anyhow!("JSON decode error for {}: {}", address, e));
                continue;
            },
        }
    }

    // Ensure we actually got data
    let json = address_info.ok_or_else(|| {
        last_error.unwrap_or_else(|| {
            anyhow!("All RPC endpoints failed for address {}", address)
        })
    })?;

    let chain = json
        .chain_stats
        .unwrap_or(Stats { funded_txo_sum: Some(0), spent_txo_sum: Some(0) });

    let mem = json
        .mempool_stats
        .unwrap_or(Stats { funded_txo_sum: Some(0), spent_txo_sum: Some(0) });

    let funded = chain.funded_txo_sum.unwrap_or(0) as u128;
    let spent = chain.spent_txo_sum.unwrap_or(0) as u128;
    let mem_funded = mem.funded_txo_sum.unwrap_or(0) as u128;
    let mem_spent = mem.spent_txo_sum.unwrap_or(0) as u128;

    // Standard UTXO balance calculation: (Confirmed In - Out) + (Unconfirmed In - Out)
    let total = funded - spent + (mem_funded - mem_spent);

    Ok(total)
}
pub async fn maybe_rebalance_btc_wallets(
    ctx: &CoreContext,
) -> anyhow::Result<()> {
    if ctx.cfg.btc_hot_address.is_none() || ctx.cfg.btc_main_address.is_none() {
        info!(
            "BTC_HOT_ADDRESS / BTC_MAIN_ADDRESS not set; skipping wallet rebalance simulation."
        );
        return Ok(());
    }

    let hot = ctx.cfg.btc_hot_address.as_ref().unwrap().clone();
    let main = ctx.cfg.btc_main_address.as_ref().unwrap().clone();

    // Fetch balance (in sats)
    // Note: get_address_balance_sats now rotates between Mempool and Esplora automatically
    let hot_bal_sats = match get_address_balance_sats(ctx, &hot).await {
        Ok(v) => v,
        Err(e) => {
            // If both RPCs fail, we log and exit gracefully to avoid crashing the worker
            error!(error=%e, "Failed to fetch hot wallet balance from all RPC providers");
            return Ok(());
        },
    };

    // Convert to U256 (Assuming 1e8 for BTC sats)
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
    pub half_hour_fee: f64,
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
    let _permit = ctx.rpc_limiter.acquire().await.map_err(|e| {
        anyhow::anyhow!("Failed to acquire RPC permit for broadcast: {}", e)
    })?;

    let dev_address = &ctx.cfg.operator_btc_wallet_address;
    let network = ctx.btc_network;
    let endpoints = [&ctx.cfg.mempool_api, &ctx.cfg.esplora_base];

    // Parse operator private key
    let wif = &ctx.cfg.operator_btc_wallet_private_key;
    let private_key = bitcoin::PrivateKey::from_wif(wif)
        .map_err(|_| anyhow::anyhow!("Invalid WIF private key"))?;
    let secret_key = private_key.inner;
    let secp = Secp256k1::new();

    // --- 1. Get operator UTXOs (With Rotation) ---
    let mut utxos: Option<Vec<Utxo>> = None;
    let mut utxo_err: Option<String> = None;

    for base_url in endpoints {
        let url = format!("{}/address/{}/utxo", base_url, dev_address);
        match ctx.http.get(&url).send().await {
            Ok(resp) => {
                if let Ok(v) = resp.json::<Vec<Utxo>>().await {
                    utxos = Some(v);
                    break;
                } else {
                    utxo_err = Some("JSON decode failed".to_string());
                }
            },
            Err(e) => utxo_err = Some(e.to_string()),
        }
    }

    let utxos = utxos.ok_or_else(|| {
        anyhow!(
            "UTXO fetch failed. Last error: {}",
            utxo_err.unwrap_or_default()
        )
    })?;

    if utxos.is_empty() {
        return Err(anyhow::anyhow!("No UTXOs available"));
    }

    // --- 2. Select UTXOs and calculate fee ---
    let mut selected = vec![];
    let mut input_sum: u64 = 0;
    let final_fee: u64 = 120; // Static fee

    for utxo in &utxos {
        selected.push(utxo.clone());
        input_sum += utxo.value;

        if input_sum >= amount_sats + final_fee {
            break;
        }
    }

    if input_sum < amount_sats + final_fee {
        return Err(anyhow::anyhow!("Not enough funds"));
    }

    let change = input_sum - amount_sats - final_fee;

    // --- 3. Build transaction ---
    let mut tx = Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: LockTime::ZERO,
        input: vec![],
        output: vec![],
    };

    tx.output.push(TxOut {
        value: Amount::from_sat(amount_sats),
        script_pubkey: ScriptBuf::from(user_program.to_vec()),
    });

    if change > 0 {
        let change_script = BTCAddress::from_str(dev_address)?
            .require_network(network)?
            .script_pubkey();
        tx.output.push(TxOut {
            value: Amount::from_sat(change),
            script_pubkey: change_script,
        });
    }

    for utxo in &selected {
        let txid = Txid::from_str(&utxo.txid)?;
        tx.input.push(TxIn {
            previous_output: OutPoint { txid, vout: utxo.vout },
            script_sig: ScriptBuf::new(),
            sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
            witness: Witness::new(),
        });
    }

    // --- 4. Sign inputs ---
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
        tx.input[i].witness.push(private_key.public_key(&secp).to_bytes());
    }

    // --- 5. Serialize and Broadcast (With Rotation) ---
    let tx_hex = hex::encode(serialize(&tx));
    let mut broadcast_resp = None;
    let mut broadcast_err: Option<String> = None;

    for base_url in endpoints {
        let url = format!("{}/tx", base_url);
        // We clone tx_hex so it's available for the next iteration if the first fails
        match ctx
            .http
            .post(&url)
            .header("Content-Type", "text/plain")
            .body(tx_hex.clone())
            .send()
            .await
        {
            Ok(res) => {
                if res.status().is_success() {
                    if let Ok(text) = res.text().await {
                        broadcast_resp = Some(text);
                        break;
                    }
                } else {
                    let status = res.status();
                    let body = res
                        .text()
                        .await
                        .unwrap_or_else(|_| "No body".to_string());
                    broadcast_err = Some(format!("HTTP {}: {}", status, body));
                }
            },
            Err(e) => broadcast_err = Some(e.to_string()),
        }
    }

    let response = broadcast_resp.ok_or_else(|| {
        anyhow!(
            "Broadcast failed on all endpoints. Last error: {}",
            broadcast_err.unwrap_or_default()
        )
    })?;

    tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;

    Ok(response)
}

pub async fn rbf_send_to_user_program(
    ctx: &CoreContext,
    original_txid: &str,
    user_program: &[u8],
    amount_sats: u64,
) -> Result<String> {
    let _permit = ctx.rpc_limiter.acquire().await?;
    let dev_address = &ctx.cfg.operator_btc_wallet_address;
    let network = ctx.btc_network;
    let endpoints = [&ctx.cfg.mempool_api, &ctx.cfg.esplora_base];

    let wif = &ctx.cfg.operator_btc_wallet_private_key;
    let private_key = bitcoin::PrivateKey::from_wif(wif)?;
    let secret_key = private_key.inner;
    let secp = Secp256k1::new();

    // --- 1. Fetch original TX (With Rotation) ---
    let mut old_tx_opt: Option<Transaction> = None;
    let mut last_err: Option<String> = None;

    for base_url in endpoints {
        let url = format!("{}/tx/{}/hex", base_url, original_txid);
        if let Ok(res) = ctx.http.get(&url).send().await {
            if let Ok(hex_str) = res.text().await {
                if let Ok(bytes) = hex::decode(hex_str.trim()) {
                    if let Ok(tx) =
                        bitcoin::consensus::encode::deserialize(&bytes)
                    {
                        old_tx_opt = Some(tx);
                        break;
                    }
                }
            }
        }
        last_err = Some(format!(
            "Failed to fetch/parse original tx from {}",
            base_url
        ));
    }
    let old_tx =
        old_tx_opt.ok_or_else(|| anyhow!(last_err.unwrap_or_default()))?;

    // --- 2. Fetch UTXO values for old fee calc (With Rotation) ---
    let mut selected = vec![];
    let mut input_sum: u64 = 0;
    for input in &old_tx.input {
        let prev_txid = input.previous_output.txid.to_string();
        let prev_vout = input.previous_output.vout;

        let mut value_opt: Option<u64> = None;
        for base_url in endpoints {
            let url = format!("{}/tx/{}", base_url, prev_txid);
            if let Ok(res) = ctx.http.get(&url).send().await {
                if let Ok(tx_val) = res.json::<serde_json::Value>().await {
                    if let Some(v) =
                        tx_val["vout"][prev_vout as usize]["value"].as_u64()
                    {
                        value_opt = Some(v);
                        break;
                    }
                }
            }
        }
        let value = value_opt.ok_or_else(|| {
            anyhow!("Could not fetch input value for {}", prev_txid)
        })?;
        selected.push(Utxo { txid: prev_txid, vout: prev_vout, value });
        input_sum += value;
    }

    // --- 3. Fee Bump Calculation (With Rotation for Recommended Fees) ---
    let current_output_sum: u64 =
        old_tx.output.iter().map(|o| o.value.to_sat()).sum();
    let old_absolute_fee = input_sum.saturating_sub(current_output_sum);
    let old_vsize = old_tx.vsize() as f64;
    let old_fee_rate = old_absolute_fee as f64 / old_vsize as f64;

    let mut fee_res_opt: Option<FeeRecommended> = None;
    for base_url in endpoints {
        let url = format!("{}/v1/fees/recommended", base_url);
        if let Ok(res) = ctx.http.get(&url).send().await {
            if let Ok(fees) = res.json::<FeeRecommended>().await {
                fee_res_opt = Some(fees);
                break;
            }
        }
    }
    let fee_res = fee_res_opt
        .ok_or_else(|| anyhow!("Could not fetch fee recommendations"))?;

    let market_rate = fee_res.economy_fee;
    let new_fee_rate = if market_rate > old_fee_rate {
        market_rate
    } else {
        old_fee_rate + 0.1
    };

    info!(
        "RBF: Old rate: {} sat/vB. Market: {} sat/vB. New rate: {} sat/vB.",
        old_fee_rate, market_rate, new_fee_rate
    );

    let final_fee = (old_vsize * new_fee_rate).ceil() as u64;
    let change =
        input_sum.checked_sub(amount_sats + final_fee).ok_or_else(|| {
            anyhow!("Insufficient funds: Fee bump would deplete change output")
        })?;

    // --- 4. Rebuild ---
    let mut tx = Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: LockTime::ZERO,
        input: old_tx.input.clone(),
        output: vec![],
    };

    tx.output.push(TxOut {
        value: Amount::from_sat(amount_sats),
        script_pubkey: ScriptBuf::from(user_program.to_vec()),
    });

    if change > 0 {
        let change_script = BTCAddress::from_str(dev_address)?
            .require_network(network)?
            .script_pubkey();
        tx.output.push(TxOut {
            value: Amount::from_sat(change),
            script_pubkey: change_script,
        });
    }

    // --- 5. Sign ---
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
        tx.input[i].witness.clear();
        tx.input[i].witness.push(sig_ser);
        tx.input[i].witness.push(private_key.public_key(&secp).to_bytes());
    }

    // --- 6. Broadcast (With Rotation) ---
    let tx_hex = hex::encode(serialize(&tx));
    let mut broadcast_resp = None;
    for base_url in endpoints {
        let url = format!("{}/tx", base_url);
        if let Ok(res) = ctx
            .http
            .post(&url)
            .header("Content-Type", "text/plain")
            .body(tx_hex.clone())
            .send()
            .await
        {
            if res.status().is_success() {
                if let Ok(text) = res.text().await {
                    broadcast_resp = Some(text);
                    break;
                }
            }
        }
    }

    broadcast_resp.ok_or_else(|| {
        anyhow!("Failed to broadcast RBF transaction to all endpoints")
    })
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

    pbkdf2_hmac::<Sha512>(
        mnemonic.as_bytes(),
        salt.as_bytes(),
        2048,
        &mut seed,
    );

    seed
}

pub async fn derive_address_from_mnemonic(
    ctx: &CoreContext,
    mnemonic_str: &str,
    indices: Vec<u32>,
) -> Result<Vec<DerivedAddressInfo>> {
    let network: Network = ctx.btc_network;

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

        let derivation_path: DerivationPath =
            path_str.parse().map_err(|e| {
                error!("Invalid derivation path {}: {}", path_str, e);
                anyhow!("invalid derivation path {}: {}", path_str, e)
            })?;

        let child_xprv =
            xprv.derive_priv(&secp, &derivation_path).map_err(|e| {
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

        let pubkey = CompressedPublicKey::from_private_key(&secp, &privkey)
            .map_err(|e| {
                error!(
                    "Failed to derive compressed pubkey for {}: {}",
                    path_str, e
                );
                anyhow!(
                    "failed to derive compressed pubkey for {}: {}",
                    path_str,
                    e
                )
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

use tokio::time::{Duration, timeout};

pub async fn sweep_btc_to_main(
    ctx: &CoreContext,
    mnemonic_str: &str,
    start_idx: u32,
    end_idx: u32,
) -> anyhow::Result<(String, u32)> {
    // --- CONFIGURABLE SETTINGS ---
    let sweep_threshold = 1;
    let max_retries = 3;
    // ----------------------------

    let network = ctx.btc_network;
    let dev_address_str = &ctx.cfg.operator_btc_wallet_address;
    let endpoints = [&ctx.cfg.mempool_api, &ctx.cfg.esplora_base];
    let secp = secp256k1::Secp256k1::new();

    let seed = mnemonic_to_seed_unchecked(mnemonic_str, "");
    let xprv = bitcoin::bip32::Xpriv::new_master(network, &seed)?;
    let coin_type =
        if network == bitcoin::Network::Bitcoin { 0u32 } else { 1u32 };

    let mut all_inputs = Vec::new();
    let mut total_balance: u64 = 0;
    let mut addresses_with_funds_count = 0;

    let mut first_funded_idx: Option<u32> = None;
    let mut last_processed_idx = start_idx;
    let mut api_failed = false;

    info!(
        start_idx,
        end_idx, sweep_threshold, "Starting BTC sweep with checkpoint logic..."
    );

    'outer: for chain in [0u32, 1u32] {
        for idx in start_idx..end_idx {
            let path_str = format!("m/84'/{}'/0'/{}/{}", coin_type, chain, idx);
            let derivation_path: bitcoin::bip32::DerivationPath =
                path_str.parse()?;
            let child_xprv = xprv.derive_priv(&secp, &derivation_path)?;

            let priv_key = bitcoin::PrivateKey {
                inner: child_xprv.private_key,
                network: network.into(),
                compressed: true,
            };

            let pubkey = bitcoin::CompressedPublicKey::from_private_key(
                &secp, &priv_key,
            )
            .map_err(|e| anyhow::anyhow!("Pubkey derivation failed: {}", e))?;

            let address = bitcoin::Address::p2wpkh(&pubkey, network);

            let mut utxos: Vec<Utxo> = vec![];
            let mut fetch_success = false;

            for _attempt in 1..=max_retries {
                for base_url in endpoints {
                    let url = format!("{}/address/{}/utxo", base_url, address);
                    let req_future = ctx.http.get(&url).send();

                    match timeout(Duration::from_secs(5), req_future).await {
                        Ok(Ok(resp)) if resp.status().is_success() => {
                            if let Ok(v) = resp.json::<Vec<Utxo>>().await {
                                utxos = v;
                                fetch_success = true;
                                break;
                            }
                        },
                        _ => continue,
                    }
                }
                if fetch_success {
                    break;
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }

            if !fetch_success {
                warn!(idx, %address, "UTXO fetch failed. Stopping scan at this checkpoint.");
                api_failed = true;
                break 'outer; // Break out of both loops immediately
            }

            // Mark this index as successfully scanned
            last_processed_idx = idx;

            if !utxos.is_empty() {
                addresses_with_funds_count += 1;
                if first_funded_idx.is_none() {
                    first_funded_idx = Some(idx);
                }
                for utxo in utxos {
                    total_balance += utxo.value;
                    all_inputs.push((utxo, priv_key, address.script_pubkey()));
                }
            }
        }
    }

    // --- DECISION LOGIC ---

    // 1. If we found some money but didn't hit the threshold
    if !all_inputs.is_empty() && addresses_with_funds_count < sweep_threshold {
        let resume_at = first_funded_idx.unwrap_or(start_idx);
        info!(
            found = addresses_with_funds_count,
            resume_at, "Threshold not met. Retrying from first funded index."
        );
        return Ok((String::new(), resume_at));
    }

    // 2. If no funds were found at all (and we might have stopped early due to API fail)
    if all_inputs.is_empty() {
        // If API failed, return an Error so the caller knows we didn't actually finish.
        if api_failed {
            anyhow::bail!(
                "Scan aborted due to API failure at index {}",
                last_processed_idx
            );
        }
        return Ok((String::new(), end_idx));
    }

    // 3. Threshold is met (3+) - Proceed to sweep
    info!(total_sats = total_balance, "Threshold met. Sweeping to main...");

    let mut fee_rate = 1u64;
    let mut fee_fetch_success = false;
    for _ in 1..=max_retries {
        for base_url in endpoints {
            let url = format!("{}/v1/fees/recommended", base_url);
            if let Ok(Ok(resp)) =
                timeout(Duration::from_secs(5), ctx.http.get(&url).send()).await
            {
                if let Ok(res) = resp.json::<FeeRecommended>().await {
                    fee_rate = (res.economy_fee.ceil() as u64).max(1);
                    fee_fetch_success = true;
                    break;
                }
            }
        }
        if fee_fetch_success {
            break;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    if !fee_fetch_success {
        // Fallback: don't error, just return the funded checkpoint to try again later
        return Ok((String::new(), first_funded_idx.unwrap_or(start_idx)));
    }

    let mut tx = bitcoin::Transaction {
        version: bitcoin::transaction::Version::TWO,
        lock_time: bitcoin::absolute::LockTime::ZERO,
        input: vec![],
        output: vec![],
    };

    let est_vbytes = 10 + (all_inputs.len() as u64 * 68) + 31;
    let total_fee = est_vbytes * fee_rate;
    if total_balance <= total_fee + 546 {
        // Log specifically that it's a dust/fee issue
        warn!(
            total_balance,
            total_fee,
            "Balance too low to cover fees. Returning to checkpoint."
        );
        return Ok((String::new(), first_funded_idx.unwrap_or(start_idx)));
    }

    let send_amount = total_balance - total_fee;
    let to_address = bitcoin::Address::from_str(dev_address_str)?
        .require_network(network)?;

    tx.output.push(bitcoin::TxOut {
        value: bitcoin::Amount::from_sat(send_amount),
        script_pubkey: to_address.script_pubkey(),
    });

    for (utxo, _, _) in &all_inputs {
        tx.input.push(bitcoin::TxIn {
            previous_output: bitcoin::OutPoint {
                txid: bitcoin::Txid::from_str(&utxo.txid)?,
                vout: utxo.vout,
            },
            script_sig: bitcoin::ScriptBuf::new(),
            sequence: bitcoin::Sequence::ENABLE_RBF_NO_LOCKTIME,
            witness: bitcoin::Witness::new(),
        });
    }

    for (i, (utxo, priv_key, script_pubkey)) in all_inputs.iter().enumerate() {
        let mut cache = bitcoin::sighash::SighashCache::new(&tx);
        let sighash = cache.p2wpkh_signature_hash(
            i,
            script_pubkey,
            bitcoin::Amount::from_sat(utxo.value),
            bitcoin::EcdsaSighashType::All,
        )?;
        let msg = secp256k1::Message::from_digest_slice(&sighash[..])?;
        let sig = secp.sign_ecdsa(&msg, &priv_key.inner);
        let mut sig_ser = sig.serialize_der().to_vec();
        sig_ser.push(bitcoin::EcdsaSighashType::All as u8);
        tx.input[i].witness.push(sig_ser);
        tx.input[i].witness.push(priv_key.public_key(&secp).to_bytes());
    }

    let tx_hex = hex::encode(bitcoin::consensus::serialize(&tx));
    let mut broadcast_res = None;
    for _ in 1..=max_retries {
        for base_url in endpoints {
            let url = format!("{}/tx", base_url);
            if let Ok(Ok(resp)) = timeout(
                Duration::from_secs(5),
                ctx.http.post(&url).body(tx_hex.clone()).send(),
            )
            .await
            {
                if resp.status().is_success() {
                    if let Ok(t) = resp.text().await {
                        broadcast_res = Some(t);
                        break;
                    }
                }
            }
        }
        if broadcast_res.is_some() {
            break;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    let response = broadcast_res
        .ok_or_else(|| anyhow::anyhow!("Broadcast failed after all retries"))?;

    // Success: Return end_idx if we finished the range, otherwise return the last processed idx
    // However, if we reached this point, we definitely want the caller to know we broadcasted.
    let final_idx = if api_failed { last_processed_idx } else { end_idx };
    Ok((response, final_idx))
}
