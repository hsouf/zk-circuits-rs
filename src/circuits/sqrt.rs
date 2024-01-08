use bellman::{Circuit, ConstraintSystem, SynthesisError};
use ff::PrimeField;
pub struct SquareRootCircuit<Scalar> {
    pub x: Option<Scalar>,
    pub root: Option<Scalar>,
}

impl<Scalar: PrimeField> Circuit<Scalar> for SquareRootCircuit<Scalar> {
    fn synthesize<CS: ConstraintSystem<Scalar>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let x = cs.alloc_input(|| "x", || self.x.ok_or(SynthesisError::AssignmentMissing))?;
        let root = cs.alloc(
            || "root",
            || self.root.ok_or(SynthesisError::AssignmentMissing),
        )?;

        // root * root = x
        cs.enforce(|| "square", |lc| lc + root, |lc| lc + root, |lc| lc + x);
        Ok(())
    }
}
