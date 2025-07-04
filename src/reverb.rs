//! Reverberator module.

use crate::delay_line::{Buffer, DelayLine};

#[derive(Copy, Clone, Debug)]
struct OnePole {
    one: f32,
    a: f32,
    b: f32,
}

impl OnePole {
    /// Constructor for a new OnePole.
    pub fn new() -> Self {
        Self {
            one: 0.0,
            a: 1.0,
            b: 0.0,
        }
    }

    pub fn damping(&mut self, value: f32) {
        self.a = 1.0 - libm::fabsf(value);
        self.b = value;
    }

    pub fn call(&mut self, i: f32) -> f32 {
        self.one = i * self.a + self.one * self.b;
        self.one
    }
}

/// Plate Reverberator.
///
/// Design from:
///
/// Dattorro, J (1997). Effect design: Part 1: Reverberator and other filters.
///
/// Journal of Audio Engineering Society
/// [45(9):660-684](https://ccrma.stanford.edu/~dattorro/EffectDesignPart1.pdf)
#[derive(Clone, Debug)]
pub struct Reverb {
    delay_feed_1: f32,
    delay_feed_2: f32,
    decay_1: f32,
    decay_2: f32,
    decay: f32,

    pre_delay: DelayLine<[f32; 10]>,
    one_pole: OnePole,
    all_pass_in_1: DelayLine<[f32; 142]>,
    all_pass_in_2: DelayLine<[f32; 107]>,
    all_pass_in_3: DelayLine<[f32; 379]>,
    all_pass_in_4: DelayLine<[f32; 277]>,

    all_pass_decay_11: DelayLine<[f32; 672]>,
    all_pass_decay_12: DelayLine<[f32; 1800]>,

    delay_11: DelayLine<[f32; 4453]>,
    delay_12: DelayLine<[f32; 3720]>,

    one_pole_1: OnePole,
    all_pass_decay_21: DelayLine<[f32; 908]>,
    all_pass_decay_22: DelayLine<[f32; 2656]>,

    delay_21: DelayLine<[f32; 4217]>,
    delay_22: DelayLine<[f32; 3163]>,

    one_pole_2: OnePole,
}

impl Default for Reverb {
    fn default() -> Self {
        Self {
            delay_feed_1: 0.0,
            delay_feed_2: 0.0,
            decay_1: 0.0,
            decay_2: 0.0,
            decay: 0.0,

            pre_delay: DelayLine::new(),
            one_pole: OnePole::new(),
            all_pass_in_1: DelayLine::new(),
            all_pass_in_2: DelayLine::new(),
            all_pass_in_3: DelayLine::new(),
            all_pass_in_4: DelayLine::new(),

            all_pass_decay_11: DelayLine::new(),
            all_pass_decay_12: DelayLine::new(),

            delay_11: DelayLine::new(),
            delay_12: DelayLine::new(),

            one_pole_1: OnePole::new(),
            all_pass_decay_21: DelayLine::new(),
            all_pass_decay_22: DelayLine::new(),

            delay_21: DelayLine::new(),
            delay_22: DelayLine::new(),

            one_pole_2: OnePole::new(),
        }
    }
}

impl Reverb {
    /// Constructs and returns a default reverb.
    pub fn new() -> Self {
        let mut verb = Self::default();
        verb.bandwidth(0.9995);
        verb.decay(0.85);
        verb.damping(0.2);
        verb.diffusion(0.76, 0.666, 0.707, 0.517);
        verb
    }

    /// Sets the input signal bandwidth in range `0.0-1.0`.
    ///
    /// This sets the cutoff frequency of a one-pole low-pass filter on the
    /// input signal.
    pub fn bandwidth(&mut self, value: f32) -> &mut Self {
        self.one_pole_1.damping(1.0 - value);
        self
    }

    /// Sets the high-frequency damping amount in range `0.0-1.0`.
    ///
    /// Higher amounts will dampen the diffuse sound more quickly.
    /// rather than high frequencies.
    pub fn damping(&mut self, value: f32) -> &mut Self {
        self.one_pole_1.damping(value);
        self.one_pole_2.damping(value);
        self
    }

    /// Sets the decay factor in range `0.0-1.0`.
    pub fn decay(&mut self, value: f32) -> &mut Self {
        self.decay = value;
        self
    }

    /// Sets the diffusion amounts in range `0.0-1.0`.
    ///
    /// Values near 0.7 are recommended. Moving further away from 0.7 will lead
    /// to more distinct echoes.
    pub fn diffusion(&mut self, in_1: f32, in_2: f32, decay_1: f32, decay_2: f32) -> &mut Self {
        self.delay_feed_1 = in_1;
        self.delay_feed_2 = in_2;
        self.decay_1 = decay_1;
        self.decay_2 = decay_2;
        self
    }

    /// Sets the input diffusion 1 amount in range `0.0-1.0`.
    pub fn diffusion1(&mut self, value: f32) -> &mut Self {
        self.delay_feed_1 = value;
        self
    }

