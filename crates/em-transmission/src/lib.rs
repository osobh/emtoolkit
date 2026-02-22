//! Chapter 2: Transmission Lines
//!
//! Computation engines for the 14 Chapter 2 simulation modules:
//! - Transmission line types (two-wire, coaxial, microstrip)
//! - TL equations (voltage/current standing waves)
//! - Smith chart coordinate mapping
//! - Impedance matching (quarter-wave, L/T/Pi networks, stub tuning)
//! - Transient response (bounce diagram)

pub mod line_types;
pub mod smith_chart;
pub mod standing_waves;
pub mod matching;
pub mod stub_tuning;
pub mod transient;
pub mod slotted_line;
pub mod lossy_line;
pub mod l_network;
pub mod multisection;
pub mod taper;
pub mod general_transformer;
