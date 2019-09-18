# monoalphabetic-substitution-cipher
A rust implementation of a monoalphabetic substitution cipher.

## Arguments 

key: String
The path to a file containing the key to use in the substitution.

input: String
The path to a file containing the text to be used as input to the substitution.

decipher: bool
A flag to specify if the file content should be deciphered.

## Example Usage
Encipher The-Adventures-of-Sherlock-Holmes.plaintext with the key The-Adventures-of-Sherlock-Holmes.key and write the result to the file The-Adventures-of-Sherlock-Holmes.ciphertext.

```
monoalphabetic-substitution-cipher --key ../examples/The-Adventures-of-Sherlock-Holmes.key --input ../examples/The-Adventures-of-Sherlock-Holmes.plaintext >> ../examples/The-Adventures-of-Sherlock-Holmes.ciphertext
```

Decipher the ciphertext The-Adventures-of-Sherlock-Holmes.ciphertext with the key The-Adventures-of-Sherlock-Holmes.key that was used to create the ciphertext from the plaintext.

````
monoalphabetic-substitution-cipher --key ../examples/The-Adventures-of-Sherlock-Holmes.key --input ../examples/The-Adventures-of-Sherlock-Holmes.ciphertext --decipher
```

## Building

## Testing

