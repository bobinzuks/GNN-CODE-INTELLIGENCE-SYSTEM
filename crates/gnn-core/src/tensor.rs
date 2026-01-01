//! Basic tensor operations for GNN computations
//! WASM-compatible pure Rust implementation without external ML frameworks

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tensor {
    pub data: Vec<f32>,
    pub shape: Vec<usize>,
}

impl Tensor {
    /// Create a new tensor with given shape, initialized to zeros
    pub fn zeros(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        Self {
            data: vec![0.0; size],
            shape,
        }
    }

    /// Create a new tensor with given shape, initialized to ones
    pub fn ones(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        Self {
            data: vec![1.0; size],
            shape,
        }
    }

    /// Create a tensor from raw data and shape
    pub fn from_vec(data: Vec<f32>, shape: Vec<usize>) -> Result<Self, String> {
        let expected_size: usize = shape.iter().product();
        if data.len() != expected_size {
            return Err(format!(
                "Data length {} does not match shape {:?} (expected {})",
                data.len(),
                shape,
                expected_size
            ));
        }
        Ok(Self { data, shape })
    }

    /// Create a tensor initialized with random values using Xavier initialization
    pub fn xavier_uniform(shape: Vec<usize>, rng: &mut impl rand::Rng) -> Self {
        let size: usize = shape.iter().product();
        let fan_in = if shape.len() >= 2 { shape[shape.len() - 2] } else { 1 };
        let fan_out = if shape.len() >= 1 { shape[shape.len() - 1] } else { 1 };
        let limit = (6.0 / (fan_in + fan_out) as f32).sqrt();

        let data: Vec<f32> = (0..size)
            .map(|_| rng.gen::<f32>() * 2.0 * limit - limit)
            .collect();

        Self { data, shape }
    }

    /// Get the number of dimensions
    pub fn ndim(&self) -> usize {
        self.shape.len()
    }

    /// Get the total number of elements
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Get element at index (for 2D tensors)
    pub fn get(&self, i: usize, j: usize) -> Result<f32, String> {
        if self.shape.len() != 2 {
            return Err("get() only works for 2D tensors".to_string());
        }
        let idx = i * self.shape[1] + j;
        self.data.get(idx).copied().ok_or_else(|| "Index out of bounds".to_string())
    }

    /// Set element at index (for 2D tensors)
    pub fn set(&mut self, i: usize, j: usize, value: f32) -> Result<(), String> {
        if self.shape.len() != 2 {
            return Err("set() only works for 2D tensors".to_string());
        }
        let idx = i * self.shape[1] + j;
        if idx < self.data.len() {
            self.data[idx] = value;
            Ok(())
        } else {
            Err("Index out of bounds".to_string())
        }
    }

    /// Matrix multiplication (2D only)
    pub fn matmul(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape.len() != 2 || other.shape.len() != 2 {
            return Err("matmul requires 2D tensors".to_string());
        }

        let (m, k1) = (self.shape[0], self.shape[1]);
        let (k2, n) = (other.shape[0], other.shape[1]);

        if k1 != k2 {
            return Err(format!(
                "Incompatible shapes for matmul: {:?} and {:?}",
                self.shape, other.shape
            ));
        }

        let mut result = Tensor::zeros(vec![m, n]);

        for i in 0..m {
            for j in 0..n {
                let mut sum = 0.0;
                for k in 0..k1 {
                    sum += self.data[i * k1 + k] * other.data[k * n + j];
                }
                result.data[i * n + j] = sum;
            }
        }

        Ok(result)
    }

    /// Element-wise addition
    pub fn add(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape != other.shape {
            return Err(format!(
                "Incompatible shapes for add: {:?} and {:?}",
                self.shape, other.shape
            ));
        }

        let data: Vec<f32> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();

        Ok(Tensor {
            data,
            shape: self.shape.clone(),
        })
    }

    /// Element-wise subtraction
    pub fn sub(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape != other.shape {
            return Err(format!(
                "Incompatible shapes for sub: {:?} and {:?}",
                self.shape, other.shape
            ));
        }

        let data: Vec<f32> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();

        Ok(Tensor {
            data,
            shape: self.shape.clone(),
        })
    }

    /// Element-wise multiplication
    pub fn mul(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape != other.shape {
            return Err(format!(
                "Incompatible shapes for mul: {:?} and {:?}",
                self.shape, other.shape
            ));
        }

        let data: Vec<f32> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a * b)
            .collect();

