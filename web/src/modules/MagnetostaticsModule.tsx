import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

type Scenario = 'wire' | 'loop' | 'helmholtz' | 'solenoid' | 'coaxial';

export function MagnetostaticsModule() {
  const [scenario, setScenario] = useState<Scenario>('helmholtz');
  const [current, setCurrent] = useState(1.0);
  const [radius, setRadius] = useState(0.1);
  const [turns, setTurns] = useState(100);

  const helmholtz = useMemo(
    () => scenario === 'helmholtz' ? wasm.helmholtz_coil(radius, current, turns, -radius * 3, radius * 3, 300) : null,
    [scenario, radius, current, turns],
  );

  const loop = useMemo(
    () => scenario === 'loop' ? wasm.current_loop_on_axis(radius, current, -radius * 5, radius * 5, 300) : null,
    [scenario, radius, current],
  );

  const solenoid = useMemo(
    () => scenario === 'solenoid' ? wasm.solenoid_params(turns, 0.5, current, radius, 1.0) : null,
    [scenario, turns, current, radius],
  );

  const coaxial = useMemo(
    () => scenario === 'coaxial' ? wasm.coaxial_cable_b(0.002, 0.005, 0.006, current, 0.015, 300) : null,
    [scenario, current],
  );

  const wireField = useMemo(
    () => scenario === 'wire' ? wasm.b_field_wire_2d(current, 2.0, 100, -1, 1, -1, 1, 30, 30) : null,
    [scenario, current],
  );

  return (
    <div className="module-panel">
      <h2>Magnetostatics</h2>
      <div className="controls">
        <div className="control-group">
          <label>Configuration</label>
          <select value={scenario} onChange={e => setScenario(e.target.value as Scenario)}>
            <option value="helmholtz">Helmholtz Coil</option>
            <option value="loop">Current Loop (On-Axis)</option>
            <option value="wire">Infinite Wire (2D Field)</option>
            <option value="solenoid">Solenoid Parameters</option>
            <option value="coaxial">Coaxial Cable B-field</option>
          </select>
        </div>
        <div className="control-group"><label>Current (A)</label><input type="number" value={current} onChange={e => setCurrent(+e.target.value)} step={0.5} min={0.1} /></div>
        {(scenario === 'helmholtz' || scenario === 'loop' || scenario === 'solenoid') && (
          <div className="control-group"><label>Radius (m)</label><input type="number" value={radius} onChange={e => setRadius(+e.target.value)} step={0.01} min={0.001} /></div>
        )}
        {(scenario === 'helmholtz' || scenario === 'solenoid') && (
          <div className="control-group"><label>Turns</label><input type="number" value={turns} onChange={e => setTurns(+e.target.value)} step={10} min={1} /></div>
        )}
      </div>

      {(scenario === 'helmholtz' || scenario === 'loop') && (
        <Plot
          data={[{
            x: (helmholtz ?? loop)!.positions,
            y: (helmholtz ?? loop)!.b_field,
            mode: 'lines', line: { color: '#9b59b6', width: 2 },
            name: 'Bz(z)',
          }]}
          layout={{
            width: 800, height: 400,
            xaxis: { title: { text: 'z (m)' } },
            yaxis: { title: { text: 'B (T)' } },
            margin: { t: 20, b: 50, l: 70, r: 20 },
            paper_bgcolor: 'transparent', plot_bgcolor: 'white',
          }}
          config={{ responsive: true }}
        />
      )}

      {scenario === 'coaxial' && coaxial && (
        <Plot
          data={[{
            x: coaxial.radii, y: coaxial.b_phi, mode: 'lines',
            line: { color: '#e67e22', width: 2 }, name: 'B_φ(r)',
          }]}
          layout={{
            width: 800, height: 400,
            xaxis: { title: { text: 'Radius (m)' } },
            yaxis: { title: { text: 'Bφ (T)' } },
            margin: { t: 20, b: 50, l: 70, r: 20 },
            paper_bgcolor: 'transparent', plot_bgcolor: 'white',
          }}
          config={{ responsive: true }}
        />
      )}

      {scenario === 'solenoid' && solenoid && (
        <div className="result-box">
          <h3 style={{ marginTop: 0 }}>Solenoid Parameters</h3>
          <div className="result-grid">
            <div className="result-item"><span className="result-label">B interior</span><span className="result-value">{solenoid.b_interior.toExponential(3)} T</span></div>
            <div className="result-item"><span className="result-label">Inductance</span><span className="result-value">{(solenoid.inductance * 1e6).toFixed(2)} μH</span></div>
            <div className="result-item"><span className="result-label">Energy</span><span className="result-value">{solenoid.energy.toExponential(3)} J</span></div>
          </div>
        </div>
      )}

      {scenario === 'wire' && wireField && (
        <Plot
          data={[{
            x: wireField.x_coords, y: wireField.y_coords,
            z: wireField.magnitude, type: 'heatmap', colorscale: 'Viridis',
            colorbar: { title: { text: '|B| (T)' } },
          } as any]}
          layout={{
            width: 600, height: 600,
            xaxis: { scaleanchor: 'y', title: { text: 'x (m)' } },
            yaxis: { title: { text: 'y (m)' } },
            margin: { t: 20, b: 50, l: 50, r: 20 },
            paper_bgcolor: 'transparent', plot_bgcolor: 'white',
          }}
          config={{ responsive: true }}
        />
      )}
    </div>
  );
}
