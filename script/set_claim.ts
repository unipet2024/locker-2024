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
  console.log("Owner: ", owner.publicKey.toString());

  const private_round_account = getClaimAccount("PRIVATE_ROUND_ACCOUNT");

  console.log("--------------SET CLAIM-----------------");
  let private_round_account_ata = await getAssociatedTokenAddress(
    unp,
    private_round_account,
    true
  );

  let operator_ata = await getAssociatedTokenAddress(unp, owner.publicKey);

  console.log("PRIVATE ROUND ATA: ", private_round_account_ata.toString());

  try {
    await program.methods
      .setClaim(
        { privateRound: {} },
        new anchor.BN(2 * 60), // 2 months = 2*30*86400
        new anchor.BN(18 * 60),
        750,
        unp,
        [new PublicKey("CfN9A1tBhC7BxoubNkNuB8CrH6W6hojNhT5kGawNdupy")],
        [new anchor.BN(60 * 10 ** 6 * 10 ** 6)]
      )
      .accounts({
        locker: locker_account,
        operatorAccount: operator_account,
        operatorAta: operator_ata,
        claimAta: private_round_account_ata,
        claimAccount: private_round_account,
        mint: unp,
      })
      .rpc();
  } catch (error) {
    console.log(error);
  }

  let claim_account_info = await program.account.claim.fetch(
    private_round_account
  );
  console.log(claim_account_info.fullLock.toNumber());
  console.log(claim_account_info.vesting.toNumber());
  console.log(claim_account_info.claimData[0].lock.toNumber());
};

init();
