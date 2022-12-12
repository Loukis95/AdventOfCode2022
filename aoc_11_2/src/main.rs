use std::{env, fs, collections::VecDeque, fmt::{Display, Debug}, ops::{Mul, MulAssign, AddAssign, Add, Div, DivAssign}};

const NB_PRIME: usize = 50*20;
const PRIME_NUMBERS: [usize; NB_PRIME] =
        [2,    3,    5,    7,    11,   13,   17,   19,   23,   29,   31,   37,   41,   43,   47,   53,   59,   61,   67,   71,
         73,   79,   83,   89,   97,   101,  103,  107,  109,  113,  127,  131,  137,  139,  149,  151,  157,  163,  167,  173,
         179,  181,  191,  193,  197,  199,  211,  223,  227,  229,  233,  239,  241,  251,  257,  263,  269,  271,  277,  281,
         283,  293,  307,  311,  313,  317,  331,  337,  347,  349,  353,  359,  367,  373,  379,  383,  389,  397,  401,  409,
         419,  421,  431,  433,  439,  443,  449,  457,  461,  463,  467,  479,  487,  491,  499,  503,  509,  521,  523,  541,
         547,  557,  563,  569,  571,  577,  587,  593,  599,  601,  607,  613,  617,  619,  631,  641,  643,  647,  653,  659,
         661,  673,  677,  683,  691,  701,  709,  719,  727,  733,  739,  743,  751,  757,  761,  769,  773,  787,  797,  809,
         811,  821,  823,  827,  829,  839,  853,  857,  859,  863,  877,  881,  883,  887,  907,  911,  919,  929,  937,  941,
         947,  953,  967,  971,  977,  983,  991,  997,  1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069,
         1087, 1091, 1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153, 1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223,
         1229, 1231, 1237, 1249, 1259, 1277, 1279, 1283, 1289, 1291, 1297, 1301, 1303, 1307, 1319, 1321, 1327, 1361, 1367, 1373,
         1381, 1399, 1409, 1423, 1427, 1429, 1433, 1439, 1447, 1451, 1453, 1459, 1471, 1481, 1483, 1487, 1489, 1493, 1499, 1511,
         1523, 1531, 1543, 1549, 1553, 1559, 1567, 1571, 1579, 1583, 1597, 1601, 1607, 1609, 1613, 1619, 1621, 1627, 1637, 1657,
         1663, 1667, 1669, 1693, 1697, 1699, 1709, 1721, 1723, 1733, 1741, 1747, 1753, 1759, 1777, 1783, 1787, 1789, 1801, 1811,
         1823, 1831, 1847, 1861, 1867, 1871, 1873, 1877, 1879, 1889, 1901, 1907, 1913, 1931, 1933, 1949, 1951, 1973, 1979, 1987,
         1993, 1997, 1999, 2003, 2011, 2017, 2027, 2029, 2039, 2053, 2063, 2069, 2081, 2083, 2087, 2089, 2099, 2111, 2113, 2129,
         2131, 2137, 2141, 2143, 2153, 2161, 2179, 2203, 2207, 2213, 2221, 2237, 2239, 2243, 2251, 2267, 2269, 2273, 2281, 2287,
         2293, 2297, 2309, 2311, 2333, 2339, 2341, 2347, 2351, 2357, 2371, 2377, 2381, 2383, 2389, 2393, 2399, 2411, 2417, 2423,
         2437, 2441, 2447, 2459, 2467, 2473, 2477, 2503, 2521, 2531, 2539, 2543, 2549, 2551, 2557, 2579, 2591, 2593, 2609, 2617,
         2621, 2633, 2647, 2657, 2659, 2663, 2671, 2677, 2683, 2687, 2689, 2693, 2699, 2707, 2711, 2713, 2719, 2729, 2731, 2741,
         2749, 2753, 2767, 2777, 2789, 2791, 2797, 2801, 2803, 2819, 2833, 2837, 2843, 2851, 2857, 2861, 2879, 2887, 2897, 2903,
         2909, 2917, 2927, 2939, 2953, 2957, 2963, 2969, 2971, 2999, 3001, 3011, 3019, 3023, 3037, 3041, 3049, 3061, 3067, 3079,
         3083, 3089, 3109, 3119, 3121, 3137, 3163, 3167, 3169, 3181, 3187, 3191, 3203, 3209, 3217, 3221, 3229, 3251, 3253, 3257,
         3259, 3271, 3299, 3301, 3307, 3313, 3319, 3323, 3329, 3331, 3343, 3347, 3359, 3361, 3371, 3373, 3389, 3391, 3407, 3413,
         3433, 3449, 3457, 3461, 3463, 3467, 3469, 3491, 3499, 3511, 3517, 3527, 3529, 3533, 3539, 3541, 3547, 3557, 3559, 3571,
         3581, 3583, 3593, 3607, 3613, 3617, 3623, 3631, 3637, 3643, 3659, 3671, 3673, 3677, 3691, 3697, 3701, 3709, 3719, 3727,
         3733, 3739, 3761, 3767, 3769, 3779, 3793, 3797, 3803, 3821, 3823, 3833, 3847, 3851, 3853, 3863, 3877, 3881, 3889, 3907,
         3911, 3917, 3919, 3923, 3929, 3931, 3943, 3947, 3967, 3989, 4001, 4003, 4007, 4013, 4019, 4021, 4027, 4049, 4051, 4057,
         4073, 4079, 4091, 4093, 4099, 4111, 4127, 4129, 4133, 4139, 4153, 4157, 4159, 4177, 4201, 4211, 4217, 4219, 4229, 4231,
         4241, 4243, 4253, 4259, 4261, 4271, 4273, 4283, 4289, 4297, 4327, 4337, 4339, 4349, 4357, 4363, 4373, 4391, 4397, 4409,
         4421, 4423, 4441, 4447, 4451, 4457, 4463, 4481, 4483, 4493, 4507, 4513, 4517, 4519, 4523, 4547, 4549, 4561, 4567, 4583,
         4591, 4597, 4603, 4621, 4637, 4639, 4643, 4649, 4651, 4657, 4663, 4673, 4679, 4691, 4703, 4721, 4723, 4729, 4733, 4751,
         4759, 4783, 4787, 4789, 4793, 4799, 4801, 4813, 4817, 4831, 4861, 4871, 4877, 4889, 4903, 4909, 4919, 4931, 4933, 4937,
         4943, 4951, 4957, 4967, 4969, 4973, 4987, 4993, 4999, 5003, 5009, 5011, 5021, 5023, 5039, 5051, 5059, 5077, 5081, 5087,
         5099, 5101, 5107, 5113, 5119, 5147, 5153, 5167, 5171, 5179, 5189, 5197, 5209, 5227, 5231, 5233, 5237, 5261, 5273, 5279,
         5281, 5297, 5303, 5309, 5323, 5333, 5347, 5351, 5381, 5387, 5393, 5399, 5407, 5413, 5417, 5419, 5431, 5437, 5441, 5443,
         5449, 5471, 5477, 5479, 5483, 5501, 5503, 5507, 5519, 5521, 5527, 5531, 5557, 5563, 5569, 5573, 5581, 5591, 5623, 5639,
         5641, 5647, 5651, 5653, 5657, 5659, 5669, 5683, 5689, 5693, 5701, 5711, 5717, 5737, 5741, 5743, 5749, 5779, 5783, 5791,
         5801, 5807, 5813, 5821, 5827, 5839, 5843, 5849, 5851, 5857, 5861, 5867, 5869, 5879, 5881, 5897, 5903, 5923, 5927, 5939,
         5953, 5981, 5987, 6007, 6011, 6029, 6037, 6043, 6047, 6053, 6067, 6073, 6079, 6089, 6091, 6101, 6113, 6121, 6131, 6133,
         6143, 6151, 6163, 6173, 6197, 6199, 6203, 6211, 6217, 6221, 6229, 6247, 6257, 6263, 6269, 6271, 6277, 6287, 6299, 6301,
         6311, 6317, 6323, 6329, 6337, 6343, 6353, 6359, 6361, 6367, 6373, 6379, 6389, 6397, 6421, 6427, 6449, 6451, 6469, 6473,
         6481, 6491, 6521, 6529, 6547, 6551, 6553, 6563, 6569, 6571, 6577, 6581, 6599, 6607, 6619, 6637, 6653, 6659, 6661, 6673,
         6679, 6689, 6691, 6701, 6703, 6709, 6719, 6733, 6737, 6761, 6763, 6779, 6781, 6791, 6793, 6803, 6823, 6827, 6829, 6833,
         6841, 6857, 6863, 6869, 6871, 6883, 6899, 6907, 6911, 6917, 6947, 6949, 6959, 6961, 6967, 6971, 6977, 6983, 6991, 6997,
         7001, 7013, 7019, 7027, 7039, 7043, 7057, 7069, 7079, 7103, 7109, 7121, 7127, 7129, 7151, 7159, 7177, 7187, 7193, 7207,
         7211, 7213, 7219, 7229, 7237, 7243, 7247, 7253, 7283, 7297, 7307, 7309, 7321, 7331, 7333, 7349, 7351, 7369, 7393, 7411,
         7417, 7433, 7451, 7457, 7459, 7477, 7481, 7487, 7489, 7499, 7507, 7517, 7523, 7529, 7537, 7541, 7547, 7549, 7559, 7561,
         7573, 7577, 7583, 7589, 7591, 7603, 7607, 7621, 7639, 7643, 7649, 7669, 7673, 7681, 7687, 7691, 7699, 7703, 7717, 7723,
         7727, 7741, 7753, 7757, 7759, 7789, 7793, 7817, 7823, 7829, 7841, 7853, 7867, 7873, 7877, 7879, 7883, 7901, 7907, 7919];

