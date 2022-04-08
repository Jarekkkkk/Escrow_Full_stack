// Restricitons of current API
// 1. only sign-externality allowed
// 2. only seed in base58 allowed
// 3. any further situation leading to web3_error will be validated beforehand during the front_end input as possibly we could

//Some nameing rule should follow up:
//  1. mostly address ( pubkey ) will not explicitly display
//  2. only {seed,link} will be appended and thereby be parsed into "Keypair"

//Buv41AopNVqW6u2qc32xLsXTQw3RQ7zXy82BN5zF8T8
const {PublicKey, Keypair, Connection} = require('@solana/web3.js')
const {
  getAccount,
  getMint,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} = require('@solana/spl-token')
const {
  get_connection,
  parse_seed_to_Uint8Array,
  create_link_from_pubkey,
} = require('./utils.js')

//mint_acc: Buv41AopNVqW6u2qc32xLsXTQw3RQ7zXy82BN5zF8T8
//token_acc: EReufHK7CbHJ416x1vQuWzUTSKCsWuBH8sWRqNcqH29M
async function create_mint_js(
  cluster,
  commitment,
  payer_seed,
  mint_authority,
  freeze_authority,
  token_decimals
) {
  try {
    const payer = Keypair.fromSecretKey(
      parse_seed_to_Uint8Array(payer_seed)
    )

    const freeze_authority_pubkey = new PublicKey(freeze_authority)
    const mint_authority_pubkey = new PublicKey(mint_authority)
    // const mintAuthority = Keypair.generate()
    // const freezeAuthority = Keypair.generate()

    const connection = new Connection(
      'http://localhost:8899',
      'confirmed'
    )

    const mint_pubkey = await createMint(
      connection,
      payer,
      mint_authority_pubkey,
      freeze_authority_pubkey,
      parseInt(token_decimals, 10)
    )
    console.log(mint_pubkey)

    //to parse account, we need to build pubkey instance
    const mint_data = await connection.getParsedAccountInfo(
      new PublicKey(mint_pubkey.toBase58()),
      'confirmed'
    )
    console.log(mint_data)
    //return struct AccountInfo

    const res = mint_data.value.data.parsed.info

    let decimals = res.decimals.toString()
    let supply = res.supply.toString()

    return {
      created_mint_address: mint_pubkey.toBase58(),
      decimals,
      supply,
      token_link: create_link_from_pubkey(
        mint_pubkey.toBase58()
      ).toString(),
    }
  } catch (error) {
    console.log(`error:\n ${error}`)
  }
}
async function create_token_account_js(
  cluster,
  commitment,
  feepayer_seed,
  token_mint,
  owner
) {
  try {
    const connection = get_connection(cluster, commitment)
    const payer = Keypair.fromSecretKey(
      parse_seed_to_Uint8Array(feepayer_seed)
    )
    const mint_pubkey = new PublicKey(token_mint)
    const owner_pubkey = new PublicKey(owner)

    const token_acc = await getOrCreateAssociatedTokenAccount(
      connection,
      payer,
      mint_pubkey,
      owner_pubkey
    )

    console.log(token_acc)
    const token_acc_info = await getAccount(
      connection,
      token_acc.address
    )

    const amount = token_acc_info.amount
    const mint_info = await getMint(connection, mint_pubkey)

    const link = create_link_from_pubkey(
      token_acc_info.address.toBase58()
    )

    return {
      token_acconut: token_acc.address.toBase58(),
      amount: amount.toString(),
      decimals: mint_info.decimals.toString(),
      token_link: link,
    }
  } catch (error) {
    console.log(`error:\n ${error}`)
  }
}
async function mint_token_js(
  cluster,
  commitment,
  feepayer_seed,
  token_mint,
  destination,
  mint_authority_seed,
  amount
) {
  try {
    const connection = get_connection(cluster, commitment)
    const payer = Keypair.fromSecretKey(
      parse_seed_to_Uint8Array(feepayer_seed)
    )
    const mint_pubkey = new PublicKey(token_mint)
    const destination_pubkey = new PublicKey(destination)
    const mint_authority = Keypair.fromSecretKey(
      parse_seed_to_Uint8Array(mint_authority_seed)
    )

    await mintTo(
      connection,
      payer,
      mint_pubkey,
      destination_pubkey,
      mint_authority,
      parseInt(amount, 10)
    )

    //check whether successfully mint action
    const token_info = await getAccount(
      connection,
      destination_pubkey
    )
    console.log('token_info')

    const token_amount = token_info.amount.toString()
    const decimals = (await getMint(connection, mint_pubkey)).decimals

    console.log(
      destination_pubkey.toBase58(),
      token_amount,
      decimals.toString(),
      create_link_from_pubkey(destination_pubkey.toBase58())
    )
    return {
      deposited_address: destination_pubkey.toBase58(),
      amount: token_amount,
      decimals: decimals.toString(),
      token_link: create_link_from_pubkey(
        destination_pubkey.toBase58()
      ),
    }
  } catch (error) {
    console.log(`error:\n ${error}`)
  }
}
async function freeze_account_js(
  cluster,
  commitment,
  feePayer_seed,
  address_to_freeze,
  freeze_authority_seed
) {
  try {
  } catch (error) {
    console.log(`error:\n ${error}`)
  }
}

module.exports = {
  create_mint_js,
  create_token_account_js,
  mint_token_js,
  freeze_account_js,
}
