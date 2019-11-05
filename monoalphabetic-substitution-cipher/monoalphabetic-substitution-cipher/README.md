# monoalphabetic-substitution-cipher
A rust implementation of a monoalphabetic substitution cipher.

## Installation 
Tested on Arch GNU/Linux
<ul>
    <li><b>sudo pacman -Syu base-devel rust</b> Installs rust, cargo and all required tooling.</li>
    <li><b>cargo build --release</b> Compile the Rust code into an optimised binary, ready for usage.</li>
</ul>

## Arguments 

key: String
The path to a file containing the key to use in the substitution.

input: String
The path to a file containing the text to be used as input to the substitution.

decipher: bool
A flag to specify if the file content should be deciphered. Defaults to false.

## Example Usage
Encipher The-Adventures-of-Sherlock-Holmes.plaintext with the key The-Adventures-of-Sherlock-Holmes.key and write the result to the file The-Adventures-of-Sherlock-Holmes.ciphertext.

```
monoalphabetic-substitution-cipher --key ../examples/The-Adventures-of-Sherlock-Holmes.key --input ../examples/The-Adventures-of-Sherlock-Holmes.plaintext >> ../examples/The-Adventures-of-Sherlock-Holmes.ciphertext
```

Decipher the ciphertext The-Adventures-of-Sherlock-Holmes.ciphertext with the key The-Adventures-of-Sherlock-Holmes.key that was used to create the ciphertext from the plaintext.

```
monoalphabetic-substitution-cipher --key ../examples/The-Adventures-of-Sherlock-Holmes.key --input ../examples/The-Adventures-of-Sherlock-Holmes.ciphertext --decipher
```

## Technical Explanation

