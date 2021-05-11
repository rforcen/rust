GCC=g++
SOURCES=fast_hsv2rgb_32bit.c fast_hsv2rgb_8bit.c
OBJS=fast_hsv2rgb_32bit.o fast_hsv2rgb_8bit.o

lib: $(SOURCES)
	g++ -c $(SOURCES)
	rm -f libhsv.a
	ar rsc libhsv.a $(OBJS)
	rm -f $(OBJS)

