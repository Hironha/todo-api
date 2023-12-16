import { type Tag } from "./tag";

/** `Date` string  in ISO 8601 YYYY-MM-DD format. Ex: `2023-12-16` */
export type DateYmd = Tag<string, "dateYmd">;
/** `Date` string in conventional format, i.e. DD/MM/YYYY . Ex: `16/12/2023` */
export type ConventionalDate = Tag<string, "conventionalDate">;
/** `Date` in RFC 3339 format, i.e ISO 8601 with time. Ex: `2023-12-16T15:57:44.580Z` */
export type DateTime = Tag<string, "dateTime">;

export function formatDateYmd(date: Date): DateYmd {
  const fullYear = date.getUTCFullYear();
  const month = (date.getUTCMonth() + 1).toString().padStart(2, "0");
  const day = date.getUTCDate().toString().padStart(2, "0");

  return `${fullYear}-${month}-${day}` as DateYmd;
}

export function formatConventionalDate(date: Date): ConventionalDate {
  const fullYear = date.getUTCFullYear();
  const month = (date.getUTCMonth() + 1).toString().padStart(2, "0");
  const day = date.getUTCDate().toString().padStart(2, "0");

  return `${day}/${month}/${fullYear}` as ConventionalDate;
}

export function formatDateTime(date: Date): DateTime {
  return date.toISOString() as DateTime;
}
