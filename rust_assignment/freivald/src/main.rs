// TODO: Import necessary libraries. Check cargo.toml and the documentation of the libraries.
struct Freivald {
    x: // Array/Vec of Fq,
}

impl Freivald {
    // TODO: Create constructor for object
    fn new(array_size: usize) -> Self {
        todo!()
        // Generate random number
        // Populate vector with values r^i for i=0..matrix_size
        // Return freivald value with this vector as its x value
    }

    // TODO: Add proper types to input matrices. Remember matrices should hold Fq values
    fn verify(&self, matrix_a, matrix_b, supposed_ab) -> bool {
        assert!(check_matrix_dimensions(matrix_a, matrix_b, supposed_ab));
        todo!()
        // TODO: check if a * b * x == c * x. Check algorithm to make sure order of operations are
        // correct
    }

    // utility function to not have to instantiate Freivalds if you just want to make one
    // verification.
    // TODO: Add types for arguments
    fn verify_once(matrix_a, matrix_b, supposed_ab) -> bool {
        let freivald = Freivald::new(supposed_ab.nrows());
        freivald.verify(matrix_a, matrix_b, supposed_ab)
    }
}
// TODO: [Bonus] Modify code to increase your certainty that A * B == C by iterating over the protocol.
// Note that you need to generate new vectors for new iterations or you'll be recomputing same
// value over and over. No problem in changing data structures used by the algorithm (currently its a struct
// but that can change if you want to)


// You can either do a test on main or just remove main function and rename this file to lib.rs to remove the
// warning of not having a main implementation
fn main() {
    todo!()
}

// TODO: Add proper types to input matrices. Remember matrices should hold Fq values
pub fn check_matrix_dimensions(matrix_a, matrix_b, supposed_ab) -> bool {
    // TODO: Check if dimensions of making matrix_a * matrix_b matches values in supposed_ab.
    // If it doesn't you know its not the correct result independently of matrix contents
    todo!()
}

#[cfg(test)]
mod tests {
    // #[macro_use]
    use lazy_static::lazy_static;
    use rstest::rstest;

    use super::*;

    lazy_static! {
        todo!("add matrices types and values")
        static ref MATRIX_A: /* Type of matrix. Values should be fq */ = /* arbitrary matrix */;
        static ref MATRIX_A_DOT_A: /* Type of matrix. Values should be fq */ = /* Correct result of A * A */;
        static ref MATRIX_B: /* Type of matrix. Values should be fq */ = /* arbitrary matrix */;
        static ref MATRIX_B_DOT_B: /* Type of matrix. Values should be fq */ = /* Correct result of B * B */;
        static ref MATRIX_C: /* Type of matrix. Values should be fq */ = /* arbitrary LARGE matrix (at least 200, 200)*/;
        static ref MATRIX_C_DOT_C: /* Type of matrix. Values should be fq */ = /* Correct result of C * C */;
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_A, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_B, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_C, &MATRIX_C_DOT_C)]
    fn freivald_verify_success_test(
        #[case] matrix_a: /* Type of matrix. Values should be fq */,
        #[case] matrix_b: /* Type of matrix. Values should be fq */,
        #[case] supposed_ab: /* Type of matrix. Values should be fq */,
    ) {
        let freivald = Freivald::new(supposed_ab.nrows());
        assert!(freivald.verify(matrix_a, matrix_b, supposed_ab));
    }

    #[rstest]
    #[case(&MATRIX_A, &MATRIX_B, &MATRIX_A_DOT_A)]
    #[case(&MATRIX_B, &MATRIX_A, &MATRIX_B_DOT_B)]
    #[case(&MATRIX_C, &MATRIX_B, &MATRIX_C_DOT_C)]
    fn freivald_verify_fail_test(
        #[case] a: /* Type of matrix. Values should be fq */,
        #[case] b: /* Type of matrix. Values should be fq */,
        #[case] c: /* Type of matrix. Values should be fq */,
    ) {
        let freivald = Freivald::new(c.nrows());
        assert!(!freivald.verify(a, b, c));
    }
}
