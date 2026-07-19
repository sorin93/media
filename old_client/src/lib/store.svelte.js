const
  API = 'https://api.sorin.cc', // process.env.NODE_ENV === 'production' ? 'https://api.sorin.cc' : 'http://192.168.0.12:4443',
  CDN = 'https://media.sorin.cc/',
  HAS_TOUCH = (typeof window !== 'undefined' && typeof navigator !== 'undefined' &&
    (navigator.maxTouchPoints > 0 || window.matchMedia('(any-pointer: coarse)').matches || 'ontouchstart' in window));

function createStore() {
  let
    cache = $state.raw(new Map()), // { expiry, value }
    displayTheme = $state('light'),
    isMobile = $state(true),
    route = $state(),
    tabs = $state.raw(new Map()),
    theme = $state('system'),
    url = $state.raw(typeof window !== 'undefined' ? new URL(window.location.href) : null),
    user = $state.raw();

  const updateTheme = () => {
    const darkMql = window.matchMedia('(prefers-color-scheme: dark)');
    displayTheme = theme === 'system' ? darkMql.matches ? 'dark' : 'light' : theme;
    if (typeof document !== 'undefined') document.documentElement.setAttribute('data-theme', displayTheme);
  };

  if (typeof window !== 'undefined') {
    const mobileMql = window.matchMedia('(min-width: 1024px)');
    const darkMql = window.matchMedia('(prefers-color-scheme: dark)');
    isMobile = !mobileMql.matches;
    theme = localStorage.getItem('theme') || 'system';
    updateTheme();
    mobileMql.addEventListener('change', e => isMobile = !e.matches);
    darkMql.addEventListener('change', updateTheme);
    window.addEventListener('storage', e => {
      if (e.key === 'theme') {
        theme = e.newValue || 'system';
        updateTheme();
      }
    });
  }

  return {
    get cache() { return cache },
    set cache(value) { cache = value },
    getCache(key) {
      const entry = cache.get(key);
      if (!entry) return;
      if (Date.now() > entry.expiry) {
        cache.delete(key);
        return;
      }
      return entry.value;
    },
    setCache(key, value, more = false, ttl = 3600000 /* 1 hour */) {
      if (value == null) {
		  	cache.delete(key);
  			return;
		  }
      if (more) {
        const old = store.getCache(key);
        if (Array.isArray(old) && Array.isArray(value)) {
          value = [...old, ...value];
        } else if (old && typeof old === 'object' && typeof value === 'object') {
          value = Object.keys({ ...old, ...value }).reduce((obj, k) => {
            obj[k] = (Array.isArray(old[k]) && Array.isArray(value[k])) ? [...old[k], ...value[k]] : value[k] ?? old[k];
            return obj;
          }, {});
        }
      }
      cache.set(key, {
  			expiry: Date.now() + ttl,
			  value,
		  });
  		return value;
    },
    get isMobile() { return isMobile },
    get route() { return route },
    set route(value) { route = value },
    get tabs() { return tabs },
    set tabs(value) { tabs = value },
    get theme() { return theme },
    toggleTheme() {
      const system = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
      if (theme === 'system') {
        theme = system === 'dark' ? 'light' : 'dark';
      } else if (theme === 'dark') {
        theme = system === 'light' ? 'system' : 'light';
      } else {
        theme = system === 'dark' ? 'system' : 'dark';
      }
      updateTheme();
      localStorage.setItem('theme', theme);
    },
    get url() { return url },
    set url(value) { url = value },
    get user() { return user },
    set user(value) { user = value },

    get API() { return API },
    get CDN() { return CDN },
    get hasTouch() { return HAS_TOUCH },
    get today() { return new Date().toISOString().split('T')[0] },
  };
};

export default createStore();
