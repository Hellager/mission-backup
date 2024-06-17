import { camelCase, snakeCase, isArray, transform, isObject  } from "lodash";

// https://stackoverflow.com/questions/59769649/recursively-convert-an-object-fields-from-snake-case-to-camelcase

/**
 * Converts keys of an object to snake_case.
 * @param data - The object to convert.
 * @returns The object with keys in snake_case.
 */
export function toSnakeCase(data: any): unknown {
    return transform(data, (acc: any, value: unknown, key: string, target) => {
        const snakeKey = isArray(target) ? key : snakeCase(key);

        acc[snakeKey] = isObject(value) ? toSnakeCase(value) : value;
    })
}

/**
 * Converts keys of an object to camelCase.
 * @param data - The object to convert.
 * @returns The object with keys in camelCase.
 */
export function toCamelCase(data: any): unknown {
    return transform(data, (acc: any, value: unknown, key: string, target) => {
        const camelKey = isArray(target) ? key : camelCase(key);

        acc[camelKey] = isObject(value) ? toCamelCase(value) : value;
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
    var aKeys = Object.keys(a).sort();
    var bKeys = Object.keys(b).sort();
    return JSON.stringify(aKeys) === JSON.stringify(bKeys);
}
