use crate::topology::TopoDsShape;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use std::time::SystemTime;

#[cfg(feature = "websocket")]
use tokio::net::{TcpListener, TcpStream};
#[cfg(feature = "websocket")]
use tokio_tungstenite::tungstenite::protocol::Message;
#[cfg(feature = "websocket")]
use tokio_tungstenite::tungstenite::ServerTlsConfig;
#[cfg(feature = "websocket")]
use tokio_tungstenite::tungstenite::WebSocketStream;
#[cfg(feature = "websocket")]
use futures_util::{SinkExt, StreamExt};
#[cfg(feature = "websocket")]
use std::net::SocketAddr;
#[cfg(feature = "websocket")]
use tokio::sync::broadcast;

/// Real-time operation type
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RealTimeOperation {
    /// Create shape operation
    CreateShape {
        id: String,
        shape: TopoDsShape,
        timestamp: u64,
    },
    /// Update shape operation
    UpdateShape {
        id: String,
        shape: TopoDsShape,
        timestamp: u64,
    },
    /// Delete shape operation
    DeleteShape { id: String, timestamp: u64 },
    /// Move shape operation
    MoveShape {
        id: String,
        position: (f64, f64, f64),
        timestamp: u64,
    },
    /// Rotate shape operation
    RotateShape {
        id: String,
        rotation: (f64, f64, f64),
        timestamp: u64,
    },
    /// Scale shape operation
    ScaleShape {
        id: String,
        scale: (f64, f64, f64),
        timestamp: u64,
    },
    /// User join operation
    UserJoin {
        user_id: String,
        username: String,
        timestamp: u64,
    },
    /// User leave operation
    UserLeave { user_id: String, timestamp: u64 },
    /// User typing operation
    UserTyping {
        user_id: String,
        is_typing: bool,
        timestamp: u64,
    },
}

/// User state
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserState {
    pub user_id: String,
    pub username: String,
    pub is_online: bool,
    pub is_typing: bool,
    pub last_activity: u64,
    pub current_selection: Vec<String>,
}

impl UserState {
    /// Create a new user state
    pub fn new(user_id: &str, username: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            username: username.to_string(),
            is_online: true,
            is_typing: false,
            last_activity: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            current_selection: Vec::new(),
        }
    }

    /// Update last activity
    pub fn update_activity(&mut self) {
        self.last_activity = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
    }

    /// Set typing status
    pub fn set_typing(&mut self, is_typing: bool) {
        self.is_typing = is_typing;
        self.update_activity();
    }

    /// Set selection
    pub fn set_selection(&mut self, selection: Vec<String>) {
        self.current_selection = selection;
        self.update_activity();
    }

    /// Set online status
    pub fn set_online(&mut self, is_online: bool) {
        self.is_online = is_online;
        self.update_activity();
    }
}

/// Conflict resolution result
#[derive(Debug, Clone)]
pub enum ConflictResolutionResult {
    /// Operation applied
    Applied(RealTimeOperation),
    /// Operation rejected
    Rejected(String),
    /// Operation merged
    Merged(RealTimeOperation),
}

/// Real-time collaborative editor
pub struct RealTimeCollaborativeEditor {
    pub session_id: String,
    pub operations: Arc<Mutex<VecDeque<RealTimeOperation>>>,
    pub user_states: Arc<RwLock<HashMap<String, UserState>>>,
    pub shape_map: Arc<RwLock<HashMap<String, TopoDsShape>>>,
    pub operation_history: Arc<Mutex<Vec<RealTimeOperation>>>,
    pub is_running: Arc<Mutex<bool>>,
    pub conflict_resolution_strategy: ConflictResolutionStrategy,
}

/// Conflict resolution strategy for real-time editing
pub enum ConflictResolutionStrategy {
    /// Last write wins
    LastWriteWins,
    /// First write wins
    FirstWriteWins,
    /// Merge operations
    Merge,
    /// User preference
    UserPreference(String),
}

