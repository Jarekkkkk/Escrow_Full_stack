//accept only read-only methods

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

export {
  readTextFile,
  create_link_from_pubkey,
  parse_seed_to_Uint8Array,
}
