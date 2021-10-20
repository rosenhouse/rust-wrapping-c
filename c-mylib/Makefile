all: libs

out:
	mkdir -p out

out/mylib.o: mylib.c mylib.h out
	cc $(LDFLAGS) -o out/mylib.o -c mylib.c

out/libmylib.a: out/mylib.o
	ar rcs out/libmylib.a out/mylib.o

libs: out/libmylib.a

clean:
	rm -f out/*.o out/*.a