impl RealTimeCollaborativeEditor {
    /// Create a new real-time collaborative editor
    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            operations: Arc::new(Mutex::new(VecDeque::new())),
            user_states: Arc::new(RwLock::new(HashMap::new())),
            shape_map: Arc::new(RwLock::new(HashMap::new())),
            operation_history: Arc::new(Mutex::new(Vec::new())),
            is_running: Arc::new(Mutex::new(false)),
            conflict_resolution_strategy: ConflictResolutionStrategy::LastWriteWins,
        }
    }

    /// Start editor
    pub fn start(&mut self) {
        *self.is_running.lock().unwrap() = true;
    }

    /// Stop editor
    pub fn stop(&mut self) {
        *self.is_running.lock().unwrap() = false;
    }

    /// Add user
    pub fn add_user(&self, user_id: &str, username: &str) -> Result<(), String> {
        let mut user_states = self.user_states.write().unwrap();
        if user_states.contains_key(user_id) {
            return Err("User already exists".to_string());
        }

        user_states.insert(user_id.to_string(), UserState::new(user_id, username));

        // Broadcast user join operation
        let operation = RealTimeOperation::UserJoin {
            user_id: user_id.to_string(),
            username: username.to_string(),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        self.operations.lock().unwrap().push_back(operation.clone());
        self.operation_history.lock().unwrap().push(operation);

        Ok(())
    }

    /// Remove user
    pub fn remove_user(&self, user_id: &str) -> Result<(), String> {
        let mut user_states = self.user_states.write().unwrap();
        if !user_states.contains_key(user_id) {
            return Err("User not found".to_string());
        }

        user_states.remove(user_id);

        // Broadcast user leave operation
        let operation = RealTimeOperation::UserLeave {
            user_id: user_id.to_string(),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        };

        self.operations.lock().unwrap().push_back(operation.clone());
        self.operation_history.lock().unwrap().push(operation);

        Ok(())
    }

    /// Add operation
    pub fn add_operation(
        &self,
        operation: RealTimeOperation,
    ) -> Result<ConflictResolutionResult, String> {
        let result = self.resolve_conflict(&operation);

        match &result {
            ConflictResolutionResult::Applied(op) => {
                self.apply_operation(op);
                self.operations.lock().unwrap().push_back(op.clone());
                self.operation_history.lock().unwrap().push(op.clone());
            }
            ConflictResolutionResult::Merged(op) => {
                self.apply_operation(op);
                self.operations.lock().unwrap().push_back(op.clone());
                self.operation_history.lock().unwrap().push(op.clone());
            }
            ConflictResolutionResult::Rejected(reason) => {
                return Ok(ConflictResolutionResult::Rejected(reason.clone()));
            }
        }

        Ok(result)
    }

    /// Apply operation to local state
    pub fn apply_operation(&self, operation: &RealTimeOperation) {
        match operation {
            RealTimeOperation::CreateShape { id, shape, .. } => {
                let mut shape_map = self.shape_map.write().unwrap();
                shape_map.insert(id.clone(), shape.clone());
            }
            RealTimeOperation::UpdateShape { id, shape, .. } => {
                let mut shape_map = self.shape_map.write().unwrap();
                shape_map.insert(id.clone(), shape.clone());
            }
            RealTimeOperation::DeleteShape { id, .. } => {
                let mut shape_map = self.shape_map.write().unwrap();
                shape_map.remove(id);
            }
            RealTimeOperation::MoveShape { id, position, .. } => {
                let mut shape_map = self.shape_map.write().unwrap();
                if let Some(shape) = shape_map.get_mut(id) {
                    // Implementation of shape movement
                    use crate::geometry::{Transform, Vector};
                    let vec = Vector::new(position.0, position.1, position.2);
                    let transform = Transform::from_translation(&vec);
                    shape.transform(&transform);
                    println!("Moving shape {} to {:?}", id, position);
                }
            }
            RealTimeOperation::RotateShape { id, rotation, .. } => {
                let mut shape_map = self.shape_map.write().unwrap();
                if let Some(shape) = shape_map.get_mut(id) {
                    // Implementation of shape rotation
                    use crate::geometry::{Transform, Axis, Point, Direction};
                    let axis = Axis::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, 0.0, 1.0));
                    let transform = Transform::from_rotation(&axis, rotation.2);
                    shape.transform(&transform);
                    println!("Rotating shape {} by {:?}", id, rotation);
                }
            }
            RealTimeOperation::ScaleShape { id, scale, .. } => {
                let mut shape_map = self.shape_map.write().unwrap();
                if let Some(shape) = shape_map.get_mut(id) {
                    // Implementation of shape scaling
                    use crate::geometry::{Transform, Matrix};
                    let scaling_matrix = Matrix::from_column_major(
                        [scale.0, 0.0, 0.0],
                        [0.0, scale.1, 0.0],
                        [0.0, 0.0, scale.2]
                    );
                    let transform = Transform::from_matrix(&scaling_matrix);
                    shape.transform(&transform);
                    println!("Scaling shape {} by {:?}", id, scale);
                }
            }
            RealTimeOperation::UserJoin {
                user_id, username, ..
            } => {
                let mut user_states = self.user_states.write().unwrap();
                user_states.insert(user_id.clone(), UserState::new(user_id, username));
            }
            RealTimeOperation::UserLeave { user_id, .. } => {
                let mut user_states = self.user_states.write().unwrap();
                user_states.remove(user_id);
            }
            RealTimeOperation::UserTyping {
                user_id, is_typing, ..
            } => {
                let mut user_states = self.user_states.write().unwrap();
                if let Some(user) = user_states.get_mut(user_id) {
                    user.set_typing(*is_typing);
                }
            }
        }
    }

    /// Resolve conflict
    pub fn resolve_conflict(&self, operation: &RealTimeOperation) -> ConflictResolutionResult {
        match self.conflict_resolution_strategy {
            ConflictResolutionStrategy::LastWriteWins => {
                // Last write wins strategy
                ConflictResolutionResult::Applied(operation.clone())
            }
            ConflictResolutionStrategy::FirstWriteWins => {
                // First write wins strategy
                // Check if there's an existing operation for the same shape
                let history = self.operation_history.lock().unwrap();
                let shape_id = match operation {
                    RealTimeOperation::CreateShape { id, .. } => Some(id),
                    RealTimeOperation::UpdateShape { id, .. } => Some(id),
                    RealTimeOperation::DeleteShape { id, .. } => Some(id),
                    RealTimeOperation::MoveShape { id, .. } => Some(id),
                    RealTimeOperation::RotateShape { id, .. } => Some(id),
                    RealTimeOperation::ScaleShape { id, .. } => Some(id),
                    _ => None,
                };

                if let Some(shape_id) = shape_id {
                    for op in history.iter().rev() {
                        match op {
                            RealTimeOperation::CreateShape { id, .. } if id == shape_id => {
                                return ConflictResolutionResult::Rejected(
                                    "First write wins".to_string(),
                                );
                            }
                            RealTimeOperation::UpdateShape { id, .. } if id == shape_id => {
                                return ConflictResolutionResult::Rejected(
                                    "First write wins".to_string(),
                                );
                            }
                            RealTimeOperation::DeleteShape { id, .. } if id == shape_id => {
                                return ConflictResolutionResult::Rejected(
                                    "First write wins".to_string(),
                                );
                            }
                            RealTimeOperation::MoveShape { id, .. } if id == shape_id => {
                                return ConflictResolutionResult::Rejected(
                                    "First write wins".to_string(),
                                );
                            }
                            RealTimeOperation::RotateShape { id, .. } if id == shape_id => {
                                return ConflictResolutionResult::Rejected(
                                    "First write wins".to_string(),
                                );
                            }
                            RealTimeOperation::ScaleShape { id, .. } if id == shape_id => {
                                return ConflictResolutionResult::Rejected(
                                    "First write wins".to_string(),
                                );
                            }
                            _ => {}
                        }
                    }
                }

                ConflictResolutionResult::Applied(operation.clone())
            }
            ConflictResolutionStrategy::Merge => {
                // Merge strategy
                ConflictResolutionResult::Merged(operation.clone())
            }
            ConflictResolutionStrategy::UserPreference(ref preferred_user_id) => {
                // User preference strategy
                // Check if operation is from preferred user
                let op_user_id = match operation {
                    RealTimeOperation::CreateShape { .. } => None,
                    RealTimeOperation::UpdateShape { .. } => None,
                    RealTimeOperation::DeleteShape { .. } => None,
                    RealTimeOperation::MoveShape { .. } => None,
                    RealTimeOperation::RotateShape { .. } => None,
                    RealTimeOperation::ScaleShape { .. } => None,
                    RealTimeOperation::UserJoin { user_id, .. } => Some(user_id),
                    RealTimeOperation::UserLeave { user_id, .. } => Some(user_id),
                    RealTimeOperation::UserTyping { user_id, .. } => Some(user_id),
                };

                if op_user_id == Some(preferred_user_id) {
                    ConflictResolutionResult::Applied(operation.clone())
                } else {
                    ConflictResolutionResult::Rejected("User preference".to_string())
                }
            }
        }
    }

    /// Get next operation to process
    pub fn get_next_operation(&self) -> Option<RealTimeOperation> {
        self.operations.lock().unwrap().pop_front()
    }

    /// Get user states
    pub fn get_user_states(&self) -> HashMap<String, UserState> {
        self.user_states.read().unwrap().clone()
    }

    /// Get shape map
    pub fn get_shape_map(&self) -> HashMap<String, TopoDsShape> {
        self.shape_map.read().unwrap().clone()
    }

    /// Get operation history
    pub fn get_operation_history(&self) -> Vec<RealTimeOperation> {
        self.operation_history.lock().unwrap().clone()
    }

    /// Set conflict resolution strategy
    pub fn set_conflict_resolution_strategy(&mut self, strategy: ConflictResolutionStrategy) {
        self.conflict_resolution_strategy = strategy;
    }

    /// Broadcast operation to all participants
    pub fn broadcast_operation(&self, operation: &RealTimeOperation) {
        // Implementation of operation broadcasting
        // This would typically use a network protocol like WebSocket
        println!("Broadcasting operation: {:?}", operation);
    }

    /// Handle incoming operation from network
    pub fn handle_incoming_operation(
        &self,
        _operation: RealTimeOperation,
    ) -> Result<ConflictResolutionResult, String> {
        self.add_operation(_operation)
    }

    /// Update user typing status
    pub fn update_user_typing(&self, user_id: &str, is_typing: bool) -> Result<(), String> {
        let mut user_states = self.user_states.write().unwrap();
        if let Some(user) = user_states.get_mut(user_id) {
            user.set_typing(is_typing);

            // Broadcast typing status
            let operation = RealTimeOperation::UserTyping {
                user_id: user_id.to_string(),
                is_typing,
                timestamp: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            };

            self.operations.lock().unwrap().push_back(operation.clone());
            self.operation_history.lock().unwrap().push(operation);

            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }

    /// Update user selection
    pub fn update_user_selection(
        &self,
        user_id: &str,
        selection: Vec<String>,
    ) -> Result<(), String> {
        let mut user_states = self.user_states.write().unwrap();
        if let Some(user) = user_states.get_mut(user_id) {
            user.set_selection(selection);
            Ok(())
        } else {
            Err("User not found".to_string())
        }
    }
}

