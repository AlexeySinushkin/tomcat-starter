import { CommonShape } from "./commonShape";
import { PropertyShape } from "./property";


export class Server implements CommonShape {
    constructor(public name: string, public properties: PropertyShape[] = []) {}
}