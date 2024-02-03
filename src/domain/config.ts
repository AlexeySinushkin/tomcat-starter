import { CommonShape } from "./commonShape";
import { PropertyShape } from "./property";
import { Task } from "./taskTemplate";

export type Config = {
  globalVars: PropertyShape[];
  releases: CommonShape[];
  platforms: CommonShape[];
  servers: CommonShape[];
};

export type ServerRun = {
  serverRunName: string;
  serverName: string;//reference to Server
  tasks: Task[];
};
