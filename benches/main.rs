use criterion::{black_box, criterion_group, criterion_main, Criterion};

use num_traits::Float;

#[inline(always)]
pub fn horners_method_f<F: Float, G>(x: F, mut n: usize, mut g: G) -> F
where
    G: FnMut(usize) -> F,
{
    let mut sum = F::zero();

    while n > 0 {
        n -= 1;
        sum = sum.mul_add(x, g(n));
    }

    sum
}

pub fn horners_method<F: Float>(x: F, coeffs: &[F]) -> F {
    horners_method_f(x, coeffs.len(), |i| unsafe { *coeffs.get_unchecked(i) })
}

fn criterion_benchmark(c: &mut Criterion) {
    #[rustfmt::skip]
    let coeffs = black_box([
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        0.9066094402137101, 0.7030666449646632, 0.8062843184510005, 1.4354479997076703, 1.1700851966666594,
        1.0036799628327977, 0.669178962803656, 0.7728758968389648, 0.5606587385173203, 1.0939290310925256,
        0.8620002023538906, 1.2530914565673503, 1.4918792702029815, 1.3154976283644524, 1.3564397050359411,
        1.0271024168686784, 1.405690756664578, 0.5449121493513336, 0.9862179238638533, 0.9124457978499287,
        0.8732207167879933, 0.6630588917237896, 0.5904674982257736, 1.4169918094580403, 0.958839837872578,
        0.5505474299309041, 0.8383676032996494, 0.9596512540091879, 0.6559726022409615, 1.0826517080111482,
        1.3846791166569572, 1.3762199390279588, 0.6807699410480192, 0.9768566731838964, 1.2572212915635828,
        0.701803747744993, 0.8273020543751974, 1.4638922915963615, 1.348778424905363, 1.3457337576150634,
        1.1274404084913705, 0.6266756469558616,
    ]);

    let x = black_box(0.5);

    let mut g = c.benchmark_group("Polynomials");

    for i in [
        1,
        4,
        8,
        16,
        18,
        29,
        64,
        84,
        128,
        256,
        300,
        256 + 128,
        512,
        600,
        coeffs.len(),
    ] {
        let coeffs = &coeffs[..i];

        g.bench_function(format!("{:04}: Estrin's Scheme", coeffs.len()), |b| {
            b.iter(|| {
                for _ in 0..100 {
                    black_box(par_poly_eval::poly(x, coeffs));
                }
            });
        });

        g.bench_function(format!("{:04}: Horner's Method", coeffs.len()), |b| {
            b.iter(|| {
                for _ in 0..100 {
                    black_box(horners_method(x, coeffs));
                }
            });
        });
    }

    g.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
