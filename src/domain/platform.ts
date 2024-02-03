import { PropertyShape } from "./property";
import { CommonShape } from './commonShape.ts';

export class Platform implements CommonShape {
  constructor(public name: string, public properties: PropertyShape[] = []) {}
}
