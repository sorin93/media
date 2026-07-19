const formatCount = n => n < 1000 ? n : `${Math.floor(n / 1000)}k`;

const pad = n => n.toString().padStart(2, '0');
const formatDate = d => `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;

const formatTimestamp = timestamp => {
  const isoString = timestamp
    ?.replace(' ', 'T')
    ?.replace(/(\.\d+)\s+([+-]\d{2}:\d{2}:\d{2})/, '$1$2')
    ?.replace(/([+-]\d{2}:\d{2}):\d{2}/, '$1');
  const date = new Date(isoString);
  if (isNaN(date.getTime())) return '…';
  const now = new Date();
  const options = {
    month: 'short',
    day: 'numeric',
    ...(date.getFullYear() !== now.getFullYear() && { year: 'numeric' })
  };
  return date.toLocaleDateString(undefined, options);
};

const plural = (no, ending = 's') => no === 1 ? '' : ending;

export {
  formatCount,
  formatDate,
  formatTimestamp,
  plural,
};