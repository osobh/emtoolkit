import { useState, useEffect } from 'react';
import { ensureInit } from './wasm';
import { ModuleSelector } from './components/ModuleSelector';
import { ConstantsModule } from './modules/ConstantsModule';
import { CoordinatesModule } from './modules/CoordinatesModule';
import { WaveformModule } from './modules/WaveformModule';
import { TravelingWaveModule } from './modules/TravelingWaveModule';
import { PhaseComparisonModule } from './modules/PhaseComparisonModule';
import { SmithChartModule } from './modules/SmithChartModule';
import { StandingWaveModule } from './modules/StandingWaveModule';
import { MatchingModule } from './modules/MatchingModule';
import { ImpedanceModule } from './modules/ImpedanceModule';
import { CoaxialLineModule } from './modules/CoaxialLineModule';
import { QuarterWaveModule } from './modules/QuarterWaveModule';
import { VectorFieldModule } from './modules/VectorFieldModule';
import { GradientModule } from './modules/GradientModule';
import { VectorOpsModule } from './modules/VectorOpsModule';
import { ElectrostaticsModule } from './modules/ElectrostaticsModule';
import { MethodOfImagesModule } from './modules/MethodOfImagesModule';
import { CapacitanceModule } from './modules/CapacitanceModule';
import { MagnetostaticsModule } from './modules/MagnetostaticsModule';
import { WireForceModule } from './modules/WireForceModule';
import { SolenoidModule } from './modules/SolenoidModule';
import { HelmholtzModule } from './modules/HelmholtzModule';
import { CoaxBFieldModule } from './modules/CoaxBFieldModule';
import { FaradayModule } from './modules/FaradayModule';
import { DisplacementCurrentModule } from './modules/DisplacementCurrentModule';
import { ChargeRelaxationModule } from './modules/ChargeRelaxationModule';
import { GeneratorModule } from './modules/GeneratorModule';
import { TransformerModule } from './modules/TransformerModule';
import { MediumPropertiesModule } from './modules/MediumPropertiesModule';
import { SkinDepthModule } from './modules/SkinDepthModule';
import { PolarizationModule } from './modules/PolarizationModule';
import { FresnelModule } from './modules/FresnelModule';
import { WaveguideModule } from './modules/WaveguideModule';
import { DipoleModule } from './modules/DipoleModule';
import { ArrayFactorModule } from './modules/ArrayFactorModule';
import { LinkBudgetModule } from './modules/LinkBudgetModule';
import './App.css';

const MODULES = [
  { id: 'constants', name: '0.1 Constants & Units', chapter: 0 },
  { id: 'coordinates', name: '0.2 Coordinate Converter', chapter: 0 },
  { id: 'waveform', name: '1.1 Sinusoidal Waveforms', chapter: 1 },
  { id: 'traveling', name: '1.2 Traveling Waves', chapter: 1 },
  { id: 'phase', name: '1.3 Phase Comparison', chapter: 1 },
  { id: 'smith', name: '2.1 Smith Chart', chapter: 2 },
  { id: 'standing', name: '2.2 Standing Waves', chapter: 2 },
  { id: 'matching', name: '2.3 Impedance Matching', chapter: 2 },
  { id: 'impedance', name: '2.4 Γ & Input Impedance', chapter: 2 },
  { id: 'coaxial', name: '2.5 Coaxial Line Params', chapter: 2 },
  { id: 'quarterwave', name: '2.6 λ/4 Transformer', chapter: 2 },
  { id: 'vectorops', name: '3.1 Vector Operations', chapter: 3 },
  { id: 'vectors', name: '3.2 Scalar & Vector Fields', chapter: 3 },
  { id: 'gradient', name: '3.3 Gradient Calculator', chapter: 3 },
  { id: 'electrostatics', name: '4.1 Electrostatic Fields', chapter: 4 },
  { id: 'images', name: '4.2 Method of Images', chapter: 4 },
  { id: 'capacitance', name: '4.3 Capacitance Calculator', chapter: 4 },
  { id: 'magnetostatics', name: '5.1 Magnetostatics', chapter: 5 },
  { id: 'wireforce', name: '5.2 Wire Forces', chapter: 5 },
  { id: 'solenoid', name: '5.3 Solenoid & Inductor', chapter: 5 },
  { id: 'helmholtz', name: '5.4 Helmholtz Coil', chapter: 5 },
  { id: 'coaxbfield', name: '5.5 Coax Cable B-Field', chapter: 5 },
  { id: 'faraday', name: '6.1 Faraday & Time-Varying', chapter: 6 },
  { id: 'displacement', name: '6.2 Displacement Current', chapter: 6 },
  { id: 'relaxation', name: '6.3 Charge Relaxation', chapter: 6 },
  { id: 'generator', name: '6.4 AC Generator', chapter: 6 },
  { id: 'transformer', name: '6.5 Ideal Transformer', chapter: 6 },
  { id: 'medium', name: '7.1 Medium Properties', chapter: 7 },
  { id: 'skindepth', name: '7.2 Skin Depth & Attenuation', chapter: 7 },
  { id: 'polarization', name: '7.3 Polarization', chapter: 7 },
  { id: 'fresnel', name: '7.4 Fresnel Coefficients', chapter: 7 },
  { id: 'waveguide', name: '7.5 Rectangular Waveguide', chapter: 7 },
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

  if (!ready) return <div className="loading">Loading EM Toolkit WASM engine...</div>;

  const renderModule = () => {
    switch (activeModule) {
      case 'constants': return <ConstantsModule />;
      case 'coordinates': return <CoordinatesModule />;
      case 'waveform': return <WaveformModule />;
      case 'traveling': return <TravelingWaveModule />;
      case 'phase': return <PhaseComparisonModule />;
      case 'smith': return <SmithChartModule />;
      case 'standing': return <StandingWaveModule />;
      case 'matching': return <MatchingModule />;
      case 'impedance': return <ImpedanceModule />;
      case 'coaxial': return <CoaxialLineModule />;
      case 'quarterwave': return <QuarterWaveModule />;
      case 'vectorops': return <VectorOpsModule />;
      case 'vectors': return <VectorFieldModule />;
      case 'gradient': return <GradientModule />;
      case 'electrostatics': return <ElectrostaticsModule />;
      case 'images': return <MethodOfImagesModule />;
      case 'capacitance': return <CapacitanceModule />;
      case 'magnetostatics': return <MagnetostaticsModule />;
      case 'wireforce': return <WireForceModule />;
      case 'solenoid': return <SolenoidModule />;
      case 'helmholtz': return <HelmholtzModule />;
      case 'coaxbfield': return <CoaxBFieldModule />;
      case 'faraday': return <FaradayModule />;
      case 'displacement': return <DisplacementCurrentModule />;
      case 'relaxation': return <ChargeRelaxationModule />;
      case 'generator': return <GeneratorModule />;
      case 'transformer': return <TransformerModule />;
      case 'medium': return <MediumPropertiesModule />;
      case 'skindepth': return <SkinDepthModule />;
      case 'polarization': return <PolarizationModule />;
      case 'fresnel': return <FresnelModule />;
      case 'waveguide': return <WaveguideModule />;
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
        <ModuleSelector modules={MODULES} active={activeModule} onSelect={setActiveModule} />
        <main className="module-content">{renderModule()}</main>
      </div>
    </div>
  );
}

export default App;
