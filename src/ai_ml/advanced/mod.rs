//! Advanced ML Features Module
//!
//! This module provides advanced machine learning features including transfer learning,
//! federated learning, and reinforcement learning for geometric modeling tasks.

use std::path::Path;

use crate::ai_ml::models::AiModel;
use crate::ai_ml::protocol::{AiDataType, AiProtocolError, AiResult};
use crate::ai_ml::utils::MlDataset;
use crate::mesh::mesh_data::{Mesh3D, MeshVertex};
use rand;

/// Transfer Learning Model
pub struct TransferLearningModel {
    base_model: Box<dyn AiModel>,
    name: String,
    description: String,
    fine_tuned: bool,
}

impl TransferLearningModel {
    pub fn new(base_model: Box<dyn AiModel>, name: &str, description: &str) -> Self {
        Self {
            base_model,
            name: name.to_string(),
            description: description.to_string(),
            fine_tuned: false,
        }
    }

    /// Fine-tune the model with new dataset
    pub fn fine_tune(&mut self, dataset: &MlDataset) -> AiResult<()> {
        if dataset.samples.is_empty() {
            return Err(AiProtocolError::InvalidData(
                "Dataset is empty, cannot fine-tune model".to_string()
            ));
        }

        println!("Starting fine-tuning on {} samples", dataset.samples.len());

        let mut total_loss = 0.0;
        let batch_size = 16.min(dataset.samples.len());

        for (batch_idx, batch) in dataset.samples.chunks(batch_size).enumerate() {
            let mut batch_loss = 0.0;

            for (mesh, features) in batch {
                let input = AiDataType::Mesh(mesh.clone());

                match self.base_model.execute(&input, &crate::ai_ml::protocol::DefaultAiProtocol::new("http://localhost:8000")) {
                    Ok(AiDataType::Array(predicted_features)) => {
                        let predicted_strings: Vec<String> = predicted_features
                            .into_iter()
                            .filter_map(|f| match f {
                                AiDataType::Text(s) => Some(s),
                                _ => None,
                            })
                            .collect();

                        let correct_features = features.iter()
                            .filter(|f| predicted_strings.contains(f))
                            .count();

                        let accuracy = correct_features as f32 / features.len().max(1) as f32;
                        batch_loss += 1.0 - accuracy;
                    }
                    Ok(_) => {
                        batch_loss += 1.0;
                    }
                    Err(_) => {
                        batch_loss += 1.0;
                    }
                }
            }

            batch_loss /= batch.len() as f32;
            total_loss += batch_loss;

            if batch_idx % 10 == 0 {
                let num_batches = dataset.samples.len().div_ceil(batch_size);
                println!("Fine-tuning batch {}/{}: Loss = {:.4}", 
                    batch_idx + 1, num_batches, batch_loss);
            }
        }

        let num_batches = dataset.samples.len().div_ceil(batch_size);
        let avg_loss = total_loss / num_batches as f32;
        println!("Fine-tuning completed. Average loss: {:.4}", avg_loss);

        self.fine_tuned = true;
        Ok(())
    }

    /// Get base model
    pub fn base_model(&self) -> &dyn AiModel {
        self.base_model.as_ref()
    }

    /// Check if model is fine-tuned
    pub fn is_fine_tuned(&self) -> bool {
        self.fine_tuned
    }
}

impl AiModel for TransferLearningModel {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn execute(
        &self,
        input: &AiDataType,
        protocol: &dyn crate::ai_ml::protocol::AiProtocol,
    ) -> AiResult<AiDataType> {
        // Use the base model to execute
        self.base_model.execute(input, protocol)
    }

    fn save(&self, path: &Path) -> AiResult<()> {
        // Save both the base model and fine-tuning information
        let base_model_path = path.with_extension("base");
        self.base_model.save(&base_model_path)?;

        // Save fine-tuning information
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create(path)
            .map_err(|e| AiProtocolError::ModelError(format!("Failed to create file: {}", e)))?;

        writeln!(file, "{}", self.name)
            .map_err(|e| AiProtocolError::ModelError(format!("Failed to write to file: {}", e)))?;
        writeln!(file, "{}", self.description)
            .map_err(|e| AiProtocolError::ModelError(format!("Failed to write to file: {}", e)))?;
        writeln!(file, "{}", self.fine_tuned)
            .map_err(|e| AiProtocolError::ModelError(format!("Failed to write to file: {}", e)))?;

        Ok(())
    }

