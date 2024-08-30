import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BwbStake } from "../target/types/bwb_stake";
import {
  clusterApiUrl,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY  
} from '@solana/web3.js';

import {
  getPayer,
  initializeSigner,
  hexToBytes
} from '../utils/utils';

import {u64, TOKEN_PROGRAM_ID} from "@solana/spl-token";


( async () => {
  
  let payer: Keypair = await getPayer();
    let cosigner: Keypair = await initializeSigner('../keys/wallet-pair-cosigner.json');
    let admin = await initializeSigner('../keys/fund_account.json');
    let receiver: Keypair = await initializeSigner('../keys/receiver-keypair.json');
    let pool_admin: Keypair = receiver;
    let operator: Keypair = await initializeSigner('../keys/operate-keypair.json');
    let programIdSelf: Keypair = await initializeSigner('../keys/bwb_stake-keypair.json');

    let user01: Keypair = await initializeSigner('../keys/userKp.json');//J9DKMBBqdnFDuPxBNqxeMTsL3vWQatz3tJJEBhg71S24
    let user02: Keypair = await initializeSigner('../keys/userKp02.json');//BKprKM553wXVWayY3X38KDRYfkC4cCqh5vpKwa7Qygr2
    let user01_ata = new PublicKey("EXwCW3QYc14XnJvCR4umvWxPBGyrRbU7MuNmkXvDd1i9");
    let user02_ata = new PublicKey("E1T9XWnS1nFwSL41nXYs5qoLQC6WrTJxkvQwo32TNzJD");
    let user : Keypair = user01;
    let user_ata = user01_ata;

    process.env.ANCHOR_WALLET = process.env.HOME + '/.config/solana/id.json';
    // // mainnet
    // process.env.ANCHOR_PROVIDER_URL = 'https://api.mainnet-beta.solana.com';
    // const connection = new Connection(clusterApiUrl('mainnet-beta'), 'confirmed');
    // let token_mint = new PublicKey("G1GV35DHibxUDJtMC9DBRzruqhLhEe6tr85WQ73XoPJ3");//mainnet TT02

    // devnet
    process.env.ANCHOR_PROVIDER_URL = 'https://api.devnet.solana.com';
    const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
    let token_mint = new PublicKey("83QLg6QKjfFCgoFWTx8x2EAytbAwVgA5G1mtAcsnybtp");//devnet

    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.BwbStake as Program<BwbStake>;
    console.log("===program is: ", program.programId);

    let poolId = new u64(1);

    let [adminInfoPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("admin_info")],
      program.programId
    );

    let adminInfoPdaData = await program.account.adminInfo.fetch(adminInfoPda);
    console.log("adminInfoPdaData is ", adminInfoPdaData);

    let [vaultTokenAccountPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault_token_account")],
      program.programId
    );
    
    let [poolPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("new_pool"), poolId.toBuffer()],
      program.programId
    );

    let now_time = Date.now();
    let time_stamp = (now_time/1000).toFixed();
    console.log("now_time is",time_stamp);

    console.log("poolId.toBuffer()", poolId.toBuffer());
    console.log("poolPda is ", poolPda);

    let [userInfoPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user_info"), user.publicKey.toBuffer()],
      program.programId
    );
    let userInfoPdaData = await program.account.userInfo.fetch(userInfoPda);
    console.log("userInfoPda is ", userInfoPda);
    console.log("userInfoPdaData is ", userInfoPdaData);

    // >= second order
    let [orderInfoPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("new_order"), user.publicKey.toBuffer(), new u64(userInfoPdaData.nextOrderId.toString()).toBuffer()],
      program.programId
    );
    console.log("orderInfoPda is ", orderInfoPda);

    let orderInfoPdaData = await program.account.orderInfo.fetch(orderInfoPda);
    console.log("orderInfoPdaData is ", orderInfoPdaData);


    let claimed_time = new anchor.BN(time_stamp);
    if(claimed_time.gte(orderInfoPdaData.unstakeTime)) {
      claimed_time = new anchor.BN(orderInfoPdaData.unstakeTime);
    }
    
    let orderId = new u64(1);
    let unstakeAmount = new anchor.BN(orderInfoPdaData.stakeAmount);
    let rewardAmount = new anchor.BN(orderInfoPdaData.rewardAmount).sub(new anchor.BN(orderInfoPdaData.claimedReward));
    let tx = await program.methods.unstake(
      orderId,
      unstakeAmount,
      rewardAmount
    ).accounts({
      user:user.publicKey,//payer,// sender
      cosigner:cosigner.publicKey,
      adminInfo: adminInfoPda,
      pool:poolPda,
      userInfo:userInfoPda,
      stakeTokenMint: token_mint,
      order:orderInfoPda,
      vaultTokenAccount: vaultTokenAccountPda,
      userTokenWallet:user_ata,// one token acount// token 接收者 + amount +
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([user,cosigner])
    .rpc({
      skipPreflight:false
    }).catch(e => console.error(e));

    console.log("Your transaction signature", tx);

    orderInfoPdaData = await program.account.orderInfo.fetch(orderInfoPda);
    console.log("orderInfoPdaData is ", orderInfoPdaData);

    userInfoPdaData = await program.account.userInfo.fetch(userInfoPda);
    console.log("userInfoPdaData is ", userInfoPdaData);

})();
