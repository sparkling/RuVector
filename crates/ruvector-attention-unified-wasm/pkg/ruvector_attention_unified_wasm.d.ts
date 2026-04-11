/* tslint:disable */
/* eslint-disable */

/**
 * Factory for creating DAG attention mechanisms
 */
export class DagAttentionFactory {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Get available DAG attention types
     */
    static availableTypes(): any;
    /**
     * Get description for a DAG attention type
     */
    static getDescription(attention_type: string): string;
}

/**
 * Factory for graph attention information
 */
export class GraphAttentionFactory {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Get available graph attention types
     */
    static availableTypes(): any;
    /**
     * Get description for a graph attention type
     */
    static getDescription(attention_type: string): string;
    /**
     * Get recommended use cases for a graph attention type
     */
    static getUseCases(attention_type: string): any;
}

/**
 * Graph attention mechanism types
 */
export enum GraphAttentionType {
    /**
     * Graph Attention Networks (Velickovic et al., 2018)
     */
    GAT = 0,
    /**
     * Graph Convolutional Networks (Kipf & Welling, 2017)
     */
    GCN = 1,
    /**
     * GraphSAGE (Hamilton et al., 2017)
     */
    GraphSAGE = 2,
}

/**
 * Hybrid layer combining Mamba SSM with standard attention
 *
 * Uses Mamba for long-range dependencies and attention for local patterns
 */
export class HybridMambaAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Forward pass
     */
    forward(input: Float32Array, seq_len: number): Float32Array;
    /**
     * Create a new hybrid Mamba-Attention layer
     */
    constructor(config: MambaConfig, local_window: number);
    /**
     * Get local window size
     */
    readonly localWindow: number;
}

/**
 * Configuration for Mamba SSM attention
 */
export class MambaConfig {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Create a new Mamba configuration
     */
    constructor(dim: number);
    /**
     * Set convolution kernel size
     */
    withConvKernelSize(size: number): MambaConfig;
    /**
     * Set expansion factor
     */
    withExpandFactor(factor: number): MambaConfig;
    /**
     * Set state space dimension
     */
    withStateDim(state_dim: number): MambaConfig;
    /**
     * Convolution kernel size
     */
    conv_kernel_size: number;
    /**
     * Model dimension (d_model)
     */
    dim: number;
    /**
     * Delta range maximum
     */
    dt_max: number;
    /**
     * Delta (discretization step) range minimum
     */
    dt_min: number;
    /**
     * Expansion factor for inner dimension
     */
    expand_factor: number;
    /**
     * State space dimension (n)
     */
    state_dim: number;
    /**
     * Whether to use learnable D skip connection
     */
    use_d_skip: boolean;
}

/**
 * Mamba Selective State Space Model for sequence attention
 *
 * Provides O(n) attention-like mechanism using selective state spaces
 */
export class MambaSSMAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Forward pass through Mamba SSM
     *
     * # Arguments
     * * `input` - Input sequence (seq_len, dim) flattened to 1D
     * * `seq_len` - Sequence length
     *
     * # Returns
     * Output sequence (seq_len, dim) flattened to 1D
     */
    forward(input: Float32Array, seq_len: number): Float32Array;
    /**
     * Compute attention-like scores (for visualization/analysis)
     *
     * Returns pseudo-attention scores showing which positions influence output
     */
    getAttentionScores(input: Float32Array, seq_len: number): Float32Array;
    /**
     * Create a new Mamba SSM attention layer
     */
    constructor(config: MambaConfig);
    /**
     * Create with default configuration
     */
    static withDefaults(dim: number): MambaSSMAttention;
    /**
     * Get the configuration
     */
    readonly config: MambaConfig;
    /**
     * Get the inner dimension
     */
    readonly innerDim: number;
}

/**
 * Unified attention mechanism selector
 * Automatically routes to the appropriate attention implementation
 */
export class UnifiedAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Create a new unified attention selector
     */
    constructor(mechanism: string);
    /**
     * Check if this mechanism supports graph/DAG structures
     */
    supportsGraphs(): boolean;
    /**
     * Check if this mechanism supports hyperbolic geometry
     */
    supportsHyperbolic(): boolean;
    /**
     * Check if this mechanism supports sequence processing
     */
    supportsSequences(): boolean;
    /**
     * Get the category of the selected mechanism
     */
    readonly category: string;
    /**
     * Get the currently selected mechanism type
     */
    readonly mechanism: string;
}

