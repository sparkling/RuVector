/* @ts-self-types="./ruvector_attention_unified_wasm.d.ts" */

/**
 * Factory for creating DAG attention mechanisms
 */
class DagAttentionFactory {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DagAttentionFactoryFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_dagattentionfactory_free(ptr, 0);
    }
    /**
     * Get available DAG attention types
     * @returns {any}
     */
    static availableTypes() {
        const ret = wasm.dagattentionfactory_availableTypes();
        return takeObject(ret);
    }
    /**
     * Get description for a DAG attention type
     * @param {string} attention_type
     * @returns {string}
     */
    static getDescription(attention_type) {
        let deferred2_0;
        let deferred2_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(attention_type, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            wasm.dagattentionfactory_getDescription(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred2_0 = r0;
            deferred2_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred2_0, deferred2_1, 1);
        }
    }
}
if (Symbol.dispose) DagAttentionFactory.prototype[Symbol.dispose] = DagAttentionFactory.prototype.free;
exports.DagAttentionFactory = DagAttentionFactory;

/**
 * Factory for graph attention information
 */
class GraphAttentionFactory {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GraphAttentionFactoryFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_graphattentionfactory_free(ptr, 0);
    }
    /**
     * Get available graph attention types
     * @returns {any}
     */
    static availableTypes() {
        const ret = wasm.graphattentionfactory_availableTypes();
        return takeObject(ret);
    }
    /**
     * Get description for a graph attention type
     * @param {string} attention_type
     * @returns {string}
     */
    static getDescription(attention_type) {
        let deferred2_0;
        let deferred2_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(attention_type, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            wasm.graphattentionfactory_getDescription(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred2_0 = r0;
            deferred2_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * Get recommended use cases for a graph attention type
     * @param {string} attention_type
     * @returns {any}
     */
    static getUseCases(attention_type) {
        const ptr0 = passStringToWasm0(attention_type, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.graphattentionfactory_getUseCases(ptr0, len0);
        return takeObject(ret);
    }
}
if (Symbol.dispose) GraphAttentionFactory.prototype[Symbol.dispose] = GraphAttentionFactory.prototype.free;
exports.GraphAttentionFactory = GraphAttentionFactory;

/**
 * Graph attention mechanism types
 * @enum {0 | 1 | 2}
 */
const GraphAttentionType = Object.freeze({
    /**
     * Graph Attention Networks (Velickovic et al., 2018)
     */
    GAT: 0, "0": "GAT",
    /**
     * Graph Convolutional Networks (Kipf & Welling, 2017)
     */
    GCN: 1, "1": "GCN",
    /**
     * GraphSAGE (Hamilton et al., 2017)
     */
    GraphSAGE: 2, "2": "GraphSAGE",
});
exports.GraphAttentionType = GraphAttentionType;

/**
 * Hybrid layer combining Mamba SSM with standard attention
 *
 * Uses Mamba for long-range dependencies and attention for local patterns
 */
class HybridMambaAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        HybridMambaAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_hybridmambaattention_free(ptr, 0);
    }
    /**
     * Forward pass
     * @param {Float32Array} input
     * @param {number} seq_len
     * @returns {Float32Array}
     */
    forward(input, seq_len) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayF32ToWasm0(input, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.hybridmambaattention_forward(retptr, this.__wbg_ptr, ptr0, len0, seq_len);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v2 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get local window size
     * @returns {number}
     */
    get localWindow() {
        const ret = wasm.hybridmambaattention_localWindow(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Create a new hybrid Mamba-Attention layer
     * @param {MambaConfig} config
     * @param {number} local_window
     */
    constructor(config, local_window) {
        _assertClass(config, MambaConfig);
        var ptr0 = config.__destroy_into_raw();
        const ret = wasm.hybridmambaattention_new(ptr0, local_window);
        this.__wbg_ptr = ret >>> 0;
        HybridMambaAttentionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) HybridMambaAttention.prototype[Symbol.dispose] = HybridMambaAttention.prototype.free;
exports.HybridMambaAttention = HybridMambaAttention;

/**
 * Configuration for Mamba SSM attention
 */
class MambaConfig {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MambaConfig.prototype);
        obj.__wbg_ptr = ptr;
        MambaConfigFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MambaConfigFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_mambaconfig_free(ptr, 0);
    }
    /**
     * Convolution kernel size
     * @returns {number}
     */
    get conv_kernel_size() {
        const ret = wasm.__wbg_get_mambaconfig_conv_kernel_size(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Model dimension (d_model)
     * @returns {number}
     */
    get dim() {
        const ret = wasm.__wbg_get_mambaconfig_dim(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Delta range maximum
     * @returns {number}
     */
    get dt_max() {
        const ret = wasm.__wbg_get_mambaconfig_dt_max(this.__wbg_ptr);
        return ret;
    }
    /**
     * Delta (discretization step) range minimum
     * @returns {number}
     */
    get dt_min() {
        const ret = wasm.__wbg_get_mambaconfig_dt_min(this.__wbg_ptr);
        return ret;
    }
    /**
     * Expansion factor for inner dimension
     * @returns {number}
     */
    get expand_factor() {
        const ret = wasm.__wbg_get_mambaconfig_expand_factor(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * State space dimension (n)
     * @returns {number}
     */
    get state_dim() {
        const ret = wasm.__wbg_get_mambaconfig_state_dim(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Whether to use learnable D skip connection
     * @returns {boolean}
     */
    get use_d_skip() {
        const ret = wasm.__wbg_get_mambaconfig_use_d_skip(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Create a new Mamba configuration
     * @param {number} dim
     */
    constructor(dim) {
        const ret = wasm.mambaconfig_new(dim);
        this.__wbg_ptr = ret >>> 0;
        MambaConfigFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Set convolution kernel size
     * @param {number} size
     * @returns {MambaConfig}
     */
    withConvKernelSize(size) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.mambaconfig_withConvKernelSize(ptr, size);
        return MambaConfig.__wrap(ret);
    }
    /**
     * Set expansion factor
     * @param {number} factor
     * @returns {MambaConfig}
     */
    withExpandFactor(factor) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.mambaconfig_withExpandFactor(ptr, factor);
        return MambaConfig.__wrap(ret);
    }
    /**
     * Set state space dimension
     * @param {number} state_dim
     * @returns {MambaConfig}
     */
    withStateDim(state_dim) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.mambaconfig_withStateDim(ptr, state_dim);
        return MambaConfig.__wrap(ret);
    }
    /**
     * Convolution kernel size
     * @param {number} arg0
     */
    set conv_kernel_size(arg0) {
        wasm.__wbg_set_mambaconfig_conv_kernel_size(this.__wbg_ptr, arg0);
    }
    /**
     * Model dimension (d_model)
     * @param {number} arg0
     */
    set dim(arg0) {
        wasm.__wbg_set_mambaconfig_dim(this.__wbg_ptr, arg0);
    }
    /**
     * Delta range maximum
     * @param {number} arg0
     */
    set dt_max(arg0) {
        wasm.__wbg_set_mambaconfig_dt_max(this.__wbg_ptr, arg0);
    }
    /**
     * Delta (discretization step) range minimum
     * @param {number} arg0
     */
    set dt_min(arg0) {
        wasm.__wbg_set_mambaconfig_dt_min(this.__wbg_ptr, arg0);
    }
    /**
     * Expansion factor for inner dimension
     * @param {number} arg0
     */
    set expand_factor(arg0) {
        wasm.__wbg_set_mambaconfig_expand_factor(this.__wbg_ptr, arg0);
    }
    /**
     * State space dimension (n)
     * @param {number} arg0
     */
    set state_dim(arg0) {
        wasm.__wbg_set_mambaconfig_state_dim(this.__wbg_ptr, arg0);
    }
    /**
     * Whether to use learnable D skip connection
     * @param {boolean} arg0
     */
    set use_d_skip(arg0) {
        wasm.__wbg_set_mambaconfig_use_d_skip(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) MambaConfig.prototype[Symbol.dispose] = MambaConfig.prototype.free;
exports.MambaConfig = MambaConfig;

/**
 * Mamba Selective State Space Model for sequence attention
 *
 * Provides O(n) attention-like mechanism using selective state spaces
 */
class MambaSSMAttention {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MambaSSMAttention.prototype);
        obj.__wbg_ptr = ptr;
        MambaSSMAttentionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MambaSSMAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_mambassmattention_free(ptr, 0);
    }
    /**
     * Get the configuration
     * @returns {MambaConfig}
     */
    get config() {
        const ret = wasm.mambassmattention_config(this.__wbg_ptr);
        return MambaConfig.__wrap(ret);
    }
    /**
     * Forward pass through Mamba SSM
     *
     * # Arguments
     * * `input` - Input sequence (seq_len, dim) flattened to 1D
     * * `seq_len` - Sequence length
     *
     * # Returns
     * Output sequence (seq_len, dim) flattened to 1D
     * @param {Float32Array} input
     * @param {number} seq_len
     * @returns {Float32Array}
     */
    forward(input, seq_len) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayF32ToWasm0(input, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.mambassmattention_forward(retptr, this.__wbg_ptr, ptr0, len0, seq_len);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v2 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Compute attention-like scores (for visualization/analysis)
     *
     * Returns pseudo-attention scores showing which positions influence output
     * @param {Float32Array} input
     * @param {number} seq_len
     * @returns {Float32Array}
     */
    getAttentionScores(input, seq_len) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayF32ToWasm0(input, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.mambassmattention_getAttentionScores(retptr, this.__wbg_ptr, ptr0, len0, seq_len);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v2 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get the inner dimension
     * @returns {number}
     */
    get innerDim() {
        const ret = wasm.mambassmattention_innerDim(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Create a new Mamba SSM attention layer
     * @param {MambaConfig} config
     */
    constructor(config) {
        _assertClass(config, MambaConfig);
        var ptr0 = config.__destroy_into_raw();
        const ret = wasm.mambassmattention_new(ptr0);
        this.__wbg_ptr = ret >>> 0;
        MambaSSMAttentionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Create with default configuration
     * @param {number} dim
     * @returns {MambaSSMAttention}
     */
    static withDefaults(dim) {
        const ret = wasm.mambassmattention_withDefaults(dim);
        return MambaSSMAttention.__wrap(ret);
    }
}
if (Symbol.dispose) MambaSSMAttention.prototype[Symbol.dispose] = MambaSSMAttention.prototype.free;
exports.MambaSSMAttention = MambaSSMAttention;

/**
 * Unified attention mechanism selector
 * Automatically routes to the appropriate attention implementation
 */
class UnifiedAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        UnifiedAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_unifiedattention_free(ptr, 0);
    }
    /**
     * Get the category of the selected mechanism
     * @returns {string}
     */
    get category() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.unifiedattention_category(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Get the currently selected mechanism type
     * @returns {string}
     */
    get mechanism() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.unifiedattention_mechanism(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Create a new unified attention selector
     * @param {string} mechanism
     */
    constructor(mechanism) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passStringToWasm0(mechanism, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len0 = WASM_VECTOR_LEN;
            wasm.unifiedattention_new(retptr, ptr0, len0);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            UnifiedAttentionFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Check if this mechanism supports graph/DAG structures
     * @returns {boolean}
     */
    supportsGraphs() {
        const ret = wasm.unifiedattention_supportsGraphs(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Check if this mechanism supports hyperbolic geometry
     * @returns {boolean}
     */
    supportsHyperbolic() {
        const ret = wasm.unifiedattention_supportsHyperbolic(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * Check if this mechanism supports sequence processing
     * @returns {boolean}
     */
    supportsSequences() {
        const ret = wasm.unifiedattention_supportsSequences(this.__wbg_ptr);
        return ret !== 0;
    }
}
if (Symbol.dispose) UnifiedAttention.prototype[Symbol.dispose] = UnifiedAttention.prototype.free;
exports.UnifiedAttention = UnifiedAttention;

/**
 * Causal cone attention based on dependency lightcones
 *
 * Nodes can only attend to ancestors in the DAG (causal predecessors).
 * Attention strength decays with causal distance.
 */
class WasmCausalConeAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmCausalConeAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmcausalconeattention_free(ptr, 0);
    }
    /**
     * Compute attention scores for the DAG
     * @param {WasmQueryDag} dag
     * @returns {Float32Array}
     */
    forward(dag) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(dag, WasmQueryDag);
            wasm.wasmcausalconeattention_forward(retptr, this.__wbg_ptr, dag.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a new causal cone attention instance
     *
     * # Arguments
     * * `future_discount` - Discount for future nodes
     * * `ancestor_weight` - Weight for ancestor influence
     * @param {number} future_discount
     * @param {number} ancestor_weight
     */
    constructor(future_discount, ancestor_weight) {
        const ret = wasm.wasmcausalconeattention_new(future_discount, ancestor_weight);
        this.__wbg_ptr = ret >>> 0;
        WasmCausalConeAttentionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmCausalConeAttention.prototype[Symbol.dispose] = WasmCausalConeAttention.prototype.free;
exports.WasmCausalConeAttention = WasmCausalConeAttention;

/**
 * Critical path attention weighted by path criticality
 *
 * Nodes on or near the critical path (longest execution path)
 * receive higher attention scores.
 */
class WasmCriticalPathAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmCriticalPathAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmcriticalpathattention_free(ptr, 0);
    }
    /**
     * Compute attention scores for the DAG
     * @param {WasmQueryDag} dag
     * @returns {Float32Array}
     */
    forward(dag) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(dag, WasmQueryDag);
            wasm.wasmcriticalpathattention_forward(retptr, this.__wbg_ptr, dag.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a new critical path attention instance
     *
     * # Arguments
     * * `path_weight` - Weight for critical path membership
     * * `branch_penalty` - Penalty for branching nodes
     * @param {number} path_weight
     * @param {number} branch_penalty
     */
    constructor(path_weight, branch_penalty) {
        const ret = wasm.wasmcriticalpathattention_new(path_weight, branch_penalty);
        this.__wbg_ptr = ret >>> 0;
        WasmCriticalPathAttentionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmCriticalPathAttention.prototype[Symbol.dispose] = WasmCriticalPathAttention.prototype.free;
exports.WasmCriticalPathAttention = WasmCriticalPathAttention;

/**
 * Flash attention with memory-efficient tiling
 *
 * Reduces memory usage from O(n^2) to O(n) by computing attention
 * in blocks and fusing operations
 */
class WasmFlashAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmFlashAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmflashattention_free(ptr, 0);
    }
    /**
     * Compute flash attention
     * @param {Float32Array} query
     * @param {any} keys
     * @param {any} values
     * @returns {Float32Array}
     */
    compute(query, keys, values) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayF32ToWasm0(query, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.wasmflashattention_compute(retptr, this.__wbg_ptr, ptr0, len0, addHeapObject(keys), addHeapObject(values));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v2 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a new flash attention instance
     *
     * # Arguments
     * * `dim` - Embedding dimension
     * * `block_size` - Block size for tiled computation
     * @param {number} dim
     * @param {number} block_size
     */
    constructor(dim, block_size) {
        const ret = wasm.wasmflashattention_new(dim, block_size);
        this.__wbg_ptr = ret >>> 0;
        WasmFlashAttentionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmFlashAttention.prototype[Symbol.dispose] = WasmFlashAttention.prototype.free;
exports.WasmFlashAttention = WasmFlashAttention;

/**
 * Graph Neural Network layer with attention mechanism
 *
 * Implements Graph Attention Networks (GAT) for HNSW topology.
 * Each node aggregates information from neighbors using learned attention weights.
 */
class WasmGNNLayer {
    static __unwrap(jsValue) {
        if (!(jsValue instanceof WasmGNNLayer)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmGNNLayerFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmgnnlayer_free(ptr, 0);
    }
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
     * @param {Float32Array} node_embedding
     * @param {any} neighbor_embeddings
     * @param {Float32Array} edge_weights
     * @returns {Float32Array}
     */
    forward(node_embedding, neighbor_embeddings, edge_weights) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayF32ToWasm0(node_embedding, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passArrayF32ToWasm0(edge_weights, wasm.__wbindgen_export);
            const len1 = WASM_VECTOR_LEN;
            wasm.wasmgnnlayer_forward(retptr, this.__wbg_ptr, ptr0, len0, addHeapObject(neighbor_embeddings), ptr1, len1);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v3 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v3;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a new GNN layer with attention
     *
     * # Arguments
     * * `input_dim` - Dimension of input node embeddings
     * * `hidden_dim` - Dimension of hidden representations
     * * `heads` - Number of attention heads
     * * `dropout` - Dropout rate (0.0 to 1.0)
     * @param {number} input_dim
     * @param {number} hidden_dim
     * @param {number} heads
     * @param {number} dropout
     */
    constructor(input_dim, hidden_dim, heads, dropout) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmgnnlayer_new(retptr, input_dim, hidden_dim, heads, dropout);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            WasmGNNLayerFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get the output dimension
     * @returns {number}
     */
    get outputDim() {
        const ret = wasm.wasmgnnlayer_outputDim(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) WasmGNNLayer.prototype[Symbol.dispose] = WasmGNNLayer.prototype.free;
exports.WasmGNNLayer = WasmGNNLayer;

/**
 * Hierarchical Lorentz attention in hyperbolic space
 *
 * Combines DAG hierarchy with Lorentz (hyperboloid) geometry
 * for multi-scale hierarchical attention.
 */
class WasmHierarchicalLorentzAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmHierarchicalLorentzAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmhierarchicallorentzattention_free(ptr, 0);
    }
    /**
     * Compute attention scores for the DAG
     * @param {WasmQueryDag} dag
     * @returns {Float32Array}
     */
    forward(dag) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(dag, WasmQueryDag);
            wasm.wasmhierarchicallorentzattention_forward(retptr, this.__wbg_ptr, dag.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a new hierarchical Lorentz attention instance
     *
     * # Arguments
     * * `curvature` - Hyperbolic curvature parameter
     * * `temperature` - Temperature for softmax
     * @param {number} curvature
     * @param {number} temperature
     */
    constructor(curvature, temperature) {
        const ret = wasm.wasmhierarchicallorentzattention_new(curvature, temperature);
        this.__wbg_ptr = ret >>> 0;
        WasmHierarchicalLorentzAttentionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmHierarchicalLorentzAttention.prototype[Symbol.dispose] = WasmHierarchicalLorentzAttention.prototype.free;
exports.WasmHierarchicalLorentzAttention = WasmHierarchicalLorentzAttention;

/**
 * Hyperbolic attention mechanism for hierarchical data
 *
 * Operates in hyperbolic space (Poincare ball model) which naturally
 * represents tree-like hierarchical structures with exponential capacity
 */
class WasmHyperbolicAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmHyperbolicAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmhyperbolicattention_free(ptr, 0);
    }
    /**
     * Compute hyperbolic attention
     * @param {Float32Array} query
     * @param {any} keys
     * @param {any} values
     * @returns {Float32Array}
     */
    compute(query, keys, values) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayF32ToWasm0(query, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.wasmhyperbolicattention_compute(retptr, this.__wbg_ptr, ptr0, len0, addHeapObject(keys), addHeapObject(values));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v2 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get the curvature parameter
     * @returns {number}
     */
    get curvature() {
        const ret = wasm.wasmhyperbolicattention_curvature(this.__wbg_ptr);
        return ret;
    }
    /**
     * Create a new hyperbolic attention instance
     *
     * # Arguments
     * * `dim` - Embedding dimension
     * * `curvature` - Hyperbolic curvature parameter (negative for hyperbolic space)
     * @param {number} dim
     * @param {number} curvature
     */
    constructor(dim, curvature) {
        const ret = wasm.wasmhyperbolicattention_new(dim, curvature);
        this.__wbg_ptr = ret >>> 0;
        WasmHyperbolicAttentionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmHyperbolicAttention.prototype[Symbol.dispose] = WasmHyperbolicAttention.prototype.free;
exports.WasmHyperbolicAttention = WasmHyperbolicAttention;

/**
 * Linear attention using random feature approximation
 *
 * Achieves O(n) complexity instead of O(n^2) by approximating
 * the softmax kernel with random Fourier features
 */
class WasmLinearAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmLinearAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmlinearattention_free(ptr, 0);
    }
    /**
     * Compute linear attention
     * @param {Float32Array} query
     * @param {any} keys
     * @param {any} values
     * @returns {Float32Array}
     */
    compute(query, keys, values) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayF32ToWasm0(query, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.wasmlinearattention_compute(retptr, this.__wbg_ptr, ptr0, len0, addHeapObject(keys), addHeapObject(values));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v2 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a new linear attention instance
     *
     * # Arguments
     * * `dim` - Embedding dimension
     * * `num_features` - Number of random features for kernel approximation
     * @param {number} dim
     * @param {number} num_features
     */
    constructor(dim, num_features) {
        const ret = wasm.wasmlinearattention_new(dim, num_features);
        this.__wbg_ptr = ret >>> 0;
        WasmLinearAttentionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmLinearAttention.prototype[Symbol.dispose] = WasmLinearAttention.prototype.free;
exports.WasmLinearAttention = WasmLinearAttention;

/**
 * Local-global sparse attention (Longformer-style)
 *
 * Combines local sliding window attention with global tokens
 * for efficient long-range dependencies
 */
class WasmLocalGlobalAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmLocalGlobalAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmlocalglobalattention_free(ptr, 0);
    }
    /**
     * Compute local-global attention
     * @param {Float32Array} query
     * @param {any} keys
     * @param {any} values
     * @returns {Float32Array}
     */
    compute(query, keys, values) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayF32ToWasm0(query, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.wasmlocalglobalattention_compute(retptr, this.__wbg_ptr, ptr0, len0, addHeapObject(keys), addHeapObject(values));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v2 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a new local-global attention instance
     *
     * # Arguments
     * * `dim` - Embedding dimension
     * * `local_window` - Size of local attention window
     * * `global_tokens` - Number of global attention tokens
     * @param {number} dim
     * @param {number} local_window
     * @param {number} global_tokens
     */
    constructor(dim, local_window, global_tokens) {
        const ret = wasm.wasmlocalglobalattention_new(dim, local_window, global_tokens);
        this.__wbg_ptr = ret >>> 0;
        WasmLocalGlobalAttentionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmLocalGlobalAttention.prototype[Symbol.dispose] = WasmLocalGlobalAttention.prototype.free;
exports.WasmLocalGlobalAttention = WasmLocalGlobalAttention;

/**
 * MinCut-gated attention using flow-based bottleneck detection
 *
 * Uses minimum cut analysis to identify bottleneck nodes
 * and gates attention through these critical points.
 */
class WasmMinCutGatedAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmMinCutGatedAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmmincutgatedattention_free(ptr, 0);
    }
    /**
     * Compute attention scores for the DAG
     * @param {WasmQueryDag} dag
     * @returns {Float32Array}
     */
    forward(dag) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(dag, WasmQueryDag);
            wasm.wasmmincutgatedattention_forward(retptr, this.__wbg_ptr, dag.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a new MinCut-gated attention instance
     *
     * # Arguments
     * * `gate_threshold` - Threshold for gating (0.0-1.0)
     * @param {number} gate_threshold
     */
    constructor(gate_threshold) {
        const ret = wasm.wasmmincutgatedattention_new(gate_threshold);
        this.__wbg_ptr = ret >>> 0;
        WasmMinCutGatedAttentionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmMinCutGatedAttention.prototype[Symbol.dispose] = WasmMinCutGatedAttention.prototype.free;
exports.WasmMinCutGatedAttention = WasmMinCutGatedAttention;

/**
 * Mixture of Experts attention mechanism
 *
 * Routes queries to specialized expert attention heads based on
 * learned gating functions for capacity-efficient computation
 */
class WasmMoEAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmMoEAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmmoeattention_free(ptr, 0);
    }
    /**
     * Compute MoE attention
     * @param {Float32Array} query
     * @param {any} keys
     * @param {any} values
     * @returns {Float32Array}
     */
    compute(query, keys, values) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayF32ToWasm0(query, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.wasmmoeattention_compute(retptr, this.__wbg_ptr, ptr0, len0, addHeapObject(keys), addHeapObject(values));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v2 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a new MoE attention instance
     *
     * # Arguments
     * * `dim` - Embedding dimension
     * * `num_experts` - Number of expert attention mechanisms
     * * `top_k` - Number of experts to activate per query
     * @param {number} dim
     * @param {number} num_experts
     * @param {number} top_k
     */
    constructor(dim, num_experts, top_k) {
        const ret = wasm.wasmmoeattention_new(dim, num_experts, top_k);
        this.__wbg_ptr = ret >>> 0;
        WasmMoEAttentionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmMoEAttention.prototype[Symbol.dispose] = WasmMoEAttention.prototype.free;
exports.WasmMoEAttention = WasmMoEAttention;

/**
 * Multi-head attention mechanism
 *
 * Splits input into multiple heads, applies attention, and concatenates results
 */
class WasmMultiHeadAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmMultiHeadAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmmultiheadattention_free(ptr, 0);
    }
    /**
     * Compute multi-head attention
     *
     * # Arguments
     * * `query` - Query vector
     * * `keys` - Array of key vectors
     * * `values` - Array of value vectors
     * @param {Float32Array} query
     * @param {any} keys
     * @param {any} values
     * @returns {Float32Array}
     */
    compute(query, keys, values) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayF32ToWasm0(query, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.wasmmultiheadattention_compute(retptr, this.__wbg_ptr, ptr0, len0, addHeapObject(keys), addHeapObject(values));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v2 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v2;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get the embedding dimension
     * @returns {number}
     */
    get dim() {
        const ret = wasm.wasmmultiheadattention_dim(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Get the dimension per head
     * @returns {number}
     */
    get headDim() {
        const ret = wasm.wasmmultiheadattention_headDim(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Create a new multi-head attention instance
     *
     * # Arguments
     * * `dim` - Embedding dimension (must be divisible by num_heads)
     * * `num_heads` - Number of parallel attention heads
     * @param {number} dim
     * @param {number} num_heads
     */
    constructor(dim, num_heads) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmmultiheadattention_new(retptr, dim, num_heads);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            this.__wbg_ptr = r0 >>> 0;
            WasmMultiHeadAttentionFinalization.register(this, this.__wbg_ptr, this);
            return this;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get the number of attention heads
     * @returns {number}
     */
    get numHeads() {
        const ret = wasm.wasmmultiheadattention_numHeads(this.__wbg_ptr);
        return ret >>> 0;
    }
}
if (Symbol.dispose) WasmMultiHeadAttention.prototype[Symbol.dispose] = WasmMultiHeadAttention.prototype.free;
exports.WasmMultiHeadAttention = WasmMultiHeadAttention;

/**
 * Parallel branch attention for concurrent DAG branches
 *
 * Identifies parallel branches in the DAG and applies
 * attention patterns that respect branch independence.
 */
class WasmParallelBranchAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmParallelBranchAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmparallelbranchattention_free(ptr, 0);
    }
    /**
     * Compute attention scores for the DAG
     * @param {WasmQueryDag} dag
     * @returns {Float32Array}
     */
    forward(dag) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(dag, WasmQueryDag);
            wasm.wasmparallelbranchattention_forward(retptr, this.__wbg_ptr, dag.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a new parallel branch attention instance
     *
     * # Arguments
     * * `max_branches` - Maximum number of branches to consider
     * * `sync_penalty` - Penalty for synchronization between branches
     * @param {number} max_branches
     * @param {number} sync_penalty
     */
    constructor(max_branches, sync_penalty) {
        const ret = wasm.wasmparallelbranchattention_new(max_branches, sync_penalty);
        this.__wbg_ptr = ret >>> 0;
        WasmParallelBranchAttentionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmParallelBranchAttention.prototype[Symbol.dispose] = WasmParallelBranchAttention.prototype.free;
exports.WasmParallelBranchAttention = WasmParallelBranchAttention;

/**
 * Minimal DAG structure for WASM attention computation
 */
class WasmQueryDag {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmQueryDagFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmquerydag_free(ptr, 0);
    }
    /**
     * Add an edge between nodes
     *
     * # Arguments
     * * `from` - Source node ID
     * * `to` - Target node ID
     *
     * # Returns
     * True if edge was added successfully
     * @param {number} from
     * @param {number} to
     * @returns {boolean}
     */
    addEdge(from, to) {
        const ret = wasm.wasmquerydag_addEdge(this.__wbg_ptr, from, to);
        return ret !== 0;
    }
    /**
     * Add a node with operator type and cost
     *
     * # Arguments
     * * `op_type` - Operator type: "scan", "filter", "join", "aggregate", "project", "sort"
     * * `cost` - Estimated execution cost
     *
     * # Returns
     * Node ID
     * @param {string} op_type
     * @param {number} cost
     * @returns {number}
     */
    addNode(op_type, cost) {
        const ptr0 = passStringToWasm0(op_type, wasm.__wbindgen_export, wasm.__wbindgen_export2);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmquerydag_addNode(this.__wbg_ptr, ptr0, len0, cost);
        return ret >>> 0;
    }
    /**
     * Get the number of edges
     * @returns {number}
     */
    get edgeCount() {
        const ret = wasm.wasmquerydag_edgeCount(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Create a new empty DAG
     */
    constructor() {
        const ret = wasm.wasmquerydag_new();
        this.__wbg_ptr = ret >>> 0;
        WasmQueryDagFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Get the number of nodes
     * @returns {number}
     */
    get nodeCount() {
        const ret = wasm.wasmquerydag_nodeCount(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Serialize to JSON
     * @returns {string}
     */
    toJson() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmquerydag_toJson(retptr, this.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) WasmQueryDag.prototype[Symbol.dispose] = WasmQueryDag.prototype.free;
exports.WasmQueryDag = WasmQueryDag;

/**
 * Search configuration for differentiable search
 */
class WasmSearchConfig {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmSearchConfigFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmsearchconfig_free(ptr, 0);
    }
    /**
     * Number of top results to return
     * @returns {number}
     */
    get k() {
        const ret = wasm.__wbg_get_wasmsearchconfig_k(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * Temperature for softmax
     * @returns {number}
     */
    get temperature() {
        const ret = wasm.__wbg_get_wasmsearchconfig_temperature(this.__wbg_ptr);
        return ret;
    }
    /**
     * Number of top results to return
     * @param {number} arg0
     */
    set k(arg0) {
        wasm.__wbg_set_wasmsearchconfig_k(this.__wbg_ptr, arg0);
    }
    /**
     * Temperature for softmax
     * @param {number} arg0
     */
    set temperature(arg0) {
        wasm.__wbg_set_wasmsearchconfig_temperature(this.__wbg_ptr, arg0);
    }
    /**
     * Create a new search configuration
     * @param {number} k
     * @param {number} temperature
     */
    constructor(k, temperature) {
        const ret = wasm.wasmsearchconfig_new(k, temperature);
        this.__wbg_ptr = ret >>> 0;
        WasmSearchConfigFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmSearchConfig.prototype[Symbol.dispose] = WasmSearchConfig.prototype.free;
exports.WasmSearchConfig = WasmSearchConfig;

/**
 * Temporal BTSP (Behavioral Time-Series Pattern) attention
 *
 * Incorporates temporal patterns and behavioral sequences
 * for time-aware DAG attention.
 */
class WasmTemporalBTSPAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmTemporalBTSPAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtemporalbtspattention_free(ptr, 0);
    }
    /**
     * Compute attention scores for the DAG
     * @param {WasmQueryDag} dag
     * @returns {Float32Array}
     */
    forward(dag) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(dag, WasmQueryDag);
            wasm.wasmtemporalbtspattention_forward(retptr, this.__wbg_ptr, dag.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a new temporal BTSP attention instance
     *
     * # Arguments
     * * `eligibility_decay` - Decay rate for eligibility traces (0.0-1.0)
     * * `baseline_attention` - Baseline attention for nodes without history
     * @param {number} eligibility_decay
     * @param {number} baseline_attention
     */
    constructor(eligibility_decay, baseline_attention) {
        const ret = wasm.wasmtemporalbtspattention_new(eligibility_decay, baseline_attention);
        this.__wbg_ptr = ret >>> 0;
        WasmTemporalBTSPAttentionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmTemporalBTSPAttention.prototype[Symbol.dispose] = WasmTemporalBTSPAttention.prototype.free;
exports.WasmTemporalBTSPAttention = WasmTemporalBTSPAttention;

/**
 * Tensor compressor with adaptive level selection
 *
 * Compresses embeddings based on access frequency for memory-efficient GNN
 */
class WasmTensorCompress {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmTensorCompressFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtensorcompress_free(ptr, 0);
    }
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
     * @param {Float32Array} embedding
     * @param {number} access_freq
     * @returns {any}
     */
    compress(embedding, access_freq) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayF32ToWasm0(embedding, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            wasm.wasmtensorcompress_compress(retptr, this.__wbg_ptr, ptr0, len0, access_freq);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Compress with explicit compression level
     *
     * # Arguments
     * * `embedding` - The input embedding vector
     * * `level` - Compression level: "none", "half", "pq8", "pq4", "binary"
     * @param {Float32Array} embedding
     * @param {string} level
     * @returns {any}
     */
    compressWithLevel(embedding, level) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            const ptr0 = passArrayF32ToWasm0(embedding, wasm.__wbindgen_export);
            const len0 = WASM_VECTOR_LEN;
            const ptr1 = passStringToWasm0(level, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            wasm.wasmtensorcompress_compressWithLevel(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            if (r2) {
                throw takeObject(r1);
            }
            return takeObject(r0);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Decompress a compressed tensor
     * @param {any} compressed
     * @returns {Float32Array}
     */
    decompress(compressed) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.wasmtensorcompress_decompress(retptr, this.__wbg_ptr, addHeapObject(compressed));
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Get compression ratio estimate for a given access frequency
     * @param {number} access_freq
     * @returns {number}
     */
    getCompressionRatio(access_freq) {
        const ret = wasm.wasmtensorcompress_getCompressionRatio(this.__wbg_ptr, access_freq);
        return ret;
    }
    /**
     * Create a new tensor compressor
     */
    constructor() {
        const ret = wasm.wasmtensorcompress_new();
        this.__wbg_ptr = ret >>> 0;
        WasmTensorCompressFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmTensorCompress.prototype[Symbol.dispose] = WasmTensorCompress.prototype.free;
exports.WasmTensorCompress = WasmTensorCompress;

/**
 * Topological attention based on DAG position
 *
 * Assigns attention scores based on node position in topological order.
 * Earlier nodes (closer to sources) get higher attention.
 */
class WasmTopologicalAttention {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmTopologicalAttentionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmtopologicalattention_free(ptr, 0);
    }
    /**
     * Compute attention scores for the DAG
     *
     * # Returns
     * Attention scores for each node
     * @param {WasmQueryDag} dag
     * @returns {Float32Array}
     */
    forward(dag) {
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            _assertClass(dag, WasmQueryDag);
            wasm.wasmtopologicalattention_forward(retptr, this.__wbg_ptr, dag.__wbg_ptr);
            var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
            var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
            var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
            var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
            if (r3) {
                throw takeObject(r2);
            }
            var v1 = getArrayF32FromWasm0(r0, r1).slice();
            wasm.__wbindgen_export4(r0, r1 * 4, 4);
            return v1;
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
        }
    }
    /**
     * Create a new topological attention instance
     *
     * # Arguments
     * * `decay_factor` - Decay factor for position-based attention (0.0-1.0)
     * @param {number} decay_factor
     */
    constructor(decay_factor) {
        const ret = wasm.wasmtopologicalattention_new(decay_factor);
        this.__wbg_ptr = ret >>> 0;
        WasmTopologicalAttentionFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}
if (Symbol.dispose) WasmTopologicalAttention.prototype[Symbol.dispose] = WasmTopologicalAttention.prototype.free;
exports.WasmTopologicalAttention = WasmTopologicalAttention;

/**
 * Get information about all available attention mechanisms
 * @returns {any}
 */
function availableMechanisms() {
    const ret = wasm.availableMechanisms();
    return takeObject(ret);
}
exports.availableMechanisms = availableMechanisms;

/**
 * Compute cosine similarity between two vectors
 * @param {Float32Array} a
 * @param {Float32Array} b
 * @returns {number}
 */
function cosineSimilarity(a, b) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArrayF32ToWasm0(a, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF32ToWasm0(b, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        wasm.cosineSimilarity(retptr, ptr0, len0, ptr1, len1);
        var r0 = getDataViewMemory0().getFloat32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return r0;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}
exports.cosineSimilarity = cosineSimilarity;

/**
 * Get summary statistics about the unified attention library
 * @returns {any}
 */
function getStats() {
    const ret = wasm.getStats();
    return takeObject(ret);
}
exports.getStats = getStats;

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
 * @param {Float32Array} query
 * @param {any} candidate_embeddings
 * @param {WasmSearchConfig} config
 * @returns {any}
 */
function graphDifferentiableSearch(query, candidate_embeddings, config) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArrayF32ToWasm0(query, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        _assertClass(config, WasmSearchConfig);
        wasm.graphDifferentiableSearch(retptr, ptr0, len0, addHeapObject(candidate_embeddings), config.__wbg_ptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}
exports.graphDifferentiableSearch = graphDifferentiableSearch;

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
 * @param {Float32Array} query
 * @param {any} layer_embeddings
 * @param {WasmGNNLayer[]} gnn_layers
 * @returns {Float32Array}
 */
function graphHierarchicalForward(query, layer_embeddings, gnn_layers) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArrayF32ToWasm0(query, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayJsValueToWasm0(gnn_layers, wasm.__wbindgen_export);
        const len1 = WASM_VECTOR_LEN;
        wasm.graphHierarchicalForward(retptr, ptr0, len0, addHeapObject(layer_embeddings), ptr1, len1);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        if (r3) {
            throw takeObject(r2);
        }
        var v3 = getArrayF32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export4(r0, r1 * 4, 4);
        return v3;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}
exports.graphHierarchicalForward = graphHierarchicalForward;

/**
 * Initialize the WASM module with panic hook for better error messages
 */
function init() {
    wasm.init();
}
exports.init = init;

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
 * @param {Float32Array} query
 * @param {any} keys
 * @param {any} values
 * @param {number | null} [scale]
 * @returns {Float32Array}
 */
function scaledDotAttention(query, keys, values, scale) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArrayF32ToWasm0(query, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        wasm.scaledDotAttention(retptr, ptr0, len0, addHeapObject(keys), addHeapObject(values), isLikeNone(scale) ? 0x100000001 : Math.fround(scale));
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var r2 = getDataViewMemory0().getInt32(retptr + 4 * 2, true);
        var r3 = getDataViewMemory0().getInt32(retptr + 4 * 3, true);
        if (r3) {
            throw takeObject(r2);
        }
        var v2 = getArrayF32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export4(r0, r1 * 4, 4);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}
exports.scaledDotAttention = scaledDotAttention;

/**
 * Softmax normalization
 * @param {Float32Array} values
 * @returns {Float32Array}
 */
function softmax(values) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArrayF32ToWasm0(values, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        wasm.softmax(retptr, ptr0, len0);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v2 = getArrayF32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export4(r0, r1 * 4, 4);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}
exports.softmax = softmax;

/**
 * Temperature-scaled softmax
 * @param {Float32Array} values
 * @param {number} temperature
 * @returns {Float32Array}
 */
function temperatureSoftmax(values, temperature) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        const ptr0 = passArrayF32ToWasm0(values, wasm.__wbindgen_export);
        const len0 = WASM_VECTOR_LEN;
        wasm.temperatureSoftmax(retptr, ptr0, len0, temperature);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        var v2 = getArrayF32FromWasm0(r0, r1).slice();
        wasm.__wbindgen_export4(r0, r1 * 4, 4);
        return v2;
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}
exports.temperatureSoftmax = temperatureSoftmax;

/**
 * Get the version of the unified attention WASM crate
 * @returns {string}
 */
function version() {
    let deferred1_0;
    let deferred1_1;
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.version(retptr);
        var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
        var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
        deferred1_0 = r0;
        deferred1_1 = r1;
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        wasm.__wbindgen_export4(deferred1_0, deferred1_1, 1);
    }
}
exports.version = version;

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg_Error_4577686b3a6d9b3a: function(arg0, arg1) {
            const ret = Error(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        },
        __wbg_Number_e89e48a2fe1a6355: function(arg0) {
            const ret = Number(getObject(arg0));
            return ret;
        },
        __wbg_String_8564e559799eccda: function(arg0, arg1) {
            const ret = String(getObject(arg1));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_bigint_get_as_i64_578010f8442e0319: function(arg0, arg1) {
            const v = getObject(arg1);
            const ret = typeof(v) === 'bigint' ? v : undefined;
            getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_boolean_get_18c4ed9422296fff: function(arg0) {
            const v = getObject(arg0);
            const ret = typeof(v) === 'boolean' ? v : undefined;
            return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
        },
        __wbg___wbindgen_debug_string_ddde1867f49c2442: function(arg0, arg1) {
            const ret = debugString(getObject(arg1));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_in_1064a108f4d18b9e: function(arg0, arg1) {
            const ret = getObject(arg0) in getObject(arg1);
            return ret;
        },
        __wbg___wbindgen_is_bigint_a157f0734ca85901: function(arg0) {
            const ret = typeof(getObject(arg0)) === 'bigint';
            return ret;
        },
        __wbg___wbindgen_is_function_d633e708baf0d146: function(arg0) {
            const ret = typeof(getObject(arg0)) === 'function';
            return ret;
        },
        __wbg___wbindgen_is_object_4b3de556756ee8a8: function(arg0) {
            const val = getObject(arg0);
            const ret = typeof(val) === 'object' && val !== null;
            return ret;
        },
        __wbg___wbindgen_is_string_7debe47dc1e045c2: function(arg0) {
            const ret = typeof(getObject(arg0)) === 'string';
            return ret;
        },
        __wbg___wbindgen_is_undefined_c18285b9fc34cb7d: function(arg0) {
            const ret = getObject(arg0) === undefined;
            return ret;
        },
        __wbg___wbindgen_jsval_eq_a6afb59d8c5e78d6: function(arg0, arg1) {
            const ret = getObject(arg0) === getObject(arg1);
            return ret;
        },
        __wbg___wbindgen_jsval_loose_eq_1562ceb9af84e990: function(arg0, arg1) {
            const ret = getObject(arg0) == getObject(arg1);
            return ret;
        },
        __wbg___wbindgen_number_get_5854912275df1894: function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_string_get_3e5751597f39a112: function(arg0, arg1) {
            const obj = getObject(arg1);
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_39bc967c0e5a9b58: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_call_08ad0d89caa7cb79: function() { return handleError(function (arg0, arg1, arg2) {
            const ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_call_73af281463ec8b58: function() { return handleError(function (arg0, arg1) {
            const ret = getObject(arg0).call(getObject(arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_crypto_38df2bab126b63dc: function(arg0) {
            const ret = getObject(arg0).crypto;
            return addHeapObject(ret);
        },
        __wbg_done_5aad55ec6b1954b1: function(arg0) {
            const ret = getObject(arg0).done;
            return ret;
        },
        __wbg_entries_28d32ba4cd93f5fc: function(arg0) {
            const ret = Object.entries(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_error_a6fa202b58aa1cd3: function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_export4(deferred0_0, deferred0_1, 1);
            }
        },
        __wbg_getRandomValues_c44a50d8cfdaebeb: function() { return handleError(function (arg0, arg1) {
            getObject(arg0).getRandomValues(getObject(arg1));
        }, arguments); },
        __wbg_get_4920fefd3451364b: function() { return handleError(function (arg0, arg1) {
            const ret = Reflect.get(getObject(arg0), getObject(arg1));
            return addHeapObject(ret);
        }, arguments); },
        __wbg_get_f09c3a16f8848381: function(arg0, arg1) {
            const ret = getObject(arg0)[arg1 >>> 0];
            return addHeapObject(ret);
        },
        __wbg_get_unchecked_3d0f4b91c8eca4f0: function(arg0, arg1) {
            const ret = getObject(arg0)[arg1 >>> 0];
            return addHeapObject(ret);
        },
        __wbg_get_with_ref_key_6412cf3094599694: function(arg0, arg1) {
            const ret = getObject(arg0)[getObject(arg1)];
            return addHeapObject(ret);
        },
        __wbg_instanceof_ArrayBuffer_15859862b80b732d: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof ArrayBuffer;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_instanceof_Uint8Array_2240b7046ac16f05: function(arg0) {
            let result;
            try {
                result = getObject(arg0) instanceof Uint8Array;
            } catch (_) {
                result = false;
            }
            const ret = result;
            return ret;
        },
        __wbg_isArray_fad08a0d12828686: function(arg0) {
            const ret = Array.isArray(getObject(arg0));
            return ret;
        },
        __wbg_isSafeInteger_10e4151eb694e42a: function(arg0) {
            const ret = Number.isSafeInteger(getObject(arg0));
            return ret;
        },
        __wbg_iterator_fc7ad8d33bab9e26: function() {
            const ret = Symbol.iterator;
            return addHeapObject(ret);
        },
        __wbg_length_5855c1f289dfffc1: function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        },
        __wbg_length_a31e05262e09b7f8: function(arg0) {
            const ret = getObject(arg0).length;
            return ret;
        },
        __wbg_msCrypto_bd5a034af96bcba6: function(arg0) {
            const ret = getObject(arg0).msCrypto;
            return addHeapObject(ret);
        },
        __wbg_new_09959f7b4c92c246: function(arg0) {
            const ret = new Uint8Array(getObject(arg0));
            return addHeapObject(ret);
        },
        __wbg_new_227d7c05414eb861: function() {
            const ret = new Error();
            return addHeapObject(ret);
        },
        __wbg_new_cbee8c0d5c479eac: function() {
            const ret = new Array();
            return addHeapObject(ret);
        },
        __wbg_new_ed69e637b553a997: function() {
            const ret = new Object();
            return addHeapObject(ret);
        },
        __wbg_new_with_length_c8449d782396d344: function(arg0) {
            const ret = new Uint8Array(arg0 >>> 0);
            return addHeapObject(ret);
        },
        __wbg_next_a5fe6f328f7affc2: function(arg0) {
            const ret = getObject(arg0).next;
            return addHeapObject(ret);
        },
        __wbg_next_e592122bb4ed4c67: function() { return handleError(function (arg0) {
            const ret = getObject(arg0).next();
            return addHeapObject(ret);
        }, arguments); },
        __wbg_node_84ea875411254db1: function(arg0) {
            const ret = getObject(arg0).node;
            return addHeapObject(ret);
        },
        __wbg_process_44c7a14e11e9f69e: function(arg0) {
            const ret = getObject(arg0).process;
            return addHeapObject(ret);
        },
        __wbg_prototypesetcall_f034d444741426c3: function(arg0, arg1, arg2) {
            Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), getObject(arg2));
        },
        __wbg_randomFillSync_6c25eac9869eb53c: function() { return handleError(function (arg0, arg1) {
            getObject(arg0).randomFillSync(takeObject(arg1));
        }, arguments); },
        __wbg_require_b4edbdcf3e2a1ef0: function() { return handleError(function () {
            const ret = module.require;
            return addHeapObject(ret);
        }, arguments); },
        __wbg_set_4c81cfb5dc3a333c: function(arg0, arg1, arg2) {
            getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
        },
        __wbg_set_6be42768c690e380: function(arg0, arg1, arg2) {
            getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
        },
        __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
            const ret = getObject(arg1).stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_export, wasm.__wbindgen_export2);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_static_accessor_GLOBAL_THIS_14325d8cca34bb77: function() {
            const ret = typeof globalThis === 'undefined' ? null : globalThis;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_GLOBAL_f3a1e69f9c5a7e8e: function() {
            const ret = typeof global === 'undefined' ? null : global;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_SELF_50cdb5b517789aca: function() {
            const ret = typeof self === 'undefined' ? null : self;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_static_accessor_WINDOW_d6c4126e4c244380: function() {
            const ret = typeof window === 'undefined' ? null : window;
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        },
        __wbg_subarray_7ad5f01d4a9c1c4d: function(arg0, arg1, arg2) {
            const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
            return addHeapObject(ret);
        },
        __wbg_value_667dcb90597486a6: function(arg0) {
            const ret = getObject(arg0).value;
            return addHeapObject(ret);
        },
        __wbg_versions_276b2795b1c6a219: function(arg0) {
            const ret = getObject(arg0).versions;
            return addHeapObject(ret);
        },
        __wbg_wasmgnnlayer_unwrap: function(arg0) {
            const ret = WasmGNNLayer.__unwrap(getObject(arg0));
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
            const ret = getArrayU8FromWasm0(arg0, arg1);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000003: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        },
        __wbindgen_cast_0000000000000004: function(arg0) {
            // Cast intrinsic for `U64 -> Externref`.
            const ret = BigInt.asUintN(64, arg0);
            return addHeapObject(ret);
        },
        __wbindgen_object_clone_ref: function(arg0) {
            const ret = getObject(arg0);
            return addHeapObject(ret);
        },
        __wbindgen_object_drop_ref: function(arg0) {
            takeObject(arg0);
        },
    };
    return {
        __proto__: null,
        "./ruvector_attention_unified_wasm_bg.js": import0,
    };
}

const DagAttentionFactoryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_dagattentionfactory_free(ptr >>> 0, 1));
const GraphAttentionFactoryFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_graphattentionfactory_free(ptr >>> 0, 1));
const HybridMambaAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_hybridmambaattention_free(ptr >>> 0, 1));
const MambaConfigFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_mambaconfig_free(ptr >>> 0, 1));
const MambaSSMAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_mambassmattention_free(ptr >>> 0, 1));
const UnifiedAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_unifiedattention_free(ptr >>> 0, 1));
const WasmCausalConeAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmcausalconeattention_free(ptr >>> 0, 1));
const WasmCriticalPathAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmcriticalpathattention_free(ptr >>> 0, 1));
const WasmFlashAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmflashattention_free(ptr >>> 0, 1));
const WasmGNNLayerFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmgnnlayer_free(ptr >>> 0, 1));
const WasmHierarchicalLorentzAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmhierarchicallorentzattention_free(ptr >>> 0, 1));
const WasmHyperbolicAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmhyperbolicattention_free(ptr >>> 0, 1));
const WasmLinearAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmlinearattention_free(ptr >>> 0, 1));
const WasmLocalGlobalAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmlocalglobalattention_free(ptr >>> 0, 1));
const WasmMinCutGatedAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmmincutgatedattention_free(ptr >>> 0, 1));
const WasmMoEAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmmoeattention_free(ptr >>> 0, 1));
const WasmMultiHeadAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmmultiheadattention_free(ptr >>> 0, 1));
const WasmParallelBranchAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmparallelbranchattention_free(ptr >>> 0, 1));
const WasmQueryDagFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmquerydag_free(ptr >>> 0, 1));
const WasmSearchConfigFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmsearchconfig_free(ptr >>> 0, 1));
const WasmTemporalBTSPAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmtemporalbtspattention_free(ptr >>> 0, 1));
const WasmTensorCompressFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmtensorcompress_free(ptr >>> 0, 1));
const WasmTopologicalAttentionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmtopologicalattention_free(ptr >>> 0, 1));

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function dropObject(idx) {
    if (idx < 1028) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getObject(idx) { return heap[idx]; }

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_export3(addHeapObject(e));
    }
}

let heap = new Array(1024).fill(undefined);
heap.push(undefined, null, true, false);

let heap_next = heap.length;

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getFloat32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    const mem = getDataViewMemory0();
    for (let i = 0; i < array.length; i++) {
        mem.setUint32(ptr + 4 * i, addHeapObject(array[i]), true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
function decodeText(ptr, len) {
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

const wasmPath = `${__dirname}/ruvector_attention_unified_wasm_bg.wasm`;
const wasmBytes = require('fs').readFileSync(wasmPath);
const wasmModule = new WebAssembly.Module(wasmBytes);
let wasm = new WebAssembly.Instance(wasmModule, __wbg_get_imports()).exports;
wasm.__wbindgen_start();
