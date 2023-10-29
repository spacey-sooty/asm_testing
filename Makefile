compile:
	mkdir -p build
	cargo build
	gcc -o build/add.o -c asm/add.S
	gcc -o build/minus.o -c asm/minus.S
	gcc -o build/main.o -c C/main.c

clean:
	rm *.o
	rm *.out
	rm target -rf
	rm build -rf

