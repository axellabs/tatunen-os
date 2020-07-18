# tatunen-os
a small os written in rust, based on the Philipp Oppermann blog post
https://os.phil-opp.com/

## Steps in creating a Rust OS
1. Create a freestandung Rust Binary
2. Write a microkernel (x86)
3. VGA text mode
4. Add unit and integration testing
5. Catch CPU Exceptions
6. Avoid triple faults
7. Forward hardware interrupts to the CPU
8. Implement paging support in the kernal
9. Add heap allocation to the kernal
10. Add support for async/await to the kernel