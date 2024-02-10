import {
  Configuration,
  ConfigurationDto,
  convertFromDto,
  convertToDto,
} from "@/domain/config";
import { invoke } from "@tauri-apps/api";

export interface ExecutionError {
  message: string;
}

export interface CommandExecutionResult<T> {
  result?: T;
  error?: ExecutionError;
}

export default interface BackendApi {
  getConfig(): Promise<Configuration>;
  setConfig(config: Configuration): void;
}

export class BackendApiImpl implements BackendApi {
  async getConfig(): Promise<Configuration> {
    return invoke<CommandExecutionResult<ConfigurationDto>>(
      "get_config", {}
    ).then((response: CommandExecutionResult<ConfigurationDto>) => {
      if (response.result) {
        return convertFromDto(response.result);
      } else {
        console.error(response.error);
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
    });
  }
  setConfig(config: Configuration): void {
    let dto = convertToDto(config);
    invoke<CommandExecutionResult<boolean>>("save_config", {
      config: dto,
    }).then((response: CommandExecutionResult<boolean>) => {
      console.log(response);
    });
  }
}
