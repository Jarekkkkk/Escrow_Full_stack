//This our external API to interact with Web3
//explict naming for file

// ------ PKG ------
import init from '../pkg/package.js'
//appending file type annotation is required so that module can be found
// ------ API ------
import {readTextFile} from './utils.js'
import {create_mint_js, create_token_account_js} from './token.js'

// ------ Const ------
const LOCAL_NET = 'http://localhost:8899'
const CUSTOM_PROGRAM_ID =
  'Ed2Djq5DtW3EN9Ttxc8jo2Q2QCJWCGT3zGcnQYsAgbeR'

// ------ Module ------
var web3 = solanaWeb3
var spl_token = splToken
console.log(web3)
console.log(spl_token)
const {Connection, Keypair} = web3

window.get_account_js = async (domain, commitment) => {
  let connection = getConnection(domain, commitment)

  let seed = readTextFile('/assets/id.json')
  let secretKey = Uint8Array.from(seed)

  let keypair = Keypair.fromSecretKey(secretKey)
  /**
   * struct Keypair:{
   *  pubkey: [u8;32]
   *  secretKey: [u8;64]
   * }
   */

  let pubkey = keypair.publicKey
  let account = await connection.getAccountInfo(pubkey, commitment)

  let lamports = account.lamports

  return {pubkey: pubkey.toString(), lamports: lamports.toString()}
  //  struct User {
  //     pubkey: String,
  //     lamports: String,
  //  }
}

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
// window.mint_token_js = () => {}
// window.freeze_account_js = () => {}

// ------  ------
//      Utils
// ------  ------

//any util function should stay in the 'root file' to directly fetch to window object instacne (solanaweb3, splToken)
function getConnection(cluster, commitment) {
  return new Connection(
    cluster == LOCAL_NET ? LOCAL_NET : web3.clusterApiUrl(cluster),
    commitment
  )
}

init('/pkg/package_bg.wasm')

export {web3, spl_token, getConnection}
