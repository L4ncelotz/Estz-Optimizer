import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';
import en from '../locales/en.json';
import th from '../locales/th.json';

const savedLang = localStorage.getItem('estz-language') || 'en';

i18n.use(initReactI18next).init({
  resources: {
    en: { translation: en },
    th: { translation: th },
  },
  lng: savedLang,
  fallbackLng: 'en',
  interpolation: {
    escapeValue: false,
  },
});

export default i18n;