#[derive(PartialEq, Eq, Clone)]
struct PrimeFactor {
    is_zero: bool,
    is_one: bool,
    factor: [usize; NB_PRIME],
}

impl PrimeFactor {
    fn new(n: usize) -> Self {
        let mut value = n;
        let mut factor: [usize; NB_PRIME] = [0; NB_PRIME];
        for idx in 0..NB_PRIME {
            let prime = PRIME_NUMBERS[idx];
            if prime > value { break }
            while value % PRIME_NUMBERS[idx] == 0 { factor[idx] += 1; value /= prime; }
        }
        Self {
            is_zero: n == 0,
            is_one: n == 1,
            factor: factor,
        }
    }

    fn is_divisible_by(&self, rhs: &Self) -> bool {
        if rhs.is_zero { return false; }
        if rhs.is_one { return true; }
        for idx in 0..NB_PRIME {
            let n = rhs.factor[idx];
            if self.factor[idx] < n {
                return false;
            }
        }
        return true;
    }

    fn to_string(&self) -> String {
        let mut factorization = String::new();
        if self.is_zero {
            factorization += "0";
        }
        else if self.is_one {
            factorization += "1";
        }
        else {
            for idx in 0..NB_PRIME {
                let nb = self.factor[idx];
                if nb == 1 {
                    factorization += &format!("*{}", PRIME_NUMBERS[idx]);
                } else if nb != 0 {
                    factorization += &format!("*{}^{}", PRIME_NUMBERS[idx], nb);
                }
            }
            factorization.remove(0);
        }
        return factorization;
    }

