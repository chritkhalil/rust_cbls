// // # Read n(number of instances) W(weight matrix) and D(distance matrix)

// instance_name = "had12.dat" // bks = 1652, tai12a bks = 224416
// instance_file = "/Users/pro/development/CSP/instances/" * instance_name

// function ReadInstance(file::String="/Users/pro/development/CSP/instances/tai12a.dat")
//     @info "Reading QAP instance.."
//     println(file)
//     println()
//     instance = readdlm(file);
//     n = instance[1,1];
//     W = instance[2:n+1,1:n];
//     D = instance[n+2:2n+1,1:n];
//     W = convert(Matrix{Int64}, W);
//     D = convert(Matrix{Int64}, D);
//     return (n, W, D);
// end