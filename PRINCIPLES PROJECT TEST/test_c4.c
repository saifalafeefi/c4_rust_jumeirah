// test_c4.c - Test program for c4 compiler
// Demonstrates basic functionality within c4's limitations

// Enums
enum { FALSE, TRUE };
enum Color { RED = 10, GREEN, BLUE };

// Global variables
int global_var;
char global_char;
int *global_ptr;

// Function declaration
int fibonacci(int n);
void print_array(int *arr, int size);
int factorial(int n);

// Main function
int main()
{
    // Basic variable declarations
    int a; 
    int b;
    char c;
    
    // Variable initialization
    a = 42;
    b = 10;
    c = 'X';
    
    // Basic arithmetic
    int sum = a + b;
    int diff = a - b;
    int product = a * b;
    int quotient = a / b;
    int remainder = a % b;
    
    // Output
    printf("---- Basic Operations ----\n");
    printf("a = %d, b = %d, c = %c\n", a, b, c);
    printf("Sum: %d\n", sum);
    printf("Difference: %d\n", diff);
    printf("Product: %d\n", product);
    printf("Quotient: %d\n", quotient);
    printf("Remainder: %d\n", remainder);
    
    // Comparison operators
    printf("\n---- Comparisons ----\n");
    printf("a == b: %d\n", a == b);
    printf("a != b: %d\n", a != b);
    printf("a > b: %d\n", a > b);
    printf("a < b: %d\n", a < b);
    printf("a >= b: %d\n", a >= b);
    printf("a <= b: %d\n", a <= b);
    
    // Logical operators
    printf("\n---- Logical Operators ----\n");
    printf("TRUE && TRUE: %d\n", TRUE && TRUE);
    printf("TRUE && FALSE: %d\n", TRUE && FALSE);
    printf("TRUE || FALSE: %d\n", TRUE || FALSE);
    printf("!TRUE: %d\n", !TRUE);
    
    // Bitwise operators
    printf("\n---- Bitwise Operators ----\n");
    printf("a & b: %d\n", a & b);
    printf("a | b: %d\n", a | b);
    printf("a ^ b: %d\n", a ^ b);
    printf("~a: %d\n", ~a);
    printf("a << 2: %d\n", a << 2);
    printf("a >> 2: %d\n", a >> 2);
    
    // Control Flow: if-else
    printf("\n---- Control Flow: if-else ----\n");
    if (a > b) {
        printf("a is greater than b\n");
    } else {
        printf("a is not greater than b\n");
    }
    
    // Control Flow: while loop
    printf("\n---- Control Flow: while ----\n");
    int i = 0;
    while (i < 5) {
        printf("i = %d\n", i);
        i = i + 1;
    }
    
    // Function calls
    printf("\n---- Function Calls ----\n");
    printf("Fibonacci(10): %d\n", fibonacci(10));
    printf("Factorial(5): %d\n", factorial(5));
    
    // Pointers and Arrays
    printf("\n---- Pointers and Arrays ----\n");
    int arr[5];
    int j = 0;
    while (j < 5) {
        arr[j] = j * j;
        j = j + 1;
    }
    print_array(arr, 5);
    
    // Dynamic memory allocation
    printf("\n---- Dynamic Memory ----\n");
    int *dynamic_arr;
    dynamic_arr = malloc(5 * sizeof(int));
    j = 0;
    while (j < 5) {
        dynamic_arr[j] = j + 10;
        j = j + 1;
    }
    print_array(dynamic_arr, 5);
    free(dynamic_arr);
    
    // Enum usage
    printf("\n---- Enums ----\n");
    printf("RED: %d, GREEN: %d, BLUE: %d\n", RED, GREEN, BLUE);
    
    return 0;
}

// Fibonacci implementation
int fibonacci(int n)
{
    if (n <= 1) return n;
    return fibonacci(n-1) + fibonacci(n-2);
}

// Function to print an array
void print_array(int *arr, int size)
{
    int i = 0;
    printf("Array: ");
    while (i < size) {
        printf("%d ", arr[i]);
        i = i + 1;
    }
    printf("\n");
}

// Factorial implementation
int factorial(int n)
{
    if (n <= 1) return 1;
    return n * factorial(n - 1);
} 