    fn lcm(&self, rhs: &Self) -> PrimeFactor {
        if self.is_zero || rhs.is_zero { return PrimeFactor::new(0) }
        if self.is_one { return rhs.clone() }
        if rhs.is_one { return self.clone() }
        let mut new_factor = PrimeFactor::new(1);
        new_factor.is_one = false;
        for idx in 0..NB_PRIME {
            if self.factor[idx] != 0 && rhs.factor[idx] != 0 {
                new_factor.factor[idx] = usize::max(self.factor[idx], rhs.factor[idx]);
            }
            else {
                new_factor.factor[idx] = 1;
            }
        }
        new_factor
    }

    fn gcd(&self, rhs: &Self) -> PrimeFactor {
        let mut new_factor = PrimeFactor::new(1);
        if self.is_zero || rhs.is_zero { return new_factor }
        if self.is_one || rhs.is_one { return new_factor }
        for idx in 0..NB_PRIME {
            if self.factor[idx] != 0 && rhs.factor[idx] != 0 {
                let nb = usize::min(self.factor[idx], rhs.factor[idx]);
                new_factor.factor[idx] = nb;
                new_factor.is_one = false;
            }
        }
        new_factor
    }
}

impl TryFrom<PrimeFactor> for usize {
    type Error = ();

    fn try_from(value: PrimeFactor) -> Result<Self, Self::Error> {
        if value.is_zero { return Ok(0) }
        if value.is_one { return Ok(1) }
        let mut n: usize = 1;
        for idx in 0..NB_PRIME {
            for _ in 0..value.factor[idx] {
                if let Some(result) = n.checked_mul(PRIME_NUMBERS[idx]) {
                    n = result;
                } else {
                    return Err(());
                }
            }
        }
        return Ok(n)
    }
}

