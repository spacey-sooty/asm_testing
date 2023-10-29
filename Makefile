compile:
	mkdir -p build
	echo "Building Rust"
	cargo build
	echo "Building Assembly"
	gcc -o build/add.o -c asm/add.S
	gcc -o build/minus.o -c asm/minus.S
	echo "Building C"
	gcc -o build/main.o -c C/main.c
	echo "Linking C and Assembly"
	gcc -o build/main build/main.o build/minus.o build/add.o
	chmod +x build/main

clean:
	rm *.o
	rm *.out
	rm target -rf
	rm build -rf

run: compile
	echo "Rust Output :"
	cargo run
	echo "C Output :"
	./build/main
	
