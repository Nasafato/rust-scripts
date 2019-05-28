fn main() {
    let string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = string.as_bytes();
    for byte in bytes {
        // this is a 4-bit value, I see... well, it's not. What? 52 does not fit in 4 bits. 
        // This is the ASCII character code
        // Really, this should be comparing the ASCII to the binary value...
        println!("{:?}", byte);
    }
}


fn hex_to_base64(bytes: &[u8]) {
    for byte in bytes {


    }
}