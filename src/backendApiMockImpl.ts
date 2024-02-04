import BackendApi from "./backendApi";
import { Variables, ServerRun } from "@/domain/config";
import { PropertyShape } from "./domain/property";
import { Release } from "./domain/release";
import { Platform } from "./domain/platform";
import { Server } from "./domain/server";
import { Task, CopyWarToRandomDir, RunTomcat } from "./domain/taskTemplate";

let sedoDir: PropertyShape = { name: "sedoDir", value: "sedo63" };
let etDir: PropertyShape = { name: "etDir", value: "et-extend-63" };
let release63 = new Release("63", [sedoDir, etDir]);

let dbConnection: PropertyShape = {
  name: "connectionFileName",
  value: "local-gg.properties",
};
let localPlatformRg: Platform = new Platform("localPlatformRg", [dbConnection]);

let dstWarName: PropertyShape = {
  name: "dstWarName",
  value: "main.war",
};

let mainServer: Server = new Server("dedo", [dstWarName]);

let sources: PropertyShape = {
  name: "soruceRoot",
  value: "/home/user/project",
};

let copyRandom: Task = new CopyWarToRandomDir(
  "${release.dedoDir}/dedo/target/dedo-${release.version}.war",
  "${commanCatalinaBase}/",
  "dedo.war"
);
let runTomcat: Task = new RunTomcat(
  "-Dpmd.skip=true -Ddedo.properties=${platform.dedoProps}/props.properties",
  9111,
  10111
);
let dedoRun = { serverRunName: "dedo-clean", serverName: "dedo", tasks: [copyRandom, runTomcat] };

export default class BackendApiMockImpl implements BackendApi {
  getConfig(): Variables {
    return {
      globalVars: [sources],
      releases: [release63],
      platforms: [localPlatformRg],
      servers: [mainServer],
    };
  }
  getRuns(): ServerRun[] {
    return [dedoRun];
  }
}
