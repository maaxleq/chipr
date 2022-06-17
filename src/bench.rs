use crate::chip8::VM;

pub struct Bench {
    pub vm: VM,
    pub max_cycles: usize,
    pub duration: std::time::Duration,
    pub score: usize
}

impl Bench {
    pub fn new(vm: VM) -> Self {
        Self {
            vm,
            max_cycles: 100_000_000,
            duration: std::time::Duration::from_secs(0),
            score: 0
        }
    }

    pub fn test(&mut self) {
        self.score = 0;
        self.duration = std::time::Duration::from_secs(0);

        let mut cycles = 0;
        let start = std::time::Instant::now();

        loop {
            if self.vm.next() == 0 || cycles >= self.max_cycles{
                break;
            };

            cycles += 1;
        }

        let end = std::time::Instant::now();

        self.duration = end.duration_since(start);
        self.score = cycles;
    }

    pub fn print_results(&self){
        println!("Cycles:\t\t{}", self.score);
        println!("Duration (ms):\t{}", self.duration.as_millis());
        println!("Cycles / s:\t{}", (self.score as f64 / (self.duration.as_millis() as f64 / 1000.0)) as usize);
    }
}
