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
export const init = async () => {
  // anchor.setProvider(provider);

  const claim_account = getClaimAccount("PRIVATE_ROUND_ACCOUNT");

  console.log("--------------SET TGE-----------------");
  const current = new Date().getTime() / 1000;

  try {
    await program.methods
      .setTge({ privateRound: {} }, new anchor.BN(current + 60))
      .accounts({
        locker: locker_account,
        operatorAccount: operator_account,
        claimAccount: claim_account,
      })
      .rpc();
  } catch (error) {
    console.log(error);
  }

  let claim_account_info = await program.account.claim.fetch(claim_account);
  console.log(claim_account_info.fullLock.toNumber());
  console.log(claim_account_info.vesting.toNumber());
  console.log(claim_account_info.startTime.toNumber());
  console.log(claim_account_info.endTime.toNumber());
  console.log(claim_account_info.claimData[0].lock.toNumber());
};

init();
