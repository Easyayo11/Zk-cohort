use ark_ff::PrimeField;
use sha2::digest::KeyInit;
use std::marker::PhantomData;

use super::multilinear_polynomial::{self, MultilinearPoly};

pub enum Operator {
    Add,
    Mul
}

pub struct Gate {
    pub left_index: usize,
    pub right_index: usize,
    pub output_index: usize,
    pub operator: Operator
}

pub struct Layer {
    pub gates: Vec<Gate>
}

pub struct Circuit<F: PrimeField> {
    pub layers: Vec<Layer>,
    pub layer_evaluations: Vec<Vec<F>>, 
    _phantom: PhantomData<F>
}

pub struct ProductPoly<F: PrimeField> {
    pub evaluation: Vec<MultilinearPoly<F>>
}

pub struct SumPoly<F: PrimeField> {
    pub polys: Vec<ProductPoly<F>>
}

// Gate Implementation 
impl Gate {
    pub fn new(left_index: usize, right_index: usize, output_index: usize, operator: Operator) -> Self {
        Self {
            left_index,
            right_index,
            output_index,
            operator
        }
    }
}
//  Layer implementation
impl Layer {
    pub fn new(gates: Vec<Gate>) -> Self {
        Self {
            gates
        }
    }
}

// Circuit Implementation 
impl <F: PrimeField>Circuit<F> {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self {
            layers,
            layer_evaluations: Vec::new(),
            _phantom: PhantomData
        }
    }

    pub fn evaluate(&mut self, values: Vec<F>) -> Vec<F> {
        let mut current_input = values;

        let mut reversed_evaluations = Vec::new();
        reversed_evaluations.push(current_input.clone());
        for layer in self.layers.iter().rev() {
            let max_output_index = layer.gates.iter()
                .map(|gate| gate.output_index)
                .max()
                .unwrap_or(0);

            let mut resultant_evaluations = vec![F::zero(); max_output_index + 1];
            for gate in layer.gates.iter() {
                let left_index_value = current_input[gate.left_index];
                let right_index_value = current_input[gate.right_index];

                let current_gate_evaluation = match gate.operator {
                    Operator::Add => left_index_value + right_index_value,
                    Operator::Mul => left_index_value * right_index_value
                };
                resultant_evaluations[gate.output_index] = current_gate_evaluation;
            }

            current_input = resultant_evaluations;
            reversed_evaluations.push(current_input.clone());
        }

        reversed_evaluations.reverse();
        self.layer_evaluations = reversed_evaluations;

        self.layer_evaluations[0].clone()
    }

    pub fn w_i_polynomial(&self, layer_index: usize) -> MultilinearPoly<F> {
        assert!(layer_index < self.layer_evaluations.len(), "layer index out of bounds");

        MultilinearPoly::new(self.layer_evaluations[layer_index].to_vec())
    }

    pub fn add_i_and_mul_i_mle(&mut self, layer_index: usize) -> (MultilinearPoly<F>, MultilinearPoly<F>) {
        let number_of_layer_variables = num_of_layer_variables(layer_index);
        let boolean_hypercube_combinations = 1 << number_of_layer_variables; // 2 ^ number_of_layer_variables

        let mut add_i_values = vec![F::zero(); boolean_hypercube_combinations];
        let mut mul_i_values = vec![F::zero(); boolean_hypercube_combinations];

        for gate in self.layers[layer_index].gates.iter() {
            match gate.operator {
                Operator::Add => {
                    let position_index = convert_to_binary_and_to_decimal(layer_index, gate.output_index, gate.left_index, gate.right_index);
                    add_i_values[position_index] = F::one();
                },
                Operator::Mul => {
                    let position_index = convert_to_binary_and_to_decimal(layer_index, gate.output_index, gate.left_index, gate.right_index);
                    mul_i_values[position_index] = F::one();
                }
            }
        }

        let add_i_polynomial = MultilinearPoly::new(add_i_values);
        let mul_i_polynomial = MultilinearPoly::new(mul_i_values);

        (add_i_polynomial, mul_i_polynomial)
    }
}


