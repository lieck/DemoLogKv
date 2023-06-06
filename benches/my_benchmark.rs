use criterion::{criterion_group, criterion_main, Criterion};
use rand::{seq::IteratorRandom};
use kvs::{KvStore, KvsEngine, SledKvsEngine};
use tempfile::NamedTempFile;
use rand::prelude::*;


fn write_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("write");
    let rng = &mut rand::thread_rng();
    let range = (1..100000).choose_multiple(rng, 1000).to_vec();
    group.bench_function("kvs", |b| {
        b.iter_batched(|| {
            let path = NamedTempFile::new().expect("error creating temporary");
            let path = path.path();
            let kvs = KvStore::open(path);
            kvs
        }, 
        |mut store| {
            for i in &range {
                store.set(i.to_string(), i.to_string()).expect("msg");
            }
        }, 
        criterion::BatchSize::SmallInput)
    });
    group.bench_function("sled", |b| {
        b.iter_batched(|| {
            let temp_dir = NamedTempFile::new().unwrap();
                (SledKvsEngine::new(sled::open(&temp_dir).unwrap()), temp_dir)
        }, 
        |(mut db, _temp_dir)| {
            for i in &range {
                db.set(i.to_string(), i.to_string()).expect("msg");
            }
        },
        criterion::BatchSize::SmallInput)
    });
    group.finish();
}


fn get_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("get_bench");

    let rng = &mut rand::thread_rng();
    let range = (1..100000).choose_multiple(rng, 1000).to_vec();

    group.bench_function("kvs", |b| {
        b.iter_batched(|| {
            let path = NamedTempFile::new().expect("error creating temporary");
            let path = path.path();
            let mut kvs = KvStore::open(path);

            for i in &range {
                kvs.set(i.to_string(), i.to_string()).expect("msg");
            }

            kvs
        }, 
        |store| {
            for i in &range {
                store.get(i.to_string()).expect("msg");
            }
        }, 
        criterion::BatchSize::SmallInput)
    });
    group.bench_function("sled", |b| {
        b.iter_batched(|| {
            let temp_dir = NamedTempFile::new().unwrap();
            let mut db = SledKvsEngine::new(sled::open(&temp_dir).unwrap());
            for i in &range {
                db.set(i.to_string(), i.to_string()).expect("msg");
            }
            db
        }, 
        |db| {
            for i in &range {
                db.get(i.to_string()).expect("msg");
            }
        },
        criterion::BatchSize::SmallInput)
    });

    group.finish();
}



criterion_group!(benches, write_benchmark, get_bench);
criterion_main!(benches);