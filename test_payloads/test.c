#define BASE 0x80000000

int get_the_answer() {
	volatile int *ptr = (int *)BASE;
	*ptr++ = 42;

	// for (int i = 0; i < 15; i++) {
	// 	*ptr++ = 0x11223344 + i;
	// }
	int val = 0x11223344;

	*ptr++ = val++;
	*ptr++ = val++;
	*ptr++ = val++;
	*ptr++ = val++;
	*ptr++ = val++;

	val = 0x11223344;

	if (val > 0x11223344) {
		*ptr++ = 0xdeadbeef;
	}

	if (val < 0x11223344) {
		*ptr++ = 0xdead1;
	}

	if (val == 0x11223344) {
		*ptr++ = 0xcafe2;
	}

	val = 0xffff0000;

	if (val > -1) {
		*ptr++ = 0xcafe3;
	}

	if (val < -1) {
		*ptr++ = 0xcafe4;
	}

	if (val == -1) {
		*ptr++ = 0xcafe5;
	}

	unsigned int unval = (unsigned int)val;

	if (unval > 0xfff00000) {
		*ptr++ = 0xcafe6;
	}

	if (unval < 0xfff00000) {
		*ptr++ = 0xcafe7;
	}

	if (unval == 0xfff00000) {
		*ptr++ = 0xcafe8;
	}

	return 42;
}

int test_loop()
{
	volatile int *ptr = (int *)(BASE + 0x50);

	for (int i = 0; i < 10; i++) {
		*ptr++ = (0xcafe << 4) + i;
	}

	return 0;
}

__attribute__((section(".text.init")))
int main() {
	int a = get_the_answer();
	int b = test_loop();
}