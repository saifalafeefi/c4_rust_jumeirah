int main() {
    char msg[15];
    msg[0] = 'H';
    msg[1] = 'e';
    msg[2] = 'l';
    msg[3] = 'l';
    msg[4] = 'o';
    msg[5] = 0;
    printf("%s world!\n", msg);
    
    char *str = "Hello world!";
    printf("%s\n", str);
    return 0;
} 