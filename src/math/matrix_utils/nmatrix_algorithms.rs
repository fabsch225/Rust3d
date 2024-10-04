use crate::math::matrix::NMatrix;
use crate::math::vector::NVector;

impl NMatrix {
    pub fn gauss_single_thread(b: &NVector, x: &NMatrix) -> Option<NVector> {
        let mut b = b.clone();
        let mut x = x.clone();
        let n = b.n;

        for i in 0..n {
            let mut max_row = i;
            for k in (i+1)..n {
                if x.get(k, i).abs() > x.get(max_row, i).abs() {
                    max_row = k;
                }
            }

            for k in 0..n {
                let temp = x.get(i, k);
                x.set(i, k, x.get(max_row, k));
                x.set(max_row, k, temp);
            }
            let temp_b = b.get(i);
            b.set(i, b.get(max_row));
            b.set(max_row, temp_b);

            for k in (i+1)..n {
                let factor = x.get(k, i) / x.get(i, i);
                b.set(k, b.get(k) - factor * b.get(i));
                for j in i..n {
                    x.set(k, j, x.get(k, j) - factor * x.get(i, j));
                }
            }
        }

        let mut result = NVector::new(n, vec![0.0; n]);
        for i in (0..n).rev() {
            let mut sum = 0.0;
            let value = x.get(i, i);
            if value == 0.0 {
                return None;
            }
            for j in (i+1)..n {
                sum += x.get(i, j) * result.get(j);
            }
            result.set(i, (b.get(i) - sum) / value);
        }

        Some(result)
    }
}