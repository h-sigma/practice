function lengthOfLastWord(s: string): number {
    return s.split(' ').filter(Boolean).at(-1)?.length ?? 0;
};