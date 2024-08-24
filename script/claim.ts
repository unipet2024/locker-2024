import { getAssociatedTokenAddress } from "@solana/spl-token";
import {
  program,
  locker_account,
  operator_account,
  getClaimAccount,
  owner,
  payer,
} from "./helper";
import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";

const unp = new PublicKey("5Et3fqFdXqKRKnTvNq8YBrdYWfQdSALJFYiCsjKdHAL7");
const user = new PublicKey("CfN9A1tBhC7BxoubNkNuB8CrH6W6hojNhT5kGawNdupy");

export const init = async () => {
  // anchor.setProvider(provider);

  const seed_round_account = getClaimAccount("PRIVATE_ROUND_ACCOUNT");

  console.log("--------------CLAIM-----------------");
  let seed_round_account_ata = await getAssociatedTokenAddress(
    unp,
    seed_round_account,
    true
  );

  let user_ata = await getAssociatedTokenAddress(unp, user);

  console.log("SEED ROUND ATA: ", seed_round_account_ata.toString());

  try {
    await program.methods
      .userClaim({ privateRound: {} })
      .accounts({
        claimAta: seed_round_account_ata,
        claimAccount: seed_round_account,
        userAta: user_ata,
        user: user,
        tokenMint: unp,
      })
      .rpc();
  } catch (error) {
    console.log(error);
  }

  let claim_account_info = await program.account.claim.fetch(
    seed_round_account
  );
  console.log(claim_account_info.startTime.toNumber());
  console.log(claim_account_info.endTime.toNumber());
  console.log(claim_account_info.claimData[0].lock.toNumber());
};

init();
