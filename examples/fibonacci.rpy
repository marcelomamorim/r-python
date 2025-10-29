def fibonacci(n: Int) -> Int:
    if n <= 1:
        return n;
    end;
    return fibonacci(n - 1) + fibonacci(n - 2);
end;

val target = 6;
val value = fibonacci(target);
asserttrue(value == 8, "Fibonacci incorreto");
