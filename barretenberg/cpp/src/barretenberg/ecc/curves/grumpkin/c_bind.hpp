// ecc_grumpkin.h

#ifndef ECC_GRUMPKIN_H
#define ECC_GRUMPKIN_H

#include "barretenberg/common/wasm_export.hpp"
#include "grumpkin.hpp"

extern "C" {

// Multiplies a point by a scalar
WASM_EXPORT void ecc_grumpkin__mul(uint8_t const* point_buf, uint8_t const* scalar_buf, uint8_t* result);

// Adds two points
WASM_EXPORT void ecc_grumpkin__add(uint8_t const* point_a_buf, uint8_t const* point_b_buf, uint8_t* result);

// Multiplies a vector of points by a single scalar
WASM_EXPORT void ecc_grumpkin__batch_mul(uint8_t const* point_buf,
                                         uint8_t const* scalar_buf,
                                         uint32_t num_points,
                                         uint8_t* result);

// Gets a random scalar modulo the circuit modulus
WASM_EXPORT void ecc_grumpkin__get_random_scalar_mod_circuit_modulus(uint8_t* result);

// Reduces a 512-bit buffer modulo the circuit modulus
WASM_EXPORT void ecc_grumpkin__reduce512_buffer_mod_circuit_modulus(uint8_t* input, uint8_t* result);
}

#endif // ECC_GRUMPKIN_H
