import { Config, ServerRun } from '@/domain/config'

export default interface BackendApi {
    getConfig() : Config;
    getRuns() : ServerRun[];
}