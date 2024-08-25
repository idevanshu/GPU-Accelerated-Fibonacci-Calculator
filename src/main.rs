use ocl::{Buffer, Context, Device, Kernel, Platform, Program, Queue};
use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;
use std::io;

fn main() {
    // Ask the user to enter a number
    println!("Enter the position up to which you want to compute Fibonacci numbers:");

    // Read the input from the user
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();

    // Parse the input to an integer
    let user_input: usize = input_text.trim().parse().expect("Please enter a valid number");

    // Define a reasonable maximum index for GPU computation
    let gpu_limit = 93; // 93 is the maximum index for i64 to avoid overflow

    // Determine the maximum Fibonacci index to compute on GPU
    let compute_max_index = user_input.min(gpu_limit);

    // Output vector for GPU calculations
    let mut output = vec![0i64; compute_max_index];

    // Get the first available platform
    let platform = Platform::default();

    // Get the first available device (GPU)
    let device = Device::first(platform).unwrap();

    // Create a context with the device
    let context = Context::builder()
        .platform(platform)
        .devices(device.clone())
        .build()
        .unwrap();

    // Create a command queue
    let queue = Queue::new(&context, device, None).unwrap();

    // Define the OpenCL kernel as a string
    let src = r#"
    __kernel void fibonacci(__global long* output, int n) {
        int id = get_global_id(0);
        if (id == 0) {
            output[0] = 0;
        } else if (id == 1) {
            output[1] = 1;
        } else if (id < n) {
            long a = 0, b = 1;
            for (int i = 2; i <= id; i++) {
                long temp = a + b;
                a = b;
                b = temp;
            }
            output[id] = b;
        }
    }
    "#;

    // Create a program from the kernel source
    let program = Program::builder()
        .devices(device)
        .src(src)
        .build(&context)
        .unwrap();

    // Create a buffer for the output
    let output_buffer = Buffer::<i64>::builder()
        .queue(queue.clone())
        .flags(ocl::flags::MEM_WRITE_ONLY)
        .len(compute_max_index)
        .build()
        .unwrap();

    // Create the kernel
    let kernel = Kernel::builder()
        .program(&program)
        .name("fibonacci")
        .queue(queue.clone())
        .global_work_size(compute_max_index)
        .arg(&output_buffer)
        .arg(compute_max_index as i32)
        .build()
        .unwrap();

    // Run the kernel
    unsafe {
        kernel.enq().unwrap();
    }

    // Read the results from the output buffer
    output_buffer.read(&mut output).enq().unwrap();

    // If necessary, use the CPU to handle large numbers beyond the GPU limit
    let mut cpu_results = Vec::new();
    for i in gpu_limit..user_input {
        cpu_results.push(fibonacci_large(i));
    }

    // Print the results from GPU calculations
    println!("Fibonacci numbers up to position {}:", compute_max_index);
    for (i, &fib) in output.iter().enumerate() {
        println!("Fibo({:<3}): {:>20}", i, fib);
    }

    // Print results from CPU calculations
    if !cpu_results.is_empty() {
        println!("\nLarge Fibonacci numbers beyond GPU computation:");
        for (i, fib) in cpu_results.iter().enumerate() {
            println!("Fibo({:<3}): {:>20}", gpu_limit + i + 1, fib);
        }
    }
}

// Function to compute Fibonacci numbers using BigInt for large values
fn fibonacci_large(n: usize) -> BigInt {
    let mut a = BigInt::zero();
    let mut b = BigInt::one();
    if n == 0 {
        return a;
    }
    for _ in 1..n {
        let temp = a.clone();
        a = b.clone();
        b = temp + b;
    }
    b
}
