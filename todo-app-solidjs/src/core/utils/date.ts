/** Create a string representation of `Date` in YYYY-MM-DD format, e.g. 2021-05-21 */
export function formatDateYmd(date: Date): string {
  const fullYear = date.getUTCFullYear();
  const month = (date.getUTCMonth() + 1).toString().padStart(2, "0");
  const day = date.getUTCDate().toString().padStart(2, "0");

  return `${fullYear}-${month}-${day}`;
}

/** Create a string representation of `Date` in DD/MM/YYYY format, e.g. 21/05/2021 */
export function formatDateConventional(date: Date): string {
  const fullYear = date.getUTCFullYear();
  const month = (date.getUTCMonth() + 1).toString().padStart(2, "0");
  const day = date.getUTCDate().toString().padStart(2, "0");

  return `${day}/${month}/${fullYear}`;
}
