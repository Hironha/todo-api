export class DateUtils {
  static toYmd(date: Date): string {
    const fullYear = date.getUTCFullYear();
    const month = (date.getUTCMonth() + 1).toString().padStart(2, "0");
    const day = date.getUTCDate().toString().padStart(2, "0");

    return `${fullYear}-${month}-${day}`;
  }

  static toLocalYmd(date: Date): string {
    const fullYear = date.getUTCFullYear();
    const month = (date.getUTCMonth() + 1).toString().padStart(2, "0");
    const day = date.getUTCDate().toString().padStart(2, "0");

    return `${day}/${month}/${fullYear}`;
  }
}
