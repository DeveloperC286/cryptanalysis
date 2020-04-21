# monoalphabetic_substitution_cipher_frequency_analysis
[![pipeline status](https://img.shields.io/badge/Version-0.1.2-blue)](https://gitlab.com/DeveloperC/cryptanalysis/commits/master)

A rust implementation of a frequency analysis technique upon monoalphabetic substitution ciphers.

## Installation
Tested on Arch GNU/Linux
<ul>
    <li><b>sudo pacman -Syu base-devel rust</b> Installs rust, cargo and all required tooling.</li>
    <li><b>cargo build --release</b> Compile the Rust code into an optimised binary, ready for usage.</li>
</ul>

## Arguments
input: String
The path to a file containing the ciphertext to perform frequency analysis upon.

## Example Usage
perform frequency analysis upon The-Adventures-of-Sherlock-Holmes.ciphertext and output the predicted plaintext to The-Adventures-of-Sherlock-Holmes.plaintext.

```
monoalphabetic_substitution_cipher_frequency_analysis --input ../examples/The-Adventures-of-Sherlock-Holmes.ciphertext >>
../examples/The-Adventures-of-Sherlock-Holmes.plaintext
```

## Technical Explanation
