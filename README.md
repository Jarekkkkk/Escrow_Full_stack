## Main Idea: 

1. create additional empty "token account" of TOKEN_PROGRAM
2. make Alice become owner of "temp token account"
3. Alice transfer desired amoutn to "temp token account"
4. create "writing account" ("PDA"), which stored all of the info of ESCROW_PROGRAM
5. initialze token account and transfer Alice's ownership of "temp token account" to "PDA"