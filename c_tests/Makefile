hello: hello.o hello-tp.o
	gcc -o hello hello.o hello-tp.o -llttng-ust -ldl

hello-tp.o: hello-tp.h hello-tp.c
	gcc -c -I. hello-tp.c

hello.o: hello.c hello-tp.h
	gcc -c hello.c

.PHONY: hello
