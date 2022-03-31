import {web3, spl_token, getConnection} from './index.js'
import {
  create_link_from_pubkey,
  parse_seed_to_Uint8Array,
} from './utils.js'

// Restricitons of current API
// 1. only sign-externality allowed
// 2. only seed in base58 allowed
// 3. any further situation leading to web3_error will be validated beforehand during the front_end input as possibly we could

//Some nameing rule should follow up:
//  1. mostly address ( pubkey ) will not explicitly display
//  2. only {seed,link} will be appended and thereby be parsed into "Keypair"

// ------  ------  ------  ------  ------  ------  ------
//    we can't destructure the 'web3'& 'spl_token'
//    but we can destructure with the function
// ------  ------  ------  ------  ------  ------  ------

async function create_mint_js(
  cluster,
  commitment,
  payer_seed,
  mint_authority,
  freeze_authority,
  token_decimals
  //we only take 'sign-externality' into consideration
) {
  try {
    const {PublicKey, Keypair} = web3

    const {TOKEN_PROGRAM_ID, Token} = spl_token

    const freeze_authority_pubkey = new PublicKey(freeze_authority)

    //return mint instance
    const mint = await Token.createMint(
      getConnection(cluster, commitment),
      Keypair.fromSecretKey(parse_seed_to_Uint8Array(payer_seed)),
      new PublicKey(mint_authority),
      freeze_authority_pubkey ? freeze_authority_pubkey : null,
      parseInt(token_decimals, 10),
      TOKEN_PROGRAM_ID
    )

    let mint_account_pubkey = mint.publicKey.toString()
    let link = create_link_from_pubkey(mint_account_pubkey)

    console.log('------- Mint\n')
    console.log(mint_account_pubkey)

    // ------ ------
    //      Test

    //to parse account, we need to build pubkey instance
    let mint_accont_pubkey_instance = new PublicKey(
      mint_account_pubkey
    )
    const mint_data = await getConnection(
      cluster,
      commitment
    ).getParsedAccountInfo(mint_accont_pubkey_instance, 'confirmed')
    console.log(mint_data) //return solana on-chain data such as "data; executable; lamports;owner"
    const mint_data_1 = mint_data.value.data
    const res = mint_data_1.parsed.info //this will return mint AccountInfo

    let decimals = res.decimals.toString()
    let supply = res.supply.toString()
    // {
    //  decimals: 6

    // freezeAuthority: "6dThiasP9bMpQoHibnr9dmpGvG6UaQTyGS155QSUP16L"

    // isInitialized: true

    // mintAuthority: "6dThiasP9bMpQoHibnr9dmpGvG6UaQTyGS155QSUP16L"

    // supply: "0"
    //}
    console.log(res)

    // ------ ------

    return {
      created_mint_address: mint_accont_pubkey_instance.toString(),
      decimals,
      supply,
      token_link: link.toString(),
    }
    //   struct Token {
    //     created_token_address: String,
    //     token_link: String,
    // }
  } catch (error) {
    console.error(error)
    throw error
  }
}

async function create_token_account_js(
  cluster,
  commitment,
  feepayer_seed,
  token_mint,
  owner
) {
  console.log(cluster, commitment, feepayer_seed, token_mint, owner)
  try {
    const {Token, TOKEN_PROGRAM_ID} = spl_token
    const {Keypair, PublicKey} = web3

    const token = new Token(
      getConnection(cluster, commitment),
      token_mint,
      TOKEN_PROGRAM_ID,
      Keypair.fromSecretKey(parse_seed_to_Uint8Array(feepayer_seed))
    )

    //return publickey<String>
    let token_account_ads = await token.createAccount(owner)
    console.log(token_account_ads.toString())

    let token_account_info = await getConnection(
      cluster,
      commitment
    ).getParsedAccountInfo(
      new PublicKey(token_account_ads),
      'confirmed'
    )
    console.log(token_account_info)

    let amount = token_account_info.amount
    let decimals = token_account_info.decimals

    // isNative: false

    // mint: "sswxow1Pmqo29aMpEpXgmeBeQgAX7zXhJEq13ABLuxQ"

    // owner: "6dThiasP9bMpQoHibnr9dmpGvG6UaQTyGS155QSUP16L"

    // state: "initialized"

    // tokenAmount: Object

    // amount: "0"

    // decimals: 6

    // uiAmount: 0

    // uiAmountString: "0"
    let link = create_link_from_pubkey(token_account_ads)

    return {
      token_acconut: token_account_ads.toString(),
      amount: amount.toString(),
      decimals: decimals.toString(),
      token_link: link,
    }
    //  struct Add_Token_Account{
    //  token_account:String,
    //  token_link:String
    //  }
  } catch (error) {
    console.log(error)
    throw error
  }
}

async function mint_token_js(
  cluster,
  commitment,
  feepayer_seed,
  tokenMint,
  destination,
  mint_authority_seed,
  amount
) {
  try {
    const {Token, TOKEN_PROGRAM_ID} = spl_token
    const {Keypair} = web3

    const token = new Token(
      getConnection(cluster, commitment),
      tokenMint,
      TOKEN_PROGRAM_ID,
      Keypair.fromSecretKey(parse_seed_to_Uint8Array(feepayer_seed))
    )

    await token.mintTo(
      destination,
      Keypair.fromSecretKey(
        parse_seed_to_Uint8Array(mint_authority_seed)
      ),
      [],
      parseInt(amount, 10)
    )

    return create_link_from_pubkey(destination)
  } catch (error) {
    console.error(error)
  }
}

async function freeze_account_js(
  cluster,
  commitment,
  feePayer_seed,
  address_to_freeze,
  freeze_authority_seed
) {
  const {Token, TOKEN_PROGRAM_ID} = spl_token
  const {Keypair, PublicKey} = web3

  //we could derive the 'mint account' from given token account
  //Connection(struct).getParsedAccountInfo()

  const tokenMint = 'mint_pubkey'
  const address_to_freeze_pubkey = new PublicKey(address_to_freeze)

  const token = new Token(
    getConnection(cluster, commitment),
    tokenMint,
    TOKEN_PROGRAM_ID,
    Keypair.fromSecretKey(parse_seed_to_Uint8Array(feePayer_seed))
  )

  await token.freezeAccount(
    address_to_freeze_pubkey,
    Keypair.fromSecretKey(freeze_authority_seed)
  )
}

export {create_mint_js, create_token_account_js}
