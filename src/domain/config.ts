import { CommonShape } from "./commonShape";
import { PropertyShape } from "./property";
import { CopyWarToRandomDir, RunTomcat, Task, TaskType } from "./taskTemplate";

export type Variables = {
  globalVars: PropertyShape[];
  releases: CommonShape[];
  platforms: CommonShape[];
  servers: CommonShape[];
};

export type ServerRun = {
  serverRunName: string;
  serverName: string; //reference to Server
  tasks: Task[];
};

export type Configuration = {
  vars: Variables;
  runs: ServerRun[];
};

type ServerRunDto = {
  serverRunName: string;
  serverName: string;
  order: string[]; //order (id)
  warToRandom: CopyWarToRandomDir[];
  runTomcat: RunTomcat[];
};

export type ConfigurationDto = {
  vars: Variables;
  runs: ServerRunDto[];
};

export function convertToDto(config: Configuration): ConfigurationDto {
  const runs: ServerRunDto[] = [];
  const configDto: ConfigurationDto = { vars: config.vars, runs };

  config.runs.forEach((run) => {
    const warToRandom: CopyWarToRandomDir[] = [];
    const runTomcat: RunTomcat[] = [];
    const order: string[] = [];
    run.tasks.forEach((task) => {
      if (task.type == TaskType.CopyWarRandom) {
        warToRandom.push(task as CopyWarToRandomDir);
        order.push(task.id);
      } else if (task.type == TaskType.RunTomcat) {
        runTomcat.push(task as RunTomcat);
        order.push(task.id);
      } else {
        throw new Error("Not implemented");
      }
    });
    runs.push({
      serverName: run.serverName,
      serverRunName: run.serverRunName,
      warToRandom,
      runTomcat,
      order,
    });
  });

  return configDto;
}

export function convertFromDto(configDto: ConfigurationDto): Configuration {
  const runs: ServerRun[] = [];
  const config: Configuration = { vars: configDto.vars, runs };

  configDto.runs.forEach((run) => {
    const tasks: Task[] = [];
    run.order.forEach((taskId) => {
      let task1 = run.warToRandom.find((task) => task.id == taskId);
      let task2 = run.runTomcat.find((task) => task.id == taskId);
      if (task1) {
        tasks.push(task1);
      } else if (task2) {
        tasks.push(task2);
      }
    });
    runs.push({
      serverName: run.serverName,
      serverRunName: run.serverRunName,
      tasks,
    });
  });

  return config;
}
