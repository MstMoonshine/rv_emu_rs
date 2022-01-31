#define BASE 0x80001000

int get_the_answer() {
	volatile int *ptr = (int *)BASE;
	*ptr= 42;

	// asm volatile("addi x0, x1, 0x123");

	return 42;
}

int test_loop()
{
	volatile int *ptr = (int *)(BASE);

	for (int i = 0; i < 10; i++) {
		*ptr++ = (0xcafe << 4) + i;
	}

	return 0;
}

int main() {
	int a = get_the_answer();
	int b = test_loop();
}