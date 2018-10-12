
macro_rules! test_graph_impl {
    ($mutable_graph_impl:ident) => {
        test_graph_impl!(test, $mutable_graph_impl);
    };
    ($module_name: ident, $mutable_graph_impl: ident) => {
        mod $module_name {
            use ::graph::*;
            use ::graph::traits::GResult;
            use ::ns::*;
            use ::term::*;
            use ::triple::*;
            use std::borrow::Borrow;
            use std::fmt::Debug;
            use resiter::oks::*;

            #[allow(unused_imports)]
            use super::*;

            const NS: &str = "http://example.org/";

            type TestResult = GResult<$mutable_graph_impl, ()>;

            lazy_static!{
                static ref C1: StaticTerm = StaticTerm::new_iri2(NS, "C1").unwrap();
                static ref C2: StaticTerm = StaticTerm::new_iri2(NS, "C2").unwrap();
                static ref P1: StaticTerm = StaticTerm::new_iri2(NS, "p1").unwrap();
                static ref P2: StaticTerm = StaticTerm::new_iri2(NS, "p2").unwrap();
                static ref I1A: StaticTerm = StaticTerm::new_iri2(NS, "I1A").unwrap();
                static ref I1B: StaticTerm = StaticTerm::new_iri2(NS, "I1B").unwrap();
                static ref I2A: StaticTerm = StaticTerm::new_iri2(NS, "I2A").unwrap();
                static ref I2B: StaticTerm = StaticTerm::new_iri2(NS, "I2B").unwrap();
            }

            fn populate<G: MutableGraph> (g: &mut G) -> GResult<G, ()> {
                g.insert(&C1, &rdf::type_, &rdfs::Class)?;

                g.insert(&C2, &rdf::type_, &rdfs::Class)?;
                g.insert(&C2, &rdfs::subClassOf, &C1)?;

                g.insert(&P1, &rdf::type_, &rdf::Property)?;
                g.insert(&P1, &rdfs::domain, &C1)?;
                g.insert(&P1, &rdfs::range, &C2)?;

                g.insert(&P2, &rdf::type_, &rdf::Property)?;
                g.insert(&P2, &rdfs::domain, &C2)?;
                g.insert(&P2, &rdfs::range, &C2)?;

                g.insert(&I1A, &rdf::type_, &C1)?;
                g.insert(&I1B, &rdf::type_, &C1)?;
                g.insert(&I2A, &rdf::type_, &C2)?;
                g.insert(&I2B, &rdf::type_, &C2)?;
                g.insert(&I1A, &P1, &I2A)?;
                g.insert(&I1B, &P1, &I2B)?;
                g.insert(&I2A, &P2, &I2B)?;
                Ok(())
            }

            #[test]
            fn test_simple_mutations() -> TestResult {
                let mut g = $mutable_graph_impl::new();
                assert_eq!(g.iter().count(), 0);
                assert!   (g.insert(&C1, &rdf::type_, &rdfs::Class)?);
                assert_eq!(g.iter().count(), 1);
                assert!   (g.insert(&C1, &rdfs::subClassOf, &C2)?);
                assert_eq!(g.iter().count(), 2);
                assert!   (g.remove(&C1, &rdf::type_, &rdfs::Class)?);
                assert_eq!(g.iter().count(), 1);
                assert!   (g.remove(&C1, &rdfs::subClassOf, &C2)?);
                assert_eq!(g.iter().count(), 0);
                Ok(())
            }

            #[test]
            fn test_no_duplicate() -> TestResult {
                let mut g = $mutable_graph_impl::new();
                assert_eq!(g.iter().count(), 0);
                assert!   (g.insert(&C1, &rdf::type_, &rdfs::Class)?);
                assert_eq!(g.iter().count(), 1);
                assert!  (!g.insert(&C1, &rdf::type_, &rdfs::Class)?);
                assert_eq!(g.iter().count(), 1);
                assert!   (g.remove(&C1, &rdf::type_, &rdfs::Class)?);
                assert_eq!(g.iter().count(), 0);
                assert!  (!g.remove(&C1, &rdf::type_, &rdfs::Class)?);
                Ok(())
            }

            #[test]
            fn test_iter() -> TestResult {
                let mut g = $mutable_graph_impl::new();
                populate(&mut g)?;

                let v: Vec<_> = g.iter().oks().map(as_ref_t).collect();
                assert_eq!(v.len(), g.iter().count());
                assert_consistent_hint(v.len(), g.hint());
                assert!(v.contains(&C1, &rdf::type_, &rdfs::Class)?);
                assert!(!v.contains(&P1, &rdf::type_, &rdfs::Class)?);
                Ok(())
            }

            #[test]
            fn test_iter_for_s() -> TestResult {
                let mut g = $mutable_graph_impl::new();
                populate(&mut g)?;

                let v: Vec<_> = g.iter_for_s(&C2).oks().map(as_ref_t).collect();
                assert_eq!(v.len(), 2);
                assert_consistent_hint(v.len(), g.hint_for_s(&C2));
                assert!(v.contains(&C2, &rdf::type_, &rdfs::Class)?);
                assert!(!v.contains(&C1, &rdf::type_, &rdfs::Class)?);
                Ok(())
            }

            #[test]
            fn test_iter_for_p() -> TestResult {
                let mut g = $mutable_graph_impl::new();
                populate(&mut g)?;

                let v: Vec<_> = g.iter_for_p(&rdfs::subClassOf).oks().map(as_ref_t).collect();
                assert_eq!(v.len(), 1);
                assert_consistent_hint(v.len(), g.hint_for_p(&rdfs::subClassOf));
                assert!(v.contains(&C2, &rdfs::subClassOf, &C1)?);
                assert!(!v.contains(&C2, &rdf::type_, &rdfs::Class)?);
                Ok(())
            }

            #[test]
            fn test_iter_for_o() -> TestResult {
                let mut g = $mutable_graph_impl::new();
                populate(&mut g)?;

                let v: Vec<_> = g.iter_for_o(&I2B).oks().map(as_ref_t).collect();
                assert_eq!(v.len(), 2);
                assert_consistent_hint(v.len(), g.hint_for_o(&I2B));
                assert!(v.contains(&I1B, &P1, &I2B)?);
                assert!(!v.contains(&I2B, &rdf::type_, &C2)?);
                Ok(())
            }

            #[test]
            fn test_iter_for_sp() -> TestResult {
                let mut g = $mutable_graph_impl::new();
                populate(&mut g)?;

                let v: Vec<_> = g.iter_for_sp(&C2, &rdf::type_).oks().map(as_ref_t).collect();
                assert_eq!(v.len(), 1);
                assert_consistent_hint(v.len(), g.hint_for_sp(&C2, &rdf::type_));
                assert!(v.contains(&C2, &rdf::type_, &rdfs::Class)?);
                assert!(!v.contains(&C2, &rdfs::subClassOf, &C1)?);
                Ok(())
            }

            #[test]
            fn test_iter_for_so() -> TestResult  {
                let mut g = $mutable_graph_impl::new();
                populate(&mut g)?;

                let v: Vec<_> = g.iter_for_so(&C2, &C1).oks().map(as_ref_t).collect();
                assert_eq!(v.len(), 1);
                assert_consistent_hint(v.len(), g.hint_for_so(&C2, &C1));
                assert!(v.contains(&C2, &rdfs::subClassOf, &C1)?);
                assert!(!v.contains(&C2, &rdf::type_, &rdfs::Class)?);
                Ok(())
            }

            #[test]
            fn test_iter_for_po() -> TestResult {
                let mut g = $mutable_graph_impl::new();
                populate(&mut g)?;

                let v: Vec<_> = g.iter_for_po(&rdf::type_, &rdfs::Class).oks().map(as_ref_t).collect();
                assert_eq!(v.len(), 2);
                assert_consistent_hint(v.len(), g.hint_for_po(&rdf::type_, &rdfs::Class));
                assert!(v.contains(&C2, &rdf::type_, &rdfs::Class)?);
                assert!(!v.contains(&P2, &rdf::type_, &rdf::Property)?);
                Ok(())
            }

            #[test]
            fn test_iter_for_spo() -> TestResult {
                let mut g = $mutable_graph_impl::new();
                populate(&mut g)?;

                let v: Vec<_> = g.iter_for_spo(&C2, &rdf::type_, &rdfs::Class).oks().map(as_ref_t).collect();
                assert_eq!(v.len(), 1);
                assert!(v.contains(&C2, &rdf::type_, &rdfs::Class)?);
                assert!(!v.contains(&C1, &rdf::type_, &rdfs::Class)?);
                Ok(())
            }

            #[test]
            fn test_contains() -> TestResult {
                let mut g = $mutable_graph_impl::new();
                populate(&mut g)?;
                assert!(g.contains(&C2, &rdfs::subClassOf, &C1)?);
                assert!(!g.contains(&C1, &rdfs::subClassOf, &C2)?);
                Ok(())
            }

            fn as_ref_t<'a, T> (triple: (&'a Term<T>, &'a Term<T>, &'a Term<T>)) -> (RefTerm<'a>, RefTerm<'a>, RefTerm<'a>) where
                T: Borrow<str> + 'a,
            {
                (RefTerm::from(triple.0), RefTerm::from(triple.1), RefTerm::from(triple.2))
            }

            #[allow(dead_code)]
            fn dump_graph<G: Graph> (g: &G) where
                G::Holder: Debug,
            {
                println!("<<<<");
                for t in g.iter() {
                    let t = t.unwrap();
                    println!("{:?}\n{:?}\n{:?}\n\n", t.s(), t.p(), t.o());
                }
                println!(">>>>");
            }

            fn assert_consistent_hint(val: usize, hint: (usize, Option<usize>)) {
                assert!(hint.0 <= val);
                assert!(val <= hint.1.or(Some(val)).unwrap())
            }
        }
    }
}
