<img src="./assets/aleo.svg" alt="drawing" width="100"/>

# Aleo SDK

A Software Development Kit (SDK) for Zero-Knowledge Transactions.

Aleo [SnarkVM](https://github.com/AleoHQ/snarkVM) low-level utilities in web
assembly. Built in Rust using wasm-pack.

To know more about the aleo concepts used in this library you can visit [Aleo
Concepts](https://developer.aleo.org/overview/#chapter-1-concepts)

## Features
  - [Private Key](#private-key)
    - [Examples](#examples)
        - [Creating a private key](#creating-a-private-key)
        - [Signing a message](#signing-a-message)
        - [Getting account info from private key](#getting-account-info-from-private-key)
        - [Get private key string](#get-private-key-string)
  - [View Key](#view-key)
    - [Examples](#example)
        - [Creating a View Key](#creating-a-view-key)
        - [Decrypting records](#decrypting-records)
        - [Getting account info from view key](#getting-account-info-from-view-key)
        - [Getting view key string](#getting-view-key-string)
  - [Record](#record)
    - [Examples](#example-1)
        - [Getting a record string](#getting-a-record-string)
        - [Getting the record balance](#getting-the-record-balance)
  - [Address](#address)
    - [Examples](#example-2)
        - [Creating the address](#creating-the-address)
        - [Verifying signatures](#verifying-signatures)
        - [Getting address string](#getting-address-string)
  - [Signature](#signature)
    - [Examples](#example-3)
        - [Creating a Signature](#creating-a-signature)
        - [Verify signatures](#verify-signatures)

## Private Key

### Examples

##### Creating a private key

You can create a private key using the default constructor
```js
let private_key = new PrivateKey();
```

Or you can create one from a secret seed, the same seed will create the same
private key. The seed must be a phrase of 32 bytes long.

```js
let private_key = PrivateKey.from_seed_unchecked("seed...");
```

Also you can create a private key from a string, the string must follow a valid
private key format, like the one described
[here](https://developer.aleo.org/concepts/accounts#account-private-key) 

```js
let private_key = PrivateKey.from_string("APrivateKey1...");
```
##### Signing a message

You can sign a message with a previously created private key.

```js
let private_key = new PrivateKey();
let signature = private_key.sign("a random message");
```

##### Getting account info from private key

You can use the following functions from a private key to get de addressand the
view key associated with the private key. To get the address from the private
key you can use `to_address()` functions.

```js
let private_key = new PrivateKey();
let address = private_key.to_address();
```

You can use `to_view_key()` to get the view key instead.

```js
let private_key = new PrivateKey();
let view_key = private_key.to_view_key();
```

##### Get private key string

You can get the string representation of the private key using the function `to_string`

```js
let private_key = new PrivateKey();
private_key.to_string(); // "APrivateKey1..."
```

## View Key

### Examples

##### Creating a View Key

You can create a view key in two different ways. The first is using a
previously created private key

```js
let private_key = new PrivateKey();
let view_key = ViewKey.from_private_key(private_key);
```

The other way it's using a string with the correct view key format, like the
one described in
[here](https://developer.aleo.org/concepts/accounts#account-view-key)

```js
let view_key = ViewKey.from_string("AViewKey1...");
```

##### Decrypting records

One of the main functions of the view key is to decrypt encrypted records
(ciphertexts). To do this the view key have the `decrypt` function that
receives a string representation of a encrypted record and returns a Record.

```js
let view_key = ViewKey.from_string("AViewKey1...");
let ciphertext = "record1...";
let record = view_key.decrypt(ciphertext); // Record Object
```

##### Getting account info from view key

You can get the address associated to the view key using the function
`to_address`

```js
let view_key = ViewKey.from_string("AViewKey1...");
let address = view_key.to_address();
```

##### Getting view key string

You can get the string representation of the view key using the function `to_string`

```js
let view_key = ViewKey.from_string("AViewKey1...");
view_key.to_string(); // "AViewKey1..."
```

## Record

### Examples

##### Getting a record string

To get the string representation of the record you can use the `to_string` method. We can use the previous example of the decryption using the view Key

```js
let view_key = ViewKey.from_string("AViewKey1...");
let ciphertext = "record1...";
let record = view_key.decrypt(ciphertext); // Record Object
record.to_string(); // "{ owner: aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah.private, gates: 99u64.public, _nonce: 0group.public }"
```
##### Getting the record balance

You can access the gates of the record using the `gates` method. It returns a
string representing the gates:

```js
let gates = record.gates(); // "99u64.public"
```

## Address

### Examples

##### Creating the address

You have three different ways to generate the address.

The first one is creating it from a private key

```js
let private_key = new PrivateKey();
let address = Address.from_private_key(private_key);
```

You can get it from the view key too

```js
let private_key = new PrivateKey();
let view_key = ViewKey.from_private_key(private_key);
let address = Address.from_view_key(view_key);
```
Lastly you can create it from a string with a valid format like the one described [here](https://developer.aleo.org/concepts/accounts#account-address)

```js
let address = Address.from_string("aleo1...");
```

##### Verifying signatures

With the address you can verify if the message was signed by the account that
it's associated with the address. In the example we sign a random message and
then check if this address corresponds to the account (or private key) that
signed that message.

```js
let private_key = new PrivateKey();
let signature = Signature.from_string("sign...");
let address = private_key.to_address();
signature.sign(address, "random message");
address.verify("random message", signature) // true
```

##### Getting address string

You can get the string representation of the address using the function `to_string`

```js
let private_key = new PrivateKey();
let address =  Address.from_private_key(private_key);
address.to_string(); // "aleo1..."
```

## Signature

### Examples

##### Creating a Signature

A signature object is created when a message is signed with a private key, the
signature is associated to that signed message.

```js
let private_key = new PrivateKey();
let signature = Signature.sign(private_key, "random message"); // Signature object
```

You can also create a previously created signature from its string representation

```js
let signature = Signature.from_string("sign...");
```

##### Verify signatures

You can use the signature to also verify if a specific address of an account
signed a certain message

```js
let private_key = new PrivateKey();
let signature = Signature.sign(private_key, "random message");
let address = private_key.to_address();
signature.verify(priaddress, "random message"); // true
```





