// simple_test_c4.c - Basic test for c4 compiler

// Enum
enum { FALSE, TRUE };

// Global variable
int global_var;

// Function declarations
int add(int x, int y);
int fibonacci(int n);

int main()
{
    // Variable declarations
    int a;
    int b;
    char c;
    
    // Initialization
    a = 10;
    b = 5;
    c = 'A';
    global_var = 100;
    
    // Print values
    printf("a = %d, b = %d, c = %c, global = %d\n", a, b, c, global_var);
    
    // Simple arithmetic
    printf("a + b = %d\n", a + b);
    printf("a - b = %d\n", a - b);
    printf("a * b = %d\n", a * b);
    printf("a / b = %d\n", a / b);
    
    // Function call
    printf("add(a, b) = %d\n", add(a, b));
    
    // If statement
    if (a > b) {
        printf("a is greater than b\n");
    } else {
        printf("a is not greater than b\n");
    }
    
    // While loop
    int i;
    i = 1;
    printf("Counting: ");
    while (i <= 5) {
        printf("%d ", i);
        i = i + 1;
    }
    printf("\n");
    
    // Fibonacci sequence
    printf("First 10 Fibonacci numbers: ");
    i = 0;
    while (i < 10) {
        printf("%d ", fibonacci(i));
        i = i + 1;
    }
    printf("\n");
    
    // Pointers
    int *ptr;
    ptr = &a;
    printf("Value of a through pointer: %d\n", *ptr);
    *ptr = 20;
    printf("Changed a through pointer: %d\n", a);
    
    // Simple array
    int arr[5];
    i = 0;
    while (i < 5) {
        arr[i] = i * 2;
        i = i + 1;
    }
    
    printf("Array elements: ");
    i = 0;
    while (i < 5) {
        printf("%d ", arr[i]);
        i = i + 1;
    }
    printf("\n");
    
    return 0;
}

// Simple addition function
int add(int x, int y)
{
    return x + y;
}

// Fibonacci function
int fibonacci(int n)
{
    if (n <= 1) return n;
    return fibonacci(n-1) + fibonacci(n-2);
} 