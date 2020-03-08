# Sigli: Signal Command Line Cryptography

Designed for encrypting and decrypting short messages using
latest cryptographic primitives while still generating
nice human readable inputs and outputs. These should make it easier
to pass the key and cipher information by hand or over the phone.

The following example generates a new key, encrypts and
then decrypts a short message.
```bash
./sigli genkey -o /tmp/demokey1
echo "a short message to encode at 1234" > plain_text1
./sigli encrypt /tmp/demokey1 -i plain_text1 -o cipher_text
./sigli decrypt /tmp/demokey1 -i cipher_text -o plain_text2
cat plain_text2
```

By default sigli use hexadecimal encoding for keys and 
a blocked uppercase encoding for cipher text. So in the above
example `/tmp/demokey1` might contain:

```text
16C1-F0AF-5B92-64CC-8A09-04F2-641E-BE64-4A41-6028-E92D-49D6-81EE-9D6A-F5AC-4E7A
```

and the `cipher-text` file might contain:
```text
ZPPCR PXPNL QPYAG QSDIL EAQCB MXTDD
ZLJDJ BCRJZ QNDRX XMPPV OHWQX GUBBO
OVYMI BRXIE JXJPS ITCOH NEULZ E
```

These values will vary for each run because key is random and the cipher also 
contains a random element. The output appears much longer than the input
because the default algorithm (AES-256-GCM) adds a fixed overhead of four
 of bytes of authentication data and twelve bytes of random nonce data.
 
 ## Selecting Algorithm
 
 A different algorithm can be selected using the --algo (or -a) parameter.
 
 ```bash
./sigli  -a aes128gcm genkey -o /tmp/demokey1
 echo "a short message to encode at 1234" > plain_text1
 ./sigli -a aes128gcm encrypt /tmp/demokey1 -i plain_text1 -o cipher_text
 ./sigli -a aes128gcm decrypt /tmp/demokey1 -i cipher_text -o plain_text2
cat plain_text2
 ```
 This works the same as the previous example only with a shorter
 key, for example:
 
 
```text
10AA-E181-8D00-113C-2A32-BF5B-A01F-017A
```

## Standard Input and Output

If the --input (-i) argument is omitted the encrypt and decrypt commands
read from the standard input.
If the --output (-o) argument is omitted the encrypt and decrypt commands
write to the standard output.

 ```bash
./sigli genkey -o /tmp/demokey1
 echo "a short message to encode at 1234" | ./sigli encrypt /tmp/demokey1 > cipher_text
cat cipher_text | ./sigli decrypt /tmp/demokey1 
 ```
## Cipher Chaining

By default sigli assumes that the input will be in plain text format and the output
will be in signal format. However if you want to recrypt a message to need to change
the input and output format accordingly.
The input format is controlled by the --input-format (-I) argument and
the output format is controlled by the --output-format (-O) argument.

 ```bash
./sigli genkey -o /tmp/demokey1
./sigli genkey -o /tmp/demokey2
 echo "a short message to encode at 1234" | ./sigli encrypt /tmp/demokey1  | ./sigli encrypt -I signal1 /tmp/demokey2 > cipher_text
cat cipher_text | ./sigli decrypt -O signal1 /tmp/demokey2 | ./sigli decrypt /tmp/demokey1 
 ```
The available formats are `raw`, `hex`, `plain1` and `signal1`.
The default key format is `hex`, the default plain text format is `plain1` and 
the default cipher text format is `signal1`.


