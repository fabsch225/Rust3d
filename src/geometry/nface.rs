use crate::math::vector::NVector;

struct NFace {
    v_count: usize,
    a: NVector,
    b: NVector,
    c: NVector,
    d: NVector,
}