import { Configuration, convertToDto } from "@/domain/config";
import { invoke } from "@tauri-apps/api";

export default interface BackendApi {
  getConfig(): Configuration;
  setConfig(config: Configuration): void;
}

export class BackendApiImpl implements BackendApi {
  getConfig(): Configuration {
    return {
      vars: {
        globalVars: [],
        releases: [],
        platforms: [],
        servers: [],
      },
      runs: [],
    };
  }
  setConfig(config: Configuration): void {
    let dto = convertToDto(config);
       invoke("save_config", {config: dto}).then((response) => console.log(response));
  }
}
