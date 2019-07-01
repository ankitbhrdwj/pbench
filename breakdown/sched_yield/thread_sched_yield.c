#include <sched.h>
#include <pthread.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const int iterations = 1000000;

static __inline__ unsigned long long rdtsc(void)
{
	unsigned hi, lo;
	__asm__ __volatile__ ("rdtsc" : "=a"(lo), "=d"(hi));
	return ( (unsigned long long)lo)|( ((unsigned long long)hi)<<32 );
}

void* thread() {
	for (int i = 0; i < iterations; i++) {
		sched_yield();
	}
	return NULL;
}

int main(void) {
	pthread_t thread_id;
	unsigned long long start = rdtsc();
	if (pthread_create(&thread_id, NULL, thread, NULL)) {
		return 1;
	}

	if (pthread_join(thread_id, NULL)) {
		fprintf(stderr, "pthread_join");
	}
	unsigned long long stop = rdtsc();

	printf("Thread ctx %f\n", (stop - start)/ (float)iterations);
	return 0;
}
