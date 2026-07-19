import store from './store.svelte';

const fetchApi = async (method, path, payload, isRetry = false) => {
  const
    hasBody = ['delete', 'patch', 'post', 'put'].includes(method.toLowerCase()) && payload,
    isFormData = payload instanceof FormData,
    query = payload && !isFormData && method.toLowerCase() === 'get' ? Object.keys(payload)
      .map(key => `${encodeURIComponent(key)}=${encodeURIComponent(payload[key])}`)
      .join('&') : undefined,
    headers = { 'Accept': 'application/json' },
    access_token = localStorage.getItem('access_token'),
    refresh_token = localStorage.getItem('refresh_token');

  if (access_token) headers['Authorization'] = `Bearer ${access_token}`;
  if (hasBody && !isFormData) headers['Content-Type'] = 'application/json';

  const res = await fetch(`${store.API}${path}${query ? `?${query}` : ''}`, {
    body: hasBody ? (isFormData ? payload : JSON.stringify(payload)) : undefined,
    cache: 'no-store',
    credentials: 'include',
    headers,
    method: method.toUpperCase(),
    mode: 'cors',
  });

  if (!res.ok) {

    // Try to refresh the access token
    if (res.status === 401 && !isRetry) {
      if (!access_token) {
        localStorage.clear();
        const error = new Error(await res.text() || res.statusText);
        error.status = res.status;
        throw error;
      }
      localStorage.removeItem('access_token');
      try {
        const res = await fetchApi('post', '/auth/refresh', { refresh_token }, true);
        // Session
        if (res.tokens) {
          localStorage.setItem('access_token', res.tokens.access_token);
          localStorage.setItem('refresh_token', res.tokens.refresh_token);
          localStorage.setItem('user', JSON.stringify(res.user));
          store.user = res.user;
        }
      } catch (e) {
        if (e.status === 401 || e.status === 403) {
          localStorage.clear();
          store.user = null;
          throw new Error('Session expired. Please log in again.');
        }
        throw e;
      }
      return await fetchApi(method, path, payload, true);
    }

    const error = new Error(await res.text() || res.statusText);
    error.status = res.status;
    throw error;
  }

  // json res body
  if (res.headers.get("Content-Length") === "0" || res.status === 204) {
    return {};
  } else {
    try {
      return await res.json();
    } catch (e) {
      throw new Error('Error');
    }
  }

}

export default fetchApi;