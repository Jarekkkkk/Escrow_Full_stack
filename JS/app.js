const {readTextFile} = require('./utils.js')

const {
  create_mint_js,
  create_token_account_js,
  mint_token_js,
} = require('./token')

const {Connection, Keypair} = require('@solana/web3.js')

// ------ ------ ------ ------
//     Helper Functions
// ------ ------ ------ ------
window.get_account_js = async (cluster, commitment) =>
  get_account_js(cluster, commitment)

// ------ ------ ------
//     Token Actions
// ------ ------ ------

window.create_mint_js = async (
  cluster,
  commitment,
  payer_secret,
  mint_authority_address,
  freeze_authority_address,
  token_decimals
) =>
  create_mint_js(
    cluster,
    commitment,
    payer_secret,
    mint_authority_address,
    freeze_authority_address,
    token_decimals
  )

window.create_token_account_js = async (
  cluster,
  commitment,
  feepayer_seed,
  token_mint,
  owner
) =>
  create_token_account_js(
    cluster,
    commitment,
    feepayer_seed,
    token_mint,
    owner
  )
window.mint_token_js = async (
  cluster,
  commitment,
  feepayer_seed,
  token_mint,
  destination,
  mint_authority_seed,
  amount
) =>
  mint_token_js(
    cluster,
    commitment,
    feepayer_seed,
    token_mint,
    destination,
    mint_authority_seed,
    amount
  )

// ------ ------ ------
//     Token Actions
// ------ ------ ------

//
async function get_account_js(cluster, commitment) {
  try {
    const seed = readTextFile('../assets/id.json')

    const connection = new Connection(cluster, commitment)

    let secretKey = Uint8Array.from(seed)
    let keypair = Keypair.fromSecretKey(secretKey)
    let acconut_info = await connection.getAccountInfo(
      keypair.publicKey,
      commitment
    )

    return {
      pubkey: keypair.publicKey.toString(),
      lamports: acconut_info.lamports.toString(),
    }
  } catch (error) {
    console.log(error)
  }
}
