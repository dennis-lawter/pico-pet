const SEED_SAMPLE_SIZE: usize = 86400;

/// Profiles the Linear Congruential Generator
fn main() {
    let mut rng = Lcg::new(1);
    const ITERATIONS: usize = 1 << 16;
    let mut counts = [0u32; 256];

    for _ in 0..ITERATIONS {
        let byte = rng.next();
        counts[byte as usize] += 1;
    }

    for (byte, &count) in counts.iter().enumerate() {
        println!("Byte {:#04x}: {}", byte, count);
    }

    println!("================================");

    let mut cycles = [0usize; SEED_SAMPLE_SIZE];

    for i in 0..SEED_SAMPLE_SIZE {
        let mut rng1 = Lcg::new(i as u32);
        let mut rng2 = Lcg::new(i as u32);

        let period = find_period(&mut rng1, &mut rng2);
        cycles[i] = period;
    }

    let mut shortest_cycle = usize::MAX;
    let mut shortest_seed = 0usize;
    let mut longest_cycle = 0usize;
    let mut longest_seed = 0usize;

    for i in 0..SEED_SAMPLE_SIZE {
        if cycles[i] < shortest_cycle {
            shortest_seed = i;
            shortest_cycle = cycles[i];
        }
        if cycles[i] > longest_cycle {
            longest_seed = i;
            longest_cycle = cycles[i];
        }
    }

    let average_cycle: usize = cycles.iter().sum::<usize>() / SEED_SAMPLE_SIZE;

    println!(
        "The shortest period of the LCG is: {} @ seed {}",
        shortest_cycle, shortest_seed
    );
    println!(
        "The longest period of the LCG is:  {} @ seed {}",
        longest_cycle, longest_seed
    );
    println!("The average period of the LCG is:  {}", average_cycle);

    println!("================================");
    println!("Worst seed:");

    let mut worst_rng = Lcg::new(shortest_seed as u32);
    for i in 0..256 {
        print!("{:#04x}\t", worst_rng.next());
        if i % 8 == 7 {
            println!("");
        }
    }
}

struct Lcg {
    state: u32,
}

impl Lcg {
    const M: u32 = (1 << 16) + 1;
    const A: u32 = 75;
    const C: u32 = 74;

    pub fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    pub fn next(&mut self) -> u8 {
        let next = self.state * Self::A + Self::C;
        let next = next % Self::M;
        self.state = next;

        // isolate bits
        let output = (next >> 0) & 0xFF;

        output as u8
    }
}

fn find_period(rng1: &mut Lcg, rng2: &mut Lcg) -> usize {
    let mut tortoise = rng1.next();

    rng2.next();
    let mut hare = rng2.next();

    // Phase 1: Detect a repetition
    while tortoise != hare {
        tortoise = rng1.next();
        rng2.next();
        hare = rng2.next();
    }

    // Phase 2: Find the length of the cycle
    let mut length = 1;
    hare = rng2.next();
    while tortoise != hare {
        hare = rng2.next();
        length += 1;
    }

    length
}
