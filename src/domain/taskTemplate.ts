import generateId from '@/idGeneration'

export enum TaskType {
  CopyWarDistinct = "CopyWarDistinct",
  CopyWarRandom = "Copy War to radom path",
  RunTomcat = "Run Tomcat",
}

export interface Task {
  type: TaskType;
  id: string;
}


export class CopyWarToDistinctDir implements Task {
  public readonly type: TaskType;
  public readonly id: string;
  constructor(
    public sourceWarPath: string,
    public dstCatalinaBase: string,
    public destinationWarName: string,
    public skipUnchanged: boolean
  ) {
    this.type = TaskType.CopyWarDistinct;
    this.id=generateId();
  }
}

export class CopyWarToRandomDir implements Task {
  public readonly type: TaskType;
  public readonly id: string;
  constructor(public sourceWarPath: string,
    public destinationCatalinaBase: string,
    public destinationWarName: string) {
    this.type = TaskType.CopyWarRandom;
    this.id=generateId();
  }
}

export class RunTomcat implements Task {
  public readonly type: TaskType;
  public readonly id: string;
  constructor(
    public catalinaOpts: string,
    public listenPort: number,
    public jdpaPort: number
  ) {
    this.type = TaskType.RunTomcat;
    this.id=generateId();
  }
}