impl TryFrom<&PrimeFactor> for usize {
    type Error = ();

    fn try_from(value: &PrimeFactor) -> Result<Self, Self::Error> {
        if value.is_zero { return Ok(0) }
        if value.is_one { return Ok(1) }
        let mut n: usize = 1;
        for idx in 0..NB_PRIME {
            for _ in 0..value.factor[idx] {
                if let Some(result) = n.checked_mul(PRIME_NUMBERS[idx]) {
                    n = result;
                } else {
                    return Err(());
                }
            }
        }
        return Ok(n)
    }
}

impl TryFrom<&mut PrimeFactor> for usize {
    type Error = ();

    fn try_from(value: &mut PrimeFactor) -> Result<Self, Self::Error> {
        if value.is_zero { return Ok(0) }
        if value.is_one { return Ok(1) }
        let mut n: usize = 1;
        for idx in 0..NB_PRIME {
            for _ in 0..value.factor[idx] {
                if let Some(result) = n.checked_mul(PRIME_NUMBERS[idx]) {
                    n = result;
                } else {
                    return Err(());
                }
            }
        }
        return Ok(n)
    }
}

impl Mul for PrimeFactor {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign for PrimeFactor {
    fn mul_assign(&mut self, rhs: Self) {
        if self.is_zero { return; }
        if rhs.is_zero {
            self.is_zero = true;
            self.is_one = false;
            for idx in 0..NB_PRIME {
                self.factor[idx] = 0;
            }
            return;
        }
        self.is_one &= rhs.is_one;
        if !rhs.is_one {
            for idx in 0..NB_PRIME {
                self.factor[idx] += rhs.factor[idx];
            }
        }
    }
}

impl Div for PrimeFactor {
    type Output = PrimeFactor;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign for PrimeFactor {
    fn div_assign(&mut self, rhs: Self) {
        if rhs.is_zero { panic!("Division by zero !") }
        if self.is_zero || rhs.is_one { return; }
        if self.is_one { *self = PrimeFactor::new(0); return; }
        let mut empty = true;
        for idx in 0..NB_PRIME {
            if let Some(result) = self.factor[idx].checked_sub(rhs.factor[idx]) {
                self.factor[idx] = result;
                if result != 0 { empty = false; }
            } else {
                println!("{} / {}", self, rhs);
                todo!();
            }
        }
        if empty {
            self.is_one = true;
        }
    }
}

impl Add for PrimeFactor {
    type Output = PrimeFactor;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for PrimeFactor {
    fn add_assign(&mut self, mut rhs: Self) {
        let gcd = self.gcd(&rhs);
        rhs /= gcd.clone();
        *self /= gcd.clone();
        let lhs: usize = self.try_into().unwrap();
        let rhs: usize = rhs.try_into().unwrap();
        *self = PrimeFactor::new(lhs + rhs) * gcd;
    }
}

impl Debug for PrimeFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimeFactor").field("factor", &self.to_string()).finish()
    }
}

impl Display for PrimeFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}










