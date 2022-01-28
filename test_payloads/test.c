#define BASE 0x80000000

int get_the_answer() {
	volatile int *ptr = (int *)BASE;
	*ptr = 42;

	return 42;
}

__attribute__((section(".text.init")))
int main() {
	int a = get_the_answer();
}