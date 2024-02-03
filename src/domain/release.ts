import { PropertyShape } from "./property.ts";
import { CommonShape } from './commonShape.ts';

export class Release implements CommonShape {
  constructor(public name: string, public properties: PropertyShape[] = []) {}
}
