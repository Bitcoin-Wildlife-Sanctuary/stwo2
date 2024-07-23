use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use stwo_prover::core::backend::simd::SimdBackend;
use stwo_prover::core::channel::sha256::BWSSha256Channel;
use stwo_prover::core::channel::Channel;
use stwo_prover::core::fields::m31::BaseField;
use stwo_prover::core::fields::IntoSlice;
use stwo_prover::core::vcs::bws_sha256_hash::BWSSha256Hasher;
use stwo_prover::core::vcs::hasher::Hasher;
use stwo_prover::examples::poseidon::{gen_trace, PoseidonAir, PoseidonComponent};
use stwo_prover::trace_generation::commit_and_prove;

pub fn simd_poseidon(c: &mut Criterion) {
    const LOG_N_ROWS: u32 = 15;
    let mut group = c.benchmark_group("poseidon2");
    group.throughput(Throughput::Elements(1u64 << (LOG_N_ROWS + 3)));
    group.bench_function(format!("poseidon2 2^{} instances", LOG_N_ROWS + 3), |b| {
        b.iter(|| {
            let component = PoseidonComponent {
                log_n_rows: LOG_N_ROWS,
            };
            let trace = gen_trace(component.log_column_size());
            let channel =
                &mut BWSSha256Channel::new(BWSSha256Hasher::hash(BaseField::into_slice(&[])));
            let air = PoseidonAir { component };
            commit_and_prove::<SimdBackend>(&air, channel, trace).unwrap()
        });
    });
}

criterion_group!(
    name = bit_rev;
    config = Criterion::default().sample_size(10);
    targets = simd_poseidon);
criterion_main!(bit_rev);