/// Collaborative editing server
pub struct CollaborativeEditingServer {
    pub editors: Arc<RwLock<HashMap<String, Arc<RealTimeCollaborativeEditor>>>>,
    pub port: u16,
    pub is_running: Arc<Mutex<bool>>,
    #[cfg(feature = "websocket")]
    pub broadcast_tx: Option<Arc<broadcast::Sender<Message>>>,
}

impl CollaborativeEditingServer {
    /// Create a new collaborative editing server
    pub fn new(port: u16) -> Self {
        Self {
            editors: Arc::new(RwLock::new(HashMap::new())),
            port,
            is_running: Arc::new(Mutex::new(false)),
            #[cfg(feature = "websocket")]
            broadcast_tx: None,
        }
    }

    /// Start server
    pub fn start(&mut self) -> Result<(), String> {
        *self.is_running.lock().unwrap() = true;
        println!("Starting collaborative editing server on port {}", self.port);

        #[cfg(feature = "websocket")]
        {
            use tokio::net::TcpListener;
            use tokio_tungstenite::tungstenite::protocol::Message;
            use tokio_tungstenite::tungstenite::accept_hdr;
            use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
            use tokio_tungstenite::tungstenite::ServerTlsConfig;
            use tokio_tungstenite::tungstenite::WebSocketStream;
            use futures_util::{SinkExt, StreamExt};
            use tokio::sync::broadcast;

            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| format!("Failed to create runtime: {}", e))?;

            let (broadcast_tx, _rx) = broadcast::channel::<Message>(16);
            self.broadcast_tx = Some(Arc::new(broadcast_tx.clone()));

            let addr = format!("0.0.0.0:{}", self.port);
            let listener = rt.block_on(TcpListener::bind(&addr))
                .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;

            println!("Server listening on {}", addr);

            let editors_clone = self.editors.clone();
            let is_running_clone = self.is_running.clone();
            let broadcast_tx_clone = broadcast_tx.clone();

            rt.block_on(async {
                while *is_running_clone.lock().await {
                    match listener.accept().await {
                        Ok((stream, addr)) => {
                            println!("New connection from {}", addr);
                            
                            let mut ws_stream = accept_hdr(stream, &Request::default(), &Response::default())
                                .await
                                .map_err(|e| format!("Failed to accept WebSocket: {}", e))?;

                            let mut rx = ws_stream.fuse();
                            let mut tx = broadcast_tx_clone.clone();

                            tokio::spawn(async move {
                                while let Some(msg_result) = rx.next().await {
                                    match msg_result {
                                        Ok(msg) => {
                                            if ws_stream.send(msg).await.is_err() {
                                                break;
                                            }
                                        }
                                        Err(_) => break,
                                    }
                                }
                            });

                            tokio::spawn(async move {
                                while let Some(msg) = rx.next().await {
                                    if let Ok(msg) = msg {
                                        if let Ok(json) = serde_json::to_string(&msg) {
                                            if let Ok(operation) = serde_json::from_str::<RealTimeOperation>(&json) {
                                                let editors = editors_clone.read().await;
                                                for (_session_id, editor) in editors.iter() {
                                                    editor.handle_incoming_operation(operation.clone());
                                                }
                                            }
                                        }
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("Error accepting connection: {}", e);
                        }
                    }
                }
            }).map_err(|e| format!("Runtime error: {}", e))?;
        }

        #[cfg(not(feature = "websocket"))]
        {
            println!("WebSocket feature not enabled, running in simulation mode");
        }

        Ok(())
    }

    /// Stop server
    pub fn stop(&mut self) -> Result<(), String> {
        *self.is_running.lock().unwrap() = false;
        println!("Stopping collaborative editing server");
        Ok(())
    }

    /// Create collaboration session
    pub fn create_session(
        &self,
        session_id: &str,
    ) -> Result<Arc<RealTimeCollaborativeEditor>, String> {
        let mut editors = self.editors.write().unwrap();
        if editors.contains_key(session_id) {
            return Err("Session already exists".to_string());
        }

        let editor = Arc::new(RealTimeCollaborativeEditor::new(session_id));
        editors.insert(session_id.to_string(), editor.clone());

        Ok(editor)
    }

    /// Get collaboration session
    pub fn get_session(&self, session_id: &str) -> Option<Arc<RealTimeCollaborativeEditor>> {
        self.editors.read().unwrap().get(session_id).cloned()
    }

    /// Remove collaboration session
    pub fn remove_session(&self, session_id: &str) -> Result<(), String> {
        let mut editors = self.editors.write().unwrap();
        if !editors.contains_key(session_id) {
            return Err("Session not found".to_string());
        }

        editors.remove(session_id);
        Ok(())
    }

    /// List active sessions
    pub fn list_sessions(&self) -> Vec<String> {
        self.editors.read().unwrap().keys().cloned().collect()
    }
}

/// Collaborative editing client
pub struct CollaborativeEditingClient {
    pub session_id: String,
    pub user_id: String,
    pub username: String,
    pub server_url: String,
    pub is_connected: Arc<Mutex<bool>>,
    pub editor: Option<Arc<RealTimeCollaborativeEditor>>,
    #[cfg(feature = "websocket")]
    pub ws_stream: Option<WebSocketStream<TcpStream>>,
}

impl CollaborativeEditingClient {
    /// Create a new collaborative editing client
    pub fn new(session_id: &str, user_id: &str, username: &str, server_url: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            user_id: user_id.to_string(),
            username: username.to_string(),
            server_url: server_url.to_string(),
            is_connected: Arc::new(Mutex::new(false)),
            editor: None,
            #[cfg(feature = "websocket")]
            ws_stream: None,
        }
    }

    /// Connect to server
    pub fn connect(&mut self) -> Result<(), String> {
        println!("Connecting to collaborative editing server at {}", self.server_url);
        *self.is_connected.lock().unwrap() = true;

        #[cfg(feature = "websocket")]
        {
            use tokio_tungstenite::tungstenite::protocol::Message;
            use tokio_tungstenite::tungstenite::handshake::client::{Request, Response};
            use tokio_tungstenite::tungstenite::WebSocketStream;
            use futures_util::{SinkExt, StreamExt};

            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| format!("Failed to create runtime: {}", e))?;

            let request = Request::builder()
                .uri(&self.server_url)
                .build()
                .map_err(|e| format!("Failed to build request: {}", e))?;

            let (stream, _) = rt.block_on(tokio::net::TcpStream::connect(&self.server_url))
                .map_err(|e| format!("Failed to connect to {}: {}", self.server_url, e))?;

            let mut ws_stream = rt.block_on(WebSocketStream::from_tcp_stream(stream, request))
                .map_err(|e| format!("Failed to upgrade to WebSocket: {}", e))?;

            self.ws_stream = Some(ws_stream);

            println!("Connected to server");
        }

        #[cfg(not(feature = "websocket"))]
        {
            println!("WebSocket feature not enabled, running in simulation mode");
        }

        Ok(())
    }

    /// Disconnect from server
    pub fn disconnect(&mut self) -> Result<(), String> {
        println!("Disconnecting from collaborative editing server");
        *self.is_connected.lock().unwrap() = false;

        #[cfg(feature = "websocket")]
        {
            if let Some(mut ws_stream) = self.ws_stream.take() {
                let rt = tokio::runtime::Runtime::new()
                    .map_err(|e| format!("Failed to create runtime: {}", e))?;

                rt.block_on(async {
                    let _ = ws_stream.close(None).await;
                }).map_err(|e| format!("Failed to close connection: {}", e))?;
            }
        }

        #[cfg(not(feature = "websocket"))]
        {
            println!("WebSocket feature not enabled, running in simulation mode");
        }

        Ok(())
    }

    /// Join session
    pub fn join_session(&mut self) -> Result<Arc<RealTimeCollaborativeEditor>, String> {
        if !*self.is_connected.lock().unwrap() {
            return Err("Not connected to server".to_string());
        }

        let editor = Arc::new(RealTimeCollaborativeEditor::new(&self.session_id));
        editor.add_user(&self.user_id, &self.username)?;
        self.editor = Some(editor.clone());

        Ok(editor)
    }

    /// Leave session
    pub fn leave_session(&mut self) -> Result<(), String> {
        if let Some(editor) = &self.editor {
            editor.remove_user(&self.user_id)?;
            self.editor = None;
            Ok(())
        } else {
            Err("Not in a session".to_string())
        }
    }

    /// Send operation
    pub fn send_operation(
        &self,
        operation: RealTimeOperation,
    ) -> Result<ConflictResolutionResult, String> {
        if let Some(editor) = &self.editor {
            editor.add_operation(operation)
        } else {
            Err("Not in a session".to_string())
        }
    }

    /// Get editor
    pub fn get_editor(&self) -> Option<Arc<RealTimeCollaborativeEditor>> {
        self.editor.clone()
    }

    /// Is connected
    pub fn is_connected(&self) -> bool {
        *self.is_connected.lock().unwrap()
    }
}
