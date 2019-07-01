all: thread_sched_yield.c process_sched_yield.c
	gcc -pthread -o thread_sched_yield thread_sched_yield.c
	gcc -o process_sched_yield process_sched_yield.c

clean:
	rm thread_sched_yield
	rm process_sched_yield