struct Monkey {
    name: String,
    items: VecDeque<PrimeFactor>,
    operation: Box<dyn Fn(PrimeFactor) -> PrimeFactor>,
    test: Box<dyn Fn(&PrimeFactor) -> bool>,
    action: Box<dyn Fn(&mut[Monkey], bool, PrimeFactor) -> ()>,
    activity: usize,
}

impl Monkey
{
    fn new( name:       &str,
            items:      VecDeque<PrimeFactor>,
            operation:  Box<dyn Fn(PrimeFactor) -> PrimeFactor>,
            test:       Box<dyn Fn(&PrimeFactor) -> bool>,
            action:     Box<dyn Fn(&mut[Monkey], bool, PrimeFactor) -> ()>) -> Self
    {
        Self { name: name.to_string(), items, operation, test, action, activity: 0 }
    }

    fn run(&mut self, monkeys: &mut[Monkey]) {
        while let Some(mut worry_level) = self.items.pop_front() {
            self.activity += 1;
            worry_level = (self.operation)(worry_level);
            let result = (self.test)(&worry_level);
            (self.action)(monkeys, result, worry_level);
        }
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey").field("name", &self.name).field("items", &self.items).field("activity", &self.activity).finish()
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.activity)
    }
}

#[cfg(test)]
mod tests {
    use crate::PrimeFactor;

    #[test]
    fn test_prime_factor_divisibility() {
        let mut bignum = PrimeFactor::new(20);
        println!("bignum: {}", bignum);
        let add: usize = 4;
        bignum += PrimeFactor::new(add);
        println!("bignum + {} = {}", add, bignum);
        println!("is divisible by 23 ? {}", bignum.is_divisible_by(&PrimeFactor::new(23)));
    }
}

