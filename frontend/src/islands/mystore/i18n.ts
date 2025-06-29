import { register, init } from 'svelte-i18n';

register('en', () => import('./locales/en.json'));
register('ar', () => import('./locales/ar.json'));
register('fr', () => import('./locales/fr.json'));

const savedLocale = localStorage.getItem('locale') || 'en';

init({
  fallbackLocale: 'en',
  initialLocale: savedLocale,
});
