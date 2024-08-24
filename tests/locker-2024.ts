import * as anchor from "@coral-xyz/anchor";
import { Program, Wallet } from "@coral-xyz/anchor";
import { assert } from "chai";

import { setTimeout } from "timers/promises";

import {
  SystemProgram,
  LAMPORTS_PER_SOL,
  sendAndConfirmRawTransaction,
  Transaction,
  sendAndConfirmTransaction,
  PublicKey,
} from "@solana/web3.js";

import {
  createMint,
  createAssociatedTokenAccount,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  transfer,
  createAccount,
  approve,
} from "@solana/spl-token";

const address0 = new PublicKey("11111111111111111111111111111111");
import { Locker2024 } from "../target/types/locker_2024";

describe("locker-2024", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Locker2024 as Program<Locker2024>;

  const owner = provider.wallet as Wallet;
  const payer = owner.payer;
  let conn = program.provider.connection;

  it("Is initialized!", async () => {
    const locker_account = getLockerAccount();
    const admin_account = getAdminAccount();
    const operator_account = getOperatorAccount();

    const seed_round_account = getClaimAccount("SEED_ROUND_ACCOUNT");
    const seed_round_user = await create_user();

    // const private_round_account = getClaimAccount("PRIVATE_ROUND_ACCOUNT");
    // const private_round__user = await create_user();

    // const public_sale_account = getClaimAccount("PUBLIC_SALE_ACCOUNT");
    // const public_sale_user = await create_user();

    // const founding_team_account = getClaimAccount("FOUNDING_TEAM_ACCOUNT");
    // const founding_team_user = await create_user();

    // const advisors_account = getClaimAccount("ADVISORS_ACCOUNT");
    // const advisors_user = await create_user();

    // const treasury_account = getClaimAccount("TREASURY_ACCOUNT");
    // const treasury_user = await create_user();

    // const ecosystem_account = getClaimAccount("ECOSYSTEM_ACCOUNT");
    // const ecosystem_user = await create_user();

    const unp = await createMint(conn, payer, owner.publicKey, null, 6);
    console.log("UNP: ", unp.toString());
    try {
      await program.methods
        .init()
        .accounts({
          locker: locker_account,
          adminAccount: admin_account,
          operatorAccount: operator_account,
        })
        .rpc();
    } catch (error) {
      console.log(error);
    }

    console.log("------------------MINT UNP TO OPERATOR------------------");
    let operator_ata = await getOrCreateAta(
      conn,
      owner.payer,
      unp,
      owner.publicKey
    );
    console.log("OPERATOR ATA: ", operator_ata.address.toString());

    let seed_round_account_ata = await getAssociatedTokenAddress(
      unp,
      seed_round_account,
      true
    );

    console.log("SEED ROUND ATA: ", seed_round_account_ata.toString());

    await mintTo(
      conn,
      owner.payer,
      unp,
      operator_ata.address,
      payer,
      BigInt(920 * 10 ** 6 * 10 ** 6)
    );

    let operator_unp_balance = await conn.getTokenAccountBalance(
      operator_ata.address
    );
    console.log(
      "LOCKER UNP BALANCE: ",
      operator_unp_balance.value.amount.toString()
    );

    try {
      const tx = await program.methods
        .setClaim(
          { seedRound: {} },
          2,
          18,
          750,
          unp,
          [seed_round_user.publicKey],
          [new anchor.BN(60 * 10 ** 6 * 10 ** 6)]
        )
        .accounts({
          locker: locker_account,
          operatorAccount: operator_account,
          operatorAta: operator_ata.address,
          claimAta: seed_round_account_ata,
          claimAccount: seed_round_account,
          mint: unp,
        })
        .rpc();
    } catch (error) {
      console.log(error);
    }

    let seed_round_account_info = await program.account.claim.fetch(
      seed_round_account
    );
    console.log(seed_round_account_info);

    console.log("------------------------SET TGE-------------------------");
    let currenct = Math.floor(new Date().getTime() / 1000) - 2;

    // try {
    //   await program.methods
    //     .setTge(new anchor.BN(currenct))
    //     .accounts({
    //       locker: locker_account,
    //       operatorAccount: operator_account,
    //       seedRoundAccount: seed_round_account,
    //       privateRoundAccount: private_round_account,
    //       publicSaleAccount: public_sale_account,
    //       foundingTeamAccount: founding_team_account,
    //       advisorsAccount: advisors_account,
    //       treasuryAccount: treasury_account,
    //       ecosystemAccount: ecosystem_account,
    //     })
    //     .rpc();
    // } catch (error) {
    //   console.log(error);
    // }

    // let seed_round_account_info = await program.account.claim.fetch(
    //   seed_round_account
    // );
    // console.log(seed_round_account_info);

    // console.log(
    //   "------------------------TEST SEED ROUND-------------------------"
    // );

    // let seed_round_unp_ata = await getAta(unp, seed_round_user.publicKey);
    // console.log("SEED ROUND ATA: ", seed_round_unp_ata.toString());

    // try {
    //   await program.methods
    //     .userClaim({ seedRound: {} })
    //     .accounts({
    //       locker: locker_account,
    //       claimAccount: seed_round_account,
    //       claimer: seed_round_user.publicKey,
    //       lockerAta: operator_ata.address,
    //       claimAta: seed_round_unp_ata,
    //       tokenMint: unp,
    //     })
    //     .signers([seed_round_user])
    //     .rpc();
    // } catch (error) {
    //   console.log(error);
    // }

    // seed_round_account_info = await program.account.claim.fetch(
    //   seed_round_account
    // );

    // console.log(seed_round_account_info.claimData[0].released.toNumber());

    // let seed_round_unp_balance = await conn.getTokenAccountBalance(
    //   seed_round_unp_ata
    // );

    // console.log(seed_round_unp_balance.value.amount)
  });

  const getLockerAccount = () => {
    const LOCKER_ACCOUNT = "LOCKER_ACCOUNT";
    const [locker] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(LOCKER_ACCOUNT)],
      program.programId
    );
    console.log("Locker account: : ", locker.toString());
    return locker;
  };

  const getOperatorAccount = () => {
    const OPERATOR_ROLE = "OPERATOR_ROLE";
    const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(OPERATOR_ROLE)],
      program.programId
    );
    console.log("Operator account: ", mint.toString());
    return mint;
  };

  const getAdminAccount = () => {
    const ADMIN_ROLE = "ADMIN_ROLE";
    const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(ADMIN_ROLE)],
      program.programId
    );
    console.log("Admin ccount: ", mint.toString());

    return mint;
  };

  const getClaimAccount = (claim_type) => {
    const [claim_account] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(claim_type)],
      program.programId
    );
    console.log(claim_type, " account: ", claim_account.toString());

    return claim_account;
  };

  const getBuyerAccount = (user) => {
    const USER_ACCOUNT = "USER_ACCOUNT";
    const [buyer_account] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from(USER_ACCOUNT), user.publicKey.toBuffer()],
      program.programId
    );
    // console.log("buyer account: ", buyer_account);

    return buyer_account;
  };

  async function create_user() {
    const user = new anchor.web3.Keypair();
    // console.log("Buyer : ", buyer1.publicKey.toString());

    await airdrop(conn, owner, user.publicKey);

    // const buyer1_account = getBuyerAccount(buyer1);
    // console.log("Buyer 1 account: ", buyer1_account.toString());

    return user;

    // return {
    //   user: buyer1,
    //   buyer_account: buyer1_account,
    // };
  }
});

async function airdrop(con, from, to) {
  let transaction = new Transaction().add(
    SystemProgram.transfer({
      fromPubkey: from.publicKey,
      toPubkey: to,
      lamports: LAMPORTS_PER_SOL,
    })
  );

  // Sign transaction, broadcast, and confirm
  await sendAndConfirmTransaction(con, transaction, [from.payer]);
}

async function getAta(mint, user) {
  return await getAssociatedTokenAddress(mint, user);
}

async function createAta(conn, payer, mint, to) {
  return await createAssociatedTokenAccount(conn, payer, mint, to);
}

async function getOrCreateAta(conn, payer, mint1, acc) {
  return await getOrCreateAssociatedTokenAccount(conn, payer, mint1, acc, true);
}
