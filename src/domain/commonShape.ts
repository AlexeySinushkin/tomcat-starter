import { appendToHash } from "@/hash/hashUtils";
import { PropertyShape, appendPropertiesToHash } from "./property";

export type CommonShape = {
  name: string;
  properties: PropertyShape[];
};

export function hashCode(cs: CommonShape): number {
  var hashCode = 0;
  if (cs.name) {
    hashCode = appendToHash(cs.name);
  }
  hashCode = appendPropertiesToHash(cs.properties, hashCode);
  return hashCode;
}