pub fn num_of_layer_variables(layer_index: usize) -> usize {
    if layer_index == 0 {
        return 3;
    }

    let var_a_length = layer_index;
    let var_b_length = var_a_length + 1;
    let var_c_length = var_a_length + 1;

    let num_of_variables = var_a_length + var_b_length + var_c_length;

    num_of_variables
}


pub fn convert_to_binary_and_to_decimal(layer_index: usize, variable_a: usize, variable_b: usize, variable_c: usize) -> usize {
    // convert decimal to binary
    let a_in_binary = convert_decimal_to_padded_binary(variable_a, layer_index);
    let b_in_binary = convert_decimal_to_padded_binary(variable_b, layer_index + 1);
    let c_in_binary = convert_decimal_to_padded_binary(variable_c, layer_index + 1);

    // combine a, b and c binaries
    let combined_binary = a_in_binary + &b_in_binary + &c_in_binary;
    
    // convert the combined binaries back to decimal
    usize::from_str_radix(&combined_binary, 2).unwrap_or(0)
}

pub fn convert_decimal_to_padded_binary(decimal_number: usize, bit_length: usize) -> String {
    format!("{:0>width$b}", decimal_number, width = bit_length)
}

pub fn tensor_addition<F:PrimeField>( w_b: &MultilinearPoly<F>, w_c: &MultilinearPoly<F>) -> MultilinearPoly<F> {
    assert!(w_b.coefficients.len() == w_c.coefficients.len(), "Polynomials must be evaluated over the same hypercube.");
    
    let result_values: Vec<F> = w_b.coefficients.iter().zip(w_c.coefficients.iter()).map(|(b, c)| *b + *c).collect();
    
    MultilinearPoly::new(result_values)
}

pub  fn tensor_multiplication<F:PrimeField>( w_b: &MultilinearPoly<F>, w_c: &MultilinearPoly<F>) -> MultilinearPoly<F> {
    assert!(w_b.coefficients.len() == w_c.coefficients.len(), "polynomial must be evaluated over the same hypercube.");

    let result_values: Vec<F> = w_b.coefficients.iter().zip(w_c.coefficients.iter()).map(|(b, c)| *b * *c).collect();

    MultilinearPoly::new(result_values)

}

impl <F: PrimeField> ProductPoly<F> {
    fn new(evaluations: Vec<Vec<F>>) -> Self {
       let multi_polys = evaluations.iter().map(|eval| MultilinearPoly::new(eval.to_vec())).collect();

        Self {evaluation: multi_polys}
    }

    fn partial_evaluate(&self, value: F) -> Self {
        let partial_polys = self.evaluation.iter().map(|poly| {
            let partial_result = poly.partial_evaluate((0, value));

            partial_result.coefficients
        }).collect();

        Self::new(partial_polys)
    }

    fn evaluate(&self, values: Vec<F>) -> F {
        self.evaluation.iter().map(|poly| poly.evaluate(&values)).product()
    }

    fn get_degree(&self) -> usize {
        self.evaluation.len()
    }

    fn reduce(&self) -> Vec<F> {
        let first_poly = &self.evaluation[0].coefficients;
        let second_poly = &self.evaluation[1].coefficients;

        first_poly.iter().zip(second_poly.iter()).map(|(a,b)| *a * *b ).collect()
    }
}

impl <F: PrimeField> SumPoly<F> {
  pub fn new(evaluations: Vec<ProductPoly<F>>) -> Self {
        Self { polys: evaluations }
    }

   pub fn partial_evaluate(&self, value: F) -> Self {
        let partial_result = self.polys.iter().map(|product_poly|product_poly.partial_evaluate(value)
        ).collect();

        Self::new(partial_result)
    }

