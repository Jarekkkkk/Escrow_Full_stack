const buffer = require('@solana/buffer-layout')

const {get_connection} = require('./utils.js')

//Data buffer-layout
const ESCROW_ACCOUNT_DATA_LAYOUT = buffer.struct([
  buffer.u8('is_initialized'),
  buffer.blob(32, 'initializer_account'),
  buffer.blob(32, 'temp_token_account'),
  buffer.blob(32, 'initializer_token_to_receive_token_account'),
  buffer.blob(8, 'expected_amount'),
])

function subscribe_to_escrow_account(
  cluster,
  commitment,
  escrow_pubkey,
  callback
) {
  const connection = get_connection(cluster, commitment)

  let time = 0

  const id = setInterval(async () => {
    time++

    const escrow_acc_data = await connection.getAccountInfo(
      escrow_pubkey,
      commitment
    )

    if (time > 11) {
      console.log('pending promise wait over 10 sec')
      clearInterval(id)
    }

    if (escrow_acc_data) {
      const escrow_writing = ESCROW_ACCOUNT_DATA_LAYOUT.decode(
        escrow_acc_data.data
      )

      callback(escrow_writing, id)
    } else {
      console.log('null escrow ')
    }

    return () => clearInterval(id)
  }, 1000)
}

module.exports = {
  subscribe_to_escrow_account,
  ESCROW_ACCOUNT_DATA_LAYOUT,
}
