import { useState } from 'react';

type Form = 'differential' | 'integral' | 'phasor';

export function MaxwellModule() {
  const [form, setForm] = useState<Form>('differential');
  const [medium, setMedium] = useState<'general' | 'free_space' | 'source_free' | 'static'>('general');

  const equations: Record<Form, Record<string, { name: string; eq: string; meaning: string }[]>> = {
    differential: {
      general: [
        { name: "Gauss's Law (E)", eq: '∇ · D = ρ_v', meaning: 'Electric charges are sources of D (and E)' },
        { name: "Gauss's Law (B)", eq: '∇ · B = 0', meaning: 'No magnetic monopoles; B-field lines always close' },
        { name: "Faraday's Law", eq: '∇ × E = −∂B/∂t', meaning: 'Time-varying B induces circulating E' },
        { name: "Ampère-Maxwell", eq: '∇ × H = J + ∂D/∂t', meaning: 'Currents and time-varying D create circulating H' },
      ],
      free_space: [
        { name: "Gauss's Law (E)", eq: '∇ · E = ρ_v / ε₀', meaning: 'D = ε₀E in free space' },
        { name: "Gauss's Law (B)", eq: '∇ · B = 0', meaning: 'Always true' },
        { name: "Faraday's Law", eq: '∇ × E = −∂B/∂t', meaning: 'Unchanged' },
        { name: "Ampère-Maxwell", eq: '∇ × B = μ₀J + μ₀ε₀ ∂E/∂t', meaning: 'B = μ₀H in free space' },
      ],
      source_free: [
        { name: "Gauss's Law (E)", eq: '∇ · E = 0', meaning: 'No free charges (ρ_v = 0)' },
        { name: "Gauss's Law (B)", eq: '∇ · B = 0', meaning: 'Always true' },
        { name: "Faraday's Law", eq: '∇ × E = −∂B/∂t', meaning: 'Unchanged' },
        { name: "Ampère-Maxwell", eq: '∇ × H = ∂D/∂t', meaning: 'No conduction current (J = 0)' },
      ],
      static: [
        { name: "Gauss's Law (E)", eq: '∇ · D = ρ_v', meaning: 'Electrostatics' },
        { name: "Gauss's Law (B)", eq: '∇ · B = 0', meaning: 'Always true' },
        { name: 'E is irrotational', eq: '∇ × E = 0', meaning: '∂B/∂t = 0 → E = −∇V' },
        { name: "Ampère's Law", eq: '∇ × H = J', meaning: '∂D/∂t = 0 → magnetostatics' },
      ],
    },
    integral: {
      general: [
        { name: "Gauss's Law (E)", eq: '∮ D · dS = Q_enc', meaning: 'Flux of D through closed surface = enclosed charge' },
        { name: "Gauss's Law (B)", eq: '∮ B · dS = 0', meaning: 'Net magnetic flux through any closed surface is zero' },
        { name: "Faraday's Law", eq: '∮ E · dl = −d/dt ∫ B · dS', meaning: 'EMF around a loop = rate of change of magnetic flux' },
        { name: "Ampère-Maxwell", eq: '∮ H · dl = I_enc + d/dt ∫ D · dS', meaning: 'MMF = conduction + displacement current' },
      ],
      free_space: [
        { name: "Gauss's Law (E)", eq: '∮ E · dS = Q_enc / ε₀', meaning: 'Free-space version' },
        { name: "Gauss's Law (B)", eq: '∮ B · dS = 0', meaning: 'Always true' },
        { name: "Faraday's Law", eq: '∮ E · dl = −d/dt ∫ B · dS', meaning: 'Unchanged' },
        { name: "Ampère-Maxwell", eq: '∮ B · dl = μ₀I_enc + μ₀ε₀ d/dt ∫ E · dS', meaning: 'Free-space' },
      ],
      source_free: [
        { name: "Gauss's Law (E)", eq: '∮ D · dS = 0', meaning: 'No enclosed charge' },
        { name: "Gauss's Law (B)", eq: '∮ B · dS = 0', meaning: 'Always true' },
        { name: "Faraday's Law", eq: '∮ E · dl = −d/dt ∫ B · dS', meaning: 'Unchanged' },
        { name: "Ampère-Maxwell", eq: '∮ H · dl = d/dt ∫ D · dS', meaning: 'No conduction current' },
      ],
      static: [
        { name: "Gauss's Law (E)", eq: '∮ D · dS = Q_enc', meaning: 'Electrostatics' },
        { name: "Gauss's Law (B)", eq: '∮ B · dS = 0', meaning: 'Always true' },
        { name: 'Kirchhoff voltage', eq: '∮ E · dl = 0', meaning: 'E is conservative (static)' },
        { name: "Ampère's Law", eq: '∮ H · dl = I_enc', meaning: 'Magnetostatics' },
      ],
    },
    phasor: {
      general: [
        { name: "Gauss's Law (E)", eq: '∇ · D̃ = ρ̃_v', meaning: 'Phasor form, ∂/∂t → jω' },
        { name: "Gauss's Law (B)", eq: '∇ · B̃ = 0', meaning: 'Always true' },
        { name: "Faraday's Law", eq: '∇ × Ẽ = −jωB̃', meaning: '−∂/∂t → −jω' },
        { name: "Ampère-Maxwell", eq: '∇ × H̃ = J̃ + jωD̃', meaning: '∂/∂t → jω' },
      ],
      free_space: [
        { name: "Gauss's Law (E)", eq: '∇ · Ẽ = ρ̃_v / ε₀', meaning: 'Free-space phasor' },
        { name: "Gauss's Law (B)", eq: '∇ · B̃ = 0', meaning: 'Always true' },
        { name: "Faraday's Law", eq: '∇ × Ẽ = −jωμ₀H̃', meaning: 'B = μ₀H' },
        { name: "Ampère-Maxwell", eq: '∇ × H̃ = J̃ + jωε₀Ẽ', meaning: 'D = ε₀E' },
      ],
      source_free: [
        { name: "Gauss's Law (E)", eq: '∇ · Ẽ = 0', meaning: 'No sources' },
        { name: "Gauss's Law (B)", eq: '∇ · B̃ = 0', meaning: 'Always true' },
        { name: "Faraday's Law", eq: '∇ × Ẽ = −jωμH̃', meaning: 'Source-free phasor' },
        { name: "Ampère-Maxwell", eq: '∇ × H̃ = jωεẼ', meaning: 'No J, leads to wave equation' },
      ],
      static: [
        { name: "Gauss's Law (E)", eq: '∇ · D = ρ_v', meaning: 'ω = 0' },
        { name: "Gauss's Law (B)", eq: '∇ · B = 0', meaning: 'Always' },
        { name: 'E irrotational', eq: '∇ × E = 0', meaning: 'Static: no time variation' },
        { name: "Ampère's Law", eq: '∇ × H = J', meaning: 'Static: no displacement current' },
      ],
    },
  };

  const eqs = equations[form][medium];

  return (
    <div className="module">
      <h2>Maxwell's Equations Reference</h2>
      <p>All four Maxwell's equations in different forms and for different media conditions.</p>

      <div className="controls">
        <label>Form:
          <select value={form} onChange={e => setForm(e.target.value as Form)}>
            <option value="differential">Differential (point form)</option>
            <option value="integral">Integral (large-scale form)</option>
            <option value="phasor">Phasor (time-harmonic)</option>
          </select>
        </label>
        <label>Medium:
          <select value={medium} onChange={e => setMedium(e.target.value as typeof medium)}>
            <option value="general">General</option>
            <option value="free_space">Free Space</option>
            <option value="source_free">Source-Free</option>
            <option value="static">Static (DC)</option>
          </select>
        </label>
      </div>

      <div style={{ marginTop: 20 }}>
        {eqs.map((eq, i) => (
          <div key={i} style={{
            padding: '16px 20px', marginBottom: 12,
            background: i % 2 === 0 ? '#E3F2FD' : '#FFF3E0',
            borderRadius: 8, borderLeft: `4px solid ${i % 2 === 0 ? '#2196F3' : '#FF9800'}`,
          }}>
            <div style={{ fontWeight: 'bold', fontSize: 14, marginBottom: 4 }}>{eq.name}</div>
            <div style={{ fontFamily: 'serif', fontSize: 20, margin: '8px 0' }}>{eq.eq}</div>
            <div style={{ fontSize: 13, color: '#666' }}>{eq.meaning}</div>
          </div>
        ))}
      </div>

      <div className="theory" style={{ marginTop: 20 }}>
        <h3>Constitutive Relations</h3>
        <p><strong>D</strong> = εE = ε₀ε_r E</p>
        <p><strong>B</strong> = μH = μ₀μ_r H</p>
        <p><strong>J</strong> = σE (Ohm's law in point form)</p>
        <h3>Key Consequences</h3>
        <p><strong>Wave equation:</strong> ∇²E − με ∂²E/∂t² = 0 (from source-free Faraday + Ampère)</p>
        <p><strong>Continuity:</strong> ∇ · J = −∂ρ_v/∂t (charge conservation, from ∇ · of Ampère)</p>
        <p><strong>Helmholtz:</strong> ∇²Ẽ + k²Ẽ = 0 (phasor wave equation, k² = ω²με)</p>
      </div>
    </div>
  );
}
