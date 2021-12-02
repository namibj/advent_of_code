use differential_dataflow::consolidation::consolidate;
use differential_dataflow::lattice::Lattice;
use differential_dataflow::operators::arrange::upsert::arrange_from_upsert;
use differential_dataflow::operators::arrange::{ArrangeByKey, ArrangeBySelf};
use differential_dataflow::operators::{Count, JoinCore};
use differential_dataflow::trace::implementations::ord::OrdValSpine;
use differential_dataflow::trace::wrappers::frontier::TraceFrontier;
use differential_dataflow::trace::{cursor::CursorDebug, TraceReader};
use differential_dataflow::AsCollection;
use dogsdogsdogs::altneu::AltNeu;
use dogsdogsdogs::calculus::Integrate;
use timely::dataflow::operators::capture::Extract;
use timely::dataflow::operators::{Capture, Map, Probe, ToStream};
use timely::dataflow::Scope;
use timely::progress::frontier::AntichainRef;
use timely::progress::Antichain;

pub fn part1(input: &str) -> impl std::fmt::Display {
    let input = input.to_owned();
    timely::execute_directly(|worker| {
        let input = match worker.index() {
            0 => input,
            _ => Default::default(),
        };
        let input = input
            .split("\n")
            .enumerate()
            .map(|(i, x)| (x.to_owned(), i as u32, 1i32))
            .collect::<Vec<_>>();

        let (mut trace, probe) = worker.dataflow::<u32, _, _>(move |scope| {
            let input = input.to_stream(scope).as_collection();

            let stream = input.flat_map(|x| u64::from_str_radix(&x, 10).ok().map(|x| ((), x)));
            // stream.inspect_batch(|t, d| {dbg!(t, d);});
            let last = arrange_from_upsert::<_, OrdValSpine<_, _, _, _>>(
                &stream.inner.map(|((_k, v), t, r)| ((), Some(v), t)),
                "Upsert",
            );

            let count = scope.scoped::<AltNeu<_>, _, _>("DelayedJoin", |inner| {
                last.enter(inner).as_collection(|k,v| (*k,*v))
		//.inspect_batch(|t, d| println!("pre-join: {:?}: {:?}", t, d))
		    ;
                last.enter(inner)
                    .join_core(
                        &last.enter_at(inner, |_k, _v: &u64, &t| AltNeu::neu(t), |t| (t.time)),
                        |&_key, &val1, &val2| {
                            use std::cmp::Ordering::*;
                            match val1.cmp(&val2) {
                                Less => None,
                                Equal => None,
                                Greater => Some(()),
                            }
                        },
                    ) //.inspect_batch(|t, d| println!("post-join: {:?}: {:?}", t, d))
                    .integrate()
                //   .leave()
            });
            let count = count.arrange_by_self();
            (count.trace, count.stream.probe())
        });
        trace.set_physical_compaction(AntichainRef::new(&[]));
        trace.set_logical_compaction(AntichainRef::new(&[u32::MAX]));
        worker.step_or_park_while(None, || !probe.done());
        let mut upper = Antichain::new();
        trace.read_upper(&mut upper);
        let (mut cursor, storage) = TraceFrontier::make_from(trace, upper.borrow()).cursor();
        let mut output = cursor.to_vec(&storage);
        for (_0, ref mut changes) in output.iter_mut() {
            for (ref mut t, _r) in changes.iter_mut() {
                t.join_assign(&u32::MAX);
            }
            consolidate(changes);
            //dbg!(changes);
        }
        output[0].1[0].1
    })
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    ""
}
