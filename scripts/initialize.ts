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

(
    async () => {
          let payer: Keypair = await getPayer();
          let cosigner: Keypair = await initializeSigner('../keys/wallet-pair-cosigner.json');
          let admin = await initializeSigner('../keys/fund_account.json');
          let receiver: Keypair = await initializeSigner('../keys/receiver-keypair.json');
          let pool_admin: Keypair = receiver;
          let operator: Keypair = await initializeSigner('../keys/operate-keypair.json');

          let user01: Keypair = await initializeSigner('../keys/userKp.json');//J9DKMBBqdnFDuPxBNqxeMTsL3vWQatz3tJJEBhg71S24
          let user02: Keypair = await initializeSigner('../keys/userKp02.json');//BKprKM553wXVWayY3X38KDRYfkC4cCqh5vpKwa7Qygr2
          let user01_ata = new PublicKey("EXwCW3QYc14XnJvCR4umvWxPBGyrRbU7MuNmkXvDd1i9");
          let user02_ata = new PublicKey("E1T9XWnS1nFwSL41nXYs5qoLQC6WrTJxkvQwo32TNzJD");
          let user : Keypair = user01;
          let user_ata = user01_ata;

          console.log("payer is : " + payer.publicKey);
          console.log("cosigner is : " + cosigner.publicKey);
          console.log("admin is : " + admin.publicKey);
          console.log("receiver is : " + receiver.publicKey.toBase58());
          console.log("operator is : " + operator.publicKey.toBase58());
          console.log("pool_admin is : " + pool_admin.publicKey.toBase58());

          let token_mint = new PublicKey("83QLg6QKjfFCgoFWTx8x2EAytbAwVgA5G1mtAcsnybtp");//devnet
          //let token_mint = new PublicKey("G1GV35DHibxUDJtMC9DBRzruqhLhEe6tr85WQ73XoPJ3");//mainnet TT02
          let payer_ata = new PublicKey("AR1aJmL5jWmV53bQXSQaHNvB6uqmbC9yH1yVqhRnHvvi");
          let receiver_ata = new PublicKey('Bx66barTesm9yjcvRd8QSedgpxphBY8EfgbvgXUthDGe');

          // process.env.ANCHOR_WALLET = process.env.HOME + '/.config/solana/id.json';
          process.env.ANCHOR_WALLET = '/Users/xp/workshop_01/gitlab-projects/bwb-contract/solana/new-stake/keys/userKp.json';
          // // minnet
          // process.env.ANCHOR_PROVIDER_URL = 'https://api.mainnet-beta.solana.com';
          // const connection = new Connection(clusterApiUrl('mainnet-beta'), 'confirmed');

          process.env.ANCHOR_PROVIDER_URL = 'https://api.devnet.solana.com';
          const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

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

          /**
           * #[account(mut)]
              pub authority: Signer<'info>,
              #[account(constraint = program.programdata_address()? == Some(program_data.key()))]
              pub program: Program<'info, System>,
              #[account(constraint = program_data.upgrade_authority_address == Some(authority.key()))]
              pub program_data: Account<'info, ProgramData>,
              
           */
          /*
            error: {
            errorCode: { code: 'ConstraintRaw', number: 2003 },
            errorMessage: 'A raw constraint was violated',
            comparedValues: undefined,
            origin: 'program_data'
          },
          */
          // payer = admin;// 使用非 authority帐户初始化，均会引起programData的帐户检查报错
          let programDataPDA = new PublicKey("H8SSWeMWBfyvTrFY4AGck1SFUitYfuHv4yCwXtEdMJ5");
          let tx = await program.methods.initialize(
            cosigner.publicKey, admin.publicKey,receiver.publicKey,operator.publicKey, pool_admin.publicKey
          ).accounts({
            authority: payer.publicKey,
            adminInfo: adminInfoPda,
            stakeTokenMint: token_mint,
            vaultTokenAccount:vaultTokenAccountPda,
            program:program.programId,
            programData:programDataPDA,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID
          }).signers([payer])
          .rpc().catch((err) => {console.log(err)});

          console.log("Your transaction signature", tx);

          let adminInfoPdaData = await program.account.adminInfo.fetch(adminInfoPda);
          console.log("adminInfoPdaData is ", adminInfoPdaData);

})();
