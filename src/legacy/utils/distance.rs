use ps_range::Range;

fn u64_from(raw: &[u8]) -> u64 {
    let range = (..8).clamp_right(raw.len());
    let mut array = [0u8; 8];

    array[range.clone()].copy_from_slice(&raw[range]);

    u64::from_be_bytes(array)
}

pub fn distance<A: AsRef<[u8]>, B: AsRef<[u8]>>(a: A, b: B) -> u64 {
    let a = u64_from(a.as_ref());
    let b = u64_from(b.as_ref());

    a ^ b
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test identical inputs of exactly 8 bytes
    #[test]
    fn test_distance_identical_8_bytes() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8];
        let b = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(
            distance(a, b),
            0,
            "Identical 8-byte inputs should have distance 0"
        );
    }

    /// Test different inputs of exactly 8 bytes, differing in the first byte
    #[test]
    fn test_distance_different_8_bytes_first_byte() {
        let a = [0, 0, 0, 0, 0, 0, 0, 0];
        let b = [1, 0, 0, 0, 0, 0, 0, 0];
        let expected = 1u64 << 56; // First byte is most significant in big-endian
        assert_eq!(
            distance(a, b),
            expected,
            "Distance should reflect difference in the first byte"
        );
    }

    /// Test different inputs of exactly 8 bytes, differing in the last byte
    #[test]
    fn test_distance_different_8_bytes_last_byte() {
        let a = [0, 0, 0, 0, 0, 0, 0, 0];
        let b = [0, 0, 0, 0, 0, 0, 0, 1];
        assert_eq!(
            distance(a, b),
            1,
            "Distance should reflect difference in the last byte"
        );
    }

    /// Test identical inputs shorter than 8 bytes
    #[test]
    fn test_distance_identical_short() {
        let a = [1, 2, 3];
        let b = [1, 2, 3];
        assert_eq!(
            distance(a, b),
            0,
            "Identical short inputs should have distance 0"
        );
    }

    /// Test different inputs shorter than 8 bytes
    #[test]
    fn test_distance_different_short() {
        let a = [1, 2, 3];
        let b = [1, 2, 4];
        let expected = 7u64 << 40; // 3 ^ 4 = 7, third byte (bits 40-47 in big-endian)
        assert_eq!(
            distance(a, b),
            expected,
            "Distance should reflect difference in short inputs"
        );
    }

    /// Test one input shorter than 8 bytes, the other exactly 8 bytes
    #[test]
    fn test_distance_mixed_length() {
        let a = [1, 2, 3];
        let b = [1, 2, 3, 4, 5, 6, 7, 8];
        let expected = u64::from_be_bytes([0, 0, 0, 4, 5, 6, 7, 8]);
        assert_eq!(
            distance(a, b),
            expected,
            "Distance should handle mixed length inputs correctly"
        );
    }

    /// Test inputs longer than 8 bytes with identical first 8 bytes
    #[test]
    fn test_distance_long_identical() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b = [1, 2, 3, 4, 5, 6, 7, 8, 11, 12];
        assert_eq!(
            distance(a, b),
            0,
            "Inputs with identical first 8 bytes should have distance 0"
        );
    }

    /// Test inputs longer than 8 bytes, differing in the 8th byte
    #[test]
    fn test_distance_long_different() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let b = [1, 2, 3, 4, 5, 6, 7, 9, 10];
        let expected = 1u64; // 8 ^ 9 = 1 in the 8th byte
        assert_eq!(
            distance(a, b),
            expected,
            "Distance should reflect difference in the 8th byte"
        );
    }

    /// Test both inputs empty
    #[test]
    fn test_distance_empty() {
        let a: &[u8] = &[];
        let b: &[u8] = &[];
        assert_eq!(distance(a, b), 0, "Empty inputs should have distance 0");
    }

    /// Test one empty input, one non-empty
    #[test]
    fn test_distance_one_empty() {
        let a: &[u8] = &[];
        let b = [1, 2, 3];
        let expected = u64::from_be_bytes([1, 2, 3, 0, 0, 0, 0, 0]);
        assert_eq!(
            distance(a, b),
            expected,
            "Distance should handle one empty input correctly"
        );
    }

    /// Test maximum possible distance with 8-byte inputs
    #[test]
    fn test_distance_max_xor() {
        let a = [255u8; 8];
        let b = [0u8; 8];
        let expected = u64::MAX;
        assert_eq!(
            distance(a, b),
            expected,
            "Distance should be u64::MAX when all bytes differ maximally"
        );
    }

    /// Test inputs as vectors
    #[test]
    fn test_distance_with_vectors() {
        let a = vec![1, 2, 3];
        let b = vec![1, 2, 4];
        let expected = 7u64 << 40;
        assert_eq!(
            distance(&a, &b),
            expected,
            "Distance should work with Vec<u8> inputs"
        );
    }

    /// Test inputs as strings
    #[test]
    fn test_distance_with_strings() {
        let a = "hello"; // [104, 101, 108, 108, 111]
        let b = "world"; // [119, 111, 114, 108, 100]
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();
        let mut a_array = [0u8; 8];
        let mut b_array = [0u8; 8];
        let len_a = a_bytes.len().min(8);
        let len_b = b_bytes.len().min(8);
        a_array[..len_a].copy_from_slice(&a_bytes[..len_a]);
        b_array[..len_b].copy_from_slice(&b_bytes[..len_b]);
        let a_u64 = u64::from_be_bytes(a_array);
        let b_u64 = u64::from_be_bytes(b_array);
        let expected = a_u64 ^ b_u64; // XOR of [104,101,108,108,111,0,0,0] and [119,111,114,108,100,0,0,0]
        assert_eq!(
            distance(a, b),
            expected,
            "Distance should work with string inputs"
        );
    }
}
