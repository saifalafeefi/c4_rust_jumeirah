int main() {
    // Test escapes in string literals
    char *str1 = "This has escapes: \n \t \\ \' \" \0 end";
    printf("Escape Test: %s\n", str1);
    
    // Test string array operations
    char str2[50];
    str2[0] = 'T';
    str2[1] = 'e';
    str2[2] = 's';
    str2[3] = 't';
    str2[4] = ' ';
    str2[5] = 'a';
    str2[6] = 'r';
    str2[7] = 'r';
    str2[8] = 'a';
    str2[9] = 'y';
    str2[10] = '\0';
    
    printf("Array string: %s\n", str2);
    
    // Test printf with multiple arguments
    printf("Multiple args: %d %s %d\n", 123, "middle", 456);
    
    return 0;
} 