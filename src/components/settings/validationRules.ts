export const nameRules = [
    (value: string) => {
        if (value) return true;
        return "Name is required.";
    },
    (value: string) => {
        if (value?.length <= 20) return true;
        return "Name must be between 1 and 20 characters.";
    },
];