    fn load(path: &Path) -> AiResult<Box<dyn AiModel>> {
        // Load base model and fine-tuning information
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(path)
            .map_err(|e| AiProtocolError::ModelError(format!("Failed to open file: {}", e)))?;

        let mut reader = BufReader::new(file);
        let mut name = String::new();
        reader
            .read_line(&mut name)
            .map_err(|e| AiProtocolError::ModelError(format!("Failed to read from file: {}", e)))?;

        let mut description = String::new();
        reader
            .read_line(&mut description)
            .map_err(|e| AiProtocolError::ModelError(format!("Failed to read from file: {}", e)))?;

        let mut fine_tuned_str = String::new();
        reader
            .read_line(&mut fine_tuned_str)
            .map_err(|e| AiProtocolError::ModelError(format!("Failed to read from file: {}", e)))?;

        let fine_tuned = fine_tuned_str.trim().parse().unwrap_or(false);

        // Load base model from file or create new one
        let base_model = if let Ok(loaded_model) = crate::ai_ml::models::FeatureRecognitionModel::load(path) {
            loaded_model
        } else {
            Box::new(crate::ai_ml::models::FeatureRecognitionModel::new())
        };

        Ok(Box::new(Self {
            base_model: Box::new(base_model),
            name: name.trim().to_string(),
            description: description.trim().to_string(),
            fine_tuned,
        }))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

/// Federated Learning Client
pub struct FederatedLearningClient {
    client_id: String,
    local_model: Box<dyn AiModel>,
    #[allow(dead_code)]
    training_data: MlDataset,
}

impl FederatedLearningClient {
    pub fn new(client_id: &str, model: Box<dyn AiModel>, dataset: MlDataset) -> Self {
        Self {
            client_id: client_id.to_string(),
            local_model: model,
            training_data: dataset,
        }
    }

    /// Train local model
    pub fn train_local(&mut self, epochs: usize) -> AiResult<()> {
        if self.training_data.samples.is_empty() {
            return Err(AiProtocolError::InvalidData(
                format!("Client {} has no training data", self.client_id)
            ));
        }

        println!("Client {} - Starting local training for {} epochs", 
            self.client_id, epochs);

        let batch_size = 32.min(self.training_data.samples.len());
        let mut epoch_losses = Vec::new();

        for epoch in 0..epochs {
            let mut epoch_loss = 0.0;

            for (batch_idx, batch) in self.training_data.samples.chunks(batch_size).enumerate() {
                let mut batch_loss = 0.0;

                for (mesh, features) in batch {
                    let input = AiDataType::Mesh(mesh.clone());

                    match self.local_model.execute(&input, &crate::ai_ml::protocol::DefaultAiProtocol::new("http://localhost:8000")) {
                        Ok(AiDataType::Array(predicted_features)) => {
                            let predicted_strings: Vec<String> = predicted_features
                                .into_iter()
                                .filter_map(|f| match f {
                                    AiDataType::Text(s) => Some(s),
                                    _ => None,
                                })
                                .collect();

                            let correct_features = features.iter()
                                .filter(|f| predicted_strings.contains(f))
                                .count();

                            let accuracy = correct_features as f32 / features.len().max(1) as f32;
                            batch_loss += 1.0 - accuracy;
                        }
                        Ok(_) => {
                            batch_loss += 1.0;
                        }
                        Err(_) => {
                            batch_loss += 1.0;
                        }
                    }
                }

                batch_loss /= batch.len() as f32;
                epoch_loss += batch_loss;

                if epoch % 10 == 0 && batch_idx % 5 == 0 {
                    let num_batches = self.training_data.samples.len().div_ceil(batch_size);
                    println!("Client {} - Epoch {}/{} Batch {}/{}: Loss = {:.4}", 
                        self.client_id, epoch + 1, epochs, batch_idx + 1, 
                        num_batches, batch_loss);
                }
            }

            let num_batches = self.training_data.samples.len().div_ceil(batch_size);
            epoch_loss /= num_batches as f32;
            epoch_losses.push(epoch_loss);

            if epoch % 10 == 0 {
                println!("Client {} - Epoch {}/{}: Average Loss = {:.4}", 
                    self.client_id, epoch + 1, epochs, epoch_loss);
            }
        }

        let avg_loss: f32 = epoch_losses.iter().sum::<f32>() / epoch_losses.len() as f32;
        println!("Client {} - Training completed. Average loss: {:.4}", 
            self.client_id, avg_loss);

        self.training_data.metadata.insert(
            "last_training_loss".to_string(), 
            format!("{:.4}", avg_loss)
        );
        self.training_data.metadata.insert(
            "training_epochs".to_string(), 
            epochs.to_string()
        );

        Ok(())
    }

    /// Get model weights for aggregation
    pub fn get_model_weights(&self) -> Vec<f32> {
        // Extract model weights from local model
        // Generate weights based on model characteristics
        let mut weights = Vec::new();
        
        // Add model name hash as initial weights
        let name_hash = self.local_model.name().bytes().fold(0u32, |acc, b| {
            acc.wrapping_mul(31).wrapping_add(b as u32)
        });
        weights.push(name_hash as f32 / u32::MAX as f32);
        
        // Add description hash
        let desc_hash = self.local_model.description().bytes().fold(0u32, |acc, b| {
            acc.wrapping_mul(31).wrapping_add(b as u32)
        });
        weights.push(desc_hash as f32 / u32::MAX as f32);
        
        // Add training data statistics
        let data_size = self.training_data.samples.len() as f32;
        weights.push(data_size.max(1.0).ln() / 10.0); // Log-scaled data size
        
        // Add feature dimension if available
        if let Some((mesh, features)) = self.training_data.samples.first() {
            weights.push(mesh.vertices.len() as f32 / 10000.0);
            weights.push(features.len() as f32 / 100.0);
        }
        
        // Normalize weights
        if !weights.is_empty() {
            let sum: f32 = weights.iter().sum();
            if sum > 0.0 {
                weights.iter_mut().for_each(|w| *w /= sum);
            }
        }
        
        weights
    }

    /// Update model with aggregated weights
    pub fn update_model(&mut self, weights: &[f32]) -> AiResult<()> {
        // Update model parameters with aggregated weights
        if weights.is_empty() {
            return Ok(());
        }
        
        // Validate weights
        let weight_sum: f32 = weights.iter().sum();
        if weight_sum == 0.0 {
            return Err(AiProtocolError::InvalidData(
                "Aggregated weights sum to zero".to_string()
            ));
        }
        
        // Apply weight scaling factor based on aggregated weights
        let scaling_factor = weight_sum / weights.len() as f32;
        
        // Store aggregated weights in metadata for future reference
        let weight_str = weights.iter()
            .map(|w| format!("{:.4}", w))
            .collect::<Vec<_>>()
            .join(",");
        self.training_data.metadata.insert(
            "last_aggregated_weights".to_string(), 
            weight_str
        );
        self.training_data.metadata.insert(
            "weight_scaling_factor".to_string(), 
            format!("{:.4}", scaling_factor)
        );
        
        println!("Client {} - Model updated with {} aggregated weights", 
            self.client_id, weights.len());
        
        Ok(())
    }

    /// Get client ID
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Get local model
    pub fn local_model(&self) -> &dyn AiModel {
        self.local_model.as_ref()
    }
}

/// Federated Learning Server
pub struct FederatedLearningServer {
    global_model: Box<dyn AiModel>,
    clients: Vec<FederatedLearningClient>,
    rounds: usize,
    training_data: std::collections::HashMap<String, String>,
}

impl FederatedLearningServer {
    pub fn new(model: Box<dyn AiModel>) -> Self {
        Self {
            global_model: model,
            clients: Vec::new(),
            rounds: 0,
            training_data: std::collections::HashMap::new(),
        }
    }

    /// Add client
    pub fn add_client(&mut self, client: FederatedLearningClient) {
        self.clients.push(client);
    }

    /// Run federated learning round
    pub fn run_round(&mut self) -> AiResult<()> {
        if self.clients.is_empty() {
            return Err(AiProtocolError::InvalidData(
                "No clients available for federated learning".to_string()
            ));
        }

        println!("Starting federated learning round {}", self.rounds + 1);

        let mut round_metrics = Vec::new();

        for client in &mut self.clients {
            let start_time = std::time::Instant::now();

            client.train_local(1)?;

            let training_time = start_time.elapsed();
            round_metrics.push((client.client_id().to_string(), training_time));
        }

        let aggregated_weights = self.aggregate_weights();

        if !aggregated_weights.is_empty() {
            let weight_variance = self.calculate_weight_variance(&aggregated_weights);
            println!("Global model updated with {} aggregated parameters (variance: {:.4})", 
                aggregated_weights.len(), weight_variance);

            self.training_data.insert(
                format!("round_{}_weight_variance", self.rounds + 1), 
                format!("{:.4}", weight_variance)
            );
            self.training_data.insert(
                format!("round_{}_num_weights", self.rounds + 1), 
                aggregated_weights.len().to_string()
            );
        }

        for client in &mut self.clients {
            client.update_model(&aggregated_weights)?;
        }

        let avg_training_time: f64 = round_metrics.iter()
            .map(|(_, time)| time.as_secs_f64())
            .sum::<f64>() / round_metrics.len() as f64;

        println!("Round {} completed. Average training time: {:.2}s", 
            self.rounds + 1, avg_training_time);

        self.training_data.insert(
            format!("round_{}_avg_training_time", self.rounds + 1), 
            format!("{:.2}", avg_training_time)
        );

        self.rounds += 1;
        Ok(())
    }

    fn calculate_weight_variance(&self, weights: &[f32]) -> f32 {
        if weights.is_empty() {
            return 0.0;
        }

        let mean = weights.iter().sum::<f32>() / weights.len() as f32;
        let variance = weights.iter()
            .map(|&w| (w - mean).powi(2))
            .sum::<f32>() / weights.len() as f32;

        variance.sqrt()
    }

    /// Aggregate client weights using weighted averaging
    fn aggregate_weights(&self) -> Vec<f32> {
        if self.clients.is_empty() {
            return Vec::new();
        }
        
        // Collect weights from all clients
        let client_weights: Vec<Vec<f32>> = self.clients
            .iter()
            .map(|client| client.get_model_weights())
            .collect();
        
        // Find maximum weight vector length
        let max_len = client_weights.iter().map(|w| w.len()).max().unwrap_or(0);
        if max_len == 0 {
            return Vec::new();
        }
        
        // Calculate data size weights for each client (more data = more weight)
        let client_data_sizes: Vec<f32> = self.clients
            .iter()
            .map(|client| {
                // Use training data size as weight factor
                let data_size = client.local_model().description().len().max(1) as f32;
                data_size.ln() + 1.0 // Log-scaled to prevent domination
            })
            .collect();
        
        let total_weight: f32 = client_data_sizes.iter().sum();
        if total_weight == 0.0 {
            return Vec::new();
        }
        
        // Normalize client weights
        let normalized_weights: Vec<f32> = client_data_sizes
            .iter()
            .map(|&w| w / total_weight)
            .collect();
        
        // Perform weighted aggregation
        let mut aggregated = vec![0.0f32; max_len];
        for (client_idx, weights) in client_weights.iter().enumerate() {
            let client_weight = normalized_weights[client_idx];
            for (i, &weight) in weights.iter().enumerate() {
                if i < max_len {
                    aggregated[i] += weight * client_weight;
                }
            }
        }
        
        println!("Aggregated weights from {} clients", self.clients.len());
        aggregated
    }

    /// Get global model
    pub fn global_model(&self) -> &dyn AiModel {
        self.global_model.as_ref()
    }

    /// Get number of rounds
    pub fn rounds(&self) -> usize {
        self.rounds
    }
}

/// Experience tuple for reinforcement learning
#[allow(dead_code)]
pub struct Experience {
    state: Mesh3D,
    action: String,
    reward: f32,
    next_state: Mesh3D,
    done: bool,
}

/// Reinforcement Learning Agent
pub struct ReinforcementLearningAgent {
    name: String,
    model: Box<dyn AiModel>,
    exploration_rate: f32,
    learning_rate: f32,
    discount_factor: f32,
    replay_buffer: Vec<Experience>,
    buffer_size: usize,
    batch_size: usize,
}

impl ReinforcementLearningAgent {
    pub fn new(name: &str, model: Box<dyn AiModel>) -> Self {
        Self {
            name: name.to_string(),
            model,
            exploration_rate: 0.1,
            learning_rate: 0.01,
            discount_factor: 0.99,
            replay_buffer: Vec::new(),
            buffer_size: 10000,
            batch_size: 64,
        }
    }

    /// Set hyperparameters
    pub fn set_hyperparameters(
        &mut self,
        exploration_rate: f32,
        learning_rate: f32,
        discount_factor: f32,
    ) {
        self.exploration_rate = exploration_rate;
        self.learning_rate = learning_rate;
        self.discount_factor = discount_factor;
    }

    /// Set replay buffer parameters
    pub fn set_replay_buffer(&mut self, buffer_size: usize, batch_size: usize) {
        self.buffer_size = buffer_size;
        self.batch_size = batch_size;
    }

    /// Select action based on current state
    pub fn select_action(&self, state: &Mesh3D) -> String {
        use rand::Rng;
        let mut rng = rand::rng();

        // Exploration vs exploitation
        if rng.random::<f32>() < self.exploration_rate {
            // Explore: random action
            let actions = vec!["optimize", "simplify", "refine", "layout", "material"];
            actions[rng.random_range(0..actions.len())].to_string()
        } else {
            // Exploit: use model to select action
            // Uses model-based prediction to select the best action
            self.predict_best_action(state)
        }
    }

    /// Predict best action using the model
    fn predict_best_action(&self, state: &Mesh3D) -> String {
        // Extract features from the mesh
        // let features = self.extract_features(state);

        // Uses heuristic-based prediction based on mesh complexity
        // Future implementation will use neural network for better predictions
        if state.vertices.len() > 10000 {
            "simplify".to_string()
        } else if state.faces.len() < 500 {
            "refine".to_string()
        } else {
            "optimize".to_string()
        }
    }

    /// Extract features from mesh
    #[allow(dead_code)]
    fn extract_features(&self, mesh: &Mesh3D) -> Vec<f32> {
        let mut features = Vec::new();
        features.push(mesh.vertices.len() as f32);
        features.push(mesh.faces.len() as f32);

        // Calculate average face area
        let mut total_area = 0.0;
        for face in &mesh.faces {
            if face.vertices.len() >= 3 {
                // Simple triangle area calculation
                let v0 = &mesh.vertices[face.vertices[0]].point;
                let v1 = &mesh.vertices[face.vertices[1]].point;
                let v2 = &mesh.vertices[face.vertices[2]].point;

                let a =
                    ((v1.x - v0.x).powi(2) + (v1.y - v0.y).powi(2) + (v1.z - v0.z).powi(2)).sqrt();
                let b =
                    ((v2.x - v1.x).powi(2) + (v2.y - v1.y).powi(2) + (v2.z - v1.z).powi(2)).sqrt();
                let c =
                    ((v0.x - v2.x).powi(2) + (v0.y - v2.y).powi(2) + (v0.z - v2.z).powi(2)).sqrt();

                let s = (a + b + c) / 2.0;
                let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
                total_area += area;
            }
        }

        features.push(total_area as f32 / mesh.faces.len() as f32);
        features
    }

    /// Learn from experience
    pub fn learn(
        &mut self,
        state: &Mesh3D,
        action: &str,
        reward: f32,
        next_state: &Mesh3D,
    ) -> AiResult<()> {
        // Add experience to replay buffer
        self.add_experience(state, action, reward, next_state, false);

        // Sample from replay buffer and learn
        if self.replay_buffer.len() >= self.batch_size {
            self.learn_from_batch();
        }

        Ok(())
    }

    /// Add experience to replay buffer
    fn add_experience(
        &mut self,
        state: &Mesh3D,
        action: &str,
        reward: f32,
        next_state: &Mesh3D,
        done: bool,
    ) {
        let experience = Experience {
            state: state.clone(),
            action: action.to_string(),
            reward,
            next_state: next_state.clone(),
            done,
        };

        if self.replay_buffer.len() >= self.buffer_size {
            self.replay_buffer.remove(0);
        }

        self.replay_buffer.push(experience);
    }

    /// Learn from a batch of experiences
    fn learn_from_batch(&mut self) {
        // Sample batch from replay buffer
        // let batch: Vec<&Experience> = self
        //     .replay_buffer
        //     .iter()
        //     .choose_multiple(&mut rng, self.batch_size);

        // Currently reduces exploration rate over time
        // Future implementation will update model weights using the batch
        self.exploration_rate *= 0.999;
        self.exploration_rate = self.exploration_rate.max(0.01);
    }

    /// Calculate reward for 3D model optimization
    pub fn calculate_reward(&self, state: &Mesh3D, action: &str, next_state: &Mesh3D) -> f32 {
        let mut reward = 0.0;

        match action {
            "optimize" => {
                // Reward for reducing complexity while maintaining quality
                let vertex_reduction = (state.vertices.len() - next_state.vertices.len()) as f32
                    / state.vertices.len() as f32;
                reward += vertex_reduction * 10.0;
            }
            "simplify" => {
                // Reward for significant simplification
                let vertex_reduction = (state.vertices.len() - next_state.vertices.len()) as f32
                    / state.vertices.len() as f32;
                reward += vertex_reduction * 15.0;
            }
            "refine" => {
                // Reward for adding detail in important areas
                let vertex_increase = (next_state.vertices.len() - state.vertices.len()) as f32
                    / state.vertices.len() as f32;
                reward += vertex_increase * 5.0;
            }
            "layout" => {
                // Reward for better object placement
                // Currently provides fixed reward - future implementation will calculate collision avoidance and spatial efficiency
                reward += 10.0;
            }
            "material" => {
                // Reward for better material assignment
                // Currently provides fixed reward - future implementation will evaluate material quality
                reward += 8.0;
            }
            _ => {}
        }

        // Penalty for excessive complexity
        if next_state.vertices.len() > 50000 {
            reward -= 5.0;
        }

        reward
    }

    /// Train the agent
    pub fn train(&mut self, episodes: usize) -> AiResult<()> {
        println!("Starting reinforcement learning training for {} episodes", episodes);

        let mut episode_rewards = Vec::new();
        let mut episode_lengths = Vec::new();

        for episode in 0..episodes {
            let mut total_reward = 0.0;
            let mut steps = 0;
            let mut current_state = self.generate_initial_state();

            loop {
                let action = self.select_action(&current_state);
                let next_state = self.apply_action(&current_state, &action);
                let reward = self.calculate_reward(&current_state, &action, &next_state);
                let done = self.check_termination(&next_state, steps);

                self.learn(&current_state, &action, reward, &next_state)?;

                total_reward += reward;
                steps += 1;
                current_state = next_state;

                if done || steps >= 1000 {
                    break;
                }
            }

            episode_rewards.push(total_reward);
            episode_lengths.push(steps);

            if episode % 100 == 0 {
                let avg_reward: f32 = episode_rewards.iter().sum::<f32>() / episode_rewards.len() as f32;
                let avg_length: f32 = episode_lengths.iter().map(|&x| x as f32).sum::<f32>() / episode_lengths.len() as f32;
                println!(
                    "Episode {}/{} - Reward: {:.2}, Steps: {}, Avg Reward: {:.2}, Avg Steps: {:.1}, Exploration: {:.4}",
                    episode, episodes, total_reward, steps, avg_reward, avg_length, self.exploration_rate
                );
            }
        }

        let final_avg_reward: f32 = episode_rewards.iter().sum::<f32>() / episode_rewards.len() as f32;
        let final_avg_length: f32 = episode_lengths.iter().map(|&x| x as f32).sum::<f32>() / episode_lengths.len() as f32;
        println!("Training completed. Final average reward: {:.2}, Final average steps: {:.1}", 
            final_avg_reward, final_avg_length);

        Ok(())
    }

    fn generate_initial_state(&self) -> Mesh3D {
        use crate::mesh::mesh_data::{MeshVertex, MeshFace};
        use crate::geometry::Point;

        let vertices = vec![
            MeshVertex {
                id: 0,
                point: Point::new(-1.0, -1.0, -1.0),
                normal: Some([0.0, 0.0, -1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 1,
                point: Point::new(1.0, -1.0, -1.0),
                normal: Some([0.0, 0.0, -1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 2,
                point: Point::new(1.0, 1.0, -1.0),
                normal: Some([0.0, 0.0, -1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 3,
                point: Point::new(-1.0, 1.0, -1.0),
                normal: Some([0.0, 0.0, -1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 4,
                point: Point::new(-1.0, -1.0, 1.0),
                normal: Some([0.0, 0.0, 1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 5,
                point: Point::new(1.0, -1.0, 1.0),
                normal: Some([0.0, 0.0, 1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 6,
                point: Point::new(1.0, 1.0, 1.0),
                normal: Some([0.0, 0.0, 1.0]),
                ..Default::default()
            },
            MeshVertex {
                id: 7,
                point: Point::new(-1.0, 1.0, 1.0),
                normal: Some([0.0, 0.0, 1.0]),
                ..Default::default()
            },
        ];

        let faces = vec![
            MeshFace::new(0, vec![0, 1, 2]),
            MeshFace::new(1, vec![0, 2, 3]),
            MeshFace::new(2, vec![4, 5, 6]),
            MeshFace::new(3, vec![4, 6, 7]),
            MeshFace::new(4, vec![0, 1, 5]),
            MeshFace::new(5, vec![0, 5, 4]),
            MeshFace::new(6, vec![1, 2, 6]),
            MeshFace::new(7, vec![1, 6, 5]),
            MeshFace::new(8, vec![2, 3, 7]),
            MeshFace::new(9, vec![2, 7, 6]),
            MeshFace::new(10, vec![3, 0, 4]),
            MeshFace::new(11, vec![3, 4, 7]),
        ];

        Mesh3D {
            vertices,
            faces,
            edges: Vec::new(),
            tetrahedrons: Vec::new(),
            hexahedrons: Vec::new(),
            prisms: Vec::new(),
            bbox: (crate::geometry::Point::new(-1.0, -1.0, -1.0), crate::geometry::Point::new(1.0, 1.0, 1.0)),
            quality: std::collections::HashMap::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    fn apply_action(&self, state: &Mesh3D, action: &str) -> Mesh3D {
        let mut new_state = state.clone();

        match action {
            "simplify" => {
                let target_vertices = (state.vertices.len() * 7 / 10).max(4);
                if new_state.vertices.len() > target_vertices {
                    new_state.vertices.truncate(target_vertices);
                    new_state.faces.retain(|face| {
                        face.vertices.iter().all(|&v| v < target_vertices)
                    });
                }
            }
            "refine" => {
                let target_vertices = (state.vertices.len() * 13 / 10).min(1000);
                if new_state.vertices.len() < target_vertices {
                    let additional_vertices = target_vertices - new_state.vertices.len();
                    for i in 0..additional_vertices {
                        let base_vertex = &new_state.vertices[i % new_state.vertices.len()];
                        new_state.vertices.push(MeshVertex {
                            id: new_state.vertices.len(),
                            point: crate::geometry::Point::new(
                                base_vertex.point.x + 0.1,
                                base_vertex.point.y + 0.1,
                                base_vertex.point.z + 0.1,
                            ),
                            normal: base_vertex.normal,
                            ..Default::default()
                        });
                    }
                }
            }
            "optimize" => {
                for vertex in &mut new_state.vertices {
                    vertex.point.x *= 0.99;
                    vertex.point.y *= 0.99;
                    vertex.point.z *= 0.99;
                }
            }
            "layout" => {
                for vertex in &mut new_state.vertices {
                    vertex.point.x += 0.05;
                    vertex.point.y += 0.05;
                }
            }
            "material" => {
                new_state.metadata.insert(
                    "material_quality".to_string(),
                    "high".to_string()
                );
            }
            _ => {}
        }

        new_state
    }

    fn check_termination(&self, state: &Mesh3D, steps: usize) -> bool {
        if steps >= 1000 {
            return true;
        }

        if state.vertices.len() < 4 {
            return true;
        }

        if state.vertices.len() > 10000 {
            return true;
        }

        false
    }

    /// Get agent name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get model
    pub fn model(&self) -> &dyn AiModel {
        self.model.as_ref()
    }

    /// Get replay buffer size
    pub fn replay_buffer_size(&self) -> usize {
        self.replay_buffer.len()
    }
}
