mod odd_sketch {
    use bytes::Bytes;
    use crypto::hashes::blake2b::*;
    use crypto::sketches::odd_sketch::*;
    use primitives::script::Script;
    use utils::byte_ops::*;

    #[test]
    fn test_sketchable_permutation() {
        let script_a = Script::new(Bytes::from(&b"hello"[..]));
        let script_b = Script::new(Bytes::from(&b"script"[..]));
        let script_c = Script::new(Bytes::from(&b"world!!"[..]));
        let vec_a = vec![script_a.clone(), script_b.clone(), script_c.clone()];
        let vec_b = vec![script_b, script_a, script_c];
        assert_eq!(
            Bytes::from(vec_a.odd_sketch()),
            Bytes::from(vec_b.odd_sketch())
        )
    }

    #[test]
    fn test_sketchable_size() {
        let script_a = Script::new(Bytes::from(&b"hello"[..]));
        let script_b = Script::new(Bytes::from(&b"script"[..]));
        let script_c = Script::new(Bytes::from(&b"world!!"[..]));
        let script_d = Script::new(Bytes::from(&b"extra"[..]));
        let script_e = Script::new(Bytes::from(&b"extra2"[..]));
        let vec_a = vec![script_a, script_b, script_c, script_d, script_e];
        let sketch_a = vec_a.odd_sketch();
        assert_eq!(sketched_size(&sketch_a), 5)
    }

    #[test]
    fn test_sketchable_symmetric_difference() {
        let script_a = Script::new(Bytes::from(&b"hello"[..]));
        let script_b = Script::new(Bytes::from(&b"script"[..]));
        let script_c = Script::new(Bytes::from(&b"world!!"[..]));
        let script_d = Script::new(Bytes::from(&b"extra"[..]));
        let script_e = Script::new(Bytes::from(&b"extra2"[..]));
        let vec_a = vec![script_a.clone(), script_b.clone(), script_c.clone()];
        let vec_b = vec![script_b, script_a, script_d, script_e];
        let sketch_a = vec_a.odd_sketch();
        let sketch_b = vec_b.odd_sketch();
        assert_eq!(sketched_size(&sketch_a.byte_xor(sketch_b)), 3)
    }
}

mod iblt {
    use bytes::Bytes;
    use crypto::sketches::iblt::*;
    use std::collections::HashSet;
    use crypto::sketches::odd_sketch::*;

    #[test]
    fn test_iblt_single() {
        let mut iblt = IBLT::with_capacity(3, 1);
        let val = Bytes::from(&b"hello"[..]);
        iblt.insert(val.clone());

        let mut left: HashSet<Bytes> = HashSet::with_capacity(3);
        let mut right: HashSet<Bytes> = HashSet::with_capacity(3);

        left.insert(val);

        assert_eq!((left, right), iblt.decode().unwrap());
    }

    #[test]
    fn test_iblt_symmetric_difference() {
        let mut hashset_a: HashSet<Bytes> = HashSet::with_capacity(64);
        let mut hashset_b: HashSet<Bytes> = HashSet::with_capacity(64);

        let mut iblt_a = IBLT::with_capacity(64, 4);
        let mut iblt_b = IBLT::with_capacity(64, 4);

        for i in 0..1000 {
            iblt_a.insert(Bytes::from(&[i as u8][..]));
            hashset_a.insert(Bytes::from(&[i as u8][..]));
        }

        for i in 32..1000 {
            iblt_b.insert(Bytes::from(&[i as u8][..]));
            hashset_b.insert(Bytes::from(&[i as u8][..]));
        }

        let (res_left, res_right) = (iblt_a - iblt_b).decode().unwrap();
        assert!(hashset_a.difference(&hashset_b).all(|x| res_left.contains(x)));
        assert!(res_left.difference(&res_right).all(|x| hashset_a.contains(x)));
        assert!(hashset_b.difference(&hashset_a).all(|x| res_right.contains(x)));
        assert!(res_right.difference(&res_left).all(|x| hashset_b.contains(x)));
    }

    #[test]
    fn test_iblt_odd_sketch_pair(){
        let mut iblt = IBLT::with_capacity(64, 4);
        let mut hashset: HashSet<Bytes> = HashSet::with_capacity(64);

        for i in 0..8 {
            iblt.insert(Bytes::from(&[i as u8][..]));
            hashset.insert(Bytes::from(&[i as u8][..]));
        }

        assert_eq!(iblt, hashset.odd_sketch());
    }
}