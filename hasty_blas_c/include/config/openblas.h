#ifndef HASTY_BLAS_C_OPENBLAS_H
#define HASTY_BLAS_C_OPENBLAS_H

#ifdef BLASINT
#	define CBLAS_INT BLASINT
#else
#	define CBLAS_INT int
#endif

#define BLAS_IMPL "OpenBLAS"

#ifndef CBLAS_INDEX
#	define CBLAS_INDEX uint64_t
#endif // CBLAS_INDEX

#ifndef HAVE_CBLAS_AXPBY
#	define HAVE_CBLAS_AXPBY
#	define BLAS_EXT(x) cblas_##x
#endif

// Set runtime threads
void openblas_set_num_threads(int num_threads);
void goto_set_num_threads(int num_threads);

// Get runtime threads
int openblas_get_num_threads();

// Get the number of processors
int openblas_get_num_procs();

// Get build configuration
char *openblas_get_config();

// Get the corename
char *openblas_get_corename();

// Get the parallelization type
#define OPENBLAS_SEQUENTIAL 0   // Sequential
#define OPENBLAS_THREAD 1       // Thread Parallel
#define OPENBLAS_OPENMP 2       // OpenMP Parallel
int openblas_get_parallel();

#include "cblas.h"

#endif // HASTY_BLAS_C_OPENBLAS_H