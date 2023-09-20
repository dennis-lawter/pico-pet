struct Lcg {
    state: u32,
}

impl Lcg {
    const M: u32 = (1 << 12);
    const A: u32 = 69069;
    const C: u32 = 1;

    // const M: u32 = (1 << 16) + 1;
    // const A: u32 = 75;
    // const C: u32 = 74;

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

fn find_period(rng: &mut Lcg) -> usize {
    let mut tortoise = rng.next();
    let mut hare = rng.next();
    hare = rng.next(); // Move hare two steps ahead

    // Phase 1: Detect a repetition
    while tortoise != hare {
        tortoise = rng.next();
        hare = rng.next();
        hare = rng.next();
    }

    // Phase 2: Find the length of the shortest cycle
    let mut length = 1;
    hare = rng.next();
    while tortoise != hare {
        hare = rng.next();
        length += 1;
    }

    length
}

fn main() {
    let mut rng = Lcg::new(1);
    const ITERATIONS: usize = 1 << 16;
    let mut counts = [0u32; 256]; // array to store counts of each byte

    for _ in 0..ITERATIONS {
        let byte = rng.next();
        counts[byte as usize] += 1;
    }

    for (byte, &count) in counts.iter().enumerate() {
        println!("Byte {:#04x}: {}", byte, count);
    }

    println!("================================");

    let period = find_period(&mut rng);
    println!("The period of the LCG is: {}", period);
}
