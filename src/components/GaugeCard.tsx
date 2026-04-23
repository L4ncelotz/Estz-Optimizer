import React from 'react';

interface GaugeCardProps {
  title: string;
  valuePercent: number;
  color: string;
  centerText: string;
  subText?: React.ReactNode;
}

export function GaugeCard({ title, valuePercent, color, centerText, subText }: GaugeCardProps) {
  const dashArray = `${Math.max(0, Math.min(100, valuePercent)) * 3.267} 326.7`;

  return (
    <div className="card" style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 12, padding: 24 }}>
      <div style={{ position: 'relative', width: 120, height: 120 }}>
        <svg viewBox="0 0 120 120" style={{ transform: 'rotate(-90deg)' }}>
          <circle cx="60" cy="60" r="52" fill="none" stroke="var(--bg-tertiary)" strokeWidth="8" />
          <circle cx="60" cy="60" r="52" fill="none" stroke={color} strokeWidth="8"
            strokeDasharray={dashArray}
            strokeLinecap="round" style={{ transition: 'stroke-dasharray 0.5s ease' }} />
        </svg>
        <div style={{ position: 'absolute', top: '50%', left: '50%', transform: 'translate(-50%, -50%)', textAlign: 'center' }}>
          <div style={{ fontSize: 28, fontWeight: 700, color }}>{centerText}</div>
        </div>
      </div>
      <div className="gauge-label">{title}</div>
      {subText && <div style={{ fontSize: 12, color: 'var(--text-tertiary)' }}>{subText}</div>}
    </div>
  );
}
