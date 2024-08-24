import {
  program,
  locker_account,
  admin_account,
  operator_account,
} from "./helper";

export const init = async () => {
  // anchor.setProvider(provider);

  console.log("--------------INIT-----------------");
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

  let locker_account_info = await program.account.locker.fetch(locker_account);
  console.log(locker_account_info);
};


init()