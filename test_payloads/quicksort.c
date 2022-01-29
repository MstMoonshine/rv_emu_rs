// Quick sort in C

// function to swap elements
void swap(int* a, int* b)
{
    int t = *a;
    *a = *b;
    *b = t;
}

// function to find the partition position
int partition(int array[], int low, int high)
{

    // select the rightmost element as pivot
    int pivot = array[high];

    // pointer for greater element
    int i = (low - 1);

    // traverse each element of the array
    // compare them with the pivot
    for (int j = low; j < high; j++) {
        if (array[j] <= pivot) {

            // if element smaller than pivot is found
            // swap it with the greater element pointed by i
            i++;

            // swap element at i with element at j
            swap(&array[i], &array[j]);
        }
    }

    // swap the pivot element with the greater element at i
    swap(&array[i + 1], &array[high]);

    // return the partition point
    return (i + 1);
}

void quickSort(int array[], int low, int high)
{
    if (low < high) {

        // find the pivot element such that
        // elements smaller than pivot are on left of pivot
        // elements greater than pivot are on right of pivot
        int pi = partition(array, low, high);

        // recursive call on the left of pivot
        quickSort(array, low, pi - 1);

        // recursive call on the right of pivot
        quickSort(array, pi + 1, high);
    }
}

#define BASE            0x80000000
#define ARRAY_BEFORE    (BASE + 0x0)
#define ARRAY_AFTER     (BASE + 0x30)

// function to print array elements
void printArray(int array[], int size, int sorted)
{
    int *ptr;

    if (!sorted) {
        ptr = (int *)ARRAY_BEFORE;
    } else {
        ptr = (int *)ARRAY_AFTER;
    }

    for (int i = 0; i < size; ++i) {
        *ptr++ = array[i];
    }
}

#define LEN 7

// main function
__attribute__((section(".text.init")))
int main()
{
    // Value assignment done in stack to avoid usage of data section.
    // int data[] = { 8, 7, 2, 1, 0, 9, 6 };
    int data[LEN];
    data[0] = 0xaf37be7;
    data[1] = 0x7dd0d4bf;
    data[2] = 0x4994fe31;
    data[3] = 0x7e6186cf;
    data[4] = 0x38e1d337;
    data[5] = 0x2e9548eb;
    data[6] = 0x1cbd0f06;

    int n = sizeof(data) / sizeof(data[0]);

    printArray(data, n, 0);

    // perform quicksort on data
    quickSort(data, 0, n - 1);

    printArray(data, n, 1);
}