CC=gcc
CFLAGS=-I.
LIB=../target/debug/liblexibook_ffi.a

main: main.o
	$(CC) -o test $(CFLAGS) main.o $(LIB) 

main.o: main.c
	$(CC) -o main.o main.c -c $(CFLAGS)
