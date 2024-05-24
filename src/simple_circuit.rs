#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate bellman;
extern crate pairing;
extern crate rand;

use self::pairing::{Engine, Field, PrimeField};
use self::bellman::{Circuit, ConstraintSystem, SynthesisError};

pub struct SimpleCircuit<E: Engine> {
    pub a: Option<E::Fr>,
    pub b: Option<E::Fr>,
    pub c: Option<E::Fr>,
}

impl<E: Engine> Circuit<E> for SimpleCircuit<E> {
    fn synthesize<CS: ConstraintSystem<E>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let a = cs.alloc(
            || "a",
            || self.a.ok_or(SynthesisError::AssignmentMissing),
        )?;
        let b = cs.alloc(
            || "b",
            || self.b.ok_or(SynthesisError::AssignmentMissing),
        )?;
        let c = cs.alloc_input(
            || "c",
            || self.c.ok_or(SynthesisError::AssignmentMissing),
        )?;

        cs.enforce(
            || "mult",
            |lc| lc + a,
            |lc| lc + b,
            |lc| lc + c,
        );

        Ok(())
    }
}