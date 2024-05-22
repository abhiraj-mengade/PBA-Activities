use std::collections::HashMap;

fn is_space(byte: u8) -> bool {
    byte == 0x00 || byte.is_ascii_alphabetic()
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(x, y)| x ^ y).collect()
}

fn track_spaces(text: &[u8]) -> HashMap<usize, usize> {
    let mut counter = HashMap::new();
    for (index, &byte) in text.iter().enumerate() {
        if is_space(byte) {
            *counter.entry(index).or_insert(0) += 1;
        }
    }
    counter
}

fn recover_partial_key(ciphertexts: &[Vec<u8>]) -> Vec<Option<u8>> {
    let shortest_len = ciphertexts.iter().map(Vec::len).min().unwrap_or(0);
    let mut key = vec![None; shortest_len];

    for (main_index, main_ciphertext) in ciphertexts.iter().enumerate() {
        let mut main_counter = HashMap::new();

        for (secondary_index, secondary_ciphertext) in ciphertexts.iter().enumerate() {
            if main_index != secondary_index {
                let xored = xor(main_ciphertext, secondary_ciphertext);
                for (index, count) in track_spaces(&xored) {
                    *main_counter.entry(index).or_insert(0) += count;
                }
            }
        }

        for (&index, &count) in main_counter.iter() {
            if count == ciphertexts.len() - 1 {
                key[index] = Some(b' ' ^ main_ciphertext[index]);
            }
        }
    }
    key
}

fn recover_key(ciphertexts: Vec<Vec<u8>>) -> Vec<Option<u8>> {
    let mut sorted_ciphertexts = ciphertexts;
    sorted_ciphertexts.sort_by_key(|text| text.len());
    let mut key = Vec::new();

    while sorted_ciphertexts.len() > 1 {
        let partial_key = recover_partial_key(&sorted_ciphertexts);
        key.extend(partial_key.iter());
        let string_length = sorted_ciphertexts[0].len();
        sorted_ciphertexts = sorted_ciphertexts.into_iter().skip(1).collect();
        for text in sorted_ciphertexts.iter_mut() {
            text.drain(0..string_length);
        }
    }
    key
}

fn main() {
    let array: Vec<Vec<u8>> = vec![
        hex::decode("160111433b00035f536110435a380402561240555c526e1c0e431300091e4f04451d1d490d1c49010d000a0a4510111100000d434202081f0755034f13031600030d0204040e").unwrap(),
        hex::decode("050602061d07035f4e3553501400004c1e4f1f01451359540c5804110c1c47560a1415491b06454f0e45040816431b144f0f4900450d1501094c1b16550f0b4e151e03031b450b4e020c1a124f020a0a4d09071f16003a0e5011114501494e16551049021011114c291236520108541801174b03411e1d124554284e141a0a1804045241190d543c00075453020a044e134f540a174f1d080444084e01491a090b0a1b4103570740").unwrap(),
        hex::decode("000000000000001a49320017071704185941034504524b1b1d40500a0352441f021b0708034e4d0008451c40450101064f071d1000100201015003061b0b444c00020b1a16470a4e051a4e114f1f410e08040554154f064f410c1c00180c0010000b0f5216060605165515520e09560e00064514411304094c1d0c411507001a1b45064f570b11480d001d4c134f060047541b185c").unwrap(),
        hex::decode("0b07540c1d0d0b4800354f501d131309594150010011481a1b5f11090c0845124516121d0e0c411c030c45150a16541c0a0b0d43540c411b0956124f0609075513051816590026004c061c014502410d024506150545541c450110521a111758001d0607450d11091d00121d4f0541190b45491e02171a0d49020a534f").unwrap(),
        hex::decode("031a5410000a075f5438001210110a011c5350080a0048540e431445081d521345111c041f0245174a0006040002001b01094914490f0d53014e570214021d00160d151c57420a0d03040b4550020e1e1f001d071a56110359420041000c0b06000507164506151f104514521b02000b0145411e05521c1852100a52411a0054180a1e49140c54071d5511560201491b0944111a011b14090c0e41").unwrap(),
        hex::decode("0b4916060808001a542e0002101309050345500b00050d04005e030c071b4c1f111b161a4f01500a08490b0b451604520d0b1d1445060f531c48124f1305014c051f4c001100262d38490f0b4450061800004e001b451b1d594e45411d014e004801491b0b0602050d41041e0a4d53000d0c411c41111c184e130a0015014f03000c1148571d1c011c55034f12030d4e0b45150c5c").unwrap(),
        hex::decode("011b0d131b060d4f5233451e161b001f59411c090a0548104f431f0b48115505111d17000e02000a1e430d0d0b04115e4f190017480c14074855040a071f4448001a050110001b014c1a07024e5014094d0a1c541052110e54074541100601014e101a5c").unwrap(),
        hex::decode("0c06004316061b48002a4509065e45221654501c0a075f540c42190b165c").unwrap(),
        hex::decode("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap(),
    ];

    let key = recover_key(array);
    let result: String = key
        .into_iter()
        .map(|opt| opt.map(|b| b as char).unwrap_or('?'))
        .collect();

    println!("{}", result);
}

// THE KEY IS:
// bitcoins implementation of a peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_space() {
        assert!(is_space(b'a'));
        assert!(is_space(b'Z'));
        assert!(is_space(0x00));
        assert!(!is_space(b'1'));
        assert!(!is_space(b'!'));
    }

    #[test]
    fn test_xor() {
        let a = b"hello";
        let b = b"world";
        let result = xor(a, b);
        assert_eq!(result, vec![15, 10, 23, 7, 10]);
    }

    #[test]
    fn test_track_spaces() {
        let text = b"hello world";
        let result = track_spaces(text);
        let expected = vec![
            (0, 1),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 1),
            (6, 1),
            (7, 1),
            (8, 1),
            (9, 1),
            (10, 1),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }
}
