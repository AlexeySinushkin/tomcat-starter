import { CommonShape } from "@/domain/commonShape";

export enum TaskType {
    CreateNew,
    Edit,
    Copy
}

export class IntentionTask {
    public constructor(public type: TaskType, public target : CommonShape){}
}