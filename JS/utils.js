//accept only read-only methods

const {
  Connection,
  clusterApiUrl,
  PublicKey,
} = require('@solana/web3.js')

const LOCAL_NET = 'http://localhost:8899'

function readTextFile(file) {
  let res
  var rawFile = new XMLHttpRequest()
  rawFile.open('GET', file, false)
  rawFile.onreadystatechange = function () {
    if (rawFile.readyState === 4) {
      if (rawFile.status === 200 || rawFile.status == 0) {
        var allText = rawFile.responseText

        res = JSON.parse(allText)
      }
    }
  }
  rawFile.send(null)
  return res
}

function create_link_from_pubkey(pubkey) {
  return `https://explorer.solana.com/address/${pubkey}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`
}

function parse_seed_to_Uint8Array(seed) {
  //we could only retrieve from *

  let parse_seed
  //if input is "[u8;64]"
  if (seed.includes(',')) {
    parse_seed = Uint8Array.from(
      seed
        .replace(/ /g, '')
        .split(',')
        .map((n) => parseInt(n))
    )
  } else {
    console.log('unable to parse into Uint8Array from given seed')
  }

  return parse_seed
}

function get_connection(cluster, commitment) {
  // "domain":"http://localhost:8899",
  // "commitment":"confirmed
  const connection = new Connection(
    cluster === LOCAL_NET ? LOCAL_NET : clusterApiUrl(cluster),
    commitment
  )

  return connection
}

//decode the struct data by the given buffer-layout
//getParsedAccountInfo(&connection) only valid when fetching some fundamental data like: owner/ lamports/ data
function subscribe_to_account(
  cluster,
  commitment,
  account_pubkey,
  callback
) {
  const connection = get_connection(cluster, commitment)

  let time = 0

  const id = setInterval(async () => {
    time++
    const data = (
      await connection.getAccountInfo(
        new PublicKey(account_pubkey),
        commitment
      )
    ).data

    callback(data, time)
  }, 1000)

  return () => clearInterval(id)
}

module.exports = {
  readTextFile,
  get_connection,
  create_link_from_pubkey,
  parse_seed_to_Uint8Array,
  subscribe_to_account,
}
