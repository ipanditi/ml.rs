# Neural Network from Scratch in Rust

This repository contains an implementation of a basic neural network built from scratch in Rust. The neural network is fully functional, with support for multiple layers, activation functions, and backpropagation for training. The purpose of this project is to demonstrate the construction of a neural network without using any external machine learning libraries.

## Table of Contents
1. [Features](#features)
2. [Installation](#installation)
3. [Usage](#usage)
4. [Network Structure](#network-structure)
5. [Training](#training)
6. [Performance](#performance)
7. [Features](#features)

## Features
Supports fully connected feedforward layers.
Implements standard activation functions:
- Sigmoid
- ReLU
- Tanh
Backpropagation with gradient descent.
Customizable loss function.
Written entirely in Rust for performance and safety.

## Installation
Make sure you have Rust installed on your machine. If not, you can install it using rustup:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Clone this repository:
```
git clone https://github.com/your-username/rust-neural-network.git
cd rust-neural-network
```
Build the project:
```
cargo build --release
```
## Usage
```
cargo run --release
```

## Network Structure
This implementation allows you to define a neural network with any number of layers. For each layer, you specify the number of neurons and the activation function. The architecture is simple but flexible enough for basic machine learning tasks.

```
pub fn new(layers: Vec<usize>, activation:Activation, learning_rate:f64) -> Self {
        let mut weights = vec![];
        let mut biases = vec![];
        for i in 0..layers.len() -1{
            weights.push(Matrix::random(layers[i+1], layers[i]));
            biases.push(Matrix::random(layers[i+1], 1));
        }
        Network {
            layers,
            weights,
            biases,
            data: vec![],
            activation,
            learning_rate
        }
}
```

Layers: The network consists of fully connected layers where each neuron is connected to every neuron in the next layer.
Activation Functions:
Sigmoid: Classic sigmoid activation function.
ReLU: Rectified Linear Unit, ideal for hidden layers.
Tanh: Hyperbolic tangent, scales inputs between -1 and 1.

## Training
You can train the neural network using the backpropagation algorithm with gradient descent.

Loss Function
The default loss function is Mean Squared Error (MSE), but this can be modified or extended to support other loss functions.

```
pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u32){
        for i in 1..=epochs {
            if epochs<100 || i%(epochs/100) == 0{
                println!("Epoch {} of {}", i,epochs);
            }
            for j in 0..inputs.len() {
                let outputs = self.feed_forward(Matrix::from(inputs[j].clone()));
                self.back_propagate(outputs, Matrix::from(targets[j].clone()));
            }
        }
}
```

training_data: Your input data for training.
expected_output: The expected results for each training sample.
learning_rate: Controls how much the weights are updated during training.
epochs: Number of times the training data is passed through the network.
Example:
XOR problem.

```
fn main(){
    env::set_var("RUST_BACKTRACE", "1");
    let inputs = vec![
        vec![0.0,0.0],
        vec![0.0,1.0],
        vec![1.0, 0.0],
        vec![1.0,1.0],
    ];
    let targets = vec![vec![0.0], vec![1.0], vec![0.0], vec![1.0]];
    let mut network = Network::new(vec![2,3,1], SIGMOID, 0.5);
    network.train(inputs, targets, 100000);
    println!("{:?}", network.feed_forward(Matrix::from(vec![0.0, 0.0])));
	  println!("{:?}", network.feed_forward(Matrix::from(vec![0.0, 1.0])));
	  println!("{:?}", network.feed_forward(Matrix::from(vec![1.0, 0.0])));
	  println!("{:?}", network.feed_forward(Matrix::from(vec![1.0, 1.0])));
}

Results:
[0.0, 0.0] -> [~0.0]
[0.0, 1.0] -> [~1.0]
[1.0, 0.0] -> [~1.0]
[1.0, 1.0] -> [~0.0]
```

## Performance
Although this implementation is intended for educational purposes, Rust's performance characteristics make it suitable for experimentation with larger neural networks. Performance optimization for larger networks (e.g., batching, parallelization) has not yet been implemented.
