import { appendToHash } from '@/hash/hashUtils.ts';

export type PropertyShape = {
  name: string;
  value: string;
};

export function appendPropertiesToHash(
  properties: PropertyShape[],
  initialNumber: number
): number {
  var hashCode = initialNumber;
  properties
    .filter((prop) => prop.name)
    .sort((a, b) => a.name.localeCompare(b.name))
    .forEach((prop) => {
      if (prop.name) {
        hashCode = appendToHash(prop.name, hashCode);
      }
      if (prop.value) {
        hashCode = appendToHash(prop.value, hashCode);
      }
    });
  return hashCode;
}
