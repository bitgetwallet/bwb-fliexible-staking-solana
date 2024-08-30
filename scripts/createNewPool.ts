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

const PRIORITY_RATE = 10000; // MICRO_LAMPORTS 
const PRIORITY_FEE_IX = ComputeBudgetProgram.setComputeUnitPrice({microLamports: PRIORITY_RATE});

async function createNewPool(
    start_at, end_at, program, pool_admin, adminInfoPda, 
    stake_cap_bn, reward_cap_bn, duration_time, duration_days
) {
    let adminInfoPdaData = await program.account.adminInfo.fetch(adminInfoPda);
    console.log("adminInfoPdaData is ", adminInfoPdaData);
    
    // new pool //seeds=[b"new_pool", &admin_info.next_pool_id.to_le_bytes()],
    let [newPoolPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("new_pool"), new u64(adminInfoPdaData.nextPoolId.toString()).toBuffer()],
      program.programId
    );
  
    console.log("newPoolPda is",newPoolPda);

    let tx = await program.methods.createNewPool(
      stake_cap_bn, reward_cap_bn, start_at, end_at,duration_time,duration_days
    ).accounts({
      poolAdmin: pool_admin.publicKey,
      adminInfo:adminInfoPda,
      newPool:newPoolPda,
      systemProgram:SystemProgram.programId,
      rent:SYSVAR_RENT_PUBKEY,
    })
    .preInstructions([PRIORITY_FEE_IX])
    .signers([pool_admin])
    .rpc();
    console.log("Your transaction signature", tx);
  
    // poolId-0 5UtdJ8zaRzLYtvDgFg1wwzJaCTLFvGHTc1GufHbJXWpS
    // poolId-1 BAWXo2bKPJnXLcLaHLN7NBqyWAQdrj3NifVqS6z43L4F
  
    let poolInfoPdaData = await program.account.poolInfo.fetch(newPoolPda);
    console.log("poolInfoPdaData is ", poolInfoPdaData);
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
    console.log("vaultTokenAccountPda is",vaultTokenAccountPda);
    console.log("adminInfoPda is",adminInfoPda);

    let now_time = Date.now();
    let time_stamp = (now_time/1000).toFixed();
    console.log("now_time is",time_stamp);
    const one_day_test = new u64("300");//test 2m per day

    // //7 days 15% //poolId =0
    // const stake_cap_bn = new u64("1500000000000000");
    // const reward_cap_bn = new u64("4315100000000");
    
    // const duration_time = one_day_test.mul(new u64(7));// test 7 days
    // const duration_time_3days = new u64("259200");// test 

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

    // 6days 15% //poolId =3
    let startTime = "1715853600";//2024-05-16 18:00:00
    let half_hour = new anchor.BN(1800);
    const start_at = new u64(startTime);
    const end_at = start_at.add(half_hour);
    const stake_cap_bn = new u64("100000000000");
    const reward_cap_bn = new u64("287671232");
    const duration_time = one_day_test.mul(new u64(90));// test 90 days
    const duration_time_1days = new u64("86400");// test 
    const duration_days = new u64("90");

    await createNewPool(start_at, end_at, program, pool_admin, adminInfoPda, 
        stake_cap_bn, reward_cap_bn, duration_time, duration_days
    )

})();
