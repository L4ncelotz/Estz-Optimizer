import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { api, BloatwareApp } from '../lib/tauri';
import { Trash2, Loader, CheckCircle, Package } from 'lucide-react';

export default function BloatwareRemover() {
  const { t } = useTranslation();
  const [apps, setApps] = useState<BloatwareApp[]>([]);
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [scanning, setScanning] = useState(true);
  const [removing, setRemoving] = useState(false);
  const [removed, setRemoved] = useState<string[]>([]);

  useEffect(() => {
    scanApps();
  }, []);

  async function scanApps() {
    setScanning(true);
    try {
      const result = await api.scanBloatware();
      setApps(result);
    } catch (err) {
      console.error('Failed to scan:', err);
    } finally {
      setScanning(false);
    }
  }

  function toggleSelect(pkg: string) {
    setSelected(prev => {
      const next = new Set(prev);
      if (next.has(pkg)) next.delete(pkg);
      else next.add(pkg);
      return next;
    });
  }

  function selectAll(category: string) {
    const catApps = apps.filter(a => a.category === category && !removed.includes(a.package_name));
    setSelected(prev => {
      const next = new Set(prev);
      catApps.forEach(a => next.add(a.package_name));
      return next;
    });
  }

  function deselectAll(category: string) {
    const catApps = apps.filter(a => a.category === category);
    setSelected(prev => {
      const next = new Set(prev);
      catApps.forEach(a => next.delete(a.package_name));
      return next;
    });
  }

  async function handleRemove() {
    setRemoving(true);
    try {
      const packages = Array.from(selected);
      const results = await api.removeBloatware(packages);
      const newRemoved = results.filter(([_, ok]) => ok).map(([pkg]) => pkg);
      setRemoved(prev => [...prev, ...newRemoved]);
      setSelected(new Set());
    } catch (err) {
      console.error('Failed to remove:', err);
    } finally {
      setRemoving(false);
    }
  }

  // Group apps by category
  const groups = apps.reduce<Record<string, BloatwareApp[]>>((acc, app) => {
    if (!acc[app.category]) acc[app.category] = [];
    acc[app.category].push(app);
    return acc;
  }, {});

  const categoryLabels: Record<string, string> = {
    microsoft: t('bloatware.microsoft'),
    oem_hp: 'HP',
    oem_dell: 'Dell',
    oem_lenovo: 'Lenovo',
    oem_asus: 'ASUS',
    oem_acer: 'Acer',
    oem_msi: 'MSI',
  };

  return (
    <div className="animate-fade-in">
      <div className="page-header flex justify-between items-center">
        <div>
          <h2>{t('bloatware.title')}</h2>
          <p>{t('bloatware.subtitle')}</p>
        </div>
        {selected.size > 0 && (
          <button className="btn btn-danger" onClick={handleRemove} disabled={removing}>
            {removing ? <Loader size={16} className="spinner" /> : <Trash2 size={16} />}
            {removing ? t('bloatware.removing') : `${t('bloatware.remove_selected')} (${selected.size})`}
          </button>
        )}
      </div>

      {scanning ? (
        <div className="card" style={{ padding: 48, textAlign: 'center' }}>
          <div className="spinner" style={{ width: 32, height: 32, margin: '0 auto 16px' }}></div>
          <p style={{ color: 'var(--text-secondary)' }}>{t('bloatware.scanning')}</p>
        </div>
      ) : apps.length === 0 ? (
        <div className="card" style={{ padding: 48, textAlign: 'center' }}>
          <CheckCircle size={48} style={{ color: 'var(--success)', margin: '0 auto 16px' }} />
          <p style={{ fontSize: 16, fontWeight: 600 }}>{t('bloatware.no_bloatware')}</p>
        </div>
      ) : (
        Object.entries(groups).map(([category, categoryApps]) => (
          <div key={category} className="card mb-4">
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 16 }}>
              <h3 style={{ fontSize: 16, fontWeight: 600, display: 'flex', alignItems: 'center', gap: 8 }}>
                <Package size={18} />
                {categoryLabels[category] || category}
                <span style={{ fontSize: 12, color: 'var(--text-tertiary)', fontWeight: 400 }}>
                  ({categoryApps.filter(a => !removed.includes(a.package_name)).length} {t('bloatware.scanning').replace('...', '').replace(t('bloatware.scanning'), 'apps')})
                </span>
              </h3>
              <div style={{ display: 'flex', gap: 8 }}>
                <button className="btn btn-sm btn-ghost" onClick={() => selectAll(category)}>{t('bloatware.select_all')}</button>
                <button className="btn btn-sm btn-ghost" onClick={() => deselectAll(category)}>{t('bloatware.deselect_all')}</button>
              </div>
            </div>

            <div style={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
              {categoryApps.map(app => {
                const isRemoved = removed.includes(app.package_name);
                return (
                  <div key={app.package_name} className="checkbox-item" style={{ opacity: isRemoved ? 0.4 : 1 }}>
                    <input
                      type="checkbox"
                      checked={selected.has(app.package_name)}
                      onChange={() => toggleSelect(app.package_name)}
                      disabled={isRemoved}
                    />
                    <label style={{ display: 'flex', justifyContent: 'space-between', width: '100%' }}>
                      <span style={{ fontWeight: 500 }}>{app.display_name}</span>
                      {isRemoved && (
                        <span style={{ fontSize: 12, color: 'var(--success)', fontWeight: 600 }}>✓ Removed</span>
                      )}
                    </label>
                  </div>
                );
              })}
            </div>
          </div>
        ))
      )}
    </div>
  );
}