fn main() {
    // let args : Vec<_> = env::args().collect();
    // let input_path = &args[1];
    let input_path = "src/input.txt";
    let raw_input = fs::read(input_path).unwrap();
    let raw_string = String::from_utf8_lossy(&raw_input);
    let input : Vec<_> = raw_string.lines().collect();

    let mut monkeys = Vec::<Monkey>::new();

    let mut line_iterator = input.iter();
    let mut next_line = line_iterator.next();
    while let Some(line) = next_line {
        // Read Monkey name
        if !line.starts_with("Monkey") { panic!("Expected line to start with \"Monkey\", got \"{}\"", line) }
        let name = &line[0..line.len()-1];
        
        // Read starting worry levels
        next_line = line_iterator.next();
        let line = next_line.unwrap().trim();
        if !line.starts_with("Starting items:") { panic!("Expected line to start with \"Starting items:\", got \"{}\"", line) }
        let worry_levels = line[15..].trim();
        let mut items = VecDeque::<PrimeFactor>::new();
        for worry_level in worry_levels.split(", ") {
            let n = worry_level.parse().unwrap();
            items.push_back(PrimeFactor::new(n));
        }
        
        // Read operation
        next_line = line_iterator.next();
        let line = next_line.unwrap().trim();
        if !line.starts_with("Operation:") { panic!("Expected line to start with \"Operation:\", got \"{}\"", line) }
        let operation_text = line[10..].trim();
        if !operation_text.starts_with("new = old") { panic!("Expected operation to start with \"new = old\", got \"{}\"", operation_text) }
        let operation_text = operation_text[9..].trim();
        let operator = operation_text.split_whitespace().nth(0).unwrap();
        let operand = operation_text.split_whitespace().nth(1).unwrap();
        let operation: Box<dyn Fn(PrimeFactor)->PrimeFactor> = match operand {
            "old" => {
                match operator {
                    "*" => Box::new(move |old: PrimeFactor| -> PrimeFactor { old.clone() * old.clone() }),
                    "+" => Box::new(move |old: PrimeFactor| -> PrimeFactor { old.clone() + old.clone() }),
                    _ => panic!("Unexpected operator \"{}\" in Operation", operator),
                }
            },
            number => {
                let n = PrimeFactor::new(number.parse().unwrap());
                match operator {
                    "*" => Box::new(move |old: PrimeFactor| -> PrimeFactor { old * n.clone() }),
                    "+" => Box::new(move |old: PrimeFactor| -> PrimeFactor { old + n.clone() }),
                    _ => panic!("Unexpected operator \"{}\" in Operation", operator),
                }
            }
        };
        
        // Read test
        next_line = line_iterator.next();
        let line = next_line.unwrap().trim();
        if !line.starts_with("Test:") { panic!("Expected line to start with \"Test:\", got \"{}\"", line) }
        let test_text = line[5..].trim();
        if !test_text.starts_with("divisible by") { panic!("Expected test to start with \"divisible by\", got \"{}\"", test_text) }
        let n = test_text[12..].trim().parse::<usize>().unwrap();
        let bignum = PrimeFactor::new(n);
        let test: Box<dyn Fn(&PrimeFactor)->bool> = Box::new(move |worry_level| worry_level.is_divisible_by(&bignum));
        
        // Read action
        next_line = line_iterator.next();
        let line = next_line.unwrap().trim();
        if !line.starts_with("If true: throw to monkey") { panic!("Expected line to start with \"If true: throw to monkey\", got \"{}\"", line) }
        let idx_if_true = line[24..].trim().parse::<usize>().unwrap();

        next_line = line_iterator.next();
        let line = next_line.unwrap().trim();
        if !line.starts_with("If false: throw to monkey") { panic!("Expected line to start with \"If false: throw to monkey\", got \"{}\"", line) }
        let idx_if_false = line[25..].trim().parse::<usize>().unwrap();

        let action: Box<dyn Fn(&mut[Monkey], bool, PrimeFactor)->()> = Box::new(move |monkeys, result, worry_level| {
            if result {
                println!("Throw to Monkey {}", idx_if_true);
                monkeys[idx_if_true].items.push_back(worry_level);
            } else {
                println!("Throw to Monkey {}", idx_if_false);
                monkeys[idx_if_false].items.push_back(worry_level);
            }
        });

        monkeys.push(Monkey::new(name, items, operation, test, action));

        // Read next line
        line_iterator.next();
        next_line = line_iterator.next();
    }

    // Run simulation
    for _round in 0..20 {
        println!("======= Round {} =======", _round+1);
        for monkey_idx in 0..monkeys.len() {
            println!("---- Monkey {} ----", monkey_idx);
            println!("Before: {:?}", monkeys[monkey_idx]);
            let monkey: *mut Monkey = &mut monkeys[monkey_idx];
            unsafe { monkey.as_mut().unwrap() }.run(&mut monkeys);
            println!("After : {:?}", monkeys[monkey_idx]);
        }
    }
    println!("");

    monkeys.iter().for_each(|monkey| {
        println!("{} activity: {}", monkey.name, monkey.activity);
    });
    println!("");

    // Find the monkeys with the highest activity
    monkeys.sort_unstable_by_key(|monkey| monkey.activity);

    let result = monkeys.iter().rev().take(2).fold(1, |state, monkey| {
        println!("{} activity: {}", monkey.name, monkey.activity);
        state * monkey.activity
    });
    println!("Result: {}", result)
}
