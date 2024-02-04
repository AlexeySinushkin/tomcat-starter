import { Variables, ServerRun } from '@/domain/config'

export default interface BackendApi {
    getConfig() : Variables;
    getRuns() : ServerRun[];
}