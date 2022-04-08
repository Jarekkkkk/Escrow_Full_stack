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
  subscribe_to_account,
  create_link_from_pubkey,
  readTextFile,
} = require('./utils.js')

const buffer = require('@solana/buffer-layout')
const BN = require('bn.js')
const ESCROW_PDA = 'escrow'

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
  console.log('escrow maker js ')
  try {
    console.log(
      cluster,
      commitment,
      fee_payer_seed,
      token_to_send,
      token_to_receive,
      amount_to_send,
      amount_to_receive,
      escrow_program_id
    )
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
    const token_to_send_acc = await getAccount(
      connection,
      token_to_send_pubkey
    )
    const token_to_receive_acc = await getAccount(
      connection,
      token_to_receive_pubkey
    )
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
      fromPubkey: maker.publicKey,
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
        maker.publicKey,
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
    // const escrow_writing_seed = readTextFile('../escrow-writing.json')
    const escrow_writing = Keypair.generate()
    console.log('--- Escrow writing Account ---')
    console.log(escrow_writing)
    const escrow_program_pubkey = new PublicKey(escrow_program_id)

    //calculate escrow writing account space
    const ESCROW_ACCOUNT_DATA_LAYOUT = buffer.struct([
      buffer.u8('is_initialized'),
      buffer.blob(32, 'initializer_account'),
      buffer.blob(32, 'temp_token_account'),
      buffer.blob(32, 'initializer_token_to_receive_token_account'),
      buffer.blob(8, 'expected_amount'),
    ])
    let min_lamports =
      await connection.getMinimumBalanceForRentExemption(
        ESCROW_ACCOUNT_DATA_LAYOUT.span,
        'confirmed'
      )
    console.log(min_lamports)

    let init_escrow_writing_ix = SystemProgram.createAccount({
      fromPubkey: maker.publicKey,
      newAccountPubkey: escrow_writing.publicKey,
      lamports: min_lamports,
      space: ESCROW_ACCOUNT_DATA_LAYOUT.span,
      programId: escrow_program_pubkey,
    })
    console.log(init_escrow_writing_ix)
    // ------  ------ ------ ------ ------
    //      5. init Escrow Program
    // ------  ------ ------ ------ ------

    const amount_b = new BN(parseInt(amount_to_receive, 10)).toArray(
      'le',
      8
    ) /**instruction be stored in [u8] */
    console.log(amount_b)
    let tagged_data = Buffer.from(Uint8Array.of(0, ...amount_b))
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
        {
          pubkey: token_to_receive_pubkey,
          isSigner: false,
          isWritable: false,
        },
        {
          pubkey: escrow_writing.publicKey,
          isSigner: false,
          isWritable: true,
        },
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

    // await connection.sendTransaction(
    //   tx,
    //   [maker, empty, escrow_writing],
    //   {skipPreflight: false, preflightCommitment: commitment}
    // )

    console.log(escrow_writing.publicKey.toBase58())
    // ------ ------ ------ ------
    //     escrow_state layout
    // ------ ------ ------ ------
    // pub is_initialized: bool,
    // pub initializer_account: Pubkey,
    // pub temp_token_account: Pubkey,
    // pub initializer_token_to_receive_token_account: Pubkey
    // pub expected_amount: u64

    // ------ ------ ------
    //     expected res
    // ------ ------ ------
    //   struct Maker {
    //     escrow_account: String, #hardcoded
    //     maker: String, #on-chain
    //     temp_token_account: String, #on-chain
    //     token_to_receive: String, #on-chain
    //     amount_to_send: String, #empty account lamports
    //     amount_to_receive: String, #on-chain
    //     token_link: String, #on our own
    // }
    let maker_res
    const id = subscribe_to_account(
      cluster,
      commitment,
      escrow_writing.publicKey.toBase58(),
      async (data, time) => {
        //decoded date will be stored in "Buffer" type
        let decoded_escrow_writing =
          ESCROW_ACCOUNT_DATA_LAYOUT.decode(data)
        if (time > 10) {
          console.log('transaction fails for loading over 10 secs')
          id()
        } else if (!decoded_escrow_writing) {
          console.log('yet updated')
        } else {
          maker_res = {
            escrow_account: escrow_writing.publicKey.toBase58(),
            signer: new PublicKey(
              decoded_escrow_writing.initializer_account
            ).toBase58(),
            token_to_send: new PublicKey(
              decoded_escrow_writing.temp_token_account
            ).toBase58(),
            token_to_receive: new PublicKey(
              decoded_escrow_writing.initializer_token_to_receive_token_account
            ).toBase58(),
            amount_to_send: (
              await connection.getAccountInfo(
                empty.publicKey,
                commitment
              )
            ).lamports.toString(),
            amount_to_receive: new BN(
              decoded_escrow_writing.expected_amount,
              10,
              'le'
            ).toString(),
            token_link: create_link_from_pubkey(
              maker.publicKey.toBase58()
            ),
          }
          id()
          console.log(maker_res)
        }
      }
    )

    if (typeof maker_res !== 'undefined') {
      return maker_res
    }
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
) {
  console.log(
    cluster,
    commitment,
    fee_payer_seed,
    token_to_send,
    token_to_receive,
    escrow_account,
    amount_to_receive,
    escrow_program_id
  )
  try {
    //declaration
    const connection = get_connection(cluster, commitment)
    const taker = Keypair.fromSecretKey(
      parse_seed_to_Uint8Array(fee_payer_seed)
    )
    const token_to_send_pubkey = new PublicKey(token_to_send)
    const token_to_receive_pubkey = new PublicKey(token_to_receive)
    const escrow_account_pubkey = new PublicKey(escrow_account)
    const escrow_program = new PublicKey(escrow_program_id)

    //fetch & decode on-chain  escrow state
    // IMPORTANT TAKE_AWAY
    // we store the state in bytes type staying in which "data" fields
    const escrow_writing_data = await connection.getAccountInfo(
      escrow_account_pubkey,
      commitment
    )
    console.log(escrow_writing_data)
    //get the escrow struct to decoded in JS
    const ESCROW_ACCOUNT_DATA_LAYOUT = buffer.struct([
      buffer.u8('is_initialized'),
      buffer.blob(32, 'initializer_account'),
      buffer.blob(32, 'temp_token_account'),
      buffer.blob(32, 'initializer_token_to_receive_token_account'),
      buffer.blob(8, 'expected_amount'),
    ])
    const escrow_state_decoded = ESCROW_ACCOUNT_DATA_LAYOUT.decode(
      escrow_writing_data.data
    )
    console.log(escrow_state_decoded)
    const empty_pubkey = new PublicKey(
      escrow_state_decoded.temp_token_account
    )
    const maker_token_to_send_pubkey = new PublicKey(
      escrow_state_decoded.initializer_account
    )
    const maker_token_to_receive_pubkey = new PublicKey(
      escrow_state_decoded.initializer_token_to_receive_token_account
    )

    // find the off-curve PDA by known seed phrase
    const PDA = await PublicKey.findProgramAddress(
      [Buffer.from(ESCROW_PDA)],
      escrow_program
    )
    console.log(PDA)

    //build ix and send tx

    //keys orders
    // taker/ take token to send / taker token to receive / temp token account / maker token to send / maker token to receive / escrow writing / PDA / escrow program
    const exchange_escrow_tx = new TransactionInstruction({
      programId: escrow_program,
      keys: [
        {pubkey: taker.publicKey, isSigner: true, isWritable: false},
        {
          pubkey: token_to_send_pubkey,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: token_to_receive_pubkey,
          isSigner: false,
          isWritable: true,
        },
        {pubkey: empty_pubkey, isSigner: false, isWritable: true},
        {
          pubkey: maker_token_to_send_pubkey,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: maker_token_to_receive_pubkey,
          isSigner: false,
          isWritable: true,
        },
        {
          pubkey: escrow_account_pubkey,
          isSigner: false,
          isWritable: true,
        },
        //no usage of signature extension
        {pubkey: PDA, isSigner: false, isWritable: false},
        {pubkey: escrow_program, isSigner: false, isWritable: false},
      ],
    })
    console.log(exchange_escrow_tx)

    //send the tx
    await connection.sendTransaction(
      tx,
      [taker, empty, escrow_writing],
      {skipPreflight: false, preflightCommitment: commitment}
    )

    const maker_prev_lamport = (
      await connection.getAccountInfo(token_to_send_pubkey)
    ).lamports.toString()

    //register the subscription
    const id = subscribe_to_account(
      cluster,
      commitment,
      escrow_account,
      async (data, time) => {
        console.log(time)

        let decoded_escrow_writing =
          ESCROW_ACCOUNT_DATA_LAYOUT.decode(data)
        if (time > 10) {
          console.log('transaction fails for loading over 10 secs')
          return () => id()
        } else if (!decoded_escrow_writing) {
          console.log('yet updated')
        } else {
          const maker_after_lamport = (
            await connection.getAccountInfo(token_to_send_pubkey)
          ).lamports.toString()

          taker_res = {
            escrow_account: escrow_account,
            signer: taker.publicKey.toBase58(),
            token_to_send: token_to_send,
            token_to_receive: token_to_receive,
            amount_to_send: (
              maker_prev_lamport - maker_after_lamport
            ).toString(),
            amount_to_receive: amount_to_receive,
            token_link: create_link_from_pubkey(
              taker.publicKey.toBase58()
            ),
          }
          id()
          return taker_res
        }
      }
    )
  } catch (error) {
    console.log(error)
  }
}

