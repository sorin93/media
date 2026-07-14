export default async (media_id, upload_url, body) => {
  if (!media_id || !upload_url || !body?.type) throw new Error('Error');

  const res = await fetch(upload_url, {
    method: 'put',
    body,
    headers: {
      'Content-Type': body.type,
      'X-Media-Id': media_id,
    }
  });

  if (!res.ok) {
    let e = 'Error';
    try {
      const json = await res.json();
      e = json?.errors?.[0]?.message || json?.error || e;
    } catch {}
    throw new Error(e);
  }
};