    /// Sets the input diffusion 2 amount in range `0.0-1.0`.
    pub fn diffusion2(&mut self, value: f32) -> &mut Self {
        self.delay_feed_2 = value;
        self
    }

    /// Sets the tank decay diffusion 1 amount in range `0.0-1.0`.
    pub fn diffusion_decay_1(&mut self, value: f32) -> &mut Self {
        self.decay_1 = value;
        self
    }

    /// Sets the tank decay diffusion 2 amount in range `0.0-1.0`.
    pub fn diffusion_decay_2(&mut self, value: f32) -> &mut Self {
        self.decay_2 = value;
        self
    }

    /// Computes the wet stereo output from dry mono input.
    /// - `input` - Dry input sample.
    /// - `gain`  - Gain of output.
    ///
    /// Returns a tuple of (wet output sample 1, wet output sample 2).
    pub fn calc_frame(&mut self, input: f32, gain: f32) -> (f32, f32) {
        let mut value = self.pre_delay.get_write_and_step(input * 0.5);
        value = self.one_pole.call(value);
        value = self.all_pass_in_1.allpass(value, self.delay_feed_1);
        value = self.all_pass_in_2.allpass(value, self.delay_feed_1);
        value = self.all_pass_in_3.allpass(value, self.delay_feed_2);
        value = self.all_pass_in_4.allpass(value, self.delay_feed_2);

        let mut a = value + self.delay_22.back() * self.decay;
        let mut b = value + self.delay_12.back() * self.decay;

        a = self.all_pass_decay_11.allpass(a, -self.decay_1);
        a = self.delay_11.get_write_and_step(a);
        a = self.one_pole_1.call(a) * self.decay;
        a = self.all_pass_decay_12.allpass(a, self.decay_2);
        self.delay_12.write(a);

        b = self.all_pass_decay_21.allpass(b, -self.decay_1);
        b = self.delay_21.get_write_and_step(b);
        b = self.one_pole_2.call(b) * self.decay;
        b = self.all_pass_decay_22.allpass(b, self.decay_2);
        self.delay_22.write(b);

        let output_1 = {
            self.delay_21.read(266) + self.delay_21.read(2974) - self.all_pass_decay_22.read(1913)
                + self.delay_22.read(1996)
                - self.delay_11.read(1990)
                - self.all_pass_decay_12.read(187)
                - self.delay_12.read(1066)
        } * gain;

        let output_2 = {
            self.delay_11.read(353) + self.delay_11.read(3627) - self.all_pass_decay_12.read(1228)
                + self.delay_12.read(2673)
                - self.delay_21.read(2111)
                - self.all_pass_decay_22.read(335)
                - self.delay_22.read(121)
        } * gain;

        (output_1, output_2)
    }

    /// Computes the stereo output for a block of mono samples.
    /// - `input`    - Dry mono input.
    /// - `output_1` - Wet stereo output 1.
    /// - `output_2` - Wet stereo output 2.
    /// - `gain` - Output gain.
    pub fn process(
        &mut self,
        input: &[f32],
        output_1: &mut [f32],
        output_2: &mut [f32],
        gain: f32,
    ) {
        for (in_sample, (out_sample_1, out_sample_2)) in input
            .iter()
            .zip(output_1.iter_mut().zip(output_2.iter_mut()))
        {
            (*out_sample_1, *out_sample_2) = self.calc_frame(*in_sample, gain);
        }
    }

    /// Computes the stereo output for a block of mono samples by adding the reverb to the mix.
    /// - `input` - Dry mono input.
    /// - `mix_1` - Stereo mix 1.
    /// - `mix_2` - Stereo mix 2.
    /// - `gain`  - Reverb gain.
    pub fn process_add(&mut self, input: &[f32], mix_1: &mut [f32], mix_2: &mut [f32], gain: f32) {
        for (in_sample, (mix_sample_1, mix_sample_2)) in
            input.iter().zip(mix_1.iter_mut().zip(mix_2.iter_mut()))
        {
            let (out_sample_1, out_sample_2) = self.calc_frame(*in_sample, gain);
            *mix_sample_1 += out_sample_1;
            *mix_sample_2 += out_sample_2;
        }
    }
}

/// Generates an implementation of Buffer for a fixed-size array with "$n" number of elements.
macro_rules! impl_buffer {
    ($n:expr) => {
        impl Buffer for [f32; $n] {
            fn zeroed() -> Self {
                [0.0; $n]
            }
            fn clone(&self) -> Self {
                *self
            }
            fn len(&self) -> usize {
                $n
            }
            fn index(&self, idx: usize) -> &f32 {
                &self[idx]
            }
            fn index_mut(&mut self, idx: usize) -> &mut f32 {
                &mut self[idx]
            }
        }
    };
}

impl_buffer!(10);
impl_buffer!(142);
impl_buffer!(107);
impl_buffer!(379);
impl_buffer!(277);
impl_buffer!(672);
impl_buffer!(1800);
impl_buffer!(4453);
impl_buffer!(3720);
impl_buffer!(908);
impl_buffer!(2656);
impl_buffer!(4217);
impl_buffer!(3163);
