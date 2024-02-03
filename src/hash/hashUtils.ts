export function appendToHash(str: string, initialNumber?: number): number {
    var h: number = initialNumber ?? 0;    
    for (var i = 0; i < str.length; i++) {
        h = 31 * h + str.charCodeAt(i);
    }
    return h;
}