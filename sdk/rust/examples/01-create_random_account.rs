use symbol_sdk::crypto_types::public_key::PublicKey;
use symbol_sdk::network::Network;
use symbol_sdk::symbol::network::MAINNET;

fn main() {
    let public_key =
        PublicKey::from("C5FB65CB902623D93DF2E682FFB13F99D50FAC24D5FF2A42F68C7CA1772FE8A0");

    let address = (*MAINNET).public_key_to_address(&public_key);

    println!("{address}");

    // # create a signing key pair that will be associated with an account
    // key_pair = facade.KeyPair(PrivateKey.random())

    // # convert the public key to a network-dependent address (unique account identifier)
    // address = facade.network.public_key_to_address(key_pair.public_key)

    // # output account public and private details
    // print(f'    address: {address}')
    // print(f' public key: {key_pair.public_key}')
    // print(f'private key: {key_pair.private_key}')
}
