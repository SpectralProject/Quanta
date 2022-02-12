# Quanta
An Operating System to be built from the ground up with RISC V hardware. Supports riscv and arm with first class riscv support.

## Purpose of repo
- be a place to collect all the deps of a working full OS like gnu linux

## Design
- Silicon Spectrum / Spectro Silicon - Hardware Suite. 
- Neutron - Kernel. Neutrons are at the centre of all great atoms.
- Carbon Software - The software suite. Carbon gives the kernel life.

## Style & Layout
- Submodules for major things
- Be as clear as possible, add a 'doc' file to each dir to describe its contents. A `doc` generator (written in rust) compiles a projects `doc` files to a single html markdown documentation browser
- Keep it simple, stupid and It Just Works
- First principles, test driven
- EAO -> Everything at Once
