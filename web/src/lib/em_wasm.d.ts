/* tslint:disable */
/* eslint-disable */

export function ac_generator(turns: number, b_field: number, area: number, rpm: number): any;

export function antenna_array(num_elements: number, spacing: number, beta_deg: number, num_points: number): any;

export function attenuation_profile(epsilon_r: number, conductivity: number, frequency: number, e0: number, z_max: number, num_points: number): any;

export function b_field_infinite_wire(current: number, rho: number): number;

export function b_field_wire_2d(current: number, half_length: number, num_segments: number, x_min: number, x_max: number, y_min: number, y_max: number, nx: number, ny: number): any;

export function capacitance_calc(geometry: string, params_json: string): any;

export function cartesian_to_spherical(x: number, y: number, z: number): any;

export function charge_above_plane(charge: number, height: number, x_min: number, x_max: number, nx: number): any;

export function charge_relaxation(rho_0: number, epsilon_r: number, conductivity: number, radius: number, t_end: number, num_points: number): any;

export function coaxial_cable_b(inner_r: number, outer_inner_r: number, outer_outer_r: number, current: number, r_max: number, num_points: number): any;

export function coaxial_line_params(inner_radius: number, outer_radius: number, epsilon_r: number, frequency: number): any;

export function current_loop_on_axis(radius: number, current: number, z_min: number, z_max: number, num_points: number): any;

export function db_to_power(db: number): number;

export function displacement_current_sim(area: number, separation: number, epsilon_r: number, v_peak: number, omega: number, t_end: number, num_points: number): any;

export function electric_field_2d(charges_json: string, x_min: number, x_max: number, y_min: number, y_max: number, nx: number, ny: number): any;

export function field_lines(charges_json: string, source_idx: number, num_lines: number, num_steps: number): any;

export function fresnel_normal(eta1: number, eta2: number): any;

export function fresnel_oblique(er1: number, er2: number, theta_i_deg: number): any;

export function fresnel_vs_angle(er1: number, er2: number, num_points: number): any;

export function friis_link(p_tx_w: number, g_tx_db: number, g_rx_db: number, frequency: number, distance: number): any;

export function gauss_line_charge(rho_l: number, epsilon_r: number, rho_min: number, rho_max: number, num_points: number): any;

export function gauss_sphere_profile(total_charge: number, radius: number, epsilon_r: number, r_max: number, num_points: number): any;

export function get_constants(): any;

export function half_wave_dipole(frequency: number, current: number, num_points: number): any;

export function helmholtz_coil(radius: number, current: number, turns: number, z_min: number, z_max: number, num_points: number): any;

export function hertzian_dipole(length: number, current: number, frequency: number, num_points: number): any;

export function inductance_calc(geometry: string, params_json: string): any;

/**
 * Initialize the WASM module (call once from JS).
 */
export function init(): void;

export function input_impedance_lossless(zl_re: number, zl_im: number, z0: number, beta_l: number): any;

export function link_vs_distance(p_tx_w: number, g_tx_db: number, g_rx_db: number, frequency: number, r_min: number, r_max: number, num_points: number): any;

export function medium_properties(epsilon_r: number, mu_r: number, conductivity: number, frequency: number): any;

export function numerical_gradient(preset: string, x: number, y: number, z: number): any;

export function parallel_wire_force(i1: number, i2: number, separation: number, length: number): any;

export function phase_comparison(a1: number, f1: number, p1: number, a2: number, f2: number, p2: number, t_end: number, num_points: number): any;

export function polarization_state(ax: number, ay: number, delta_deg: number, num_trace: number): any;

export function power_calculations(e0: number, eta: number, gamma_mag: number): any;

export function power_to_db(linear: number): number;

export function quarter_wave_match(z_load: number, z_line: number, frequency: number): any;

export function reflection_coefficient(zl_re: number, zl_im: number, z0: number): any;

export function rl_step(voltage: number, resistance: number, inductance_val: number, t_end: number, num_points: number): any;

export function scalar_field_2d(preset: string, x_min: number, x_max: number, y_min: number, y_max: number, nx: number, ny: number): any;

export function single_stub_match(zl_re: number, zl_im: number, z0: number, use_short: boolean): any;

export function sinusoidal_emf(b_peak: number, area: number, omega: number, t_end: number, num_points: number): any;

export function sinusoidal_wave(amplitude: number, frequency: number, phase_rad: number, damping: number, t_end: number, num_points: number): any;

export function skin_depth_vs_frequency(epsilon_r: number, conductivity: number, f_min: number, f_max: number, num_points: number): any;

export function smith_chart_point(zl_re: number, zl_im: number): any;

export function smith_chart_swr_circle(gamma_mag: number, num_points: number): any;

export function smith_chart_trace(zl_re: number, zl_im: number, electrical_length: number, num_points: number): any;

export function solenoid_params(turns: number, length: number, current: number, radius: number, mu_r: number): any;

export function spherical_to_cartesian(r: number, theta: number, phi: number): any;

export function standing_wave_pattern(z0: number, zl_re: number, zl_im: number, frequency: number, length: number, num_points: number): any;

