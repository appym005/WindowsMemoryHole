export function looksLikeUrl(input: string) {
  const trimmed = input.trim();
  if (!trimmed) {
    return false;
  }
  return /^https?:\/\/[^\s]+$/i.test(trimmed);
}

export function truncate(value: string, max = 42) {
  if (value.length <= max) {
    return value;
  }
  return `${value.slice(0, max)}…`;
}
