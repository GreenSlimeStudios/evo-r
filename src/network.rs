// use macroquad::prelude::*;
// use utils::math::sigmoi
// use rand::prelude::*;
use rand::*;

const OUTPUT_NEURON_COUNT: i8 = 2;

#[derive(Clone, Debug)]
pub struct Network {
    pub layers: Vec<Layer>,
}
impl Network {
    pub fn new(layer_count: i8, layer_neuron_count: i8, input_neuron_count: i8) -> Self {
        let mut network = Self { layers: Vec::new() };
        for i in 0..layer_count {
            if i == layer_count - 1 {
                network.layers.push(Layer::new(OUTPUT_NEURON_COUNT)); // 4 becouse there are for buttons the car can press
            } else {
                network.layers.push(Layer::new(layer_neuron_count));
            }
        }
        for i in 0..network.layers.len() {
            let input_len = if i == 0 {
                input_neuron_count
            } else {
                network.layers[i - 1].neurons.len() as i8
            };
            network.layers[i].generate_new_weights(input_len);
        }
        network
    }
    pub fn calcuate_layers(&mut self, input_neurons: &Vec<Neouron>) {
        for i in 0..self.layers.len() {
            if i == 0 {
                self.layers[i].update_neurons(input_neurons);
            } else {
                let pre_neurons: &Vec<Neouron> = &self.layers[i - 1].neurons.clone();
                self.layers[i].update_neurons(pre_neurons);
            }
        }
    }
    pub fn alter_values(&mut self) {
        for i in 0..self.layers.len() {
            //weights
            for j in 0..self.layers[i].weights.len() {
                for k in 0..self.layers[i].weights[j].len() {
                    // self.layers[i].weights[j][k] += rand::thread_rng().gen_range(-10.0..10.0) / 2.0;
                    self.layers[i].weights[j][k] +=
                        macroquad::prelude::rand::gen_range(-10.0, 10.0) / 2.0;
                }
            }
            // for j in 0..self.layers[i].neurons.len() {
            //     self.layers[i].neurons[j].bias += rand::thread_rng().gen_range(-10.0..10.0)
            // }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Layer {
    pub weights: Vec<Vec<f32>>,
    pub neurons: Vec<Neouron>,
}
impl Layer {
    fn new(neuron_count: i8) -> Self {
        let mut layer = Self {
            weights: Vec::new(),
            neurons: Vec::new(),
        };
        for _i in 0..neuron_count {
            layer.neurons.push(Neouron::new());
        }
        layer
    }
    fn update_neurons(&mut self, input_neurons: &Vec<Neouron>) {
        for i in 0..self.neurons.len() {
            self.neurons[i].calculate(&self.weights[i], input_neurons);
        }
    }
    fn generate_new_weights(&mut self, in_neuron_count: i8) {
        for i in 0..self.neurons.len() {
            self.weights.push(Vec::new());
            for _j in 0..in_neuron_count {
                let rng = rand::thread_rng().gen_range(-100.0..=100.0) / 20.0;
                // let rng: f32 = rand::srand(10);
                // let rng = 0.0;
                self.weights[i].push(rng);
            }
        }
    }
}
#[derive(Clone, Debug, Copy)]
pub struct Neouron {
    // inputs: Vec,
    pub value: f32,
    pub bias: f32,
}
impl Neouron {
    fn new() -> Self {
        Self {
            value: 0.0,
            bias: rand::thread_rng().gen_range(-100.0..=100.0) / 1000.0,
            // bias: 0.0,
        }
    }
    pub fn from_val(value: f32) -> Self {
        Self { value, bias: 0.0 }
    }
    fn calculate(&mut self, weights: &Vec<f32>, input_neurons: &Vec<Neouron>) {
        let mut val: f32 = 0.0;
        for i in 0..input_neurons.len() {
            val += input_neurons[i].value * weights[i];
        }
        // println!("{}", val);
        self.value = squishify(val + self.bias);
        // self.value = sigmoid(val);
    }
}

pub fn squishify(x: f32) -> f32 {
    let val = 1.0 / (1.0 + std::f32::consts::E.powf(-x));
    // println!("{} {}", x, val);
    val
}
