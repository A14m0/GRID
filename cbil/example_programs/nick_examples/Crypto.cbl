// This file implements RSA encryption from a private key

// could imports look like this?
import std::math::powm; // modular exponentiation


// define a private key structure
struct PrivateKey {
    num exponent,
    num modulus
}

// Encrypts a string with the private key
byte[] RSA_Encrypt(string message, PrivateKey priv_key) {
    // first thing we do is convert the message to an array of bytes
    byte[] byte_msg = message.to_bytes();

    // then we take the byte array and raise it to the public exponent 
    // modulus the public modulus
    return powm(byte_msg, priv_key.exponent, priv_key.modulus);
}
