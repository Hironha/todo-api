export class DateUtils {
  /** Transforms UTC date into YMD format using the `separator` between each part */
  static utcYMD(date: Date, separator: string = '-'): string {
    const day = date.getUTCDate().toString().padEnd(2, '0')
    const month = (date.getUTCMonth() + 1).toString().padStart(2, '0')
    const year = date.getUTCFullYear()

    return [year, month, day].join(separator)
  }

  static utcRFC3339(date: Date): string {
    return date.toUTCString()
  }

  static utcTimestamp(date: Date): number {
    const msTimezoneDiff = date.getTimezoneOffset() * 60 * 1000
    return date.getTime() + msTimezoneDiff
  }
}
