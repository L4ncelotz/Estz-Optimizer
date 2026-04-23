import { useTranslation } from 'react-i18next';
import { Globe, Info } from 'lucide-react';

export default function Settings() {
  const { t, i18n } = useTranslation();

  function changeLanguage(lang: string) {
    i18n.changeLanguage(lang);
    localStorage.setItem('estz-language', lang);
  }

  return (
    <div className="animate-fade-in">
      <div className="page-header">
        <h2>{t('settings.title')}</h2>
        <p>{t('settings.subtitle')}</p>
      </div>

      {/* Language */}
      <div className="card mb-4">
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
            <div className="stat-icon blue"><Globe size={20} /></div>
            <div>
              <h4 style={{ fontSize: 14, fontWeight: 600 }}>{t('settings.language')}</h4>
            </div>
          </div>
          <div className="lang-switch">
            <button className={`lang-btn ${i18n.language === 'en' ? 'active' : ''}`} onClick={() => changeLanguage('en')}>
              EN
            </button>
            <button className={`lang-btn ${i18n.language === 'th' ? 'active' : ''}`} onClick={() => changeLanguage('th')}>
              TH
            </button>
          </div>
        </div>
      </div>

      {/* About */}
      <div className="card">
        <div style={{ display: 'flex', alignItems: 'flex-start', gap: 12 }}>
          <div className="stat-icon purple"><Info size={20} /></div>
          <div>
            <h4 style={{ fontSize: 14, fontWeight: 600, marginBottom: 8 }}>{t('settings.about')}</h4>
            <p style={{ fontSize: 13, color: 'var(--text-secondary)', lineHeight: 1.6 }}>
              {t('settings.about_desc')}
            </p>
            <div style={{ marginTop: 12, display: 'flex', gap: 8 }}>
              <span className="badge badge-none">Tauri 2.0</span>
              <span className="badge badge-low">React</span>
              <span className="badge badge-medium">TypeScript</span>
              <span className="badge" style={{ background: 'rgba(139, 92, 246, 0.15)', color: '#8b5cf6' }}>Rust</span>
            </div>
            <div style={{ marginTop: 12, fontSize: 12, color: 'var(--text-tertiary)' }}>
              {t('app.version')}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
