const {
  getMint,
  getAccount,
  AccountLayout,
  TOKEN_PROGRAM_ID,
  createInitializeAccountInstruction,
  createTransferInstruction,
} = require('@solana/spl-token')
const {
  PublicKey,
  Keypair,
  SystemProgram,
  TransactionInstruction,
  SYSVAR_RENT_PUBKEY,
  Transaction,
} = require('@solana/web3.js')
const {
  get_connection,
  parse_seed_to_Uint8Array,
} = require('./utils.js')

const buffer = require('@solana/buffer-layout')
const BN = require('bn.js')

async function escrow_maker_js(
  cluster,
  commitment,
  fee_payer_seed,
  token_to_send,
  token_to_receive,
  amount_to_send,
  amount_to_receive,
  escrow_program_id
) {
  try {
    let cluster = 'http://localhost:8899'
    let commitment = 'confirmed'
    let fee_payer_seed =
      '137,130,83,5,93,236,161,23,186,182,176,60,55,116,11,2,172,220,167,87,71,180,59,201,99,75,48,81,180,30,5,199,83,161,137,43,210,109,163,151,33,160,186,85,185,16,173,64,254,52,88,20,79,22,155,31,123,211,66,136,17,61,247,181'
    let token_to_send = 'EEiTLXtocBBVS2cfZ91wp4cqMUFSvTRJD6gmPtJwuDMw'
    let token_to_receive =
      'EEiTLXtocBBVS2cfZ91wp4cqMUFSvTRJD6gmPtJwuDMw'
    let amount_to_send = '12'
    let amount_to_receive = '12'
    let escrow_program_id =
      'A8bkizaAC3EePjYJjVSzfsUpKqTGREpyb89eT1FJyrzn'

    // Declaration
    const connection = get_connection(cluster, commitment)
    const token_to_send_pubkey = new PublicKey(token_to_send)
    const token_to_receive_pubkey = new PublicKey(token_to_receive)

    // 5 instructions required
    // 1. createTempTokenAccountIx
    // 2. initTempAccountIx,
    // 3. transferXTokensToTempAccIx,
    // 4. createEscrowAccountIx,
    // 5. initEscrowIx

    if (
      token_to_send_acc.mint.toBase58() ==
      token_to_receive_acc.mint.toBase58()
    ) {
      throw Error('token must be different !')
    }

    const token_to_send_mint_pubkey = token_to_send_acc.mint
    const token_to_receive_mint_pubeky = token_to_receive_acc.mint

    // ------  ------ ------ ------ ------  ------
    //      1. create empty account of Program
    // ------  ------ ------ ------ ------  ------
    const maker = Keypair.fromSecretKey(
      parse_seed_to_Uint8Array(fee_payer_seed)
    )
    const empty = Keypair.generate()

    let create_empty_account_ix = SystemProgram.createAccount({
      fromPubkey: maker,
      newAccountPubkey: empty.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(
        AccountLayout.span,
        'confirmed'
      ),
      space: AccountLayout.span,
      programId: TOKEN_PROGRAM_ID,
    })
    console.log(create_empty_account_ix)

    // ------  ------ ------ ------ ------ ------
    //      2. init empty acconut as token account
    // ------  ------ ------ ------ ------ ------

    let init_empty_as_token_acconut_ix =
      createInitializeAccountInstruction(
        empty.publicKey,
        token_to_send_mint_pubkey,
        maker,
        TOKEN_PROGRAM_ID
      )
    console.log(init_empty_as_token_acconut_ix)
    // ------  ------ ------ ------ ------
    //      3. maker transfer token to temp account
    // ------  ------ ------ ------ ------

    let maker_transfer_to_temp_token_account_ix =
      createTransferInstruction(
        token_to_send_pubkey,
        empty.publicKey,
        maker.publicKey,
        parseInt(amount_to_send, 10),
        [],
        TOKEN_PROGRAM_ID
      )
    console.log(maker_transfer_to_temp_token_account_ix)
    // ------  ------ ------ ------ ------
    //      4. create escrow account (PDA)
    // ------  ------ ------ ------ ------
    const escrow_writing = Keypair.generate()
    const escrow_program_pubkey = new PublicKey(escrow_program_id)

    //calculate escrow writing account space
    const ESCROW_ACCOUNT_DATA_LAYOUT = buffer.struct([
      buffer.u8('is_initialized'),
      buffer.blob(32, 'initializer_account'),
      buffer.blob(32, 'temp_token_account'),
      buffer.blob(32, 'initializer_token_to_receive_token_account'),
      buffer.blob(8, 'expected_amount'),
    ])

    let init_escrow_writing_ix = SystemProgram.createAccount({
      fromPubkey: maker.publickey,
      newAccountPubkey: escrow_writing.publicKey,
      lamports: await connection.getMinimumBalanceForRentExemption(
        AccountLayout.span,
        'confirmed'
      ),
      space: ESCROW_ACCOUNT_DATA_LAYOUT.span,
      programId: escrow_program_pubkey,
    })
    console.log(init_escrow_writing_ix)
    // ------  ------ ------ ------ ------
    //      5. init Escrow Program
    // ------  ------ ------ ------ ------

    const amount_b = new BN(
      parseInt(amount_to_receive, 10).toArray(
        'le',
        8
      ) /**instruction be stored in [u8] */
    )
    console.log(amount_b)
    let tagged_data = Buffer.from(0, Uint8Array.of(0, ...amount_b))
    console.log(tagged_data)

    // accounts:{
    //      maker,empty_token,token_to_recevie,escrow_writing,rent_sysvar, token_progrma_id
    let init_escrow_ix = new TransactionInstruction({
      programId: escrow_program_pubkey,
      keys: [
        {
          pubkey: maker.publicKey,
          isSigner: true,
          isWritable: false,
        },
        {pubkey: empty.publicKey, isSigner: false, isWritable: true},
        {pubkey: token_to_receive_pubkey, isSigner: false},
        {isWritable: false},
        {pubkey: escrow_writing.publicKey, isSigner: false},
        {isWritable: true},
        {
          pubkey: SYSVAR_RENT_PUBKEY,
          isSigner: false,
          isWritable: false,
        },
        {
          pubkey: TOKEN_PROGRAM_ID,
          isSigner: false,
          isWritable: false,
        },
      ],
      data: tagged_data,
    })
    console.log(init_escrow_ix)

    const tx = new Transaction().add(
      create_empty_account_ix,
      init_empty_as_token_acconut_ix,
      maker_transfer_to_temp_token_account_ix,
      init_escrow_writing_ix,
      init_escrow_ix
    )

    await connection.sendTransaction(tx, [
      maker,
      empty,
      escrow_writing,
    ])
  } catch (error) {
    console.log('escrow maker error')
    console.log(error)
  }
}
async function escrow_taker_js(
  cluster,
  commitment,
  fee_payer_seed,
  token_to_send,
  token_to_receive,
  escrow_account,
  amount_to_receive,
  escrow_program_id
) {}
