import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { api, OptimizationResult } from '../lib/tauri';
import { CheckCircle, Loader, Zap, AlertCircle } from 'lucide-react';

export default function SafeOptimize() {
  const { t } = useTranslation();
  const [phase, setPhase] = useState<'idle' | 'running' | 'done' | 'error'>('idle');
  const [steps, setSteps] = useState<{ id: string; label: string; status: 'pending' | 'active' | 'done' | 'error' }[]>([]);
  const [results, setResults] = useState<OptimizationResult[]>([]);
  const [error, setError] = useState('');

  async function runSafeOptimize() {
    setPhase('running');
    setError('');
    const allResults: OptimizationResult[] = [];

    // Step 0: Create restore point
    const restoreStep = { id: 'restore', label: t('safe_optimize.creating_restore'), status: 'active' as const };
    setSteps([restoreStep]);

    try {
      const rpResult = await api.createRestorePoint('Safe Optimization');
      setSteps(prev => prev.map(s => s.id === 'restore' ? { ...s, status: rpResult.success ? 'done' : 'error' } : s));
      if (!rpResult.success) {
        // Continue anyway, restore point is not critical
        console.warn('Restore point failed:', rpResult.message);
      }
    } catch (err) {
      setSteps(prev => prev.map(s => s.id === 'restore' ? { ...s, status: 'error' } : s));
    }

    // Get safe optimization IDs
    try {
      const safeIds = await api.getSafeOptimizations();
      const catalog = await api.getOptimizationCatalog();

      const optimizationSteps = safeIds.map(id => {
        const item = catalog.find(c => c.id === id);
        return {
          id,
          label: item?.name || id,
          status: 'pending' as const,
        };
      });

      setSteps(prev => [...prev, ...optimizationSteps]);

      // Run each optimization sequentially
      for (const sid of safeIds) {
        setSteps(prev => prev.map(s => s.id === sid ? { ...s, status: 'active' } : s));

        try {
          const result = await api.applyOptimization(sid);
          allResults.push(result);
          setSteps(prev => prev.map(s => s.id === sid ? { ...s, status: result.success ? 'done' : 'error' } : s));
        } catch (err) {
          setSteps(prev => prev.map(s => s.id === sid ? { ...s, status: 'error' } : s));
          allResults.push({ id: sid, success: false, message: String(err) });
        }

        // Small delay for visual feedback
        await new Promise(r => setTimeout(r, 200));
      }

      setResults(allResults);
      setPhase('done');
    } catch (err) {
      setError(String(err));
      setPhase('error');
    }
  }

  const successCount = results.filter(r => r.success).length;

  return (
    <div className="animate-fade-in">
      <div className="page-header">
        <h2>{t('safe_optimize.title')}</h2>
        <p>{t('safe_optimize.subtitle')}</p>
      </div>

      {phase === 'idle' && (
        <div className="optimize-hero card">
          {/* Checklist */}
          <div style={{ marginBottom: 32, textAlign: 'left', maxWidth: 400 }}>
            <h3 style={{ fontSize: 16, fontWeight: 600, marginBottom: 16 }}>{t('safe_optimize.checklist_title')}</h3>
            {[t('safe_optimize.check_1'), t('safe_optimize.check_2'), t('safe_optimize.check_3')].map((text, i) => (
              <div key={i} style={{ display: 'flex', gap: 10, alignItems: 'flex-start', marginBottom: 10, fontSize: 14, color: 'var(--text-secondary)' }}>
                <CheckCircle size={18} style={{ color: 'var(--success)', flexShrink: 0, marginTop: 1 }} />
                <span>{text}</span>
              </div>
            ))}
          </div>

          {/* Big optimize button */}
          <div className="optimize-btn-wrapper">
            <div className="optimize-btn-ring"></div>
            <button className="optimize-btn" onClick={runSafeOptimize}>
              <Zap size={40} />
              {t('safe_optimize.button')}
            </button>
          </div>
        </div>
      )}

      {phase === 'running' && (
        <div className="card" style={{ padding: 32 }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: 12, marginBottom: 24 }}>
            <Loader size={20} className="spinner" style={{ border: 'none', borderTop: 'none', animation: 'spin 1s linear infinite' }} />
            <h3 style={{ fontSize: 16, fontWeight: 600 }}>{t('safe_optimize.button_running')}</h3>
          </div>
          <div className="progress-stepper">
            {steps.map((step, i) => (
              <div key={step.id} className={`step-item ${step.status === 'active' ? 'active' : step.status === 'done' ? 'done' : step.status === 'error' ? 'error' : ''}`}>
                <div className="step-icon">
                  {step.status === 'done' ? '✓' : step.status === 'error' ? '✗' : step.status === 'active' ? <span className="spinner" style={{ width: 14, height: 14 }}></span> : i + 1}
                </div>
                <span className="step-label">{step.label}</span>
              </div>
            ))}
          </div>
        </div>
      )}

      {phase === 'done' && (
        <div className="card" style={{ padding: 32, textAlign: 'center' }}>
          <div style={{ width: 64, height: 64, borderRadius: '50%', background: 'rgba(16, 185, 129, 0.15)', display: 'flex', alignItems: 'center', justifyContent: 'center', margin: '0 auto 16px' }}>
            <CheckCircle size={32} style={{ color: 'var(--success)' }} />
          </div>
          <h3 style={{ fontSize: 20, fontWeight: 700, marginBottom: 8 }}>{t('safe_optimize.complete')}</h3>
          <p style={{ color: 'var(--text-secondary)', marginBottom: 8 }}>{t('safe_optimize.complete_sub')}</p>
          <p style={{ color: 'var(--success)', fontWeight: 600 }}>{successCount} {t('safe_optimize.items_applied')}</p>

          <div className="progress-stepper mt-6">
            {steps.map((step, i) => (
              <div key={step.id} className={`step-item ${step.status}`}>
                <div className="step-icon">
                  {step.status === 'done' ? '✓' : step.status === 'error' ? '✗' : i + 1}
                </div>
                <span className="step-label">{step.label}</span>
              </div>
            ))}
          </div>

          <button className="btn btn-ghost mt-6" onClick={() => { setPhase('idle'); setSteps([]); setResults([]); }}>
            {t('common.back')}
          </button>
        </div>
      )}

      {phase === 'error' && (
        <div className="card" style={{ padding: 32, textAlign: 'center' }}>
          <AlertCircle size={48} style={{ color: 'var(--danger)', marginBottom: 16 }} />
          <h3 style={{ fontSize: 18, fontWeight: 600, marginBottom: 8 }}>{t('common.error')}</h3>
          <p style={{ color: 'var(--text-secondary)' }}>{error}</p>
          <button className="btn btn-ghost mt-4" onClick={() => setPhase('idle')}>
            {t('common.back')}
          </button>
        </div>
      )}
    </div>
  );
}
