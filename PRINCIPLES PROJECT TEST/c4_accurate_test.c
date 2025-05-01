// c4_accurate_test.c - Accurate test of what works in the C4 interpreter

// FEATURES THAT ACTUALLY WORK:
// - Basic variable declarations and initializations
// - Simple arithmetic operations (+, -, *, /)
// - Some operator precedence (a + b * 2)
// - Simple function calls (add function)
// - Basic if statements (one branch)
// - Basic global variable usage

// FEATURES THAT DON'T WORK:
// - Array operations
// - Complex function calls with multiple parameters
// - Nested expressions with parentheses
// - If-else statements (both branches execute)

// Global variables - WORKS
int global1 = 100;
int global2 = 200;

// Simple function declaration - WORKS
int add(int x, int y) {
    return x + y;
}

// Function that doesn't work
int complex_calc(int a, int b, int c) {
    return (a + b) * c;
}

int main() {
    printf("=== ACCURATE C4 INTERPRETER TEST ===\n");
    
    printf("\n--- WORKING FEATURES ---\n");
    
    // Basic variables - WORKS
    printf("\n[WORKS] Basic variables:\n");
    int a = 5;
    int b = 10;
    char c = 'X';
    printf("a = %d, b = %d, c = %c\n", a, b, c);
    
    // Simple arithmetic - WORKS
    printf("\n[WORKS] Basic arithmetic:\n");
    printf("a + b = %d\n", a + b);     // 15
    printf("b - a = %d\n", b - a);     // 5
    printf("a * b = %d\n", a * b);     // 50
    printf("b / a = %d\n", b / a);     // 2
    
    // Global variables - WORKS
    printf("\n[WORKS] Global variables:\n");
    printf("global1 = %d, global2 = %d\n", global1, global2);
    
    // Basic function call - WORKS
    printf("\n[WORKS] Simple function call:\n");
    printf("add(a, b) = %d\n", add(a, b));   // 15
    
    // Simple if statement - WORKS
    printf("\n[WORKS] Simple if statement:\n");
    if (a < b) {
        printf("a is less than b\n");
    }
    
    // Operator precedence - SURPRISINGLY WORKS
    printf("\n[WORKS] Operator precedence:\n");
    int precedence = a + b * 2;
    printf("a + b * 2 = %d (correct: should be 25)\n", precedence);
    
    printf("\n--- FEATURES THAT DON'T WORK ---\n");
    
    // If-else statement - BROKEN (both branches execute)
    printf("\n[BROKEN] If-else statement:\n");
    if (a < b) {
        printf("TRUE branch: a is less than b (should show)\n");
    } else {
        printf("FALSE branch: a is not less than b (should NOT show)\n");
    }
    
    // Arrays - BROKEN
    printf("\n[BROKEN] Array operations:\n");
    int arr[5];
    int i = 0;
    while (i < 5) {
        arr[i] = i * 3;
        i = i + 1;
    }
    
    printf("Array values: ");
    i = 0;
    while (i < 5) {
        printf("%d ", arr[i]);
        i = i + 1;
    }
    printf("(Should show: 0 3 6 9 12)\n");
    
    // Complex function - BROKEN
    printf("\n[BROKEN] Complex function call:\n");
    int result = complex_calc(2, 3, 4);
    printf("complex_calc(2, 3, 4) = %d (should be 20)\n", result);
    
    // Nested parenthesized expression - BROKEN
    printf("\n[BROKEN] Nested expression:\n");
    int expr_result = (a + b) * (a - 2);
    printf("(a + b) * (a - 2) = %d (should be 45)\n", expr_result);
    
    printf("\n=== TEST COMPLETE ===\n");
    return 0;
} 