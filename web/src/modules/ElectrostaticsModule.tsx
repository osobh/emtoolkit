import { useState, useMemo, useCallback } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

interface Charge { x: number; y: number; q: number; }

export function ElectrostaticsModule() {
  const [charges, setCharges] = useState<Charge[]>([
    { x: -0.5, y: 0, q: 1e-9 },
    { x: 0.5, y: 0, q: -1e-9 },
  ]);

  const addCharge = useCallback(() => {
    setCharges(c => [...c, { x: 0, y: 0.5, q: 1e-9 }]);
  }, []);

  const removeCharge = useCallback((i: number) => {
    setCharges(c => c.filter((_, j) => j !== i));
  }, []);

  const updateCharge = useCallback((i: number, key: keyof Charge, val: number) => {
    setCharges(c => c.map((ch, j) => j === i ? { ...ch, [key]: val } : ch));
  }, []);

  const chargesJson = JSON.stringify(charges.map(c => [c.x, c.y, 0, c.q]));

  const field = useMemo(
    () => wasm.electric_field_2d(chargesJson, -2, 2, -2, 2, 40, 40),
    [chargesJson],
  );

  const fieldLines = useMemo(() => {
    const posIdx = charges.findIndex(c => c.q > 0);
    if (posIdx < 0) return null;
    return wasm.field_lines(chargesJson, posIdx, 12, 200);
  }, [chargesJson, charges]);

  return (
    <div className="module-panel">
      <h2>Electrostatic Field Visualization</h2>
      <div className="controls" style={{ flexDirection: 'column', alignItems: 'stretch' }}>
        {charges.map((c, i) => (
          <div key={i} style={{ display: 'flex', gap: 8, alignItems: 'end' }}>
            <div className="control-group"><label>x</label><input type="number" value={c.x} onChange={e => updateCharge(i, 'x', +e.target.value)} step={0.1} /></div>
            <div className="control-group"><label>y</label><input type="number" value={c.y} onChange={e => updateCharge(i, 'y', +e.target.value)} step={0.1} /></div>
            <div className="control-group"><label>Q (nC)</label><input type="number" value={c.q * 1e9} onChange={e => updateCharge(i, 'q', +e.target.value * 1e-9)} step={0.5} /></div>
            <button onClick={() => removeCharge(i)} style={{ padding: '6px 10px', cursor: 'pointer' }}>✕</button>
          </div>
        ))}
        <button onClick={addCharge} style={{ width: 'fit-content', padding: '6px 16px', cursor: 'pointer' }}>+ Add Charge</button>
      </div>

      <Plot
        data={[
          {
            z: field.magnitude, x: field.x_coords, y: field.y_coords,
            type: 'heatmap', colorscale: 'Hot', reversescale: true,
            showscale: true, zmax: field.max_mag * 0.3,
            colorbar: { title: { text: '|E| (V/m)' } },
          } as any,
          ...(fieldLines ? fieldLines.lines.map((line: any, i: number) => ({
            x: line.x, y: line.y, mode: 'lines',
            line: { color: '#4ecdc4', width: 1 },
            showlegend: i === 0, name: 'Field lines',
          })) : []),
          {
            x: charges.map(c => c.x), y: charges.map(c => c.y),
            mode: 'markers+text',
            marker: { size: 14, color: charges.map(c => c.q > 0 ? '#e63946' : '#2196f3') },
            text: charges.map(c => c.q > 0 ? '+' : '−'),
            textposition: 'middle center',
            textfont: { size: 16, color: 'white' },
            showlegend: false,
          },
        ]}
        layout={{
          width: 600, height: 600,
          xaxis: { range: [-2, 2], scaleanchor: 'y', title: { text: 'x (m)' } },
          yaxis: { range: [-2, 2], title: { text: 'y (m)' } },
          margin: { t: 20, b: 50, l: 50, r: 20 },
          paper_bgcolor: 'transparent', plot_bgcolor: 'white',
        }}
        config={{ responsive: true }}
      />
    </div>
  );
}
