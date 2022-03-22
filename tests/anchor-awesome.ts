import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import kp from "./keypair.json";


const main = async() => {
  console.log("starting test...")

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.AnchorAwesome;

  // Create an account keypair for our program
  
  const secret = new Uint8Array(Object.values(kp._keypair.secretKey))
  const baseAccount = anchor.web3.Keypair.fromSecretKey(secret);  

  
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  // console.log("Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("GIF count", account.totalGifs.toString())

  await program.rpc.addGif("https://maryoku.s3.amazonaws.com/proposal/background-default-large.jpg",{
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("GIF count", account.totalGifs.toString())
  console.log("GIF list", account.gifList)

  await program.rpc.updateItem("https://maryoku.s3.amazonaws.com/proposal/background-default-large.jpg", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    }
  })
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("check.votes", account.gifList);
}

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  }catch (e) {
    console.error(e);
    process.exit(1);
  }
}

runMain();