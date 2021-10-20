all: libs

out:
	mkdir -p out

out/mylib.o: mylib.c mylib.h out
	clang $(LDFLAGS) -o out/mylib.o -c mylib.c
	# must be clang to support LTO with Rust

out/libmylib.a: out/mylib.o
	ar rcs out/libmylib.a out/mylib.o

libs: out/libmylib.a

clean:
	rm -rf out