/**
 * Causal cone attention based on dependency lightcones
 *
 * Nodes can only attend to ancestors in the DAG (causal predecessors).
 * Attention strength decays with causal distance.
 */
export class WasmCausalConeAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compute attention scores for the DAG
     */
    forward(dag: WasmQueryDag): Float32Array;
    /**
     * Create a new causal cone attention instance
     *
     * # Arguments
     * * `future_discount` - Discount for future nodes
     * * `ancestor_weight` - Weight for ancestor influence
     */
    constructor(future_discount: number, ancestor_weight: number);
}

/**
 * Critical path attention weighted by path criticality
 *
 * Nodes on or near the critical path (longest execution path)
 * receive higher attention scores.
 */
export class WasmCriticalPathAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compute attention scores for the DAG
     */
    forward(dag: WasmQueryDag): Float32Array;
    /**
     * Create a new critical path attention instance
     *
     * # Arguments
     * * `path_weight` - Weight for critical path membership
     * * `branch_penalty` - Penalty for branching nodes
     */
    constructor(path_weight: number, branch_penalty: number);
}

/**
 * Flash attention with memory-efficient tiling
 *
 * Reduces memory usage from O(n^2) to O(n) by computing attention
 * in blocks and fusing operations
 */
export class WasmFlashAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compute flash attention
     */
    compute(query: Float32Array, keys: any, values: any): Float32Array;
    /**
     * Create a new flash attention instance
     *
     * # Arguments
     * * `dim` - Embedding dimension
     * * `block_size` - Block size for tiled computation
     */
    constructor(dim: number, block_size: number);
}

/**
 * Graph Neural Network layer with attention mechanism
 *
 * Implements Graph Attention Networks (GAT) for HNSW topology.
 * Each node aggregates information from neighbors using learned attention weights.
 */
export class WasmGNNLayer {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Forward pass through the GNN layer
     *
     * # Arguments
     * * `node_embedding` - Current node's embedding (Float32Array)
     * * `neighbor_embeddings` - Embeddings of neighbor nodes (array of Float32Arrays)
     * * `edge_weights` - Weights of edges to neighbors (Float32Array)
     *
     * # Returns
     * Updated node embedding (Float32Array)
     */
    forward(node_embedding: Float32Array, neighbor_embeddings: any, edge_weights: Float32Array): Float32Array;
    /**
     * Create a new GNN layer with attention
     *
     * # Arguments
     * * `input_dim` - Dimension of input node embeddings
     * * `hidden_dim` - Dimension of hidden representations
     * * `heads` - Number of attention heads
     * * `dropout` - Dropout rate (0.0 to 1.0)
     */
    constructor(input_dim: number, hidden_dim: number, heads: number, dropout: number);
    /**
     * Get the output dimension
     */
    readonly outputDim: number;
}

/**
 * Hierarchical Lorentz attention in hyperbolic space
 *
 * Combines DAG hierarchy with Lorentz (hyperboloid) geometry
 * for multi-scale hierarchical attention.
 */
export class WasmHierarchicalLorentzAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compute attention scores for the DAG
     */
    forward(dag: WasmQueryDag): Float32Array;
    /**
     * Create a new hierarchical Lorentz attention instance
     *
     * # Arguments
     * * `curvature` - Hyperbolic curvature parameter
     * * `temperature` - Temperature for softmax
     */
    constructor(curvature: number, temperature: number);
}

/**
 * Hyperbolic attention mechanism for hierarchical data
 *
 * Operates in hyperbolic space (Poincare ball model) which naturally
 * represents tree-like hierarchical structures with exponential capacity
 */
export class WasmHyperbolicAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compute hyperbolic attention
     */
    compute(query: Float32Array, keys: any, values: any): Float32Array;
    /**
     * Create a new hyperbolic attention instance
     *
     * # Arguments
     * * `dim` - Embedding dimension
     * * `curvature` - Hyperbolic curvature parameter (negative for hyperbolic space)
     */
    constructor(dim: number, curvature: number);
    /**
     * Get the curvature parameter
     */
    readonly curvature: number;
}

/**
 * Linear attention using random feature approximation
 *
 * Achieves O(n) complexity instead of O(n^2) by approximating
 * the softmax kernel with random Fourier features
 */
export class WasmLinearAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compute linear attention
     */
    compute(query: Float32Array, keys: any, values: any): Float32Array;
    /**
     * Create a new linear attention instance
     *
     * # Arguments
     * * `dim` - Embedding dimension
     * * `num_features` - Number of random features for kernel approximation
     */
    constructor(dim: number, num_features: number);
}

