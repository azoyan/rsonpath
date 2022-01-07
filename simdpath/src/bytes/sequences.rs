//! This module is autogenerated from [`simdpath-codegen::bytes::sequences`].

include!(concat!(
    env!("OUT_DIR"),
    "/simdpath-codegen/bytes/sequences.rs"
));

#[cfg(test)]
mod tests {
    #[cfg(feature = "nosimd")]
    use super::nosimd::*;
    #[cfg(not(feature = "nosimd"))]
    use super::simd::*;

    static LONG_SEQUENCE: [u8; 128] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71,
        72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94,
        95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113,
        114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128,
    ];

    #[test]
    fn find_byte_sequence_when_the_sequence_is_prefix_of_bytes_returns_0() {
        for i in 1..LONG_SEQUENCE.len() {
            let sequence = &LONG_SEQUENCE[..i];
            let result = find_byte_sequence(sequence, &LONG_SEQUENCE);

            assert_eq!(Some(0), result, "failed for sequence of length{}", i);
        }
    }

    #[test]
    #[should_panic]
    fn find_byte_sequence_when_sequence_is_empty_panics() {
        find_byte_sequence(&[], &[1, 2, 3, 4, 5, 6, 7, 8]);
    }

    struct TestByteStreamParameters<'a> {
        pub base_byte: u8,
        pub bytes_length: usize,
        pub sequence: &'a [u8],
        pub sequence_start_idx: usize,
    }

    fn test_byte_stream(parameters: TestByteStreamParameters<'_>) -> Vec<u8> {
        let mut bytes: Vec<_> = std::iter::repeat(parameters.base_byte)
            .take(parameters.bytes_length)
            .collect();

        let mut i = parameters.sequence_start_idx;
        for &b in parameters.sequence {
            assert!(i < bytes.len());
            bytes[i] = b;
            i += 1;
        }

        bytes
    }

    #[test]
    fn find_byte_sequence_on_sequences_at_every_position_in_stream() {
        const BYTES_LENGTH: usize = 256;

        for sequence_length in 1..128 {
            for starting_index in 0..BYTES_LENGTH - sequence_length {
                let sequence = &LONG_SEQUENCE[..sequence_length];
                let parameters = TestByteStreamParameters {
                    base_byte: 200,
                    bytes_length: BYTES_LENGTH,
                    sequence,
                    sequence_start_idx: starting_index,
                };
                let bytes = test_byte_stream(parameters);

                let expected = Some(starting_index);
                let result = find_byte_sequence(sequence, &bytes);

                assert_eq!(
                    expected, result,
                    "failed for sequence of length {} with starting_index {}",
                    sequence_length, starting_index
                );
            }
        }
    }

    #[test]
    fn find_byte_sequence_on_byte_stream_with_a_fake_match() {
        for sequence_length in 2..128 {
            let sequence = &LONG_SEQUENCE[..sequence_length];
            let mut bytes = vec![];
            let mut expected = 0;
            for subsequence_length in 1..sequence_length - 1 {
                bytes.extend_from_slice(&LONG_SEQUENCE[..subsequence_length]);
                expected += subsequence_length;
            }

            bytes.extend_from_slice(&LONG_SEQUENCE);
            let result = find_byte_sequence(sequence, &bytes);

            assert_eq!(
                Some(expected),
                result,
                "failed for sequence of length {}",
                sequence_length
            );
        }
    }

    // This is duplicated test case from the `find_byte_sequence_benches` benchmark,
    // but for a smaller length so that tests run smoothly.
    // We know of no clean way of sharing the code without duplicating.
    #[test]
    fn find_byte_sequence_bench_correctness() {
        const LENGTH: usize = 16 * 1024 * 1024;
        const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";
        const SEQUENCE: &str = "umaxzlvhjkncfidewpyqrsbgotkfsniubghjlycmqxertwdzpvoa";

        let bytes = setup_bytes();
        let expected = Some(16777228);

        for sequence_length in [2, 3, 4, 8, 15, 16, 32, 33, 48] {
            let sequence = &SEQUENCE[..sequence_length];
            let result = find_byte_sequence(sequence.as_bytes(), bytes.as_bytes());

            assert_eq!(
                expected, result,
                "failed for sequence of length {}",
                sequence_length
            );
        }

        fn setup_bytes() -> String {
            let mut contents = String::new();
            while contents.len() < LENGTH {
                contents += LETTERS;
            }
            contents += SEQUENCE;
            contents += LETTERS;
            while contents.len() % 32 != 0 {
                contents += "x";
            }
            contents
        }
    }

    #[test]
    fn find_any_of_sequences_correctness_1() {
        let sequences = ["doggy".as_bytes(), "dog".as_bytes(), "cat".as_bytes()];
        let contents = "Petting a doggy.";
        let result = find_any_of_sequences(&sequences, contents.as_bytes());

        assert_eq!(result, Some((10, 0)));
    }

    #[test]
    fn find_any_of_sequences_correctness_2() {
        let sequences = ["doggy".as_bytes(), "dog".as_bytes(), "cat".as_bytes()];
        let contents = "Petting a dog.";
        let result = find_any_of_sequences(&sequences, contents.as_bytes());

        assert_eq!(result, Some((10, 1)));
    }

    #[test]
    fn find_any_of_sequences_correctness_3() {
        let sequences = ["doggy".as_bytes(), "dog".as_bytes(), "cat".as_bytes()];
        let contents = "Scratched by a cat.";
        let result = find_any_of_sequences(&sequences, contents.as_bytes());

        assert_eq!(result, Some((15, 2)));
    }
    #[test]
    fn find_any_of_sequences_correctness_4() {
        let sequences = ["doggy".as_bytes(), "dog".as_bytes(), "cat".as_bytes()];
        let contents = "I have no pets :(";
        let result = find_any_of_sequences(&sequences, contents.as_bytes());

        assert_eq!(result, None);
    }

    #[test]
    fn find_any_of_sequences_long_sequence_correctness_1() {
        let sequences = ["pretty long".as_bytes(), "pretty".as_bytes()];
        let contents = "Very pretty, but not pretty long.";
        let result = find_any_of_sequences(&sequences, contents.as_bytes());

        assert_eq!(result, Some((5, 1)));
    }

    #[test]
    fn find_any_of_sequences_long_sequence_correctness_2() {
        let sequences = ["pretty long".as_bytes(), "pretty".as_bytes()];
        let contents = "This sentence is not pretty long.";
        let result = find_any_of_sequences(&sequences, contents.as_bytes());

        assert_eq!(result, Some((21, 0)));
    }

    #[test]
    fn find_any_of_sequences_eight_sequences_correctness() {
        let sequences = [
            "aaaaaaab".as_bytes(),
            "aaaaaaac".as_bytes(),
            "aaaaaaad".as_bytes(),
            "aaaaaaae".as_bytes(),
            "aaaaaaaf".as_bytes(),
            "aaaaaaag".as_bytes(),
            "aaaaaaah".as_bytes(),
            "aaaaaaai".as_bytes(),
        ];
        let contents = std::iter::repeat(b'a')
            .take(1024)
            .chain(sequences[7].iter().copied())
            .collect::<Vec<_>>();
        let result = find_any_of_sequences(&sequences, &contents);

        assert_eq!(result, Some((1024, 7)));
    }
}
