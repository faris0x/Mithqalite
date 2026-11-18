use mithqalite_base::num::basic::integers::PrimitiveInt;
use mithqalite_nz::platform::Limb;

fn main() {
    let i = std::hint::black_box(100u64);
    // Remainder by a type's bit width: flagged.
    let _ = i % Limb::WIDTH;
    let _ = i % u64::WIDTH;
    // A remainder by a non-width modulus: fine.
    let _ = i % 10;
}
