import { expect, test } from 'bun:test'
import { DateUtils } from './date'

test('should create ISO-8601 on YYYY-MM-DD format', () => {
  const date = new Date('2023-08-22')
  const output = DateUtils.utcYMD(date)

  expect(typeof output === 'string').toBeTrue()

  // validate if output is valid date by parsing to `Date`
  expect(Number.isNaN(new Date(output).getTime())).toBeFalse()

  // validate if it is on format YYYY-MM-DD
  const parts = output.split('-')
  expect(parts.length === 3).toBeTrue()
  expect(parts.at(0)).toBe('2023')
  expect(parts.at(1)).toBe('08')
  expect(parts.at(2)).toBe('22')
})
