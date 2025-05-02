// test for C4 Rust
// tests functions, vars, conditionals, loops, pointers

// adds two numbers
int add(int x, int y) {
    return x + y;
}

// modifies via pointer
int modify(int *value) {
    *value = *value + 10;
    return *value;
}

// returns larger value
int max(int a, int b) {
    if (a > b) {
        return a;
    } else {
        return b;
    }
}

// sums from 1 to n
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

// tests everything
int main() {
    int result;
    int value;
    int n1;
    int n2;
    int n3;
    int n4;
    int n5;
    int *ptr;
    
    // set initial value
    result = 42;
    printf("Initial value: %d\n", result);
    
    // call function
    result = add(10, 20);
    printf("After add function: %d\n", result);
    
    // test conditional
    result = max(15, 25);
    printf("Max of 15 and 25: %d\n", result);
    
    // test loop
    result = sum_to_n(5);
    printf("Sum from 1 to 5: %d\n", result);
    
    // test pointer use
    value = 100;
    printf("Before modify: %d\n", value);
    
    // pass address
    result = modify(&value);
    printf("After modify: %d (returned %d)\n", value, result);
    
    // setup for pointers
    n1 = 10;
    n2 = 20;
    n3 = 30;
    n4 = 40;
    n5 = 50;
    
    // set pointer
    ptr = &n1;
    printf("First value via pointer: %d\n", *ptr);
    
    // change pointer
    ptr = &n2;
    printf("Second value via pointer: %d\n", *ptr);
    
    ptr = &n3;
    printf("Third value via pointer: %d\n", *ptr);
    
    // modify through pointer
    *ptr = 35;
    printf("Updated third value: %d\n", n3);
    
    return result;
} 