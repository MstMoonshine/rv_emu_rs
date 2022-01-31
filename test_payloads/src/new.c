int add(int a, int b)
{
	return a + b;
}

int main()
{
	int a = 1;
	int b = 2;

	int c = add(a, b);

	int *ptr = (int *)0x80000000; // start of DRAM
	*ptr = c;
}