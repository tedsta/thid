use criterion::{Criterion, criterion_group, criterion_main};

fn set_core_affinity(_: &mut Criterion) {
    core_affinity::set_for_current(core_affinity::CoreId { id: 1 });
}

fn get_current_thread_initial_1000x(c: &mut Criterion) {
    c.bench_function("get_current_thread_initial_1000x", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                core::hint::black_box(thid::ThreadId::current());
                // Reset the local thread ID for the next run
                thid::ThreadId::clear();
            }
        });
    });
}

fn get_current_thread_id_1000x(c: &mut Criterion) {
    thid::ThreadId::current();

    c.bench_function("get_current_thread_id_1000x", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                core::hint::black_box(thid::ThreadId::current());
            }
        });
    });
}

fn get_current_thread_id_std_1000x(c: &mut Criterion) {
    thid::ThreadId::current();

    c.bench_function("get_current_thread_id_std_1000x", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                core::hint::black_box(std::thread::current().id());
            }
        });
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default();
    targets =
        set_core_affinity,
        get_current_thread_initial_1000x,
        get_current_thread_id_1000x,
        get_current_thread_id_std_1000x,
}
criterion_main!(benches);