        Ok(Tensor {
            data,
            shape: self.shape.clone(),
        })
    }

    /// Scalar multiplication
    pub fn scale(&self, scalar: f32) -> Tensor {
        let data: Vec<f32> = self.data.iter().map(|x| x * scalar).collect();
        Tensor {
            data,
            shape: self.shape.clone(),
        }
    }

    /// ReLU activation
    pub fn relu(&self) -> Tensor {
        let data: Vec<f32> = self.data.iter().map(|x| x.max(0.0)).collect();
        Tensor {
            data,
            shape: self.shape.clone(),
        }
    }

    /// Leaky ReLU activation
    pub fn leaky_relu(&self, alpha: f32) -> Tensor {
        let data: Vec<f32> = self.data.iter()
            .map(|x| if *x > 0.0 { *x } else { alpha * x })
            .collect();
        Tensor {
            data,
            shape: self.shape.clone(),
        }
    }

    /// Sigmoid activation
    pub fn sigmoid(&self) -> Tensor {
        let data: Vec<f32> = self.data.iter()
            .map(|x| 1.0 / (1.0 + (-x).exp()))
            .collect();
        Tensor {
            data,
            shape: self.shape.clone(),
        }
    }

    /// Tanh activation
    pub fn tanh(&self) -> Tensor {
        let data: Vec<f32> = self.data.iter().map(|x| x.tanh()).collect();
        Tensor {
            data,
            shape: self.shape.clone(),
        }
    }

    /// Softmax activation (along last dimension)
    pub fn softmax(&self) -> Result<Tensor, String> {
        if self.shape.is_empty() {
            return Err("Cannot apply softmax to scalar".to_string());
        }

        let last_dim = self.shape[self.shape.len() - 1];
        let batch_size = self.data.len() / last_dim;
        let mut result = self.clone();

        for b in 0..batch_size {
            let offset = b * last_dim;
            let slice = &self.data[offset..offset + last_dim];

            // Numerical stability: subtract max
            let max_val = slice.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            let exp_sum: f32 = slice.iter().map(|x| (x - max_val).exp()).sum();

            for i in 0..last_dim {
                result.data[offset + i] = (slice[i] - max_val).exp() / exp_sum;
            }
        }

        Ok(result)
    }

    /// Transpose (2D only)
    pub fn transpose(&self) -> Result<Tensor, String> {
        if self.shape.len() != 2 {
            return Err("transpose only works for 2D tensors".to_string());
        }

        let (m, n) = (self.shape[0], self.shape[1]);
        let mut result = Tensor::zeros(vec![n, m]);

        for i in 0..m {
            for j in 0..n {
                result.data[j * m + i] = self.data[i * n + j];
            }
        }

        Ok(result)
    }

    /// Sum along dimension
    pub fn sum(&self, dim: usize, keep_dim: bool) -> Result<Tensor, String> {
        if dim >= self.shape.len() {
            return Err(format!("Dimension {} out of range for shape {:?}", dim, self.shape));
        }

        let mut new_shape = self.shape.clone();
        if keep_dim {
            new_shape[dim] = 1;
        } else {
            new_shape.remove(dim);
        }

        let outer_size: usize = self.shape[..dim].iter().product();
        let dim_size = self.shape[dim];
        let inner_size: usize = self.shape[dim + 1..].iter().product();

        let result_size = if new_shape.is_empty() { 1 } else { new_shape.iter().product() };
        let mut result_data = vec![0.0; result_size];

        for outer in 0..outer_size {
            for inner in 0..inner_size {
                let mut sum = 0.0;
                for d in 0..dim_size {
                    let idx = outer * dim_size * inner_size + d * inner_size + inner;
                    sum += self.data[idx];
                }
                let result_idx = outer * inner_size + inner;
                result_data[result_idx] = sum;
            }
        }

        Ok(Tensor {
            data: result_data,
            shape: new_shape,
        })
    }

    /// Mean along dimension
    pub fn mean(&self, dim: usize, keep_dim: bool) -> Result<Tensor, String> {
        let dim_size = self.shape[dim] as f32;
        let sum = self.sum(dim, keep_dim)?;
        Ok(sum.scale(1.0 / dim_size))
    }

    /// L2 normalization
    pub fn l2_normalize(&self, dim: usize, epsilon: f32) -> Result<Tensor, String> {
        if dim >= self.shape.len() {
            return Err(format!("Dimension {} out of range for shape {:?}", dim, self.shape));
        }

        // Compute squared values
        let squared = self.mul(self)?;

        // Sum along dimension and keep dimension
        let sum_squared = squared.sum(dim, true)?;

        // Add epsilon and take sqrt
        let norm_data: Vec<f32> = sum_squared.data.iter()
            .map(|x| (x + epsilon).sqrt())
            .collect();
        let norm = Tensor {
            data: norm_data,
            shape: sum_squared.shape.clone(),
        };

        // Broadcast division
        self.broadcast_div(&norm)
    }

    /// Broadcast division (for normalization)
    fn broadcast_div(&self, other: &Tensor) -> Result<Tensor, String> {
        let mut result = self.clone();

        if self.shape == other.shape {
            for i in 0..result.data.len() {
                result.data[i] /= other.data[i];
            }
            return Ok(result);
        }

        // Simple broadcasting for normalization case
        let other_size = other.data.len();
        for i in 0..result.data.len() {
            let broadcast_idx = i % other_size;
            result.data[i] /= other.data[broadcast_idx];
        }

        Ok(result)
    }

    /// Concatenate tensors along dimension 0
    pub fn concat(tensors: &[Tensor], dim: usize) -> Result<Tensor, String> {
        if tensors.is_empty() {
            return Err("Cannot concatenate empty list of tensors".to_string());
        }

        if dim != 0 {
            return Err("Only concatenation along dimension 0 is currently supported".to_string());
        }

        let first_shape = &tensors[0].shape;
        if first_shape.is_empty() {
            return Err("Cannot concatenate scalar tensors".to_string());
        }

        // Verify all tensors have compatible shapes
        for tensor in tensors.iter().skip(1) {
            if tensor.shape.len() != first_shape.len() {
                return Err("All tensors must have same number of dimensions".to_string());
            }
            for (i, (&s1, &s2)) in first_shape.iter().zip(tensor.shape.iter()).enumerate() {
                if i != dim && s1 != s2 {
                    return Err(format!(
                        "Incompatible shapes for concatenation: {:?} and {:?}",
                        first_shape, tensor.shape
                    ));
                }
            }
        }

        let mut new_shape = first_shape.clone();
        new_shape[dim] = tensors.iter().map(|t| t.shape[dim]).sum();

        let mut result_data = Vec::new();
        for tensor in tensors {
            result_data.extend_from_slice(&tensor.data);
        }

        Ok(Tensor {
            data: result_data,
            shape: new_shape,
        })
    }

    /// Reshape tensor
    pub fn reshape(&self, new_shape: Vec<usize>) -> Result<Tensor, String> {
        let new_size: usize = new_shape.iter().product();
        if new_size != self.data.len() {
            return Err(format!(
                "Cannot reshape tensor of size {} to shape {:?} (size {})",
                self.data.len(),
                new_shape,
                new_size
            ));
        }

        Ok(Tensor {
            data: self.data.clone(),
            shape: new_shape,
        })
    }

    /// Get a row from a 2D tensor
    pub fn get_row(&self, row: usize) -> Result<Tensor, String> {
        if self.shape.len() != 2 {
            return Err("get_row only works for 2D tensors".to_string());
        }

        if row >= self.shape[0] {
            return Err(format!("Row index {} out of bounds for shape {:?}", row, self.shape));
        }

        let ncols = self.shape[1];
        let start = row * ncols;
        let end = start + ncols;

        Ok(Tensor {
            data: self.data[start..end].to_vec(),
            shape: vec![ncols],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_creation() {
        let t = Tensor::zeros(vec![2, 3]);
        assert_eq!(t.shape, vec![2, 3]);
        assert_eq!(t.data.len(), 6);
        assert!(t.data.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn test_matmul() {
        let a = Tensor::from_vec(vec![1.0, 2.0, 3.0, 4.0], vec![2, 2]).unwrap();
        let b = Tensor::from_vec(vec![5.0, 6.0, 7.0, 8.0], vec![2, 2]).unwrap();
        let c = a.matmul(&b).unwrap();

        assert_eq!(c.shape, vec![2, 2]);
        assert_eq!(c.data, vec![19.0, 22.0, 43.0, 50.0]);
    }

    #[test]
    fn test_relu() {
        let t = Tensor::from_vec(vec![-1.0, 0.0, 1.0, 2.0], vec![4]).unwrap();
        let r = t.relu();
        assert_eq!(r.data, vec![0.0, 0.0, 1.0, 2.0]);
    }

    #[test]
    fn test_softmax() {
        let t = Tensor::from_vec(vec![1.0, 2.0, 3.0], vec![3]).unwrap();
        let s = t.softmax().unwrap();
        let sum: f32 = s.data.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }
}
