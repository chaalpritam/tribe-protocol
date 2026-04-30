#!/usr/bin/env ts-node
/**
 * Rotate the social_graph sequencer authority on-chain.
 *
 * The on-chain `SequencerConfig` PDA records two pubkeys: `authority`
 * (the ER server signer used by `*_delegated` instructions) and
 * `admin` (the only key allowed to rotate `authority`). This script
 * builds + sends an `update_sequencer` instruction signed by the
 * admin, replacing `authority` with the pubkey of a new keypair.
 *
 * Typical usage from inside `tribe-protocol/`:
 *
 *   pnpm ts-node scripts/rotate-sequencer.ts \
 *     --new-sequencer ../tribe-er-server/server-wallet.json
 *
 * Optional flags:
 *   --admin     keypair file for the SequencerConfig admin
 *               (default: ~/.config/solana/id.json)
 *   --rpc       Solana RPC URL  (default: https://api.devnet.solana.com)
 *   --program   social_graph program id
 *               (default: 8kKnWvbmTjWq5uPePk79RRbQMAXCszNFzHdRwUS4N74w)
 *   --dry-run   print the resolved arguments and exit without sending
 */

import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { createHash } from "crypto";
import { readFileSync } from "fs";
import { homedir } from "os";

const DEFAULT_PROGRAM_ID = "8kKnWvbmTjWq5uPePk79RRbQMAXCszNFzHdRwUS4N74w";
const DEFAULT_RPC_URL = "https://api.devnet.solana.com";
const DEFAULT_ADMIN_PATH = `${homedir()}/.config/solana/id.json`;

function flag(name: string): string | undefined {
  const i = process.argv.indexOf(`--${name}`);
  return i === -1 ? undefined : process.argv[i + 1];
}

function bool(name: string): boolean {
  return process.argv.includes(`--${name}`);
}

function expand(path: string): string {
  return path.startsWith("~/") ? `${homedir()}${path.slice(1)}` : path;
}

function loadKeypair(path: string): Keypair {
  const secret = Uint8Array.from(JSON.parse(readFileSync(expand(path), "utf-8")));
  return Keypair.fromSecretKey(secret);
}

/** Anchor instruction discriminator: first 8 bytes of sha256("global:<ix_name>"). */
function discriminator(ixName: string): Buffer {
  return createHash("sha256").update(`global:${ixName}`).digest().subarray(0, 8);
}

async function main(): Promise<void> {
  const newSeqPath = flag("new-sequencer");
  if (!newSeqPath) {
    console.error(
      "usage: rotate-sequencer.ts --new-sequencer <keypair.json>\n" +
        "                          [--admin <keypair.json>]\n" +
        "                          [--rpc <url>] [--program <id>] [--dry-run]"
    );
    process.exit(1);
  }

  const adminPath = flag("admin") ?? DEFAULT_ADMIN_PATH;
  const rpcUrl = flag("rpc") ?? DEFAULT_RPC_URL;
  const programId = new PublicKey(flag("program") ?? DEFAULT_PROGRAM_ID);
  const dryRun = bool("dry-run");

  const admin = loadKeypair(adminPath);
  const newSequencer = loadKeypair(newSeqPath);

  const [sequencerConfig] = PublicKey.findProgramAddressSync(
    [Buffer.from("sequencer_config")],
    programId
  );

  const data = Buffer.concat([
    discriminator("update_sequencer"),
    newSequencer.publicKey.toBuffer(),
  ]);

  const ix = new TransactionInstruction({
    programId,
    keys: [
      { pubkey: sequencerConfig, isSigner: false, isWritable: true },
      { pubkey: admin.publicKey, isSigner: true, isWritable: false },
    ],
    data,
  });

  console.log("rotate-sequencer:");
  console.log(`  rpc:              ${rpcUrl}`);
  console.log(`  program:          ${programId.toBase58()}`);
  console.log(`  sequencerConfig:  ${sequencerConfig.toBase58()}`);
  console.log(`  admin:            ${admin.publicKey.toBase58()}`);
  console.log(`  new authority:    ${newSequencer.publicKey.toBase58()}`);

  if (dryRun) {
    console.log("--dry-run set, not sending.");
    return;
  }

  const connection = new Connection(rpcUrl, "confirmed");
  const sig = await sendAndConfirmTransaction(
    connection,
    new Transaction().add(ix),
    [admin],
    { commitment: "confirmed" }
  );
  console.log(`  tx:               ${sig}`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
