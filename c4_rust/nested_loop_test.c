// Test with nested loops

int main() {
    int i;
    int j;
    
    printf("Nested loop test:\n");
    
    // Outer for loop
    for (i = 0; i < 3; i = i + 1) {
        printf("Outer loop i = %d\n", i);
        
        // Inner while loop
        j = 0;
        while (j < 3) {
            printf("  Inner loop j = %d\n", j);
            j = j + 1;
        }
    }
    
    return 0;
} 