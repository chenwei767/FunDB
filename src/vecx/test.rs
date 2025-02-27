//!
//! # Test Cases
//!

use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct SampleBlock {
    idx: usize,
    data: Vec<usize>,
}

fn gen_sample(idx: usize) -> SampleBlock {
    SampleBlock {
        idx,
        data: vec![idx],
    }
}


#[test]
fn t_vecx_iter_should_work() {
    {
        let cnt = 5;

        let db: serde_json::Value = {
            let mut db = crate::new_vecx!(3);
            assert_eq!(None, db.iter().next());

            (0..cnt).map(|i| (i, gen_sample(i))).for_each(|(i, b)| {
                db.push(b.clone());
                assert_eq!(1 + i as usize, db.len());
                assert_eq!(pnk!(db.get(i as usize)).into_inner().into_owned(), b);
                assert_eq!(pnk!(db.last()).into_inner().into_owned(), b);
            });

            assert_eq!(cnt, db.len());

            pnk!(serde_json::to_value(&db))
        };
        assert!(db.as_str().unwrap().contains(r#""in_mem_cnt":3"#));
        let db_restore: Vecx<SampleBlock> = pnk!(serde_json::from_value::<Vecx<SampleBlock>>(db));

        let mut iter = db_restore.iter_test();
        assert_eq!(0, iter.mem_hits);

        iter.next();
        iter.next();
        iter.next();
        iter.next();
        iter.next();

        assert_eq!(3, iter.mem_hits);
    }

    {
        let cnt = 5;

        let db: serde_json::Value = {
            let mut db = crate::new_vecx!(30);
            assert_eq!(None, db.iter().next());

            (0..cnt).map(|i| (i, gen_sample(i))).for_each(|(i, b)| {
                db.push(b.clone());
                assert_eq!(1 + i as usize, db.len());
                assert_eq!(pnk!(db.get(i as usize)).into_inner().into_owned(), b);
                assert_eq!(pnk!(db.last()).into_inner().into_owned(), b);
            });

            assert_eq!(cnt, db.len());

            pnk!(serde_json::to_value(&db))
        };
        assert!(db.as_str().unwrap().contains(r#""in_mem_cnt":30"#));
        let db_restore: Vecx<SampleBlock> = pnk!(serde_json::from_value::<Vecx<SampleBlock>>(db));

        let mut iter = db_restore.iter_test();
        assert_eq!(0, iter.mem_hits);

        iter.next();
        iter.next();
        iter.next();
        iter.next();
        iter.next();

        assert_eq!(5, iter.mem_hits);
    }
}

#[test]
fn t_vecx() {
    let cnt = 200;

    let db = {
        let mut db = crate::new_vecx!();

        assert_eq!(0, db.len());
        (0..cnt).for_each(|i| {
            assert!(db.get(i).is_none());
        });

        (0..cnt).map(|i| (i, gen_sample(i))).for_each(|(i, b)| {
            db.push(b.clone());
            assert_eq!(1 + i as usize, db.len());
            assert_eq!(pnk!(db.get(i as usize)).into_inner().into_owned(), b);
            assert_eq!(pnk!(db.last()).into_inner().into_owned(), b);
        });

        assert_eq!(cnt, db.len());

        pnk!(serde_json::to_vec(&db))
    };

    let db_restore = pnk!(serde_json::from_slice::<Vecx<SampleBlock>>(&db));

    (0..cnt).for_each(|i| {
        assert_eq!(i, db_restore.get(i).unwrap().idx);
    });

    assert_eq!(cnt, db_restore.len());
}
