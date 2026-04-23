import React from 'react';

interface StatCardProps {
  title: string;
  value: React.ReactNode;
  subText?: React.ReactNode;
  icon: React.ReactNode;
  colorTheme: 'blue' | 'green' | 'purple' | 'amber' | 'red';
}

export function StatCard({ title, value, subText, icon, colorTheme }: StatCardProps) {
  return (
    <div className="card stat-card">
      <div className={`stat-icon ${colorTheme}`}>{icon}</div>
      <div className="stat-info">
        <h4>{title}</h4>
        <div className="stat-value" style={{ fontSize: 16 }}>{value || '-'}</div>
        {subText && <div className="stat-sub">{subText}</div>}
      </div>
    </div>
  );
}
