import { useState, useEffect } from 'react';
import { ensureInit } from './wasm';
import { ModuleSelector } from './components/ModuleSelector';
import { WaveformModule } from './modules/WaveformModule';
import { TravelingWaveModule } from './modules/TravelingWaveModule';
import { PhaseComparisonModule } from './modules/PhaseComparisonModule';
import { SmithChartModule } from './modules/SmithChartModule';
import { StandingWaveModule } from './modules/StandingWaveModule';
import { MatchingModule } from './modules/MatchingModule';
import { VectorFieldModule } from './modules/VectorFieldModule';
import { ElectrostaticsModule } from './modules/ElectrostaticsModule';
import { MagnetostaticsModule } from './modules/MagnetostaticsModule';
import { FaradayModule } from './modules/FaradayModule';
import { DisplacementCurrentModule } from './modules/DisplacementCurrentModule';
import { MediumPropertiesModule } from './modules/MediumPropertiesModule';
import { PolarizationModule } from './modules/PolarizationModule';
import { FresnelModule } from './modules/FresnelModule';
import { DipoleModule } from './modules/DipoleModule';
import { ArrayFactorModule } from './modules/ArrayFactorModule';
import { LinkBudgetModule } from './modules/LinkBudgetModule';
import './App.css';

const MODULES = [
  { id: 'waveform', name: '1.1 Sinusoidal Waveforms', chapter: 1 },
  { id: 'traveling', name: '1.2 Traveling Waves', chapter: 1 },
  { id: 'phase', name: '1.3 Phase Comparison', chapter: 1 },
  { id: 'smith', name: '2.1 Smith Chart', chapter: 2 },
  { id: 'standing', name: '2.2 Standing Waves', chapter: 2 },
  { id: 'matching', name: '2.3 Impedance Matching', chapter: 2 },
  { id: 'vectors', name: '3.1 Scalar & Vector Fields', chapter: 3 },
  { id: 'electrostatics', name: '4.1 Electrostatic Fields', chapter: 4 },
  { id: 'magnetostatics', name: '5.1 Magnetostatics', chapter: 5 },
  { id: 'faraday', name: '6.1 Faraday & Time-Varying', chapter: 6 },
  { id: 'displacement', name: '6.2 Displacement Current', chapter: 6 },
  { id: 'medium', name: '7.1 Medium Properties', chapter: 7 },
  { id: 'polarization', name: '7.3 Polarization', chapter: 7 },
  { id: 'fresnel', name: '7.4 Fresnel Coefficients', chapter: 7 },
  { id: 'dipole', name: '8.1 Dipole Antennas', chapter: 8 },
  { id: 'array', name: '8.2 Antenna Arrays', chapter: 8 },
  { id: 'link', name: '8.3 Friis Link Budget', chapter: 8 },
];

function App() {
  const [ready, setReady] = useState(false);
  const [activeModule, setActiveModule] = useState('smith');

  useEffect(() => {
    ensureInit().then(() => setReady(true));
  }, []);

  if (!ready) {
    return <div className="loading">Loading EM Toolkit WASM engine...</div>;
  }

  const renderModule = () => {
    switch (activeModule) {
      case 'waveform': return <WaveformModule />;
      case 'traveling': return <TravelingWaveModule />;
      case 'phase': return <PhaseComparisonModule />;
      case 'smith': return <SmithChartModule />;
      case 'standing': return <StandingWaveModule />;
      case 'matching': return <MatchingModule />;
      case 'vectors': return <VectorFieldModule />;
      case 'electrostatics': return <ElectrostaticsModule />;
      case 'magnetostatics': return <MagnetostaticsModule />;
      case 'faraday': return <FaradayModule />;
      case 'displacement': return <DisplacementCurrentModule />;
      case 'medium': return <MediumPropertiesModule />;
      case 'polarization': return <PolarizationModule />;
      case 'fresnel': return <FresnelModule />;
      case 'dipole': return <DipoleModule />;
      case 'array': return <ArrayFactorModule />;
      case 'link': return <LinkBudgetModule />;
      default: return <div>Select a module</div>;
    }
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>⚡ EM Toolkit</h1>
        <span className="subtitle">Interactive Electromagnetics Education Platform</span>
        <span className="module-count">{MODULES.length} modules</span>
      </header>
      <div className="app-body">
        <ModuleSelector
          modules={MODULES}
          active={activeModule}
          onSelect={setActiveModule}
        />
        <main className="module-content">
          {renderModule()}
        </main>
      </div>
    </div>
  );
}

export default App;
