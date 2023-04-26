# lanceverb

[![Build Status](https://travis-ci.org/MindBuffer/lanceverb.svg?branch=master)](https://travis-ci.org/MindBuffer/lanceverb)

A super-fast mono-to-stereo plate reverberator based on the design from Jon Dattorro (1997).  
[Effect design: Part 1: Reverberator and other filters](https://ccrma.stanford.edu/~dattorro/EffectDesignPart1.pdf).

Originally written in C++ by Lance Putnam, ported to Rust by MindBuffer.

This crate is `no_std` with an overall RAM usage of ~90kB and therefore usable in embedded environments.
