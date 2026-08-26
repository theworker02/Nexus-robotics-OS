'use client';

import { useState } from 'react';

const levels = [
  ['L0', 'Software Verified', 'Runtime, schema, state-machine, and contract checks.'],
  ['L1', 'Virtual Hardware Verified', 'Adapter-facing servos, sensors, battery, and injected faults.'],
  ['L2', 'Physics Verified', 'Recorded Gazebo Harmonic execution required.'],
  ['L3', 'Adversarially Verified', 'L2 plus seeded world variation and fault evidence required.'],
  ['L4', 'HIL Verified', 'A real controller, sensor, or robot component is required.'],
  ['L5', 'Robot Verified', 'A physical robot demonstration is required.'],
] as const;

export default function ProvingGroundPage() {
  const [selected, setSelected] = useState(0);
  const [level, title, description] = levels[selected];
  return <main>
    <nav className="nav"><a className="brand" href="/"><span className="mark">N</span> NEXUS</a><div className="links"><a href="/">Platform</a><a href="/sensehopping">SenseHopping</a><a href="/structurescan">StructureScan</a><a href="/active-learning">Active Learning</a></div></nav>
    <section className="hero"><p className="eyebrow">NEXUS PROVING GROUND</p><h1>Evidence before<br />actuators.</h1><p className="lede">A local-first certification system for proving exactly what a Nexus capability has passed—and showing what has not yet been performed.</p></section>
    <section className="feature-grid">
      <div className="model-card"><p className="panel-label">VALIDATION LADDER</p><h2>{level} — {title}</h2><p>{description}</p><div className="capability-tabs" role="tablist">{levels.map(([id], index) => <button key={id} role="tab" aria-selected={selected === index} className={selected === index ? 'active' : ''} onClick={() => setSelected(index)}>{id}</button>)}</div><div className="attempt success"><span>FOUNDATION</span> L0 + L1 evidence recorded</div><div className="attempt success"><span>PHYSICS</span> move_forward L2 verified: 0.650 m</div><div className="attempt partial"><span>ADVERSARY</span> L3 requires randomized physics faults</div><div className="attempt fail"><span>HARDWARE</span> NOT YET PERFORMED</div></div>
      <div className="feature-copy"><p className="panel-label">WORLDFORGE</p><h2>Reproduce the failure.</h2><p>Every trial is seeded. Door geometry, lighting, friction, sensor noise, network latency, and robot start pose can be replayed from the certification report.</p><ul><li>VirtualRobotBus servo and sensor faults</li><li>Live ROS 2 → Gazebo command transport</li><li>Bridged NXR-2 odometry as a physics assertion</li><li>Plain Markdown reports with safety evidence</li></ul><p className="panel-label">HARD RULE</p><p>No capability is production-ready merely because it compiles.</p></div>
    </section>
    <footer><span className="brand"><span className="mark">N</span> NEXUS</span><p>Repeatable evidence, conservative claims.</p></footer>
  </main>;
}
