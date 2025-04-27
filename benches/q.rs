use math::math::Q::*;
use criterion::*;

#[macro_export]
macro_rules! bench_group {
    (
        $($group_name:literal, $func_name:ident, $body:block),* 
    ) => {
        $(
            fn $func_name(c: &mut Criterion) {
                let mut group = c.benchmark_group($group_name);
                group.bench_function($group_name, |b| {
                    b.iter(|| $body);
                });
                group.finish();
            }
        )*

        criterion_group!(benches, $($func_name),*);
        criterion_main!(benches);
    };
}

bench_group!(
    "mul_with_bit_simulation_group",
    benchmark_mul_with_bit_simulation, {
        let a: Q<2u8, u128> = q::<2u8, u128>(black_box(500));
        let b_: Q<2u8, u128> = q(black_box(50u128));
        let _ = a.mul_with_bit_simulation(b_).unwrap();
    },
    "mul_with_native",
    mul_with_native, {
        let a: Q<2u8, u128> = q::<2u8, u128>(black_box(500));
        let b_: Q<2u8, u128> = q(black_box(50u128));
        let _ = a.mul_with_native(b_).unwrap();
    },
    "add",
    add, {
        (q::<2u8, u128>(black_box(100u128)) + q(black_box(100u128))).unwrap();
    },
    "sub",
    sub, {
        (q::<2u8, u128>(black_box(100u128)) - q(black_box(100u128))).unwrap();
    }
);