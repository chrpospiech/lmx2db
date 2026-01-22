#!/bin/bash

# PATH settings
BASE_DIR=/home/xcpospiech/bench/4paper
INPUT_DIR=$BASE_DIR/input
LMX_DIR=$BASE_DIR/lmxtrace_build_aocc/lmx_trace-2.0.0.rc3/install_aocc/lib
GROMACS_DIR=$BASE_DIR/gromacs_aocc/install_aocc
RUN_DIR=run_${SLURM_NTASKS}_maxh_${SLURM_JOB_ID}

# Loading modules
source ~/.bashrc
module load \
	amd-compilers \
	aocc/5.0.0 \
	aocl/aocc/5.0.0 \
	openmpi/5.0.8 \
	eb-env \
	binutils

# run directory
rm -rf $RUN_DIR
mkdir -p $RUN_DIR
cd $RUN_DIR

# LD_LIBRARY_PATH, PATH, LD_PRELOAD
export LD_LIBRARY_PATH=$GROMACS_DIR/lib:$LD_LIBRARY_PATH
export PATH=$GROMACS_DIR/bin:$PATH
export OMP_NUM_THREADS=4

# run GROMACS
mpirun -n ${SLURM_NTASKS} --bind-to none  \
	env LMX_ITIMERPROF=1 LMX_IMBALANCE=1 \
	LD_PRELOAD=$LMX_DIR/libmpitrace.so \
	gmx_mpi mdrun \
	-nsteps -1 \
	-maxh 0.3 \
	-s $INPUT_DIR/benchMEM.tpr