/**
 * Local-global sparse attention (Longformer-style)
 *
 * Combines local sliding window attention with global tokens
 * for efficient long-range dependencies
 */
export class WasmLocalGlobalAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compute local-global attention
     */
    compute(query: Float32Array, keys: any, values: any): Float32Array;
    /**
     * Create a new local-global attention instance
     *
     * # Arguments
     * * `dim` - Embedding dimension
     * * `local_window` - Size of local attention window
     * * `global_tokens` - Number of global attention tokens
     */
    constructor(dim: number, local_window: number, global_tokens: number);
}

/**
 * MinCut-gated attention using flow-based bottleneck detection
 *
 * Uses minimum cut analysis to identify bottleneck nodes
 * and gates attention through these critical points.
 */
export class WasmMinCutGatedAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compute attention scores for the DAG
     */
    forward(dag: WasmQueryDag): Float32Array;
    /**
     * Create a new MinCut-gated attention instance
     *
     * # Arguments
     * * `gate_threshold` - Threshold for gating (0.0-1.0)
     */
    constructor(gate_threshold: number);
}

/**
 * Mixture of Experts attention mechanism
 *
 * Routes queries to specialized expert attention heads based on
 * learned gating functions for capacity-efficient computation
 */
export class WasmMoEAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compute MoE attention
     */
    compute(query: Float32Array, keys: any, values: any): Float32Array;
    /**
     * Create a new MoE attention instance
     *
     * # Arguments
     * * `dim` - Embedding dimension
     * * `num_experts` - Number of expert attention mechanisms
     * * `top_k` - Number of experts to activate per query
     */
    constructor(dim: number, num_experts: number, top_k: number);
}

/**
 * Multi-head attention mechanism
 *
 * Splits input into multiple heads, applies attention, and concatenates results
 */
export class WasmMultiHeadAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compute multi-head attention
     *
     * # Arguments
     * * `query` - Query vector
     * * `keys` - Array of key vectors
     * * `values` - Array of value vectors
     */
    compute(query: Float32Array, keys: any, values: any): Float32Array;
    /**
     * Create a new multi-head attention instance
     *
     * # Arguments
     * * `dim` - Embedding dimension (must be divisible by num_heads)
     * * `num_heads` - Number of parallel attention heads
     */
    constructor(dim: number, num_heads: number);
    /**
     * Get the embedding dimension
     */
    readonly dim: number;
    /**
     * Get the dimension per head
     */
    readonly headDim: number;
    /**
     * Get the number of attention heads
     */
    readonly numHeads: number;
}

/**
 * Parallel branch attention for concurrent DAG branches
 *
 * Identifies parallel branches in the DAG and applies
 * attention patterns that respect branch independence.
 */
export class WasmParallelBranchAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compute attention scores for the DAG
     */
    forward(dag: WasmQueryDag): Float32Array;
    /**
     * Create a new parallel branch attention instance
     *
     * # Arguments
     * * `max_branches` - Maximum number of branches to consider
     * * `sync_penalty` - Penalty for synchronization between branches
     */
    constructor(max_branches: number, sync_penalty: number);
}

/**
 * Minimal DAG structure for WASM attention computation
 */
export class WasmQueryDag {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Add an edge between nodes
     *
     * # Arguments
     * * `from` - Source node ID
     * * `to` - Target node ID
     *
     * # Returns
     * True if edge was added successfully
     */
    addEdge(from: number, to: number): boolean;
    /**
     * Add a node with operator type and cost
     *
     * # Arguments
     * * `op_type` - Operator type: "scan", "filter", "join", "aggregate", "project", "sort"
     * * `cost` - Estimated execution cost
     *
     * # Returns
     * Node ID
     */
    addNode(op_type: string, cost: number): number;
    /**
     * Create a new empty DAG
     */
    constructor();
    /**
     * Serialize to JSON
     */
    toJson(): string;
    /**
     * Get the number of edges
     */
    readonly edgeCount: number;
    /**
     * Get the number of nodes
     */
    readonly nodeCount: number;
}

/**
 * Search configuration for differentiable search
 */
export class WasmSearchConfig {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Create a new search configuration
     */
    constructor(k: number, temperature: number);
    /**
     * Number of top results to return
     */
    k: number;
    /**
     * Temperature for softmax
     */
    temperature: number;
}

/**
 * Temporal BTSP (Behavioral Time-Series Pattern) attention
 *
 * Incorporates temporal patterns and behavioral sequences
 * for time-aware DAG attention.
 */
