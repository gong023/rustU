extern crate __;

#[test]
fn test_vec_first() {
    assert_eq!(1i, *__::Vect::new(vec!(1i, 2, 3)).first());
    assert_eq!("aa", *__::Vect::new(vec!("aa", "bb")).first())
}

#[test]
fn test_vec_last() {
    assert_eq!(3i, *__::Vect::new(vec!(1i, 2, 3)).last());
    assert_eq!("bb", *__::Vect::new(vec!("aa", "bb")).last())
}

#[test]
fn test_vec_initial() {
    let vec_int = vec!(1i, 1i, 2i);
    for x in __::Vect::new(vec_int).initial(2u).iter() {
        assert_eq!(1i, **x)
    }

    let vec_str = vec!("aa", "aa", "bb");
    assert_eq!(2u, __::Vect::new(vec_str).initial(2u).len());
}

#[test]
fn test_vec_rest() {
    let vec_int = vec!(1i, 2i, 3i, 3i);
    for x in __::Vect::new(vec_int).rest(2u).iter() {
        assert_eq!(3i, **x);
    }

    let vec_str = vec!("aa", "bb", "cc", "cc");
    assert_eq!(2u, __::Vect::new(vec_str).rest(2u).len());
}

#[test]
fn test_vec_exists() {
    let vec_int = vec!(1i, 2i, 3i);
    assert!(__::Vect::new(vec_int).exists::<int>(&1i))
//    assert!(__.exists::<int>(&1i, &vec_int));
//
//    let vec_str = vec!("aa", "bb", "cc");
//    assert!(__.exists::<&str>(&"aa", &vec_str));
}
//
//#[test]
//fn test_vec_without() {
//    let __ = underscore::collection::Vect;
//
//    let vec_int = vec!(1i, 2i, 2i);
//    let without_vec = __.without(&vec_int, &vec!(1i));
//    for x in without_vec.iter() {
//        assert_eq!(2i, **x);
//    }
//
//    let vec_str = vec!("aa", "bb", "bb", "cc");
//    let without_vec_str = __.without(&vec_str, &vec!("aa", "cc"));
//    for x in without_vec_str.iter() {
//        assert_eq!("bb", **x);
//    }
//}
//
//#[test]
//fn test_vec_union() {
//    let __ = underscore::collection::Vect;
//
//    let union_int_vec = __.union(&vec!(1i, 2i, 3i), &vec!(4i, 5i, 6i));
//    assert_eq!(vec!(1i, 2, 3, 4, 5, 6), union_int_vec);
//
//    let union_str_vec = __.union(&vec!("aa", "bb"), &vec!("cc", "dd"));
//    assert_eq!(vec!("aa", "bb", "cc", "dd"), union_str_vec);
//}
//
//#[test]
//fn test_vec_intersection() {
//    let __ = underscore::collection::Vect;
//
//    let vec_int = vec!(1i, 2, 3);
//    let intersect_int_vec = __.intersection(&vec_int, &vec!(2i, 4));
//    for x in intersect_int_vec.iter() {
//        assert_eq!(2i, **x);
//    }
//}
//
//#[test]
//fn test_vec_uniq() {
//    let __ = underscore::collection::Vect;
//
//    let vec_int = vec!(0u, 1, 1, 2, 2, 3);
//    let uniq_int_vec = __.uniq(vec_int);
//    for i in range(0u, 3) {
//        assert_eq!(i, *uniq_int_vec.get(i));
//    }
//}