export function transformer(n_primary: number, n_secondary: number, v_primary: number, i_primary: number): any;

export function traveling_wave_snapshot(amplitude: number, frequency: number, t: number, x_max: number, num_points: number): any;

export function unit_conversions(value: number, from_unit: string): any;

export function vector_add(ax: number, ay: number, az: number, bx: number, by: number, bz: number): any;

export function vector_cross(ax: number, ay: number, az: number, bx: number, by: number, bz: number): any;

export function vector_field_2d(preset: string, x_min: number, x_max: number, y_min: number, y_max: number, nx: number, ny: number): any;

export function vector_project(vx: number, vy: number, vz: number, rx: number, ry: number, rz: number): any;

/**
 * Get the toolkit version.
 */
export function version(): string;

export function waveguide_rect(a_mm: number, b_mm: number, epsilon_r: number, frequency: number): any;

export function wavelength(frequency_hz: number): number;

export function wavenumber(frequency_hz: number): number;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly numerical_gradient: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly scalar_field_2d: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => any;
    readonly vector_add: (a: number, b: number, c: number, d: number, e: number, f: number) => any;
    readonly vector_cross: (a: number, b: number, c: number, d: number, e: number, f: number) => any;
    readonly vector_field_2d: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => any;
    readonly vector_project: (a: number, b: number, c: number, d: number, e: number, f: number) => any;
    readonly attenuation_profile: (a: number, b: number, c: number, d: number, e: number, f: number) => any;
    readonly fresnel_normal: (a: number, b: number) => any;
    readonly fresnel_oblique: (a: number, b: number, c: number) => any;
    readonly fresnel_vs_angle: (a: number, b: number, c: number) => any;
    readonly medium_properties: (a: number, b: number, c: number, d: number) => any;
    readonly polarization_state: (a: number, b: number, c: number, d: number) => any;
    readonly skin_depth_vs_frequency: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly waveguide_rect: (a: number, b: number, c: number, d: number) => any;
    readonly b_field_wire_2d: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => any;
    readonly coaxial_cable_b: (a: number, b: number, c: number, d: number, e: number, f: number) => any;
    readonly current_loop_on_axis: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly helmholtz_coil: (a: number, b: number, c: number, d: number, e: number, f: number) => any;
    readonly inductance_calc: (a: number, b: number, c: number, d: number) => any;
    readonly parallel_wire_force: (a: number, b: number, c: number, d: number) => any;
    readonly rl_step: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly solenoid_params: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly b_field_infinite_wire: (a: number, b: number) => number;
    readonly capacitance_calc: (a: number, b: number, c: number, d: number) => any;
    readonly charge_above_plane: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly electric_field_2d: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => any;
    readonly field_lines: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly gauss_line_charge: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly gauss_sphere_profile: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly coaxial_line_params: (a: number, b: number, c: number, d: number) => any;
    readonly quarter_wave_match: (a: number, b: number, c: number) => any;
    readonly single_stub_match: (a: number, b: number, c: number, d: number) => any;
    readonly smith_chart_point: (a: number, b: number) => any;
    readonly smith_chart_swr_circle: (a: number, b: number) => any;
    readonly smith_chart_trace: (a: number, b: number, c: number, d: number) => any;
    readonly standing_wave_pattern: (a: number, b: number, c: number, d: number, e: number, f: number) => any;
    readonly cartesian_to_spherical: (a: number, b: number, c: number) => any;
    readonly db_to_power: (a: number) => number;
    readonly get_constants: () => any;
    readonly init: () => void;
    readonly input_impedance_lossless: (a: number, b: number, c: number, d: number) => any;
    readonly power_to_db: (a: number) => number;
    readonly reflection_coefficient: (a: number, b: number, c: number) => any;
    readonly spherical_to_cartesian: (a: number, b: number, c: number) => any;
    readonly version: () => [number, number];
    readonly wavelength: (a: number) => number;
    readonly wavenumber: (a: number) => number;
    readonly phase_comparison: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => any;
    readonly power_calculations: (a: number, b: number, c: number) => any;
    readonly sinusoidal_wave: (a: number, b: number, c: number, d: number, e: number, f: number) => any;
    readonly traveling_wave_snapshot: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly unit_conversions: (a: number, b: number, c: number) => any;
    readonly antenna_array: (a: number, b: number, c: number, d: number) => any;
    readonly friis_link: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly half_wave_dipole: (a: number, b: number, c: number) => any;
    readonly hertzian_dipole: (a: number, b: number, c: number, d: number) => any;
    readonly link_vs_distance: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => any;
    readonly ac_generator: (a: number, b: number, c: number, d: number) => any;
    readonly charge_relaxation: (a: number, b: number, c: number, d: number, e: number, f: number) => any;
    readonly displacement_current_sim: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => any;
    readonly sinusoidal_emf: (a: number, b: number, c: number, d: number, e: number) => any;
    readonly transformer: (a: number, b: number, c: number, d: number) => any;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
