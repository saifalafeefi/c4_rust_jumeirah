// Test combining different control flow constructs

int main() {
    int i;
    int j;
    
    // Nested if-else inside loops
    printf("Nested control flow test:\n");
    
    // Outer for loop
    for (i = 0; i < 3; i = i + 1) {
        printf("Outer loop i = %d\n", i);
        
        // Inner while loop
        j = 0;
        while (j < 3) {
            printf("  Inner loop j = %d: ", j);
            
            if (i == j) {
                printf("i equals j\n");
            } else if (i > j) {
                printf("i greater than j\n");
            } else {
                printf("i less than j\n");
            }
            
            j = j + 1;
        }
    }
    
    return 0;
} 