  pub fn evaluate(&self, values: Vec<F>) -> F {
        self.polys.iter().map(|product_poly| product_poly.evaluate(values.clone())).sum()
    }

  pub fn get_degree(&self) -> usize {
        self.polys[0].get_degree()
    }

  pub  fn reduce(&self) -> Vec<F> {
        let first_poly = self.polys[0].reduce();
        let second_poly = self.polys[1].reduce();

        first_poly.iter().zip(second_poly.iter()).map(|(a, b)| *a + *b).collect()
    } 
}


#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_circuit_evaluation() {
        let input = vec![Fq::from(2), Fq::from(3), Fq::from(4), Fq::from(5)];

        let gate1 = Gate::new(0, 1, 0, Operator::Mul);
        let gate2 = Gate::new(0, 1, 0, Operator::Add);
        let gate3 = Gate::new(2, 3, 1,  Operator::Mul);
        
        let layer0 = Layer::new(vec![gate1]);
        let layer1 = Layer::new(vec![gate2, gate3]);

        let mut circuit = Circuit::<Fq>::new(vec![layer0, layer1]);

        let result = circuit.evaluate(input);

        let expected_layers_evaluation = vec![
            vec![Fq::from(100)],
            vec![Fq::from(5), Fq::from(20)],
            vec![Fq::from(2), Fq::from(3), Fq::from(4), Fq::from(5)]
        ];

        assert_eq!(result[0], Fq::from(100));
        assert_eq!(circuit.layer_evaluations, expected_layers_evaluation);
    }

    #[test]
    fn test_circuit_evaluation2() {
        let input = vec![Fq::from(1), Fq::from(2), Fq::from(3), Fq::from(4)];
        
        let gate1 = Gate::new(0, 1, 0, Operator::Add);
        // switched output index
        let gate2 = Gate::new(0, 1, 1, Operator::Add);
        let gate3 = Gate::new(2, 3, 0, Operator::Mul);

        let layer0 = Layer::new(vec![gate1]);
        let layer1 = Layer::new(vec![gate2, gate3]);

        let mut circuit = Circuit::<Fq>::new(vec![layer0, layer1]);
        let result = circuit.evaluate(input);

        let expected_layers_evaluation = vec![
            vec![Fq::from(15)],
            vec![Fq::from(12), Fq::from(3)],
            vec![Fq::from(1), Fq::from(2), Fq::from(3), Fq::from(4)]
        ];

        assert_eq!(result[0], Fq::from(15));
        assert_eq!(circuit.layer_evaluations, expected_layers_evaluation)
    }

    #[test]
    fn test_circuit_evaluation3() {
        let input = vec![Fq::from(1), Fq::from(2), Fq::from(3), Fq::from(4), Fq::from(5), Fq::from(6), Fq::from(7), Fq::from(8)];

        // layer 0 gates
        let gate1 = Gate::new(0, 1, 0, Operator::Add);
        
        // layer 1 gates
        let gate2 = Gate::new(0, 1, 0, Operator::Add);
        let gate3 = Gate::new(2, 3, 1, Operator::Mul);

        // layer 2 gates
        let gate4 = Gate::new(0, 1, 0, Operator::Add);
        let gate5 = Gate::new(2, 3, 1, Operator::Mul);
        let gate6 = Gate::new(4, 5, 2, Operator::Mul);
        let gate7 = Gate::new(6, 7, 3, Operator::Mul);

        // Layers
        let layer0 = Layer::new(vec![gate1]);
        let layer1 = Layer::new(vec![gate2, gate3]);
        let layer2 = Layer::new(vec![gate4, gate5, gate6, gate7]);

        let mut circuit = Circuit::<Fq>::new(vec![layer0, layer1, layer2]);
        let result = circuit.evaluate(input);

        assert_eq!(result[0], Fq::from(1695));
    }

    #[test]
    fn test_num_of_layer_variables() {
        // Assert Equal
        assert_eq!(num_of_layer_variables(0), 3);
        assert_eq!(num_of_layer_variables(1), 5);
        assert_eq!(num_of_layer_variables(2), 8);
        assert_eq!(num_of_layer_variables(3), 11);
        assert_eq!(num_of_layer_variables(4), 14);

        // Assert Not Equal
        assert_ne!(num_of_layer_variables(2), 7);
        assert_ne!(num_of_layer_variables(3), 9);
    }

    #[test]
    fn test_add_i_and_mul_i_mle_layer0() {
        let gate1 = Gate::new(0, 1, 0, Operator::Add);
        // switched output index
        let gate2 = Gate::new(0, 1, 1, Operator::Add);
        let gate3 = Gate::new(2, 3, 0, Operator::Mul);

        let layer0 = Layer::new(vec![gate1]);
        let layer1 = Layer::new(vec![gate2, gate3]);

        let mut circuit = Circuit::<Fq>::new(vec![layer0, layer1]);

        let (add_i_poly, mul_i_poly) = circuit.add_i_and_mul_i_mle(0);
        let expected_add_i_poly = MultilinearPoly::new(
            vec![Fq::from(0), Fq::from(1), Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(0)]
        );

        let expected_mul_i_poly = MultilinearPoly::new(
            vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(0)]
        );

        assert_eq!(add_i_poly, expected_add_i_poly);
        assert_eq!(mul_i_poly, expected_mul_i_poly);
    }

    #[test]
    fn test_add_i_and_mul_i_mle_layer1() {
        let gate1 = Gate::new(0, 1, 0, Operator::Add);
        // switched output index
        let gate2 = Gate::new(0, 1, 1, Operator::Add);
        let gate3 = Gate::new(2, 3, 0, Operator::Mul);

        let layer0 = Layer::new(vec![gate1]);
        let layer1 = Layer::new(vec![gate2, gate3]);

        let mut circuit = Circuit::<Fq>::new(vec![layer0, layer1]);
        // let result = circuit.evaluate(input);

        let (add_i_poly, mul_i_poly) = circuit.add_i_and_mul_i_mle(1);

        // For layer 1: 2^5 = 32 combinations
        let mut expected_add = vec![Fq::from(0); 32];
        expected_add[17] = Fq::from(1);  // position from gate2: "10001" = 17
        let expected_add_i_poly = MultilinearPoly::new(expected_add);

        let mut expected_mul = vec![Fq::from(0); 32];
        expected_mul[11] = Fq::from(1);  // position from gate3: "01011" = 11
        let expected_mul_i_poly = MultilinearPoly::new(expected_mul);

        assert_eq!(add_i_poly, expected_add_i_poly);
        assert_eq!(mul_i_poly, expected_mul_i_poly);
    }


     #[test]
    fn test_tensor_addition() {
        let w_b = MultilinearPoly::new(vec![Fq::from(2), Fq::from(5), Fq::from(4), Fq::from(7)]);
        let w_c = MultilinearPoly::new(vec![Fq::from(3), Fq::from(3), Fq::from(3), Fq::from(3)]);
        let result = tensor_addition(&w_b, &w_c);
        assert_eq!(result.coefficients, vec![Fq::from(5), Fq::from(8), Fq::from(7), Fq::from(10)]);
    }

    #[test]
    fn test_tensor_multiplication() {
        let w_b = MultilinearPoly::new(vec![Fq::from(3), Fq::from(3), Fq::from(5), Fq::from(5)]);
        let w_c = MultilinearPoly::new(vec![Fq::from(4), Fq::from(7), Fq::from(4), Fq::from(7)]);
        let result = tensor_multiplication(&w_b, &w_c);
        assert_eq!(result.coefficients, vec![Fq::from(12), Fq::from(21), Fq::from(20), Fq::from(35)]);
    }
}