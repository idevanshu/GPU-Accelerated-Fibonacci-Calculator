
  <h1>Fibonacci Number Computation with OpenCL and Rust</h1>
  
  <h3>Overview</h3>
  <strong>This project demonstrates how to compute Fibonacci numbers using both GPU and CPU in Rust. The GPU computation is performed using OpenCL, while the CPU handles large Fibonacci numbers that exceed the GPU's capacity. This approach combines the power of parallel processing on the GPU with the flexibility of arbitrary-precision arithmetic on the CPU.</strong>
  
  <h3>Features</h3>
  <strong>
      <ul>
          <li><strong>OpenCL Kernel:</strong> Computes Fibonacci numbers up to a specified limit on the GPU.</li>
          <li><strong>Rust Application:</strong> Manages user input, handles GPU and CPU computations, and displays the results.</li>
          <li><strong>BigInt:</strong> Utilizes the <code>num-bigint</code> crate to handle Fibonacci numbers larger than what the GPU can compute.</li>
      </ul>
  </strong>
  
  <h3>Prerequisites</h3>
  <strong>
      Before running the project, ensure that you have the following installed:
      <ul>
          <li><strong>Rust:</strong> Install Rust using <a href="https://rustup.rs/">rustup</a>.</li>
          <li><strong>OpenCL:</strong> Ensure that you have an OpenCL runtime installed. This may include drivers for your GPU or a CPU-based OpenCL runtime.</li>
          <li><strong>OpenCL SDK:</strong> Some systems may require an OpenCL SDK for development. Check your GPU manufacturer’s website for installation instructions.</li>
          <li><strong>Clang:</strong> For compiling OpenCL kernels (if needed).</li>
      </ul>
  </strong>
  
  <h3>Quick Start</h3>
  <strong>
      <ol>
          <li><strong>Clone the Repository:</strong> <code>git clone https://github.com/idevanshu/GPU-Accelerated-Fibonacci-Calculator.git</code></li>
          <li><strong>Navigate to the Project Directory:</strong> <code>cd your-repo</code></li>
          <li><strong>Build the Project & Run it:</strong> <code>cargo run build</code></li>
      </ol>
  </strong>

