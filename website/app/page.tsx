'use client';

import { useState } from 'react';

const capabilities = {
  Vision: ['4 RGB cameras', 'Depth camera', 'Lidar fusion'],
  Manipulation: ['Dual 7-DOF arms', 'Safety-limited grippers', 'Virtual servo diagnostics'],
  Mobility: ['Mobile base', 'Motion policy', 'Dry-run tasks'],
  Audio: ['Microphone input', 'Local speech output', 'Permissioned skills'],
  Sensors: ['IMU', 'Battery model', 'VirtualBus fault injection'],
};

export default function Home() {
  const [selected, setSelected] = useState<keyof typeof capabilities>('Vision');

  return (
    <main>
      <nav className="nav">
        <a className="brand" href="#top" aria-label="Nexus Robotics OS home"><span className="mark">N</span> NEXUS</a>
        <div className="links"><a href="#platform">Platform</a><a href="#simulator">Simulator</a><a href="/proving-ground">Proving Ground</a><a href="#integrations">Integrations</a><a href="#developers">Developers</a></div>
        <a className="nav-button" href="#developers">Start in Docker</a>
      </nav>

      <section id="top" className="hero">
        <p className="eyebrow">NEXUS ROBOTICS OS 4.2 RC.1</p>
        <h1>Make simple robots<br />capable.</h1>
        <p className="lede">A Rust-first adaptive robotics runtime for skills, configurable intelligence, learning, simulation, safety, and connected systems.</p>
        <div className="actions"><a className="primary" href="#developers">Start in Docker <span>→</span></a><a className="secondary" href="#integrations">Explore integrations</a></div>
        <div className="status"><span className="live" /> NXR-2 SIMULATION CONNECTED <i>Warehouse / Station B</i></div>
      </section>

      <section id="platform" className="machine-section">
        <div className="section-heading"><p className="eyebrow">REFERENCE ROBOT</p><h2>A capability view, not a vendor lock-in.</h2><p>NXR-2 exercises the same operating model used for simulation, adapters, and custom hardware.</p></div>
        <div className="machine-grid">
          <div className="robot-stage" aria-label="Stylized NXR-2 capability diagram"><div className="halo" /><div className="robot"><div className="head" /><div className="torso" /><div className="arm left" /><div className="arm right" /><div className="base" /></div><span className="tag vision">Vision</span><span className="tag arms">Manipulation</span><span className="tag base-tag">Mobility</span></div>
          <div className="capability-panel"><p className="panel-label">NXR-2 CAPABILITIES</p><div className="capability-tabs" role="tablist">{Object.keys(capabilities).map((name) => <button key={name} role="tab" aria-selected={selected === name} className={selected === name ? 'active' : ''} onClick={() => setSelected(name as keyof typeof capabilities)}>{name}</button>)}</div><div className="capability-copy" role="tabpanel"><h3>{selected}</h3><ul>{capabilities[selected].map((item) => <li key={item}>{item}</li>)}</ul><p>Reported through NCM 2.0 with source provenance and compatibility checks.</p></div></div>
        </div>
      </section>

      <section id="simulator" className="dark-band"><div><p className="eyebrow blue">NO ROBOT REQUIRED</p><h2>Start with evidence,<br />not assumptions.</h2><p>Run deterministic warehouse tasks through skills, safety, telemetry, replay, and a VirtualBus before connecting hardware.</p></div><pre><code>$ nexus prove skill fetch-object --trials 100{`\n`}L0 / L1 EVIDENCE RECORDED{`\n`}L2–L5: NOT RUN{`\n`}Replay evidence: AVAILABLE</code></pre></section>

      <section id="integrations" className="integrations"><p className="eyebrow">CAPABILITY LAYER</p><h2>Works beside the stack you already have.</h2><div className="integration-grid"><article><b>ROS 2</b><p>Capability mapper for common topics, actions, services, and sensor interfaces.</p><span>Contract surface; live graph transport unvalidated</span></article><article><b>LeRobot</b><p>Loss-aware episode bridge for observations, actions, timestamps, and metadata.</p><span>Data bridge available</span></article><article><b>Nori</b><p>Community-built Nori-Lab and MotorLab compatibility surface with a simulated profile.</p><span>Community integration</span></article><article><b>Custom hardware</b><p>Integration SDK, VirtualBus, and a capability-first adapter contract.</p><span>SDK available</span></article></div></section>

      <section id="developers" className="developers"><div><p className="eyebrow">DEVELOPER EXPERIENCE</p><h2>One interface from simulation to hardware.</h2><p>Native, adapter, and compatibility deployment modes keep the capability layer stable while lower-level systems evolve.</p></div><div className="code-card"><div><span>01</span> Discover capabilities</div><div><span>02</span> Validate a dry run</div><div><span>03</span> Execute through safety</div><div><span>04</span> Inspect replay evidence</div><code>nexus task run fetch-object --dry-run</code></div></section>

      <footer id="roadmap"><span className="brand"><span className="mark">N</span> NEXUS</span><p>Open robotics interoperability, built local-first.</p><a href="https://github.com/magnexis/nexus-robotics">GitHub ↗</a></footer>
    </main>
  );
}
