// Comprehensive test program for C4 Rust implementation
// Tests: functions, local variables, conditionals, loops, and pointers

// Function with parameters and return value
int add(int x, int y) {
    return x + y;
}

// Function with local vars and a pointer parameter
int modify(int *value) {
    *value = *value + 10;
    return *value;
}

// Function with conditional logic
int max(int a, int b) {
    if (a > b) {
        return a;
    } else {
        return b;
    }
}

// Function that demonstrates a loop
int sum_to_n(int n) {
    int sum;
    int i;
    
    sum = 0;
    i = 1;
    while (i <= n) {
        sum = sum + i;
        i = i + 1;
    }
    
    return sum;
}

// Main function to test everything
int main() {
    int result;
    int value;
    int n1;
    int n2;
    int n3;
    int n4;
    int n5;
    int *ptr;
    
    // Test basic variable assignment
    result = 42;
    printf("Initial value: %d\n", result);
    
    // Test function call
    result = add(10, 20);
    printf("After add function: %d\n", result);
    
    // Test conditional function
    result = max(15, 25);
    printf("Max of 15 and 25: %d\n", result);
    
    // Test loop function
    result = sum_to_n(5);
    printf("Sum from 1 to 5: %d\n", result);
    
    // Test pointer dereferencing
    value = 100;
    printf("Before modify: %d\n", value);
    
    // Pass address to function
    result = modify(&value);
    printf("After modify: %d (returned %d)\n", value, result);
    
    // Test simple pointer usage
    n1 = 10;
    n2 = 20;
    n3 = 30;
    n4 = 40;
    n5 = 50;
    
    // Initialize pointer
    ptr = &n1;
    printf("First value via pointer: %d\n", *ptr);
    
    // Access different variables via pointers
    ptr = &n2;
    printf("Second value via pointer: %d\n", *ptr);
    
    ptr = &n3;
    printf("Third value via pointer: %d\n", *ptr);
    
    // Test pointer assignment
    *ptr = 35;
    printf("Updated third value: %d\n", n3);
    
    return result;
} 