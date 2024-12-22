#include <gmp.h>
#include <stdio.h>
#include <time.h>

int main() {
    // Initialize modulus
    mpz_t modulus;
    mpz_init_set_str(modulus, "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF43", 16);

    // Initialize value to 1
    mpz_t value;
    mpz_init_set_ui(value, 1);

    // Count number of squarings
    unsigned long count = 0;

    // Measure elapsed time
    clock_t start = clock();
    double elapsed_time = 0.0;

    while (elapsed_time < 1.0) {
        // value = (value * value) % modulus
        mpz_mul(value, value, value);
        mpz_mod(value, value, modulus);
        count++;

        // Update elapsed time
        elapsed_time = (double)(clock() - start) / CLOCKS_PER_SEC;
    }

    printf("Modular Squarings Per Second: %lu\n", count);

    // Clean up
    mpz_clear(modulus);
    mpz_clear(value);

    return 0;
}
