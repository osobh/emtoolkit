import { useState, useMemo } from 'react';
import Plot from 'react-plotly.js';
import { wasm } from '../wasm';

const SCALAR_PRESETS = ['gaussian', 'saddle', 'dipole_potential', 'ridge', 'sine_product', 'cone'];
const VECTOR_PRESETS = ['radial', 'vortex', 'dipole', 'uniform_shear', 'source_sink', 'saddle'];

type FieldType = 'scalar' | 'vector';

export function VectorFieldModule() {
  const [fieldType, setFieldType] = useState<FieldType>('scalar');
  const [scalarPreset, setScalarPreset] = useState('gaussian');
  const [vectorPreset, setVectorPreset] = useState('vortex');

  const scalarData = useMemo(
    () => fieldType === 'scalar' ? wasm.scalar_field_2d(scalarPreset, -3, 3, -3, 3, 60, 60) : null,
    [fieldType, scalarPreset],
  );

  const vectorData = useMemo(
    () => fieldType === 'vector' ? wasm.vector_field_2d(vectorPreset, -3, 3, -3, 3, 20, 20) : null,
    [fieldType, vectorPreset],
  );

  return (
    <div className="module-panel">
      <h2>Scalar &amp; Vector Fields</h2>
      <div className="controls">
        <div className="control-group">
          <label>Field Type</label>
          <select value={fieldType} onChange={e => setFieldType(e.target.value as FieldType)}>
            <option value="scalar">Scalar Field</option>
            <option value="vector">Vector Field</option>
          </select>
        </div>
        {fieldType === 'scalar' && (
          <div className="control-group">
            <label>Preset</label>
            <select value={scalarPreset} onChange={e => setScalarPreset(e.target.value)}>
              {SCALAR_PRESETS.map(p => <option key={p} value={p}>{p}</option>)}
            </select>
          </div>
        )}
        {fieldType === 'vector' && (
          <div className="control-group">
            <label>Preset</label>
            <select value={vectorPreset} onChange={e => setVectorPreset(e.target.value)}>
              {VECTOR_PRESETS.map(p => <option key={p} value={p}>{p}</option>)}
            </select>
          </div>
        )}
      </div>

      {fieldType === 'scalar' && scalarData && (
        <Plot
          data={[{
            z: scalarData.values, x: scalarData.x_coords, y: scalarData.y_coords,
            type: 'contour', colorscale: 'RdBu', reversescale: false,
            contours: { coloring: 'heatmap' },
            colorbar: { title: { text: 'f(x,y)' } },
          } as any]}
          layout={{
            width: 600, height: 600,
            xaxis: { scaleanchor: 'y', title: { text: 'x' } },
            yaxis: { title: { text: 'y' } },
            margin: { t: 20, b: 50, l: 50, r: 20 },
            paper_bgcolor: 'transparent', plot_bgcolor: 'white',
          }}
          config={{ responsive: true }}
        />
      )}

      {fieldType === 'vector' && vectorData && (
        <Plot
          data={[{
            x: vectorData.x_flat, y: vectorData.y_flat,
            mode: 'markers',
            marker: {
              size: 8,
              color: vectorData.mag_flat,
              colorscale: 'Viridis',
              showscale: true,
              colorbar: { title: { text: '|F|' } },
            },
          } as any]}
          layout={{
            width: 600, height: 600,
            xaxis: { scaleanchor: 'y', title: { text: 'x' }, range: [-3, 3] },
            yaxis: { title: { text: 'y' }, range: [-3, 3] },
            margin: { t: 20, b: 50, l: 50, r: 20 },
            annotations: vectorData.x_flat.map((x: number, i: number) => ({
              x, y: vectorData.y_flat[i],
              ax: x + vectorData.u_flat[i] * 0.15,
              ay: vectorData.y_flat[i] + vectorData.v_flat[i] * 0.15,
              xref: 'x', yref: 'y', axref: 'x', ayref: 'y',
              showarrow: true, arrowhead: 2, arrowsize: 1, arrowwidth: 1.5, arrowcolor: '#333',
            })),
            paper_bgcolor: 'transparent', plot_bgcolor: 'white',
          }}
          config={{ responsive: true }}
        />
      )}
    </div>
  );
}
