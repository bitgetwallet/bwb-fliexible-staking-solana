import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BwbStake } from "../target/types/bwb_stake";
import {
  clusterApiUrl,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  ComputeBudgetProgram
} from '@solana/web3.js';

import {
  getPayer,
  initializeSigner,
  hexToBytes
} from '../utils/utils';

import {u64, TOKEN_PROGRAM_ID} from "@solana/spl-token";

const PRIORITY_RATE = 1000; // MICRO_LAMPORTS 
const PRIORITY_FEE_IX = ComputeBudgetProgram.setComputeUnitPrice({microLamports: PRIORITY_RATE});

async function updatePool(
   poolId, start_at, end_at, program, pool_admin, adminInfoPda, 
    stake_cap_bn, reward_cap_bn, duration_time,poolPda,duration_days
) {
    let tx = await program.methods.updatePool(
      poolId,stake_cap_bn, reward_cap_bn, start_at, end_at, duration_time,duration_days
    ).accounts({
      poolAdmin: pool_admin.publicKey,
      adminInfo:adminInfoPda,
      pool:poolPda,
      systemProgram:SystemProgram.programId,
      rent:SYSVAR_RENT_PUBKEY,
    }).preInstructions([PRIORITY_FEE_IX])
    .signers([pool_admin])
    .rpc();
    console.log("Your transaction signature", tx);
  
    let poolInfoPdaData = await program.account.poolInfo.fetch(poolPda);
    console.log("duration is ", poolInfoPdaData.duration.toString());
    console.log("stakeStartAt is ", poolInfoPdaData.stakeStartAt.toString());
    console.log("stakeEndAt is ", poolInfoPdaData.stakeEndAt.toString());
    console.log("stakeCap is ", poolInfoPdaData.stakeCap.toString());
    console.log("rewardCap is ", poolInfoPdaData.rewardCap.toString());
  
}

(async () => {
    let payer: Keypair = await getPayer();
    let cosigner: Keypair = await initializeSigner('../keys/wallet-pair-cosigner.json');
    let admin = await initializeSigner('../keys/fund_account.json');
    let receiver: Keypair = await initializeSigner('../keys/receiver-keypair.json');
    let pool_admin: Keypair = receiver;
    let operator: Keypair = await initializeSigner('../keys/operate-keypair.json');
    let programIdSelf: Keypair = await initializeSigner('../keys/bwb_stake-keypair.json');

    let user01: Keypair = await initializeSigner('../keys/userKp.json');//J9DKMBBqdnFDuPxBNqxeMTsL3vWQatz3tJJEBhg71S24
    let user02: Keypair = await initializeSigner('../keys/userKp02.json');//BKprKM553wXVWayY3X38KDRYfkC4cCqh5vpKwa7Qygr2

    process.env.ANCHOR_WALLET = process.env.HOME + '/.config/solana/id.json';
    // minnet
    process.env.ANCHOR_PROVIDER_URL = 'https://api.mainnet-beta.solana.com';
    const connection = new Connection(clusterApiUrl('mainnet-beta'), 'confirmed');
    let token_mint = new PublicKey("G1GV35DHibxUDJtMC9DBRzruqhLhEe6tr85WQ73XoPJ3");//mainnet TT02

    //process.env.ANCHOR_PROVIDER_URL = 'https://api.devnet.solana.com';
    //const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
    //let token_mint = new PublicKey("83QLg6QKjfFCgoFWTx8x2EAytbAwVgA5G1mtAcsnybtp");//devnet

    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.BwbStake as Program<BwbStake>;
    console.log("===program is: ", program.programId);

    let [vaultTokenAccountPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("vault_token_account")],
        program.programId
    );
    let [adminInfoPda] = PublicKey.findProgramAddressSync(
        [Buffer.from("admin_info")],
        program.programId
    );

    let now_time = Date.now();
    let time_stamp = (now_time/1000).toFixed();
    console.log("now_time is",time_stamp);

    const one_day_test = new u64("300");//test 2m per day

    // //7 days 15% //poolId =0
    // const stake_cap_bn = new u64("1500000000000000");
    // const reward_cap_bn = new u64("4315100000000");

    //6 days 15% //poolId =3
    const stake_cap_bn = new u64("100000000000");//1000
    const reward_cap_bn = new u64("287671232");//2.876
    
    const duration_time = one_day_test.mul(new u64(7));// test 7 days
    const duration_time_3days = new u64("259200");// test 

    // 30days 25% poolId =1
    // const stake_cap_bn = new u64("300000000000000");
    // const reward_cap_bn = new u64("6164400000000");
    // const one_day_test = new u64("300");//test 2m per day
    // const duration_time = one_day_test.mul(new u64(30));// test 30 days
    // const duration_time_3days = new u64("259200");// 

    //90days 35% //poolId =2
    // const stake_cap_bn = new u64("150000000000000");
    // const reward_cap_bn = new u64("12945200000000");
    // const one_day_test = new u64("300");//test 2m per day
    // const duration_time = one_day_test.mul(new u64(90));// test 90 days
    // const duration_time_3days = new u64("259200");// test 
    
    let adminInfoPdaData = await program.account.adminInfo.fetch(adminInfoPda);
    console.log("adminInfoPdaData is ", adminInfoPdaData);

    let poolId = new u64(3);
    // new pool //seeds=[b"new_pool", &admin_info.next_pool_id.to_le_bytes()],
    let [poolPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("new_pool"), new u64(poolId).toBuffer()],
      program.programId
    );
    console.log("newPoolPda is",poolPda);

    let poolInfoPdaData = await program.account.poolInfo.fetch(poolPda);
    console.log("poolInfoPdaData is ", poolInfoPdaData);

    const start_at = new u64(poolInfoPdaData.stakeStartAt);
    let newEndAt = "1715940000";
    const end_at = new u64(newEndAt);
  
    await updatePool(
      poolId,start_at, end_at, program, pool_admin, adminInfoPda, 
      stake_cap_bn, reward_cap_bn, duration_time,poolPda,duration_days
    )

})();
