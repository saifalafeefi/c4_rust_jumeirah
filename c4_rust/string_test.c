#include <stdio.h>

int main() {
    // Test escapes in string literals
    // Note: C4/our implementation might not handle all standard escapes
    char *str1 = "Escapes: \n \t \\ \" \0 end"; 
    printf("Escape Test: %s\n", str1);
    
    // Test string array operations
    char str2[50];
    str2[0] = 'H';
    str2[1] = 'e';
    str2[2] = 'l';
    str2[3] = 'l';
    str2[4] = 'o';
    str2[5] = ' ';
    str2[6] = 'S';
    str2[7] = 't';
    str2[8] = 'a';
    str2[9] = 'c';
    str2[10] = 'k';
    str2[11] = '!';
    str2[12] = '\n'; // Newline at the end
    str2[13] = 0;    // Null terminator
    
    printf("Array string: %s", str2);
    
    // Test printf with multiple arguments including string
    char *middle_str = "middle";
    printf("Multiple args: %d %s %d\n", 123, middle_str, 456);
    
    return 0;
} 