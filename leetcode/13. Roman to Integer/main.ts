function romanToInt(s: string): number {
    const map = {
        'I': 1,
        'V': 5,
        'X': 10,
        'L': 50,
        'C': 100,
        'D': 500,
        'M': 1000,
        // pairs
        'IV': 4,
        'IX': 9,
        'XL': 40,
        'XC': 90,
        'CD': 400,
        'CM': 900,
    };
    let sum = 0;
    for(let i = 0; i < s.length; i++) {
        if(i < s.length - 1) {
            const end = s.substring(i, i + 2);
            if (end in map) {
                sum += map[end];
                i++;
                continue;
            }
        }
        const end = s.substring(i, i + 1);
        if (end in map) {
            sum += map[end];
            continue;
        }
    }
    return sum;
};