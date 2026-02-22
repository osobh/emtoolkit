/* @ts-self-types="./em_wasm.d.ts" */

/**
 * @param {number} turns
 * @param {number} b_field
 * @param {number} area
 * @param {number} rpm
 * @returns {any}
 */
export function ac_generator(turns, b_field, area, rpm) {
    const ret = wasm.ac_generator(turns, b_field, area, rpm);
    return ret;
}

/**
 * @param {number} num_elements
 * @param {number} spacing
 * @param {number} beta_deg
 * @param {number} num_points
 * @returns {any}
 */
export function antenna_array(num_elements, spacing, beta_deg, num_points) {
    const ret = wasm.antenna_array(num_elements, spacing, beta_deg, num_points);
    return ret;
}

/**
 * @param {number} epsilon_r
 * @param {number} conductivity
 * @param {number} frequency
 * @param {number} e0
 * @param {number} z_max
 * @param {number} num_points
 * @returns {any}
 */
export function attenuation_profile(epsilon_r, conductivity, frequency, e0, z_max, num_points) {
    const ret = wasm.attenuation_profile(epsilon_r, conductivity, frequency, e0, z_max, num_points);
    return ret;
}

/**
 * @param {number} current
 * @param {number} rho
 * @returns {number}
 */
export function b_field_infinite_wire(current, rho) {
    const ret = wasm.b_field_infinite_wire(current, rho);
    return ret;
}

/**
 * @param {number} current
 * @param {number} half_length
 * @param {number} num_segments
 * @param {number} x_min
 * @param {number} x_max
 * @param {number} y_min
 * @param {number} y_max
 * @param {number} nx
 * @param {number} ny
 * @returns {any}
 */
export function b_field_wire_2d(current, half_length, num_segments, x_min, x_max, y_min, y_max, nx, ny) {
    const ret = wasm.b_field_wire_2d(current, half_length, num_segments, x_min, x_max, y_min, y_max, nx, ny);
    return ret;
}

/**
 * @param {string} geometry
 * @param {string} params_json
 * @returns {any}
 */
