int main() {
    int a = 5;
    int b = 7;
    
    printf("a = %d, b = %d\n", a, b);
    printf("a + b = %d\n", a + b);
    printf("a - b = %d\n", a - b);
    printf("a * b = %d\n", a * b);
    printf("a / b = %d\n", a / b);
    
    int c = add(a, b);
    printf("add(a, b) = %d\n", c);
    
    if (a > b) {
        printf("a is greater than b\n");
    } else {
        printf("a is not greater than b\n");
    }
    
    printf("Counting:\n");
    int i = 0;
    while (i < 5) {
        printf("%d\n", i);
        i = i + 1;
    }
    
    printf("\nFirst 10 Fibonacci numbers:\n");
    int fib[10] = {0, 1, 1, 2, 3, 5, 8, 13, 21, 34};
    for (i = 0; i < 10; i = i + 1) {
        printf("%d ", fib[i]);
    }
    printf("\n\n");
    
    // Pointers
    int *ptr = &a;
    printf("Value of a through pointer: %d\n", *ptr);
    
    *ptr = 10;
    printf("Changed a through pointer: %d\n", a);
    
    // Array
    int arr[5] = {10, 20, 30, 40, 50};
    printf("Array elements:\n");
    for (i = 0; i < 5; i = i + 1) {
        printf("%d ", arr[i]);
    }
    printf("\n\n");
    
    return 0;
} 