export class WasmTemporalBTSPAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compute attention scores for the DAG
     */
    forward(dag: WasmQueryDag): Float32Array;
    /**
     * Create a new temporal BTSP attention instance
     *
     * # Arguments
     * * `eligibility_decay` - Decay rate for eligibility traces (0.0-1.0)
     * * `baseline_attention` - Baseline attention for nodes without history
     */
    constructor(eligibility_decay: number, baseline_attention: number);
}

/**
 * Tensor compressor with adaptive level selection
 *
 * Compresses embeddings based on access frequency for memory-efficient GNN
 */
export class WasmTensorCompress {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compress an embedding based on access frequency
     *
     * # Arguments
     * * `embedding` - The input embedding vector
     * * `access_freq` - Access frequency in range [0.0, 1.0]
     *   - f > 0.8: Full precision (hot data)
     *   - f > 0.4: Half precision (warm data)
     *   - f > 0.1: 8-bit PQ (cool data)
     *   - f > 0.01: 4-bit PQ (cold data)
     *   - f <= 0.01: Binary (archive)
     */
    compress(embedding: Float32Array, access_freq: number): any;
    /**
     * Compress with explicit compression level
     *
     * # Arguments
     * * `embedding` - The input embedding vector
     * * `level` - Compression level: "none", "half", "pq8", "pq4", "binary"
     */
    compressWithLevel(embedding: Float32Array, level: string): any;
    /**
     * Decompress a compressed tensor
     */
    decompress(compressed: any): Float32Array;
    /**
     * Get compression ratio estimate for a given access frequency
     */
    getCompressionRatio(access_freq: number): number;
    /**
     * Create a new tensor compressor
     */
    constructor();
}

/**
 * Topological attention based on DAG position
 *
 * Assigns attention scores based on node position in topological order.
 * Earlier nodes (closer to sources) get higher attention.
 */
export class WasmTopologicalAttention {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compute attention scores for the DAG
     *
     * # Returns
     * Attention scores for each node
     */
    forward(dag: WasmQueryDag): Float32Array;
    /**
     * Create a new topological attention instance
     *
     * # Arguments
     * * `decay_factor` - Decay factor for position-based attention (0.0-1.0)
     */
    constructor(decay_factor: number);
}

/**
 * Get information about all available attention mechanisms
 */
export function availableMechanisms(): any;

/**
 * Compute cosine similarity between two vectors
 */
export function cosineSimilarity(a: Float32Array, b: Float32Array): number;

/**
 * Get summary statistics about the unified attention library
 */
export function getStats(): any;

/**
 * Differentiable search using soft attention mechanism
 *
 * # Arguments
 * * `query` - The query vector
 * * `candidate_embeddings` - List of candidate embedding vectors
 * * `config` - Search configuration
 *
 * # Returns
 * Object with indices and weights for top-k candidates
 */
export function graphDifferentiableSearch(query: Float32Array, candidate_embeddings: any, config: WasmSearchConfig): any;

/**
 * Hierarchical forward pass through multiple GNN layers
 *
 * # Arguments
 * * `query` - The query vector
 * * `layer_embeddings` - Embeddings organized by layer
 * * `gnn_layers` - Array of GNN layers
 *
 * # Returns
 * Final embedding after hierarchical processing
 */
export function graphHierarchicalForward(query: Float32Array, layer_embeddings: any, gnn_layers: WasmGNNLayer[]): Float32Array;

/**
 * Initialize the WASM module with panic hook for better error messages
 */
export function init(): void;

/**
 * Compute scaled dot-product attention
 *
 * Standard transformer attention: softmax(QK^T / sqrt(d)) * V
 *
 * # Arguments
 * * `query` - Query vector (Float32Array)
 * * `keys` - Array of key vectors (JsValue - array of Float32Arrays)
 * * `values` - Array of value vectors (JsValue - array of Float32Arrays)
 * * `scale` - Optional scaling factor (defaults to 1/sqrt(dim))
 *
 * # Returns
 * Attention-weighted output vector
 */
export function scaledDotAttention(query: Float32Array, keys: any, values: any, scale?: number | null): Float32Array;

/**
 * Softmax normalization
 */
export function softmax(values: Float32Array): Float32Array;

/**
 * Temperature-scaled softmax
 */
export function temperatureSoftmax(values: Float32Array, temperature: number): Float32Array;

/**
 * Get the version of the unified attention WASM crate
 */
export function version(): string;