export function capacitance_calc(geometry, params_json) {
    const ptr0 = passStringToWasm0(geometry, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(params_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.capacitance_calc(ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * @param {number} x
 * @param {number} y
 * @param {number} z
 * @returns {any}
 */
export function cartesian_to_spherical(x, y, z) {
    const ret = wasm.cartesian_to_spherical(x, y, z);
    return ret;
}

/**
 * @param {number} charge
 * @param {number} height
 * @param {number} x_min
 * @param {number} x_max
 * @param {number} nx
 * @returns {any}
 */
export function charge_above_plane(charge, height, x_min, x_max, nx) {
    const ret = wasm.charge_above_plane(charge, height, x_min, x_max, nx);
    return ret;
}

/**
 * @param {number} rho_0
 * @param {number} epsilon_r
 * @param {number} conductivity
 * @param {number} radius
 * @param {number} t_end
 * @param {number} num_points
 * @returns {any}
 */
export function charge_relaxation(rho_0, epsilon_r, conductivity, radius, t_end, num_points) {
    const ret = wasm.charge_relaxation(rho_0, epsilon_r, conductivity, radius, t_end, num_points);
    return ret;
}

/**
 * @param {number} inner_r
 * @param {number} outer_inner_r
 * @param {number} outer_outer_r
 * @param {number} current
 * @param {number} r_max
 * @param {number} num_points
 * @returns {any}
 */
export function coaxial_cable_b(inner_r, outer_inner_r, outer_outer_r, current, r_max, num_points) {
    const ret = wasm.coaxial_cable_b(inner_r, outer_inner_r, outer_outer_r, current, r_max, num_points);
    return ret;
}

/**
 * @param {number} inner_radius
 * @param {number} outer_radius
 * @param {number} epsilon_r
 * @param {number} frequency
 * @returns {any}
 */
export function coaxial_line_params(inner_radius, outer_radius, epsilon_r, frequency) {
    const ret = wasm.coaxial_line_params(inner_radius, outer_radius, epsilon_r, frequency);
    return ret;
}

/**
 * @param {number} radius
 * @param {number} current
 * @param {number} z_min
 * @param {number} z_max
 * @param {number} num_points
 * @returns {any}
 */
export function current_loop_on_axis(radius, current, z_min, z_max, num_points) {
    const ret = wasm.current_loop_on_axis(radius, current, z_min, z_max, num_points);
    return ret;
}

/**
 * @param {number} db
 * @returns {number}
 */
export function db_to_power(db) {
    const ret = wasm.db_to_power(db);
    return ret;
}

/**
 * @param {number} d_um
 * @param {number} epsilon_core
 * @param {number} epsilon_clad
 * @param {number} wavelength_um
 * @param {number} max_modes
 * @returns {any}
 */
export function dielectric_slab_waveguide(d_um, epsilon_core, epsilon_clad, wavelength_um, max_modes) {
    const ret = wasm.dielectric_slab_waveguide(d_um, epsilon_core, epsilon_clad, wavelength_um, max_modes);
    return ret;
}

/**
 * @param {number} area
 * @param {number} separation
 * @param {number} epsilon_r
 * @param {number} v_peak
 * @param {number} omega
 * @param {number} t_end
 * @param {number} num_points
 * @returns {any}
 */
export function displacement_current_sim(area, separation, epsilon_r, v_peak, omega, t_end, num_points) {
    const ret = wasm.displacement_current_sim(area, separation, epsilon_r, v_peak, omega, t_end, num_points);
    return ret;
}

/**
 * @param {number} zl_re
 * @param {number} zl_im
 * @param {number} z0
 * @param {number} stub_separation
 * @param {boolean} use_short
 * @returns {any}
 */
export function double_stub_match(zl_re, zl_im, z0, stub_separation, use_short) {
    const ret = wasm.double_stub_match(zl_re, zl_im, z0, stub_separation, use_short);
    return ret;
}

/**
 * @param {string} charges_json
 * @param {number} x_min
 * @param {number} x_max
 * @param {number} y_min
 * @param {number} y_max
 * @param {number} nx
 * @param {number} ny
 * @returns {any}
 */
export function electric_field_2d(charges_json, x_min, x_max, y_min, y_max, nx, ny) {
    const ptr0 = passStringToWasm0(charges_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.electric_field_2d(ptr0, len0, x_min, x_max, y_min, y_max, nx, ny);
    return ret;
}

/**
 * @param {string} charges_json
 * @param {number} source_idx
 * @param {number} num_lines
 * @param {number} num_steps
 * @returns {any}
 */
export function field_lines(charges_json, source_idx, num_lines, num_steps) {
    const ptr0 = passStringToWasm0(charges_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.field_lines(ptr0, len0, source_idx, num_lines, num_steps);
    return ret;
}

/**
 * @param {number} epsilon1
 * @param {number} epsilon2
 * @param {number} sigma2
 * @param {number} frequency
 * @param {number} angle_deg
 * @returns {any}
 */
export function fresnel_lossy_coefficients(epsilon1, epsilon2, sigma2, frequency, angle_deg) {
    const ret = wasm.fresnel_lossy_coefficients(epsilon1, epsilon2, sigma2, frequency, angle_deg);
    return ret;
}

/**
 * @param {number} epsilon1
 * @param {number} epsilon2
 * @param {number} sigma2
 * @param {number} frequency
 * @param {number} num_points
 * @returns {any}
 */
export function fresnel_lossy_vs_angle(epsilon1, epsilon2, sigma2, frequency, num_points) {
    const ret = wasm.fresnel_lossy_vs_angle(epsilon1, epsilon2, sigma2, frequency, num_points);
    return ret;
}

/**
 * @param {number} eta1
 * @param {number} eta2
 * @returns {any}
 */
export function fresnel_normal(eta1, eta2) {
    const ret = wasm.fresnel_normal(eta1, eta2);
    return ret;
}

/**
 * @param {number} er1
 * @param {number} er2
 * @param {number} theta_i_deg
 * @returns {any}
 */
export function fresnel_oblique(er1, er2, theta_i_deg) {
    const ret = wasm.fresnel_oblique(er1, er2, theta_i_deg);
    return ret;
}

/**
 * @param {number} er1
 * @param {number} er2
 * @param {number} num_points
 * @returns {any}
 */
export function fresnel_vs_angle(er1, er2, num_points) {
    const ret = wasm.fresnel_vs_angle(er1, er2, num_points);
    return ret;
}

/**
 * @param {number} p_tx_w
 * @param {number} g_tx_db
 * @param {number} g_rx_db
 * @param {number} frequency
 * @param {number} distance
 * @returns {any}
 */
export function friis_link(p_tx_w, g_tx_db, g_rx_db, frequency, distance) {
    const ret = wasm.friis_link(p_tx_w, g_tx_db, g_rx_db, frequency, distance);
    return ret;
}

/**
 * @param {number} rho_l
 * @param {number} epsilon_r
 * @param {number} rho_min
 * @param {number} rho_max
 * @param {number} num_points
 * @returns {any}
 */
export function gauss_line_charge(rho_l, epsilon_r, rho_min, rho_max, num_points) {
    const ret = wasm.gauss_line_charge(rho_l, epsilon_r, rho_min, rho_max, num_points);
    return ret;
}

/**
 * @param {number} total_charge
 * @param {number} radius
 * @param {number} epsilon_r
 * @param {number} r_max
 * @param {number} num_points
 * @returns {any}
 */
export function gauss_sphere_profile(total_charge, radius, epsilon_r, r_max, num_points) {
    const ret = wasm.gauss_sphere_profile(total_charge, radius, epsilon_r, r_max, num_points);
    return ret;
}

/**
 * @param {number} z0
 * @param {number} zl_re
 * @param {number} zl_im
 * @param {number} z_transformer
 * @param {number} length_wavelengths
 * @param {number} frequency
 * @returns {any}
 */
export function general_transformer_analysis(z0, zl_re, zl_im, z_transformer, length_wavelengths, frequency) {
    const ret = wasm.general_transformer_analysis(z0, zl_re, zl_im, z_transformer, length_wavelengths, frequency);
    return ret;
}

/**
 * @returns {any}
 */
export function get_constants() {
    const ret = wasm.get_constants();
    return ret;
}

/**
 * @param {number} frequency
 * @param {number} current
 * @param {number} num_points
 * @returns {any}
 */
export function half_wave_dipole(frequency, current, num_points) {
    const ret = wasm.half_wave_dipole(frequency, current, num_points);
    return ret;
}

/**
 * @param {number} radius
 * @param {number} current
 * @param {number} turns
 * @param {number} z_min
 * @param {number} z_max
 * @param {number} num_points
 * @returns {any}
 */
export function helmholtz_coil(radius, current, turns, z_min, z_max, num_points) {
    const ret = wasm.helmholtz_coil(radius, current, turns, z_min, z_max, num_points);
    return ret;
}

/**
 * @param {number} length
 * @param {number} current
 * @param {number} frequency
 * @param {number} num_points
 * @returns {any}
 */
export function hertzian_dipole(length, current, frequency, num_points) {
    const ret = wasm.hertzian_dipole(length, current, frequency, num_points);
    return ret;
}

/**
 * @param {string} geometry
 * @param {string} params_json
 * @returns {any}
 */
export function inductance_calc(geometry, params_json) {
    const ptr0 = passStringToWasm0(geometry, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(params_json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.inductance_calc(ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * Initialize the WASM module (call once from JS).
 */
export function init() {
    wasm.init();
}

/**
 * @param {number} zl_re
 * @param {number} zl_im
 * @param {number} z0
 * @param {number} beta_l
 * @returns {any}
 */
export function input_impedance_lossless(zl_re, zl_im, z0, beta_l) {
    const ret = wasm.input_impedance_lossless(zl_re, zl_im, z0, beta_l);
    return ret;
}

/**
 * @param {number} z_source
 * @param {number} zl_re
 * @param {number} zl_im
 * @param {number} frequency
 * @returns {any}
 */
export function l_network_match(z_source, zl_re, zl_im, frequency) {
    const ret = wasm.l_network_match(z_source, zl_re, zl_im, frequency);
    return ret;
}

/**
 * @param {number} p_tx_w
 * @param {number} g_tx_db
 * @param {number} g_rx_db
 * @param {number} frequency
 * @param {number} r_min
 * @param {number} r_max
 * @param {number} num_points
 * @returns {any}
 */
export function link_vs_distance(p_tx_w, g_tx_db, g_rx_db, frequency, r_min, r_max, num_points) {
    const ret = wasm.link_vs_distance(p_tx_w, g_tx_db, g_rx_db, frequency, r_min, r_max, num_points);
    return ret;
}

/**
 * @param {number} r
 * @param {number} l
 * @param {number} g
 * @param {number} c
 * @param {number} frequency
 * @param {number} length
 * @param {number} z_load_re
 * @param {number} z_load_im
 * @returns {any}
 */
export function lossy_line_analysis(r, l, g, c, frequency, length, z_load_re, z_load_im) {
    const ret = wasm.lossy_line_analysis(r, l, g, c, frequency, length, z_load_re, z_load_im);
    return ret;
}

/**
 * @param {number} z0_re
 * @param {number} z0_im
 * @param {number} gamma_re
 * @param {number} gamma_im
 * @param {number} z_load_re
 * @param {number} z_load_im
 * @param {number} length
 * @param {number} num_points
 * @returns {any}
 */
export function lossy_line_profile(z0_re, z0_im, gamma_re, gamma_im, z_load_re, z_load_im, length, num_points) {
    const ret = wasm.lossy_line_profile(z0_re, z0_im, gamma_re, gamma_im, z_load_re, z_load_im, length, num_points);
    return ret;
}

/**
 * @param {number} epsilon_r
 * @param {number} mu_r
 * @param {number} conductivity
 * @param {number} frequency
 * @returns {any}
 */
export function medium_properties(epsilon_r, mu_r, conductivity, frequency) {
    const ret = wasm.medium_properties(epsilon_r, mu_r, conductivity, frequency);
    return ret;
}

/**
 * @param {number} z0
 * @param {number} z_load
 * @param {number} num_sections
 * @returns {any}
 */
export function multisection_binomial(z0, z_load, num_sections) {
    const ret = wasm.multisection_binomial(z0, z_load, num_sections);
    return ret;
}

/**
 * @param {number} z0
 * @param {number} z_load
 * @param {number} num_sections
 * @param {number} max_gamma
 * @returns {any}
 */
export function multisection_chebyshev(z0, z_load, num_sections, max_gamma) {
    const ret = wasm.multisection_chebyshev(z0, z_load, num_sections, max_gamma);
    return ret;
}

/**
 * @param {string} preset
 * @param {number} x
 * @param {number} y
 * @param {number} z
 * @returns {any}
 */
export function numerical_gradient(preset, x, y, z) {
    const ptr0 = passStringToWasm0(preset, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.numerical_gradient(ptr0, len0, x, y, z);
    return ret;
}

/**
 * @param {number} d_mm
 * @param {number} epsilon_r
 * @param {number} frequency
 * @param {number} max_modes
 * @returns {any}
 */
export function parallel_plate_waveguide(d_mm, epsilon_r, frequency, max_modes) {
    const ret = wasm.parallel_plate_waveguide(d_mm, epsilon_r, frequency, max_modes);
    return ret;
}

/**
 * @param {number} i1
 * @param {number} i2
 * @param {number} separation
 * @param {number} length
 * @returns {any}
 */
export function parallel_wire_force(i1, i2, separation, length) {
    const ret = wasm.parallel_wire_force(i1, i2, separation, length);
    return ret;
}

/**
 * @param {number} a1
 * @param {number} f1
 * @param {number} p1
 * @param {number} a2
 * @param {number} f2
 * @param {number} p2
 * @param {number} t_end
 * @param {number} num_points
 * @returns {any}
 */
export function phase_comparison(a1, f1, p1, a2, f2, p2, t_end, num_points) {
    const ret = wasm.phase_comparison(a1, f1, p1, a2, f2, p2, t_end, num_points);
    return ret;
}

/**
 * @param {number} ax
 * @param {number} ay
 * @param {number} delta_deg
 * @param {number} num_trace
 * @returns {any}
 */
export function polarization_state(ax, ay, delta_deg, num_trace) {
    const ret = wasm.polarization_state(ax, ay, delta_deg, num_trace);
    return ret;
}

/**
 * @param {number} e0
 * @param {number} eta
 * @param {number} gamma_mag
 * @returns {any}
 */
export function power_calculations(e0, eta, gamma_mag) {
    const ret = wasm.power_calculations(e0, eta, gamma_mag);
    return ret;
}

/**
 * @param {number} linear
 * @returns {number}
 */
export function power_to_db(linear) {
    const ret = wasm.power_to_db(linear);
    return ret;
}

/**
 * @param {number} z_load
 * @param {number} z_line
 * @param {number} frequency
 * @returns {any}
 */
export function quarter_wave_match(z_load, z_line, frequency) {
    const ret = wasm.quarter_wave_match(z_load, z_line, frequency);
    return ret;
}

/**
 * @param {number} zl_re
 * @param {number} zl_im
 * @param {number} z0
 * @returns {any}
 */
export function reflection_coefficient(zl_re, zl_im, z0) {
    const ret = wasm.reflection_coefficient(zl_re, zl_im, z0);
    return ret;
}

/**
 * @param {number} voltage
 * @param {number} resistance
 * @param {number} inductance_val
 * @param {number} t_end
 * @param {number} num_points
 * @returns {any}
 */
export function rl_step(voltage, resistance, inductance_val, t_end, num_points) {
    const ret = wasm.rl_step(voltage, resistance, inductance_val, t_end, num_points);
    return ret;
}

/**
 * @param {string} preset
 * @param {number} x_min
 * @param {number} x_max
 * @param {number} y_min
 * @param {number} y_max
 * @param {number} nx
 * @param {number} ny
 * @returns {any}
 */
export function scalar_field_2d(preset, x_min, x_max, y_min, y_max, nx, ny) {
    const ptr0 = passStringToWasm0(preset, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.scalar_field_2d(ptr0, len0, x_min, x_max, y_min, y_max, nx, ny);
    return ret;
}

/**
 * @param {number} zl_re
 * @param {number} zl_im
 * @param {number} z0
 * @param {boolean} use_short
 * @returns {any}
 */
export function series_stub_match(zl_re, zl_im, z0, use_short) {
    const ret = wasm.series_stub_match(zl_re, zl_im, z0, use_short);
    return ret;
}

/**
 * @param {number} zl_re
 * @param {number} zl_im
 * @param {number} z0
 * @param {boolean} use_short
 * @returns {any}
 */
export function single_stub_match(zl_re, zl_im, z0, use_short) {
    const ret = wasm.single_stub_match(zl_re, zl_im, z0, use_short);
    return ret;
}

/**
 * @param {number} b_peak
 * @param {number} area
 * @param {number} omega
 * @param {number} t_end
 * @param {number} num_points
 * @returns {any}
 */
export function sinusoidal_emf(b_peak, area, omega, t_end, num_points) {
    const ret = wasm.sinusoidal_emf(b_peak, area, omega, t_end, num_points);
    return ret;
}

/**
 * @param {number} amplitude
 * @param {number} frequency
 * @param {number} phase_rad
 * @param {number} damping
 * @param {number} t_end
 * @param {number} num_points
 * @returns {any}
 */
export function sinusoidal_wave(amplitude, frequency, phase_rad, damping, t_end, num_points) {
    const ret = wasm.sinusoidal_wave(amplitude, frequency, phase_rad, damping, t_end, num_points);
    return ret;
}

/**
 * @param {number} epsilon_r
 * @param {number} conductivity
 * @param {number} f_min
 * @param {number} f_max
 * @param {number} num_points
 * @returns {any}
 */
export function skin_depth_vs_frequency(epsilon_r, conductivity, f_min, f_max, num_points) {
    const ret = wasm.skin_depth_vs_frequency(epsilon_r, conductivity, f_min, f_max, num_points);
    return ret;
}

/**
 * @param {number} v_max
 * @param {number} v_min
 * @param {number} d_min_from_load
 * @param {number} wavelength
 * @param {number} z0
 * @returns {any}
 */
export function slotted_line_measurement(v_max, v_min, d_min_from_load, wavelength, z0) {
    const ret = wasm.slotted_line_measurement(v_max, v_min, d_min_from_load, wavelength, z0);
    return ret;
}

/**
 * @param {number} gamma_magnitude
 * @param {number} gamma_angle_rad
 * @param {number} num_wavelengths
 * @param {number} num_points
 * @returns {any}
 */
export function slotted_line_standing_wave(gamma_magnitude, gamma_angle_rad, num_wavelengths, num_points) {
    const ret = wasm.slotted_line_standing_wave(gamma_magnitude, gamma_angle_rad, num_wavelengths, num_points);
    return ret;
}

/**
 * @param {number} zl_re
 * @param {number} zl_im
 * @param {number} alpha
 * @param {number} beta
 * @param {number} length
 * @param {number} num_points
 * @returns {any}
 */
export function smith_chart_lossy_trace(zl_re, zl_im, alpha, beta, length, num_points) {
    const ret = wasm.smith_chart_lossy_trace(zl_re, zl_im, alpha, beta, length, num_points);
    return ret;
}

/**
 * @param {number} zl_re
 * @param {number} zl_im
 * @returns {any}
 */
export function smith_chart_point(zl_re, zl_im) {
    const ret = wasm.smith_chart_point(zl_re, zl_im);
    return ret;
}

/**
 * @param {number} gamma_mag
 * @param {number} num_points
 * @returns {any}
 */
export function smith_chart_swr_circle(gamma_mag, num_points) {
    const ret = wasm.smith_chart_swr_circle(gamma_mag, num_points);
    return ret;
}

/**
 * @param {number} zl_re
 * @param {number} zl_im
 * @param {number} electrical_length
 * @param {number} num_points
 * @returns {any}
 */
export function smith_chart_trace(zl_re, zl_im, electrical_length, num_points) {
    const ret = wasm.smith_chart_trace(zl_re, zl_im, electrical_length, num_points);
    return ret;
}

/**
 * @param {number} turns
 * @param {number} length
 * @param {number} current
 * @param {number} radius
 * @param {number} mu_r
 * @returns {any}
 */
export function solenoid_params(turns, length, current, radius, mu_r) {
    const ret = wasm.solenoid_params(turns, length, current, radius, mu_r);
    return ret;
}

/**
 * @param {number} r
 * @param {number} theta
 * @param {number} phi
 * @returns {any}
 */
export function spherical_to_cartesian(r, theta, phi) {
    const ret = wasm.spherical_to_cartesian(r, theta, phi);
    return ret;
}

/**
 * @param {number} z0
 * @param {number} vswr
 * @param {number} gamma_angle_deg
 * @param {number} frequency
 * @param {number} length
 * @param {number} num_points
 * @returns {any}
 */
export function standing_wave_from_vswr(z0, vswr, gamma_angle_deg, frequency, length, num_points) {
    const ret = wasm.standing_wave_from_vswr(z0, vswr, gamma_angle_deg, frequency, length, num_points);
    return ret;
}

/**
 * @param {number} z0
 * @param {number} zl_re
 * @param {number} zl_im
 * @param {number} frequency
 * @param {number} length
 * @param {number} num_points
 * @returns {any}
 */
export function standing_wave_pattern(z0, zl_re, zl_im, frequency, length, num_points) {
    const ret = wasm.standing_wave_pattern(z0, zl_re, zl_im, frequency, length, num_points);
    return ret;
}

/**
 * @param {number} width
 * @param {number} ground_spacing
 * @param {number} epsilon_r
 * @returns {any}
 */
export function stripline_params(width, ground_spacing, epsilon_r) {
    const ret = wasm.stripline_params(width, ground_spacing, epsilon_r);
    return ret;
}

/**
 * @param {number} z0
 * @param {number} z_load
 * @param {number} length_wavelengths
 * @param {string} taper_type_str
 * @returns {any}
 */
export function taper_design(z0, z_load, length_wavelengths, taper_type_str) {
    const ptr0 = passStringToWasm0(taper_type_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.taper_design(z0, z_load, length_wavelengths, ptr0, len0);
    return ret;
}

/**
 * @param {number} n_primary
 * @param {number} n_secondary
 * @param {number} v_primary
 * @param {number} i_primary
 * @returns {any}
 */
export function transformer(n_primary, n_secondary, v_primary, i_primary) {
    const ret = wasm.transformer(n_primary, n_secondary, v_primary, i_primary);
    return ret;
}

/**
 * @param {number} amplitude
 * @param {number} frequency
 * @param {number} t
 * @param {number} x_max
 * @param {number} num_points
 * @returns {any}
 */
export function traveling_wave_snapshot(amplitude, frequency, t, x_max, num_points) {
    const ret = wasm.traveling_wave_snapshot(amplitude, frequency, t, x_max, num_points);
    return ret;
}

/**
 * @param {number} element_length_wavelengths
 * @param {number} spacing_wavelengths
 * @param {number} phase_shift_deg
 * @param {number} num_points
 * @returns {any}
 */
export function two_element_array(element_length_wavelengths, spacing_wavelengths, phase_shift_deg, num_points) {
    const ret = wasm.two_element_array(element_length_wavelengths, spacing_wavelengths, phase_shift_deg, num_points);
    return ret;
}

/**
 * @param {number} z0
 * @param {number} zl_re
 * @param {number} zl_im
 * @param {number} z1
 * @param {number} l1_wavelengths
 * @param {number} z2
 * @param {number} l2_wavelengths
 * @param {number} frequency
 * @returns {any}
 */
export function two_line_transformer_analysis(z0, zl_re, zl_im, z1, l1_wavelengths, z2, l2_wavelengths, frequency) {
    const ret = wasm.two_line_transformer_analysis(z0, zl_re, zl_im, z1, l1_wavelengths, z2, l2_wavelengths, frequency);
    return ret;
}

/**
 * @param {number} wire_radius
 * @param {number} separation
 * @param {number} epsilon_r
 * @returns {any}
 */
export function two_wire_params(wire_radius, separation, epsilon_r) {
    const ret = wasm.two_wire_params(wire_radius, separation, epsilon_r);
    return ret;
}

/**
 * @param {number} value
 * @param {string} from_unit
 * @returns {any}
 */
export function unit_conversions(value, from_unit) {
    const ptr0 = passStringToWasm0(from_unit, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.unit_conversions(value, ptr0, len0);
    return ret;
}

/**
 * @param {number} ax
 * @param {number} ay
 * @param {number} az
 * @param {number} bx
 * @param {number} by
 * @param {number} bz
 * @returns {any}
 */
export function vector_add(ax, ay, az, bx, by, bz) {
    const ret = wasm.vector_add(ax, ay, az, bx, by, bz);
    return ret;
}

/**
 * @param {number} ax
 * @param {number} ay
 * @param {number} az
 * @param {number} bx
 * @param {number} by
 * @param {number} bz
 * @returns {any}
 */
export function vector_cross(ax, ay, az, bx, by, bz) {
    const ret = wasm.vector_cross(ax, ay, az, bx, by, bz);
    return ret;
}

/**
 * @param {string} preset
 * @param {number} x_min
 * @param {number} x_max
 * @param {number} y_min
 * @param {number} y_max
 * @param {number} nx
 * @param {number} ny
 * @returns {any}
 */
export function vector_field_2d(preset, x_min, x_max, y_min, y_max, nx, ny) {
    const ptr0 = passStringToWasm0(preset, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.vector_field_2d(ptr0, len0, x_min, x_max, y_min, y_max, nx, ny);
    return ret;
}

/**
 * @param {number} vx
 * @param {number} vy
 * @param {number} vz
 * @param {number} rx
 * @param {number} ry
 * @param {number} rz
 * @returns {any}
 */
export function vector_project(vx, vy, vz, rx, ry, rz) {
    const ret = wasm.vector_project(vx, vy, vz, rx, ry, rz);
    return ret;
}

/**
 * Get the toolkit version.
 * @returns {string}
 */
export function version() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.version();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

/**
 * @param {number} a_mm
 * @param {number} b_mm
 * @param {number} epsilon_r
 * @param {number} frequency
 * @returns {any}
 */
export function waveguide_rect(a_mm, b_mm, epsilon_r, frequency) {
    const ret = wasm.waveguide_rect(a_mm, b_mm, epsilon_r, frequency);
    return ret;
}

/**
 * @param {number} frequency_hz
 * @returns {number}
 */
export function wavelength(frequency_hz) {
    const ret = wasm.wavelength(frequency_hz);
    return ret;
}

/**
 * @param {number} frequency_hz
 * @returns {number}
 */
export function wavenumber(frequency_hz) {
    const ret = wasm.wavenumber(frequency_hz);
    return ret;
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_8c4e43fe74559d73: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return ret;
        },
        __wbg___wbindgen_debug_string_0bc8482c6e3508ae: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_is_string_cd444516edc5b180: function(arg0) {
            const ret = typeof(arg0) === 'string';
            return ret;
        },
        __wbg___wbindgen_throw_be289d5034ed271b: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_new_361308b2356cecd0: function() {
            const ret = new Object();
            return ret;
        },
        __wbg_new_3eb36ae241fe6f44: function() {
            const ret = new Array();
            return ret;
        },
        __wbg_new_dca287b076112a51: function() {
            const ret = new Map();
            return ret;
        },
        __wbg_set_1eb0999cf5d27fc8: function(arg0, arg1, arg2) {
            const ret = arg0.set(arg1, arg2);
            return ret;
        },
        __wbg_set_3f1d0b984ed272ed: function(arg0, arg1, arg2) {
            arg0[arg1] = arg2;
        },
        __wbg_set_f43e577aea94465b: function(arg0, arg1, arg2) {
            arg0[arg1 >>> 0] = arg2;
        },
        __wbindgen_cast_0000000000000001: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_0000000000000002: function(arg0) {
            // Cast intrinsic for `I64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_0000000000000003: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_cast_0000000000000004: function(arg0) {
            // Cast intrinsic for `U64 -> Externref`.
            const ret = BigInt.asUintN(64, arg0);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./em_wasm_bg.js": import0,
    };
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('em_wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