module.exports = {escrow_maker_js, escrow_taker_js}

const id = subscribe_to_account(
  cluster,
  commitment,
  '7qvGVVaXVqohzcXb4dm6F1B9fzUeLW3xunPzQnTWYuLq',
  async (data, time) => {
    console.log(data)
    let decoded_escrow_writing =
      ESCROW_ACCOUNT_DATA_LAYOUT.decode(data)
    if (time > 10) {
      console.log('transaction fails for loading over 10 secs')
      id()
    } else if (!decoded_escrow_writing) {
      console.log('yet updated')
    } else {
      const maker_res = {
        escrow_account:
          '7qvGVVaXVqohzcXb4dm6F1B9fzUeLW3xunPzQnTWYuLq',
        signer: new PublicKey(
          decoded_escrow_writing.initializer_account
        ).toBase58(),
        token_to_send: new PublicKey(
          decoded_escrow_writing.temp_token_account
        ).toBase58(),
        token_to_receive: new PublicKey(
          decoded_escrow_writing.initializer_token_to_receive_token_account
        ).toBase58(),
        amount_to_send: (
          await connection.getAccountInfo(
            new PublicKey(token_to_send),
            commitment
          )
        ).lamports.toString(),
        amount_to_receive: new BN(
          decoded_escrow_writing.expected_amount,
          10,
          'le'
        ).toString(),
        token_link: create_link_from_pubkey(
          Keypair.fromSecretKey(
            parse_seed_to_Uint8Array(fee_payer_seed)
          ).publicKey.toBase58()
        ),
      }
      console.log(maker_res)
      id()
      return maker_res
    }
  }
)
