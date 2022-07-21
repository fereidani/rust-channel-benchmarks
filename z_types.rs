trait BenchType: Send + Sync {
    fn new(v: usize) -> Self;
    // ensure that received data is tested and used to avoid compiler optimizer removing receive part.
    fn test(&self);
}

struct BenchEmpty {}

struct BenchUsize {
    a: usize,
}

struct BenchFixedArray([usize; 4]);

struct BenchBoxed(Box<BenchFixedArray>);

impl BenchType for BenchEmpty {
    fn new(_v: usize) -> Self {
        Self {}
    }
    #[inline(always)]
    fn test(&self) {}
}

impl BenchType for BenchUsize {
    fn new(v: usize) -> Self {
        return Self { a: v };
    }
    #[inline(always)]
    fn test(&self) {
        if self.a < 1 {
            panic!("invalid_value")
        }
    }
}

impl BenchType for BenchFixedArray {
    fn new(v: usize) -> Self {
        return BenchFixedArray([v; 4]);
    }

    #[inline(always)]
    fn test(&self) {
        // only to ensure result is being used and compiler does not remove bench type
        if self.0[0] < 1 {
            panic!("invalid_value")
        }
    }
}

impl BenchType for BenchBoxed {
    fn new(v: usize) -> Self {
        return BenchBoxed(BenchFixedArray([v; 4]).into());
    }

    #[inline(always)]
    fn test(&self) {
        // only to ensure result is being used and compiler does not remove bench type
        if self.0 .0[0] < 1 {
            panic!("invalid_value")
        }
    }
}
