#![allow(clippy::let_with_type_underscore)]

use ::math::core::*;
use ::core::time;


fn trig_conversion(c: &mut ::criterion::Criterion) {
    let duration: time::Duration = time::Duration::from_secs(60);
    let mut c: _ = c.benchmark_group("trig_conversion");
    c.sample_size(10000).measurement_time(duration);
    macro_rules! op {
        ($($size:ty),*) => { 
            paste::paste! { $(
                c.bench_function(concat!("trig_conversion", stringify!($size)), |b| {
                    b.iter(|| {
                        let n: $size = ::criterion::black_box(10);
                        unsafe {
                            trig_conversion::to_degree::<2, _>(n).unwrap_unchecked();
                        }
                    });
                });
            )* }
        };
    }
    //op!(u16, u32, u64, u128, i16, i32, i64, i128);

    macro_rules! cos {
        ($($size:ty),*) => { 
            paste::paste! { $(
                c.bench_function(concat!("cos", stringify!($size)), |b| {
                    b.iter(|| {
                        let n: $size = ::criterion::black_box(15_0);
                        unsafe {
                            trig::cos::<1, _>(n).unwrap_unchecked();
                        }
                        
                    });
                });
            )* }
        };
    }
    cos!(u8);
}

fn cast(c: &mut ::criterion::Criterion) {
    let duration: time::Duration = time::Duration::from_secs(60);
    let mut c: _ = c.benchmark_group("cast");
    c.sample_size(10000).measurement_time(duration);
    macro_rules! op {
        ($($size:ty),*) => { 
            paste::paste! { $(
                c.bench_function(concat!("cast_", stringify!($size)), |b| {
                    b.iter(|| {
                        let n: $size = ::criterion::black_box(10);
                        conversion::cast::<1, 2, _>(n).unwrap();
                    });
                });
            )* }
        };
    }
    op!(u16, u32, u64, u128, i16, i32, i64, i128);
}


fn arithmetic(c: &mut criterion::Criterion) {
    let duration: core::time::Duration = core::time::Duration::from_secs(60);
    let warm_up_duration: core::time::Duration = core::time::Duration::from_secs(60);
    let mut c = c.benchmark_group("mul");
    c.sample_size(10000).measurement_time(duration).warm_up_time(warm_up_duration);
    macro_rules! mul {
        ($($size:ty),*) => { 
            paste::paste! { $(
                c.bench_function(concat!("mul_", stringify!($size)), |b| {
                    b.iter(|| {
                        let x: $size = criterion::black_box(10);
                        let y: $size = criterion::black_box(10);
                        math::core::arithmetic::mul::<2, _>(x, y).unwrap();
                    });
                });
            )* }
        };
    }
    mul!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
}

// 128 bit size 76% cheaper
// u - 30$ expensive
// i -> 50 - 70% expensive
fn div(c: &mut criterion::Criterion) {
    let duration: core::time::Duration = core::time::Duration::from_secs(60);
    let warm_up_duration: core::time::Duration = core::time::Duration::from_secs(60);
    let mut c = c.benchmark_group("mul");
    c.sample_size(10000).measurement_time(duration).warm_up_time(warm_up_duration);
    macro_rules! div {
        ($($size:ty),*) => {
            paste::paste! { $(
                c.bench_function(concat!("div_", stringify!($size)), |b| {
                    b.iter(|| {
                        let x: $size = criterion::black_box(300);
                        let y: $size = criterion::black_box(200);
                        math::core::arithmetic::div::<2, _>(x, y).unwrap();
                    });
                });
            )* }
        };
    }
    //div!(u16, u32, u64, u128, i16, i32, i64, i128);
    c.bench_function("sqrt", |b| {
        b.iter(|| {
            math::core::arithmetic::sqrt::<u8>(criterion::black_box(100));
        });
    });
}

fn bench_muldiv(c: &mut criterion::Criterion) {
    let duration: core::time::Duration = core::time::Duration::from_secs(60);
    let warm_up_duration: core::time::Duration = core::time::Duration::from_secs(60);
    let mut c = c.benchmark_group("muldiv");
    c.sample_size(10000).measurement_time(duration).warm_up_time(warm_up_duration);
    c.bench_function("muldiv_u8", |b| {
        b.iter(|| {
            let x: u8 = criterion::black_box(10);
            let y: u8 = criterion::black_box(10);
            let z: u8 = criterion::black_box(10);
            math::core::base_arithmetic::muldiv(x, y, z).unwrap();
        });
    });
    c.bench_function("muldiv_u16", |b| {
        b.iter(|| {
            let x: u16 = criterion::black_box(10);
            let y: u16 = criterion::black_box(10);
            let z: u16 = criterion::black_box(10);
            math::core::base_arithmetic::muldiv(x, y, z).unwrap();
        });
    });
    c.bench_function("muldiv_u32", |b| {
        b.iter(|| {
            let x: u32 = criterion::black_box(10);
            let y: u32 = criterion::black_box(10);
            let z: u32 = criterion::black_box(10);
            math::core::base_arithmetic::muldiv(x, y, z).unwrap();
        });
    });
    c.bench_function("muldiv_u64", |b| {
        b.iter(|| {
            let x: u64 = criterion::black_box(10);
            let y: u64 = criterion::black_box(10);
            let z: u64 = criterion::black_box(10);
            math::core::base_arithmetic::muldiv(x, y, z).unwrap();
        });
    });
    c.bench_function("muldiv_u128", |b| {
        b.iter(|| {
            let x: u128 = criterion::black_box(10);
            let y: u128 = criterion::black_box(10);
            let z: u128 = criterion::black_box(10);
            math::core::base_arithmetic::muldiv(x, y, z).unwrap();
        });
    });
    c.bench_function("muldiv_i8", |b| {
        b.iter(|| {
            let x: i8 = criterion::black_box(10);
            let y: i8 = criterion::black_box(10);
            let z: i8 = criterion::black_box(10);
            math::core::base_arithmetic::muldiv(x, y, z).unwrap();
        });
    });
    c.bench_function("muldiv_i16", |b| {
        b.iter(|| {
            let x: i16 = criterion::black_box(10);
            let y: i16 = criterion::black_box(10);
            let z: i16 = criterion::black_box(10);
            math::core::base_arithmetic::muldiv(x, y, z).unwrap();
        });
    });
    c.bench_function("muldiv_i32", |b| {
        b.iter(|| {
            let x: i32 = criterion::black_box(10);
            let y: i32 = criterion::black_box(10);
            let z: i32 = criterion::black_box(10);
            math::core::base_arithmetic::muldiv(x, y, z).unwrap();
        });
    });
    c.bench_function("muldiv_i64", |b| {
        b.iter(|| {
            let x: i64 = criterion::black_box(10);
            let y: i64 = criterion::black_box(10);
            let z: i64 = criterion::black_box(10);
            math::core::base_arithmetic::muldiv(x, y, z).unwrap();
        });
    });
    c.bench_function("muldiv_i128", |b| {
        b.iter(|| {
            let x: i128 = criterion::black_box(10);
            let y: i128 = criterion::black_box(10);
            let z: i128 = criterion::black_box(10);
            math::core::base_arithmetic::muldiv(x, y, z).unwrap();
        });
    });
}

criterion::criterion_group!(math, trig_conversion);
criterion::criterion_main!(math);