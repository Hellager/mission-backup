import { camelCase, isArray, isObject, snakeCase, transform } from 'lodash'

// https://stackoverflow.com/questions/59769649/recursively-convert-an-object-fields-from-snake-case-to-camelcase

/**
 * Converts keys of an object to snake_case.
 * @param data - The object to convert.
 * @returns The object with keys in snake_case.
 */
export function toSnakeCase(data: any): unknown {
  return transform(data, (acc: any, value: unknown, key: string, target) => {
    const snakeKey = isArray(target) ? key : snakeCase(key)

    acc[snakeKey] = isObject(value) ? toSnakeCase(value) : value
  })
}

/**
 * Converts keys of an object to camelCase.
 * @param data - The object to convert.
 * @returns The object with keys in camelCase.
 */
export function toCamelCase(data: any): unknown {
  return transform(data, (acc: any, value: unknown, key: string, target) => {
    const camelKey = isArray(target) ? key : camelCase(key)

    acc[camelKey] = isObject(value) ? toCamelCase(value) : value
  })
}

/**
 * Compares the keys of two objects to determine if they are the same.
 *
 * @param a - The first object
 * @param b - The second object
 * @returns Returns true if the two objects have the same keys, otherwise returns false.
 */
export function compareKeys(a: any, b: any) {
  const aKeys = Object.keys(a).sort()
  const bKeys = Object.keys(b).sort()
  return JSON.stringify(aKeys) === JSON.stringify(bKeys)
}

/**
 * Function that count occurrences of a substring in a string;
 * @param src - The string
 * @param sub - The subString            The sub string to search for
 * @param allowOverlapping - Optional. (Default:false)
 *
 * @author Vitim.us https://gist.github.com/victornpb/7736865
 * @see Unit Test https://jsfiddle.net/Victornpb/5axuh96u/
 * @see https://stackoverflow.com/a/7924240/938822
 */
export function subStringOccurrences(src: string, sub: string, allowOverlapping?: boolean) {
  src += ''
  sub += ''
  if (sub.length <= 0)
    return (src.length + 1)

  let n = 0
  let pos = 0
  const step = allowOverlapping ? 1 : sub.length

  while (true) {
    pos = src.indexOf(sub, pos)
    if (pos >= 0) {
      ++n
      pos += step
    }
    else {
      break
    }
  }
  return n
}
