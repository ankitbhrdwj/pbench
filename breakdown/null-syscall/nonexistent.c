#include <stdio.h>
#include <unistd.h>
#include <sys/syscall.h>

static __inline__ unsigned long long rdtsc(void)
{
	unsigned hi, lo;
	__asm__ __volatile__ ("rdtsc" : "=a"(lo), "=d"(hi));
	return ( (unsigned long long)lo)|( ((unsigned long long)hi)<<32 );
}

void main() {
	unsigned long long start = rdtsc();
	for(int i = 0; i < 100000; i++)
	{
		syscall(10000000);
	}
	unsigned long long stop = rdtsc();

	printf("Per syscall %lld\n", (stop - start) / 100000);
}

